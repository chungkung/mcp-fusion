use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "tauri-runtime")]
use tauri::{Emitter, WebviewWindow};
use tokio::sync::Mutex;
#[cfg(feature = "tauri-runtime")]
use tokio::sync::Semaphore;

use crate::gateway::McpClient;
use crate::storage::sqlite::{Database, WorkflowEdge, WorkflowNode};
#[cfg(feature = "tauri-runtime")]
use crate::storage::sqlite::Workflow;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex as StdMutex;
use std::sync::OnceLock;

// ============================================================
// Per-workflow 中止标志（支持多工作流并发）
// ============================================================

fn abort_flags() -> &'static StdMutex<HashMap<String, Arc<AtomicBool>>> {
    static FLAGS: OnceLock<StdMutex<HashMap<String, Arc<AtomicBool>>>> = OnceLock::new();
    FLAGS.get_or_init(|| StdMutex::new(HashMap::new()))
}

/// 注册工作流，返回该工作流的中止标志
pub(crate) fn register_workflow(workflow_id: &str) -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    let mut flags = abort_flags().lock().unwrap();
    flags.insert(workflow_id.to_string(), flag.clone());
    crate::metrics::set_active_workflows(flags.len() as i64);
    flag
}

/// 注销工作流，清理中止标志
pub(crate) fn unregister_workflow(workflow_id: &str) {
    let mut flags = abort_flags().lock().unwrap();
    flags.remove(workflow_id);
    crate::metrics::set_active_workflows(flags.len() as i64);
}

/// 请求中止指定工作流
pub fn request_abort(workflow_id: &str) {
    let flags = abort_flags().lock().unwrap();
    if let Some(flag) = flags.get(workflow_id) {
        flag.store(true, Ordering::SeqCst);
    }
}

/// 请求中止所有正在运行的工作流
pub fn request_abort_all() {
    let flags = abort_flags().lock().unwrap();
    for flag in flags.values() {
        flag.store(true, Ordering::SeqCst);
    }
}



/// 检查是否有任何工作流正在运行
pub fn is_running() -> bool {
    let flags = abort_flags().lock().unwrap();
    !flags.is_empty()
}

/// 工作流守卫：在 Drop 时自动调用 unregister_workflow，
/// 防止 panic 导致 abort flag 泄漏。
struct WorkflowGuard {
    workflow_id: String,
}

impl WorkflowGuard {
    fn new(workflow_id: &str) -> Self {
        Self {
            workflow_id: workflow_id.to_string(),
        }
    }
}

impl Drop for WorkflowGuard {
    fn drop(&mut self) {
        unregister_workflow(&self.workflow_id);
    }
}
// ============================================================
// 执行模式
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExecutionMode {
    /// 串行执行：逐节点执行
    Serial,
    /// 并行执行：同一层的节点并发执行
    Parallel,
    /// 条件分支：根据节点输出决定下游路径
    Conditional,
    /// 循环执行：重复执行指定节点 N 次
    Loop { count: u32 },
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::Parallel
    }
}

// ============================================================
// 节点执行状态（与前端 RunStatus 对应）
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeState {
    Idle,
    Running,
    Success,
    Failed,
    Skipped,
    Timeout,
}

// ============================================================
// 调度器配置
// ============================================================

#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    /// 单节点执行超时时间
    pub timeout: Duration,
    /// 最大并发数（Parallel 模式）
    pub max_concurrency: usize,
    /// 执行模式
    pub mode: ExecutionMode,
    /// 是否在节点失败时中断整个工作流
    pub abort_on_error: bool,
    /// 工具调用失败最大重试次数
    pub retry_count: u32,
    /// 重试退避时间（毫秒）
    pub retry_backoff_ms: u64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_concurrency: 4,
            mode: ExecutionMode::Parallel,
            abort_on_error: false,
            retry_count: 3,
            retry_backoff_ms: 500,
        }
    }
}

// ============================================================
// 执行结果
// ============================================================

/// 单个节点执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeExecutionResult {
    pub node_id: String,
    pub state: NodeState,
    pub output: Option<Value>,
    pub error: Option<String>,
    /// 节点执行耗时（毫秒）
    pub duration_ms: u64,
}

/// 工作流执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionResult {
    pub workflow_id: String,
    pub state: NodeState,
    pub node_results: Vec<NodeExecutionResult>,
    /// 总耗时（毫秒）
    pub total_duration_ms: u64,
    /// 错误信息（拓扑错误等）
    pub error: Option<String>,
}

/// 推送给前端的节点状态事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStateEvent {
    pub workflow_id: String,
    pub node_id: String,
    pub state: NodeState,
    pub output: Option<Value>,
    pub error: Option<String>,
    pub trace_id: String,
}

// ============================================================
// 拓扑排序：按依赖层级分组
// ============================================================

/// 使用 Kahn 算法将节点按依赖层级分组。
///
/// 同一层的节点相互之间没有依赖，可以并行执行。
/// 如果存在循环依赖或无效边，返回错误。
pub fn topological_layers(node_ids: &[String], edges: &[WorkflowEdge]) -> Result<Vec<Vec<String>>> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut invalid_edge_count = 0;

    // 初始化所有节点
    for id in node_ids {
        in_degree.entry(id.as_str()).or_insert(0);
        adjacency.entry(id.as_str()).or_default();
    }

    // 使用 HashSet 实现 O(1) 节点查找（替代 O(n) 的 slice::contains）
    let node_set: HashSet<&str> = node_ids.iter().map(|s| s.as_str()).collect();

    // 计算入度，同时统计无效边
    for edge in edges {
        if node_set.contains(edge.source.as_str()) && node_set.contains(edge.target.as_str()) {
            *in_degree.entry(edge.target.as_str()).or_insert(0) += 1;
            adjacency
                .entry(edge.source.as_str())
                .or_default()
                .push(edge.target.as_str());
        } else {
            invalid_edge_count += 1;
        }
    }

    // 存在无效边直接返回配置错误
    if invalid_edge_count > 0 {
        return Err(anyhow!(
            "工作流包含 {} 条无效边，存在源或目标节点不存在的连接",
            invalid_edge_count
        ));
    }

    // 零入度节点入队
    let mut queue: VecDeque<&str> = VecDeque::new();
    for (node, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(node);
        }
    }

    let mut layers: Vec<Vec<String>> = Vec::new();
    let mut sorted_count = 0;

    while !queue.is_empty() {
        // 当前层：所有入度为 0 的节点
        let layer: Vec<String> = queue.iter().map(|s| s.to_string()).collect();
        sorted_count += layer.len();
        layers.push(layer);

        let current_size = queue.len();
        for _ in 0..current_size {
            if let Some(node) = queue.pop_front() {
                if let Some(neighbors) = adjacency.get(node) {
                    for &neighbor in neighbors {
                        let deg = in_degree
                            .get_mut(neighbor)
                            .expect("neighbor must exist in in_degree");
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }

    if sorted_count != node_ids.len() {
        return Err(anyhow!("工作流存在循环依赖，无法执行"));
    }

    Ok(layers)
}

/// 收集某个节点的所有下游节点（迭代，避免递归栈溢出）
fn collect_downstream(
    node_id: &str,
    adjacency: &HashMap<&str, Vec<&str>>,
    visited: &mut HashSet<String>,
) {
    let mut stack = vec![node_id];
    while let Some(current) = stack.pop() {
        if !visited.insert(current.to_string()) {
            continue;
        }
        if let Some(neighbors) = adjacency.get(current) {
            for &neighbor in neighbors {
                stack.push(neighbor);
            }
        }
    }
}

// ============================================================
// 数据传递：解析输入中的模板引用
// ============================================================

/// 将输入中的 `${node_id}` 或 `${node_id.field.sub_field}` 引用
/// 替换为上游节点的实际输出值。
///
/// 支持嵌套字段解析与 Object / Array 递归遍历。
/// 支持部分字符串插值，如 "Hello ${node1}" 或 "${a} and ${b}"。
pub fn resolve_inputs(inputs: &Value, outputs: &HashMap<String, Value>) -> Value {
    match inputs {
        Value::String(s) => {
            if s.contains("${") {
                let mut result = String::with_capacity(s.len());
                let mut rest = s.as_str();
                let mut modified = false;

                while let Some(dollar_pos) = rest.find("${") {
                    result.push_str(&rest[..dollar_pos]);
                    modified = true;

                    let after_dollar = &rest[dollar_pos + 2..];
                    if let Some(close_pos) = after_dollar.find('}') {
                        let ref_path = &after_dollar[..close_pos];
                        result.push_str(&resolve_ref_value(ref_path, outputs));
                        rest = &after_dollar[close_pos + 1..];
                    } else {
                        // 没有匹配的闭合括号，视为普通文本
                        result.push_str("${");
                        rest = &rest[dollar_pos + 2..];
                    }
                }
                result.push_str(rest);

                if modified {
                    Value::String(result)
                } else {
                    Value::String(s.clone())
                }
            } else {
                Value::String(s.clone())
            }
        }
        Value::Object(map) => {
            let mut resolved = serde_json::Map::new();
            for (k, v) in map {
                resolved.insert(k.clone(), resolve_inputs(v, outputs));
            }
            Value::Object(resolved)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(|v| resolve_inputs(v, outputs)).collect()),
        other => other.clone(),
    }
}

/// 解析单个 `${...}` 引用，返回替换后的字符串值
fn resolve_ref_value(ref_path: &str, outputs: &HashMap<String, Value>) -> String {
    let mut parts = ref_path.splitn(2, '.');
    if let Some(node_id) = parts.next() {
        if let Some(node_output) = outputs.get(node_id) {
            match parts.next() {
                Some(field_path) => {
                    let mut current = node_output;
                    for key in field_path.split('.') {
                        match current.get(key) {
                            Some(v) => current = v,
                            None => return format!("${{{}}}", ref_path),
                        }
                    }
                    return value_to_string(current);
                }
                None => return value_to_string(node_output),
            }
        }
    }
    format!("${{{}}}", ref_path)
}

/// 将 JSON Value 转换为字符串表示
fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

// ============================================================
// 调度器
// ============================================================

pub struct Scheduler {
    config: SchedulerConfig,
}

// `new` 方法无条件可用，不依赖 tauri 特性
impl Scheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        Self { config }
    }
}

#[cfg(feature = "tauri-runtime")]
impl Scheduler {
    /// 执行工作流，在节点状态变化时通过 Tauri 事件推送到前端。
    pub async fn execute(
        &self,
        db: &Database,
        workflow: &Workflow,
        window: Option<&WebviewWindow>,
        completed_node_ids: &[String],
        completed_outputs: &HashMap<String, Value>,
    ) -> WorkflowExecutionResult {
        let start = std::time::Instant::now();
        let workflow_id = workflow.id.clone();
        let trace_id = uuid::Uuid::new_v4().to_string();

        // OpenTelemetry: 创建工作流级 Span
        let workflow_span = tracing::info_span!(
            "workflow_execute",
            workflow_id = %workflow_id,
            otel.trace_id = %trace_id,
        );
        let _workflow_span_guard = workflow_span.enter();

        let completed_set: HashSet<String> = completed_node_ids.iter().cloned().collect();

        tracing::info!(
            trace_id = %trace_id,
            workflow_id = %workflow_id,
            completed_count = completed_set.len(),
            "开始执行工作流{}",
            if completed_set.is_empty() { "" } else { "（断点续传）" }
        );

        // 注册工作流，获取 per-workflow 中止标志
        let wf_abort_flag = register_workflow(&workflow_id);
        // 守卫：确保无论何种退出路径都会清理 abort flag
        let _guard = WorkflowGuard::new(&workflow_id);

        let node_ids: Vec<String> = workflow.nodes.iter().map(|n| n.id.clone()).collect();

        // 1. 拓扑排序
        let layers = match topological_layers(&node_ids, &workflow.edges) {
            Ok(l) => l,
            Err(e) => {
                return WorkflowExecutionResult {
                    workflow_id,
                    state: NodeState::Failed,
                    node_results: vec![],
                    total_duration_ms: start.elapsed().as_millis() as u64,
                    error: Some(e.to_string()),
                };
            }
        };

        // 2. 构建邻接表（用于失败时跳过下游），包装在 Arc 中避免并行模式大量克隆
        let adjacency = Arc::new(build_adjacency(&node_ids, &workflow.edges));

        // 3. 节点索引（一次性构建，所有模式共享），包装在 Arc 中避免并行模式大量克隆
        let node_map: Arc<HashMap<String, WorkflowNode>> = Arc::new(
            workflow
                .nodes
                .iter()
                .map(|n| (n.id.clone(), n.clone()))
                .collect(),
        );

        // 4. 共享状态
        let outputs: Arc<Mutex<HashMap<String, Value>>> = Arc::new(Mutex::new(HashMap::new()));
        let results: Arc<Mutex<Vec<NodeExecutionResult>>> = Arc::new(Mutex::new(Vec::new()));
        let failed_nodes: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
        let skip_set: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

        // ============================================================
        // 断点续传：将已完成的节点标记为跳过，并注入其输出
        // ============================================================
        {
            let mut skip = skip_set.lock().await;
            for node_id in completed_node_ids {
                skip.insert(node_id.clone());
            }
        }
        // 注入已完成节点的输出，供下游节点引用
        {
            let mut out = outputs.lock().await;
            for (node_id, value) in completed_outputs {
                out.insert(node_id.clone(), value.clone());
            }
        }
        // 预填充已完成节点的结果，防止错误的 skip 消息
        {
            let mut res = results.lock().await;
            for node_id in completed_node_ids {
                let output = completed_outputs.get(node_id).cloned();
                res.push(NodeExecutionResult {
                    node_id: node_id.clone(),
                    state: NodeState::Success,
                    output,
                    error: None,
                    duration_ms: 0,
                });
            }
        }

        let should_abort: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

        // 全局共享 MCP 客户端连接池，并行任务复用连接
        let client_cache: Arc<Mutex<HashMap<String, Arc<Mutex<McpClient>>>>> =
            Arc::new(Mutex::new(HashMap::new()));

        let mut overall_state = NodeState::Success;

        // 5. 按层执行
        for layer in &layers {
            // 重置本层中止标记
            *should_abort.lock().await = false;

            // 检查 per-workflow 中止标志（联动 runtime_stop）
            if wf_abort_flag.load(Ordering::SeqCst) {
                *should_abort.lock().await = true;
                overall_state = NodeState::Failed;
                break;
            }

            match self.config.mode {
                ExecutionMode::Serial => {
                    for node_id in layer {
                        let exec_result = self
                            .execute_single_node(
                                node_id,
                                &node_map,
                                db,
                                &adjacency,
                                &outputs,
                                &results,
                                &failed_nodes,
                                &skip_set,
                                &client_cache,
                                window,
                                &should_abort,
                                &workflow_id,
                                &trace_id,
                            )
                            .await;

                        // 节点不存在属于严重错误，无条件终止
                        if exec_result.is_err() {
                            overall_state = NodeState::Failed;
                            break;
                        }

                        // 串行模式每节点执行后检查中止标志
                        if *should_abort.lock().await && self.config.abort_on_error {
                            overall_state = NodeState::Failed;
                            break;
                        }
                    }
                }
                ExecutionMode::Parallel => {
                    let semaphore = Arc::new(Semaphore::new(self.config.max_concurrency));
                    let mut futures = Vec::new();

                    for node_id in layer {
                        let node_id = node_id.clone();
                        let sem = semaphore.clone();
                        let outputs = outputs.clone();
                        let results = results.clone();
                        let failed_nodes = failed_nodes.clone();
                        let skip_set = skip_set.clone();
                        let node_map = Arc::clone(&node_map);
                        let adjacency = Arc::clone(&adjacency);
                        let should_abort = should_abort.clone();
                        let workflow_id = workflow_id.clone();
                        let trace_id = trace_id.clone();
                        let client_cache = client_cache.clone();
                        let default_timeout = self.config.timeout;

                        futures.push(async move {
                            // 并行模式下，开始执行前检查是否已触发中止
                            if *should_abort.lock().await {
                                return;
                            }
                            let _permit = sem.acquire().await.unwrap_or_else(|_| {
                                tracing::warn!(
                                    "semaphore closed unexpectedly, proceeding without permit"
                                );
                                // Semaphore::acquire 返回的错误是 AcquireError，仅在 Semaphore 被关闭时发生
                                // 这里我们无法真正获取许可，但可以通过创建一个拥有所有权限的许可来继续
                                // 实际上这种情况不会发生，因为 Semaphore 不会被显式关闭
                                unreachable!("semaphore should never be closed")
                            });
                            let result = Self::execute_single_node_static(
                                &node_id,
                                &node_map,
                                db,
                                &adjacency,
                                &outputs,
                                &results,
                                &failed_nodes,
                                &skip_set,
                                &client_cache,
                                window,
                                &should_abort,
                                &workflow_id,
                                &trace_id,
                                default_timeout,
                                self.config.retry_count,
                                self.config.retry_backoff_ms,
                            )
                            .await;
                            if let Err(e) = result {
                                tracing::warn!(node_id = %node_id, error = %crate::crypto::sanitize_log(&e.to_string()), "并行执行节点失败");
                            }
                        });
                    }

                    futures::future::join_all(futures).await;
                }
                ExecutionMode::Conditional => {
                    // 条件分支：根据输出筛选下游路径
                    for node_id in layer {
                        let exec_result = self
                            .execute_single_node(
                                node_id,
                                &node_map,
                                db,
                                &adjacency,
                                &outputs,
                                &results,
                                &failed_nodes,
                                &skip_set,
                                &client_cache,
                                window,
                                &should_abort,
                                &workflow_id,
                                &trace_id,
                            )
                            .await;

                        // 节点不存在，无条件终止
                        if exec_result.is_err() {
                            overall_state = NodeState::Failed;
                            break;
                        }

                        // 读取节点输出中的允许下游列表，标记未选中分支为跳过
                        let outputs_guard = outputs.lock().await;
                        if let Some(node_output) = outputs_guard.get(node_id) {
                            if let Some(allowed_next) =
                                node_output.get("__allowed_next").and_then(|v| v.as_array())
                            {
                                let allowed_set: HashSet<String> = allowed_next
                                    .iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect();

                                if let Some(downstream) = adjacency.get(node_id) {
                                    let mut skip_guard = skip_set.lock().await;
                                    let mut adj_ref: HashMap<&str, Vec<&str>> = HashMap::new();
                                    for (k, v) in adjacency.iter() {
                                        adj_ref.insert(
                                            k.as_str(),
                                            v.iter().map(|s| s.as_str()).collect(),
                                        );
                                    }

                                    for downstream_id in downstream {
                                        if !allowed_set.contains(downstream_id) {
                                            let mut to_skip = HashSet::new();
                                            collect_downstream(
                                                downstream_id.as_str(),
                                                &adj_ref,
                                                &mut to_skip,
                                            );
                                            for id in to_skip {
                                                skip_guard.insert(id);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if *should_abort.lock().await && self.config.abort_on_error {
                            overall_state = NodeState::Failed;
                            break;
                        }
                    }
                }
                ExecutionMode::Loop { count } => {
                    // 循环执行，支持提前终止
                    'loop_layer: for _ in 0..count {
                        for node_id in layer {
                            let exec_result = self
                                .execute_single_node(
                                    node_id,
                                    &node_map,
                                    db,
                                    &adjacency,
                                    &outputs,
                                    &results,
                                    &failed_nodes,
                                    &skip_set,
                                    &client_cache,
                                    window,
                                    &should_abort,
                                    &workflow_id,
                                    &trace_id,
                                )
                                .await;
                            if exec_result.is_err() {
                                overall_state = NodeState::Failed;
                                break 'loop_layer;
                            }

                            // 检查提前终止标志
                            let outputs_guard = outputs.lock().await;
                            if let Some(node_output) = outputs_guard.get(node_id) {
                                if node_output
                                    .get("__loop_break")
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or(false)
                                {
                                    break 'loop_layer;
                                }
                            }

                            if *should_abort.lock().await && self.config.abort_on_error {
                                overall_state = NodeState::Failed;
                                break 'loop_layer;
                            }
                        }
                    }
                }
            }

            if *should_abort.lock().await && self.config.abort_on_error {
                overall_state = NodeState::Failed;
                break;
            }
        }

        // 6. 清理连接池引用
        drop(client_cache);

        let node_results = results.lock().await.clone();
        let total_duration_ms = start.elapsed().as_millis() as u64;

        // 失败或超时均判定为整体失败
        if node_results
            .iter()
            .any(|r| r.state == NodeState::Failed || r.state == NodeState::Timeout)
        {
            overall_state = NodeState::Failed;
        }

        // ============================================================
        // 记录 Prometheus 指标
        // ============================================================
        let wf_status = match overall_state {
            NodeState::Success => "success",
            NodeState::Failed => "failed",
            NodeState::Timeout => "timeout",
            _ => "unknown",
        };
        crate::metrics::record_workflow_execution(
            wf_status,
            total_duration_ms as f64 / 1000.0,
        );
        for r in &node_results {
            let node_status = match r.state {
                NodeState::Success => "success",
                NodeState::Failed => "failed",
                NodeState::Skipped => "skipped",
                NodeState::Timeout => "timeout",
                _ => continue,
            };
            crate::metrics::record_node_execution(
                node_status,
                r.duration_ms as f64 / 1000.0,
            );
        }

        WorkflowExecutionResult {
            workflow_id,
            state: overall_state,
            node_results,
            total_duration_ms,
            error: None,
        }
    }

    /// 执行单个节点（实例方法）
    #[allow(clippy::too_many_arguments)]
    async fn execute_single_node(
        &self,
        node_id: &str,
        node_map: &HashMap<String, WorkflowNode>,
        db: &Database,
        adjacency: &HashMap<String, Vec<String>>,
        outputs: &Arc<Mutex<HashMap<String, Value>>>,
        results: &Arc<Mutex<Vec<NodeExecutionResult>>>,
        failed_nodes: &Arc<Mutex<HashSet<String>>>,
        skip_set: &Arc<Mutex<HashSet<String>>>,
        client_cache: &Arc<Mutex<HashMap<String, Arc<Mutex<McpClient>>>>>,
        window: Option<&WebviewWindow>,
        should_abort: &Arc<Mutex<bool>>,
        workflow_id: &str,
        trace_id: &str,
    ) -> Result<()> {
        Self::execute_single_node_static(
            node_id,
            node_map,
            db,
            adjacency,
            outputs,
            results,
            failed_nodes,
            skip_set,
            client_cache,
            window,
            should_abort,
            workflow_id,
            trace_id,
            self.config.timeout,
            self.config.retry_count,
            self.config.retry_backoff_ms,
        )
        .await
    }

    /// 执行单个节点（静态方法，供并行模式调用）
    #[allow(clippy::too_many_arguments)]
    async fn execute_single_node_static(
        node_id: &str,
        node_map: &HashMap<String, WorkflowNode>,
        db: &Database,
        adjacency: &HashMap<String, Vec<String>>,
        outputs: &Arc<Mutex<HashMap<String, Value>>>,
        results: &Arc<Mutex<Vec<NodeExecutionResult>>>,
        failed_nodes: &Arc<Mutex<HashSet<String>>>,
        skip_set: &Arc<Mutex<HashSet<String>>>,
        client_cache: &Arc<Mutex<HashMap<String, Arc<Mutex<McpClient>>>>>,
        window: Option<&WebviewWindow>,
        should_abort: &Arc<Mutex<bool>>,
        workflow_id: &str,
        trace_id: &str,
        default_timeout: Duration,
        retry_count: u32,
        retry_backoff_ms: u64,
    ) -> Result<()> {
        // OpenTelemetry: 创建节点级 Span
        let node_label = node_map
            .get(node_id)
            .map(|n| n.data.label.as_str())
            .unwrap_or(node_id);
        let node_span = tracing::info_span!(
            "node_execute",
            node_id = %node_id,
            node_label = %node_label,
        );
        let _node_span_guard = node_span.enter();

        // 检查是否已被跳过
        {
            let skip = skip_set.lock().await;
            if skip.contains(node_id) {
                let mut res = results.lock().await;
                res.push(NodeExecutionResult {
                    node_id: node_id.to_string(),
                    state: NodeState::Skipped,
                    output: None,
                    error: Some("上游节点执行失败，已跳过".to_string()),
                    duration_ms: 0,
                });
                return Ok(());
            }
        }

        let node = match node_map.get(node_id) {
            Some(n) => n.clone(),
            None => {
                let mut res = results.lock().await;
                res.push(NodeExecutionResult {
                    node_id: node_id.to_string(),
                    state: NodeState::Failed,
                    output: None,
                    error: Some(format!("节点不存在: {node_id}")),
                    duration_ms: 0,
                });
                *should_abort.lock().await = true;
                return Err(anyhow!("节点不存在: {node_id}"));
            }
        };

        emit_event(
            window,
            workflow_id,
            &node_id,
            NodeState::Running,
            None,
            None,
            &trace_id,
        );

        let node_start = std::time::Instant::now();

        // 解析输入参数（数据传递）
        let output_map = outputs.lock().await.clone();
        let resolved_inputs = resolve_inputs(&node.data.inputs, &output_map);

        // 执行节点（带超时控制）
        let timeout_dur = self_or_default_timeout(&node, default_timeout);
        let result = tokio::time::timeout(
            timeout_dur,
            execute_node_inner(
                node,
                db,
                resolved_inputs,
                client_cache,
                retry_count,
                retry_backoff_ms,
            ),
        )
        .await;

        let duration_ms = node_start.elapsed().as_millis() as u64;

        match result {
            Ok(Ok(output)) => {
                // 成功
                let mut out = outputs.lock().await;
                out.insert(node_id.to_string(), output.clone());

                let mut res = results.lock().await;
                res.push(NodeExecutionResult {
                    node_id: node_id.to_string(),
                    state: NodeState::Success,
                    output: Some(output.clone()),
                    error: None,
                    duration_ms,
                });

                emit_event(
                    window,
                    workflow_id,
                    &node_id,
                    NodeState::Success,
                    Some(&output),
                    None,
                    &trace_id,
                );
            }
            Ok(Err(e)) => {
                // 执行失败
                let mut failed = failed_nodes.lock().await;
                failed.insert(node_id.to_string());

                // 标记下游节点为跳过
                mark_downstream_skipped(node_id, adjacency, skip_set).await;

                let mut res = results.lock().await;
                res.push(NodeExecutionResult {
                    node_id: node_id.to_string(),
                    state: NodeState::Failed,
                    output: None,
                    error: Some(e.to_string()),
                    duration_ms,
                });

                emit_event(
                    window,
                    workflow_id,
                    &node_id,
                    NodeState::Failed,
                    None,
                    Some(&e.to_string()),
                    &trace_id,
                );

                *should_abort.lock().await = true;
            }
            Err(_elapsed) => {
                // 超时
                let mut failed = failed_nodes.lock().await;
                failed.insert(node_id.to_string());

                mark_downstream_skipped(node_id, adjacency, skip_set).await;

                let mut res = results.lock().await;
                res.push(NodeExecutionResult {
                    node_id: node_id.to_string(),
                    state: NodeState::Timeout,
                    output: None,
                    error: Some("节点执行超时".to_string()),
                    duration_ms,
                });

                emit_event(
                    window,
                    workflow_id,
                    &node_id,
                    NodeState::Timeout,
                    None,
                    Some("节点执行超时"),
                    &trace_id,
                );

                *should_abort.lock().await = true;
            }
        }

        Ok(())
    }
}

// ============================================================
// 辅助函数
// ============================================================

/// 从节点配置中读取超时设置，否则使用全局默认值
/// 同时支持驼峰 timeoutMs 和下划线 timeout_ms
fn self_or_default_timeout(node: &WorkflowNode, default_timeout: Duration) -> Duration {
    let timeout_ms = node
        .data
        .config
        .get("timeoutMs")
        .and_then(|v| v.as_u64())
        .or_else(|| node.data.config.get("timeout_ms").and_then(|v| v.as_u64()));

    match timeout_ms {
        Some(ms) => Duration::from_millis(ms),
        None => default_timeout,
    }
}

/// 带重试的 MCP 连接
async fn connect_with_retry(
    protocol: &str,
    command: &str,
    args: &[String],
    env: &std::collections::HashMap<String, String>,
    endpoint: &str,
    max_retries: u32,
    retry_backoff_ms: u64,
) -> Result<McpClient> {
    let mut last_error = None;
    for attempt in 0..max_retries {
        match crate::gateway::create_mcp_client(protocol, command, args, env, endpoint).await {
            Ok(client) => return Ok(client),
            Err(e) => {
                last_error = Some(e);
                if attempt < max_retries - 1 {
                    let delay = retry_backoff_ms * (2u64.pow(attempt));
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
            }
        }
    }
    Err(anyhow!(
        "连接失败: {}",
        last_error.unwrap_or_else(|| "未知原因".to_string())
    ))
}

/// 实际执行节点逻辑：连接 MCP 服务器并调用工具
async fn execute_node_inner(
    node: WorkflowNode,
    db: &Database,
    inputs: Value,
    client_cache: &Arc<Mutex<HashMap<String, Arc<Mutex<McpClient>>>>>,
    retry_count: u32,
    retry_backoff_ms: u64,
) -> Result<Value> {
    let tool = node
        .data
        .tool
        .as_ref()
        .ok_or_else(|| anyhow!("节点未关联 MCP 工具"))?;

    let server = db
        .get_server(&tool.server_id)
        .map_err(|e| anyhow!("{}", e))?
        .ok_or_else(|| anyhow!("MCP 服务器未找到: {}", tool.server_id))?;

    // 从共享连接池获取或创建客户端（先释放锁再 connect，避免阻塞其他并行任务）
    let client = {
        let cached = {
            let cache = client_cache.lock().await;
            cache.get(&server.id).cloned()
        };
        if let Some(c) = cached {
            c
        } else {
            // 释放锁后再创建新连接，避免阻塞其他并行任务（带重试）
            let c = connect_with_retry(
                &server.protocol,
                &server.command,
                &server.args,
                &server.env,
                &server.endpoint,
                retry_count,
                retry_backoff_ms,
            )
            .await?;

            let client_arc = Arc::new(Mutex::new(c));
            let mut cache = client_cache.lock().await;
            cache.entry(server.id.clone()).or_insert(client_arc).clone()
        }
    };
    let mut retry_remaining = retry_count;
    let mut backoff = retry_backoff_ms;

    // 互斥调用工具，带重试逻辑
    let tool_call_start = std::time::Instant::now();

    // OpenTelemetry: 创建 MCP 工具调用 Span
    let tool_span = tracing::info_span!(
        "mcp_tool_call",
        server_id = %server.id,
        tool_name = %tool.name,
    );
    let _tool_span_guard = tool_span.enter();

    let result = loop {
        let attempt = client.lock().await.call_tool(&tool.name, inputs.clone()).await;
        match attempt {
            Ok(r) => break r,
            Err(_e) if retry_remaining > 0 => {
                retry_remaining -= 1;
                tracing::warn!(
                    node_id = %node.id,
                    tool = %tool.name,
                    remaining = retry_remaining,
                    backoff_ms = backoff,
                    "工具调用失败，将在 {}ms 后重试",
                    backoff
                );
                tokio::time::sleep(std::time::Duration::from_millis(backoff)).await;
                backoff *= 2; // 指数退避
            }
            Err(e) => {
                crate::metrics::record_tool_call(
                    "failed",
                    tool_call_start.elapsed().as_secs_f64(),
                );
                return Err(anyhow::anyhow!("工具调用失败（已重试）: {e}"));
            }
        }
    };

    if result.is_error.unwrap_or(false) {
        crate::metrics::record_tool_call(
            "failed",
            tool_call_start.elapsed().as_secs_f64(),
        );
        let error_text = result
            .content
            .iter()
            .filter_map(|c| c.text.clone())
            .collect::<Vec<_>>()
            .join("\n");

        return Err(anyhow!(
            "{}",
            if error_text.is_empty() {
                "工具返回错误".to_string()
            } else {
                error_text
            }
        ));
    }

    let output_text = result
        .content
        .iter()
        .filter_map(|c| c.text.clone())
        .collect::<Vec<_>>()
        .join("\n");

    // 尝试解析为 JSON，失败则返回纯文本
    if let Ok(json) = serde_json::from_str::<Value>(&output_text) {
        crate::metrics::record_tool_call(
            "success",
            tool_call_start.elapsed().as_secs_f64(),
        );
        Ok(json)
    } else {
        crate::metrics::record_tool_call(
            "success",
            tool_call_start.elapsed().as_secs_f64(),
        );
        Ok(Value::String(output_text))
    }
}

/// 构建邻接表
fn build_adjacency(node_ids: &[String], edges: &[WorkflowEdge]) -> HashMap<String, Vec<String>> {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for id in node_ids {
        adj.entry(id.clone()).or_default();
    }
    // 使用 HashSet 实现 O(1) 节点查找
    let node_set: HashSet<&str> = node_ids.iter().map(|s| s.as_str()).collect();
    for edge in edges {
        if node_set.contains(edge.source.as_str()) && node_set.contains(edge.target.as_str()) {
            adj.entry(edge.source.clone())
                .or_default()
                .push(edge.target.clone());
        }
    }
    adj
}

/// 标记下游节点为跳过
async fn mark_downstream_skipped(
    failed_node_id: &str,
    adjacency: &HashMap<String, Vec<String>>,
    skip_set: &Arc<Mutex<HashSet<String>>>,
) {
    let mut downstream = HashSet::new();
    // 一次构建 adj_map，使用迭代器替代手动 for 循环
    let adj_map: HashMap<&str, Vec<&str>> = adjacency
        .iter()
        .map(|(k, v)| (k.as_str(), v.iter().map(|s| s.as_str()).collect()))
        .collect();
    collect_downstream(failed_node_id, &adj_map, &mut downstream);

    let mut skip = skip_set.lock().await;
    for id in downstream {
        if id != failed_node_id {
            skip.insert(id);
        }
    }
}

/// 发送节点状态变更事件到前端
#[cfg(feature = "tauri-runtime")]
fn emit_event(
    window: Option<&WebviewWindow>,
    workflow_id: &str,
    node_id: &str,
    state: NodeState,
    output: Option<&Value>,
    error: Option<&str>,
    trace_id: &str,
) {
    if let Some(win) = window {
        let event = NodeStateEvent {
            workflow_id: workflow_id.to_string(),
            node_id: node_id.to_string(),
            state,
            output: output.cloned(),
            error: error.map(|s| s.to_string()),
            trace_id: trace_id.to_string(),
        };
        let _ = win.emit("node-state-change", event);
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::sqlite::WorkflowEdge;

    #[test]
    fn test_topological_layers_linear() {
        let nodes = vec!["A".into(), "B".into(), "C".into()];
        let edges = vec![edge("e1", "A", "B"), edge("e2", "B", "C")];

        let layers = topological_layers(&nodes, &edges).unwrap();
        assert_eq!(layers.len(), 3);
        assert_eq!(layers[0], vec!["A"]);
        assert_eq!(layers[1], vec!["B"]);
        assert_eq!(layers[2], vec!["C"]);
    }

    #[test]
    fn test_topological_layers_parallel() {
        let nodes = vec!["A".into(), "B".into(), "C".into(), "D".into()];
        let edges = vec![
            edge("e1", "A", "C"),
            edge("e2", "B", "C"),
            edge("e3", "C", "D"),
        ];

        let layers = topological_layers(&nodes, &edges).unwrap();
        assert_eq!(layers.len(), 3);
        assert_eq!(layers[0].len(), 2);
        assert_eq!(layers[1], vec!["C"]);
        assert_eq!(layers[2], vec!["D"]);
    }

    #[test]
    fn test_topological_layers_cycle() {
        let nodes = vec!["A".into(), "B".into()];
        let edges = vec![edge("e1", "A", "B"), edge("e2", "B", "A")];

        let result = topological_layers(&nodes, &edges);
        assert!(result.is_err());
    }

    #[test]
    fn test_topological_layers_no_edges() {
        let nodes = vec!["A".into(), "B".into(), "C".into()];
        let edges = vec![];

        let layers = topological_layers(&nodes, &edges).unwrap();
        assert_eq!(layers.len(), 1);
        assert_eq!(layers[0].len(), 3);
    }

    #[test]
    fn test_topological_layers_invalid_edge() {
        let nodes = vec!["A".into(), "B".into()];
        let edges = vec![edge("e1", "A", "C")]; // C 不存在

        let result = topological_layers(&nodes, &edges);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_inputs() {
        let mut outputs = HashMap::new();
        outputs.insert(
            "node1".to_string(),
            Value::String("hello world".to_string()),
        );
        outputs.insert(
            "node2".to_string(),
            serde_json::json!({"name": "test", "count": 42}),
        );
        outputs.insert(
            "node3".to_string(),
            serde_json::json!({"a": {"b": {"c": "nested_value"}}}),
        );

        // 简单引用
        let input = Value::String("${node1}".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(resolved, Value::String("hello world".to_string()));

        // 单层字段引用
        let input = Value::String("${node2.name}".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(resolved, Value::String("test".to_string()));

        // 嵌套字段引用
        let input = Value::String("${node3.a.b.c}".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(resolved, Value::String("nested_value".to_string()));

        // 嵌套对象
        let input = serde_json::json!({
            "message": "${node1}",
            "user": "${node2.name}",
            "count": 1
        });
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(
            resolved,
            serde_json::json!({
                "message": "hello world",
                "user": "test",
                "count": 1
            })
        );
    }

    #[test]
    fn test_resolve_inputs_partial_interpolation() {
        let mut outputs = HashMap::new();
        outputs.insert("node1".to_string(), Value::String("world".to_string()));
        outputs.insert("node2".to_string(), Value::String("foo".to_string()));
        outputs.insert(
            "node3".to_string(),
            serde_json::json!({"name": "Alice", "age": 30}),
        );

        // 前缀 + 引用
        let input = Value::String("Hello ${node1}!".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(resolved, Value::String("Hello world!".to_string()));

        // 引用 + 后缀
        let input = Value::String("${node1} says hi".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(resolved, Value::String("world says hi".to_string()));

        // 多个引用
        let input = Value::String("${node2} and ${node1}".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(resolved, Value::String("foo and world".to_string()));

        // 字段引用 + 文字
        let input = Value::String("User: ${node3.name}, Age: ${node3.age}".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(resolved, Value::String("User: Alice, Age: 30".to_string()));

        // 无引用字符串保持不变
        let input = Value::String("plain text".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(resolved, Value::String("plain text".to_string()));

        // 未闭合的 ${ 视为普通文本
        let input = Value::String("unclosed ${node1".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(resolved, Value::String("unclosed ${node1".to_string()));

        // 引用不存在的节点保持原样
        let input = Value::String("${nonexistent} and ${node1}".to_string());
        let resolved = resolve_inputs(&input, &outputs);
        assert_eq!(
            resolved,
            Value::String("${nonexistent} and world".to_string())
        );
    }

    fn edge(id: &str, source: &str, target: &str) -> WorkflowEdge {
        WorkflowEdge {
            id: id.to_string(),
            source: source.to_string(),
            target: target.to_string(),
            source_handle: None,
            target_handle: None,
            edge_type: None,
            animated: None,
        }
    }

    // ============================================================
    // build_adjacency 测试
    // ============================================================

    #[test]
    fn test_build_adjacency_empty() {
        let nodes: Vec<String> = vec![];
        let edges: Vec<WorkflowEdge> = vec![];
        let adj = build_adjacency(&nodes, &edges);
        assert!(adj.is_empty());
    }

    #[test]
    fn test_build_adjacency_no_edges() {
        let nodes = vec!["A".into(), "B".into(), "C".into()];
        let edges = vec![];
        let adj = build_adjacency(&nodes, &edges);
        assert_eq!(adj.len(), 3);
        assert!(adj.get("A").unwrap().is_empty());
        assert!(adj.get("B").unwrap().is_empty());
        assert!(adj.get("C").unwrap().is_empty());
    }

    #[test]
    fn test_build_adjacency_linear() {
        let nodes = vec!["A".into(), "B".into(), "C".into()];
        let edges = vec![edge("e1", "A", "B"), edge("e2", "B", "C")];
        let adj = build_adjacency(&nodes, &edges);
        assert_eq!(adj["A"], vec!["B"]);
        assert_eq!(adj["B"], vec!["C"]);
        assert!(adj["C"].is_empty());
    }

    #[test]
    fn test_build_adjacency_branching() {
        let nodes = vec!["A".into(), "B".into(), "C".into(), "D".into()];
        let edges = vec![
            edge("e1", "A", "B"),
            edge("e2", "A", "C"),
            edge("e3", "B", "D"),
            edge("e4", "C", "D"),
        ];
        let adj = build_adjacency(&nodes, &edges);
        assert_eq!(adj["A"].len(), 2);
        assert!(adj["A"].contains(&"B".to_string()));
        assert!(adj["A"].contains(&"C".to_string()));
        assert_eq!(adj["B"], vec!["D"]);
        assert_eq!(adj["C"], vec!["D"]);
        assert!(adj["D"].is_empty());
    }

    #[test]
    fn test_build_adjacency_filter_invalid_edges() {
        let nodes = vec!["A".into(), "B".into()];
        let edges = vec![
            edge("e1", "A", "B"),
            edge("e2", "A", "C"), // C 不在 nodes 中，应被过滤
            edge("e3", "X", "A"), // X 不在 nodes 中，应被过滤
        ];
        let adj = build_adjacency(&nodes, &edges);
        // 只有 A->B 应被保留
        assert_eq!(adj["A"], vec!["B"]);
        assert!(adj["B"].is_empty());
    }

    // ============================================================
    // collect_downstream 测试
    // ============================================================

    #[test]
    fn test_collect_downstream_linear() {
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        adj.insert("A", vec!["B"]);
        adj.insert("B", vec!["C"]);
        adj.insert("C", vec![]);

        let mut visited = HashSet::new();
        collect_downstream("A", &adj, &mut visited);
        assert_eq!(visited.len(), 3);
        assert!(visited.contains("A"));
        assert!(visited.contains("B"));
        assert!(visited.contains("C"));
    }

    #[test]
    fn test_collect_downstream_branching() {
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        adj.insert("A", vec!["B", "C"]);
        adj.insert("B", vec!["D"]);
        adj.insert("C", vec!["D"]);
        adj.insert("D", vec![]);

        let mut visited = HashSet::new();
        collect_downstream("A", &adj, &mut visited);
        assert_eq!(visited.len(), 4);
        assert!(visited.contains("A"));
        assert!(visited.contains("B"));
        assert!(visited.contains("C"));
        assert!(visited.contains("D"));
    }

    #[test]
    fn test_collect_downstream_leaf() {
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        adj.insert("A", vec!["B"]);
        adj.insert("B", vec![]);

        let mut visited = HashSet::new();
        collect_downstream("B", &adj, &mut visited);
        assert_eq!(visited.len(), 1);
        assert!(visited.contains("B"));
    }

    #[test]
    fn test_collect_downstream_cycle_safe() {
        // 有环的图，迭代版本不会无限递归
        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        adj.insert("A", vec!["B"]);
        adj.insert("B", vec!["C"]);
        adj.insert("C", vec!["A"]); // 环

        let mut visited = HashSet::new();
        collect_downstream("A", &adj, &mut visited);
        assert_eq!(visited.len(), 3);
    }
}
