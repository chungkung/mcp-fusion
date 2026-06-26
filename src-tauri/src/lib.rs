mod crypto;
mod gateway;
mod llm;
mod marketplace;
mod metrics;
mod orchestrator;
mod storage;
mod tracing_otel;

use std::collections::HashMap;

// ============================================================
// 结构化日志系统
// ============================================================

#[cfg(feature = "tauri-runtime")]
use tracing_appender::non_blocking::WorkerGuard;
#[cfg(feature = "tauri-runtime")]
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// 初始化结构化日志系统和 OpenTelemetry 追踪
/// 返回 WorkerGuard 需要在程序生命周期内持有，否则日志会丢失
#[cfg(feature = "tauri-runtime")]
pub fn init_logging() -> Vec<WorkerGuard> {
    let mut guards = Vec::new();

    let log_dir = dirs_log_path();
    let _ = std::fs::create_dir_all(&log_dir);

    let file_appender = tracing_appender::rolling::daily(&log_dir, "mcp-fusion.log");
    let (non_blocking, file_guard) = tracing_appender::non_blocking(file_appender);
    guards.push(file_guard);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 尝试初始化 OpenTelemetry 追踪（通过环境变量配置）
    let otel_endpoint =
        std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").unwrap_or_default();
    let otel_service_name =
        std::env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "mcp-fusion".to_string());

    let otel_layer = tracing_otel::init_tracer(&otel_endpoint, &otel_service_name);

    // OpenTelemetryLayer 必须直接加在 Registry 上，因此需要分两个分支构建
    if let Some(otel) = otel_layer {
        let file_layer = fmt::layer()
            .json()
            .with_writer(non_blocking.clone())
            .with_target(true);
        let console_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_line_number(true);
        tracing_subscriber::registry()
            .with(otel)
            .with(env_filter)
            .with(console_layer)
            .with(file_layer)
            .init();
    } else {
        let file_layer = fmt::layer()
            .json()
            .with_writer(non_blocking)
            .with_target(true);
        let console_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_line_number(true);
        tracing_subscriber::registry()
            .with(env_filter)
            .with(console_layer)
            .with(file_layer)
            .init();
    }

    tracing::info!("日志系统初始化完成");

    guards
}

/// 日志目录
fn dirs_log_path() -> String {
    if let Ok(dir) = std::env::var("APPDATA") {
        format!("{dir}/mcp-fusion/logs")
    } else if let Ok(dir) = std::env::var("HOME") {
        format!("{dir}/.local/share/mcp-fusion/logs")
    } else {
        "logs".to_string()
    }
}

#[cfg(feature = "tauri-runtime")]
use chrono::Utc;
use storage::sqlite::Database;
#[cfg(feature = "tauri-runtime")]
use storage::sqlite::{
    McpServer, Workflow, WorkflowEdge, WorkflowNode, WorkflowNodeData,
    WorkflowNodePosition,
};
#[cfg(feature = "tauri-runtime")]
use storage::sqlite::{AuditLog, WorkflowExecution};

#[cfg(feature = "tauri-runtime")]
use tauri::Manager;

// ============================================================
// 认证管理
// ============================================================

#[derive(Debug, Clone, PartialEq)]
enum UserRole {
    Admin,
    Developer,
    Viewer,
}

impl UserRole {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => UserRole::Admin,
            "developer" => UserRole::Developer,
            _ => UserRole::Viewer,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Developer => "developer",
            UserRole::Viewer => "viewer",
        }
    }

    /// 检查是否有权限执行指定操作
    fn can(&self, action: &str) -> bool {
        match self {
            UserRole::Admin => true,
            UserRole::Developer => matches!(
                action,
                "server.list" | "server.get" | "server.add" | "server.update" | "server.remove"
                    | "server.ping" | "server.list_tools" | "server.execute_tool"
                    | "workflow.list" | "workflow.get" | "workflow.save" | "workflow.remove"
                    | "workflow.execute" | "workflow.retry" | "workflow.force_unlock"
                    | "execution.list" | "runtime.status" | "runtime.stop" | "health"
                    | "backup" | "restore" | "intent_parse"
            ),
            UserRole::Viewer => matches!(
                action,
                "server.list" | "server.get" | "server.ping" | "server.list_tools"
                    | "workflow.list" | "workflow.get" | "execution.list"
                    | "runtime.status" | "health"
            ),
        }
    }
}

struct AuthManager {
    api_key_hash: std::sync::Mutex<Option<String>>,
    current_role: std::sync::Mutex<UserRole>,
}

impl AuthManager {
    fn new() -> Self {
        Self {
            api_key_hash: std::sync::Mutex::new(None),
            current_role: std::sync::Mutex::new(UserRole::Admin),
        }
    }

    fn generate_key(&self) -> String {
        use sha2::{Digest, Sha256};
        let key = uuid::Uuid::new_v4().to_string();
        let hash = hex::encode(Sha256::digest(key.as_bytes()));
        *self.api_key_hash.lock().unwrap() = Some(hash);
        key
    }

    fn set_key(&self, key: &str) {
        use sha2::{Digest, Sha256};
        let hash = hex::encode(Sha256::digest(key.as_bytes()));
        *self.api_key_hash.lock().unwrap() = Some(hash);
    }

    fn verify(&self, key: &str) -> bool {
        use sha2::{Digest, Sha256};
        let guard = self.api_key_hash.lock().unwrap();
        match guard.as_ref() {
            Some(hash) => {
                let key_hash = hex::encode(Sha256::digest(key.as_bytes()));
                key_hash == *hash
            }
            None => false,
        }
    }

    fn is_set(&self) -> bool {
        self.api_key_hash.lock().unwrap().is_some()
    }

    fn clear(&self) {
        *self.api_key_hash.lock().unwrap() = None;
    }

    fn set_key_hash(&self, hash: &str) {
        *self.api_key_hash.lock().unwrap() = Some(hash.to_string());
    }

    fn set_role(&self, role: &str) {
        *self.current_role.lock().unwrap() = UserRole::from_str(role);
    }

    fn get_role(&self) -> UserRole {
        self.current_role.lock().unwrap().clone()
    }

    fn require_role(&self, action: &str) -> Result<(), String> {
        let role = self.current_role.lock().unwrap();
        if role.can(action) {
            Ok(())
        } else {
            Err(format!(
                "权限不足：角色 '{}' 不允许执行操作 '{}'",
                role.as_str(),
                action
            ))
        }
    }
}

// ============================================================
// 速率限制
// ============================================================

struct RateLimitRule {
    window_secs: u64,
    max_requests: usize,
}

struct RateLimiter {
    rules: std::collections::HashMap<String, RateLimitRule>,
    histories: std::sync::Mutex<std::collections::HashMap<String, Vec<std::time::Instant>>>,
}

impl RateLimiter {
    fn new() -> Self {
        let mut rules = std::collections::HashMap::new();
        rules.insert(
            "execute_workflow".to_string(),
            RateLimitRule {
                window_secs: 60,
                max_requests: 5,
            },
        );
        rules.insert(
            "execute_tool".to_string(),
            RateLimitRule {
                window_secs: 60,
                max_requests: 30,
            },
        );
        rules.insert(
            "list_tools".to_string(),
            RateLimitRule {
                window_secs: 60,
                max_requests: 20,
            },
        );
        rules.insert(
            "auth_verify_key".to_string(),
            RateLimitRule {
                window_secs: 60,
                max_requests: 5,
            },
        );
        rules.insert(
            "health_check".to_string(),
            RateLimitRule {
                window_secs: 60,
                max_requests: 10,
            },
        );
        Self {
            rules,
            histories: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    fn check(&self, operation: &str) -> Result<(), String> {
        let rule = self.rules.get(operation).unwrap_or(&RateLimitRule {
            window_secs: 60,
            max_requests: 10,
        });
        let now = std::time::Instant::now();
        let window_start = now - std::time::Duration::from_secs(rule.window_secs);
        let mut histories = self.histories.lock().unwrap();
        let timestamps = histories.entry(operation.to_string()).or_default();
        timestamps.retain(|t| *t > window_start);
        if timestamps.len() >= rule.max_requests {
            return Err(format!(
                "操作 '{}' 请求过于频繁（{} 次/{} 秒），请稍后重试",
                operation, rule.max_requests, rule.window_secs
            ));
        }
        timestamps.push(now);
        Ok(())
    }
}

// ============================================================
// 熔断器
// ============================================================

struct CircuitBreaker {
    failures: std::sync::Mutex<std::collections::HashMap<String, (u32, std::time::Instant)>>,
}

impl CircuitBreaker {
    fn new() -> Self {
        Self {
            failures: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    fn allow_request(&self, service_id: &str) -> bool {
        let guard = self.failures.lock().unwrap();
        if let Some((count, opened_at)) = guard.get(service_id) {
            if *count >= 3 {
                // 熔断打开 30 秒后自动恢复
                if opened_at.elapsed() < std::time::Duration::from_secs(30) {
                    return false;
                }
            }
        }
        true
    }

    fn record_failure(&self, service_id: &str) {
        let mut guard = self.failures.lock().unwrap();
        let entry = guard
            .entry(service_id.to_string())
            .or_insert((0, std::time::Instant::now()));
        entry.0 += 1;
        entry.1 = std::time::Instant::now();
    }

    fn record_success(&self, service_id: &str) {
        let mut guard = self.failures.lock().unwrap();
        guard.remove(service_id);
    }
}

// ============================================================
// 应用状态
// ============================================================

struct AppState {
    db: Database,
    db_path: String,
    auth: AuthManager,
    rate_limiter: RateLimiter,
    circuit_breaker: CircuitBreaker,
    encrypt_passphrase: String,
    client_cache: tokio::sync::Mutex<
        std::collections::HashMap<
            String,
            std::sync::Arc<tokio::sync::Mutex<crate::gateway::McpClient>>,
        >,
    >,
}

// ============================================================
// Tauri 命令（仅在 tauri-runtime feature 启用时编译）
// ============================================================

/// 创建审计日志条目（prev_hash 和 chain_hash 由 insert_audit_log 自动计算）
#[cfg(feature = "tauri-runtime")]
fn new_audit_log(action: &str, resource: &str, detail: String) -> AuditLog {
    AuditLog {
        id: uuid::Uuid::new_v4().to_string(),
        action: action.to_string(),
        resource: resource.to_string(),
        detail,
        prev_hash: String::new(),
        chain_hash: String::new(),
        created_at: Utc::now().timestamp_millis(),
    }
}

#[cfg(feature = "tauri-runtime")]
mod commands {
    use super::*;
    use tauri;

    #[tauri::command]
    pub fn list_servers(state: tauri::State<AppState>) -> Result<Vec<McpServer>, String> {
        state.auth.require_role("server.list")?;
        state.db.list_servers().map(|servers| {
            servers
                .into_iter()
                .map(|mut s| {
                    decrypt_server_fields(&mut s, &state.encrypt_passphrase);
                    s
                })
                .collect()
        })
    }

    /// 分页查询服务器列表
    #[tauri::command]
    pub fn list_servers_paginated(
        state: tauri::State<AppState>,
        offset: usize,
        limit: usize,
    ) -> Result<storage::sqlite::PaginatedResult<McpServer>, String> {
        state.auth.require_role("server.list")?;
        state.db.list_servers_paginated(offset, limit).map(|mut result| {
            for s in &mut result.items {
                decrypt_server_fields(s, &state.encrypt_passphrase);
            }
            result
        })
    }

    #[tauri::command]
    pub fn add_server(
        state: tauri::State<AppState>,
        mut server: McpServer,
    ) -> Result<McpServer, String> {
        state.auth.require_role("server.add")?;
        if server.id.trim().is_empty() {
            return Err("server id cannot be empty".to_string());
        }
        if server.name.trim().is_empty() {
            return Err("server name cannot be empty".to_string());
        }
        let valid_protocols = ["stdio", "sse", "streamable-http", "streamable_http"];
        if !valid_protocols.contains(&server.protocol.as_str()) {
            return Err(format!("unsupported protocol: {}", server.protocol));
        }
        // 协议特定校验
        if server.protocol == "stdio" && server.command.trim().is_empty() {
            return Err("stdio 协议必须提供 command".to_string());
        }
        if (server.protocol == "sse" || server.protocol.starts_with("streamable"))
            && server.endpoint.trim().is_empty()
        {
            return Err("sse/streamable-http 协议必须提供 endpoint".to_string());
        }
        let now = Utc::now().timestamp_millis();
        server.created_at = now;
        server.updated_at = now;
        // 加密敏感字段
        encrypt_server_fields(&mut server, &state.encrypt_passphrase);
        state.db.insert_server(&server)?;
        // 审计日志
        if let Err(e) = state.db.insert_audit_log(&new_audit_log(
            "server.add",
            &format!("server:{}", server.id),
            serde_json::json!({"name": server.name, "protocol": server.protocol}).to_string(),
        )) {
            tracing::error!(action = "audit_log_write", error = %e, "写入审计日志失败");
        }
        tracing::info!(action = "add_server", server_id = %server.id, server_name = %server.name, "添加 MCP 服务器");
        // 解密后返回给前端
        decrypt_server_fields(&mut server, &state.encrypt_passphrase);
        Ok(server)
    }

    #[tauri::command]
    pub fn update_server(
        state: tauri::State<AppState>,
        mut server: McpServer,
    ) -> Result<McpServer, String> {
        state.auth.require_role("server.update")?;
        server.updated_at = Utc::now().timestamp_millis();
        // 加密敏感字段
        encrypt_server_fields(&mut server, &state.encrypt_passphrase);
        state.db.update_server(&server)?;
        tracing::info!(action = "update_server", server_id = %server.id, "更新 MCP 服务器");
        // 解密后返回
        decrypt_server_fields(&mut server, &state.encrypt_passphrase);
        Ok(server)
    }

    #[tauri::command]
    pub fn remove_server(state: tauri::State<AppState>, id: String) -> Result<(), String> {
        state.auth.require_role("server.remove")?;
        state.db.delete_server(&id)?;
        // 审计日志
        if let Err(e) = state.db.insert_audit_log(&new_audit_log(
            "server.remove",
            &format!("server:{id}"),
            "{}".to_string(),
        )) {
            tracing::error!(action = "audit_log_write", error = %e, "写入审计日志失败");
        }
        tracing::info!(action = "remove_server", server_id = %id, "删除 MCP 服务器");
        Ok(())
    }

    #[tauri::command]
    pub fn get_server(
        state: tauri::State<AppState>,
        id: String,
    ) -> Result<Option<McpServer>, String> {
        state.auth.require_role("server.get")?;
        state.db.get_server(&id).map(|opt| {
            opt.map(|mut s| {
                decrypt_server_fields(&mut s, &state.encrypt_passphrase);
                s
            })
        })
    }

    #[tauri::command]
    pub fn list_workflows(state: tauri::State<AppState>) -> Result<Vec<Workflow>, String> {
        state.auth.require_role("workflow.list")?;
        state.db.list_workflows()
    }

    /// 分页查询工作流列表
    #[tauri::command]
    pub fn list_workflows_paginated(
        state: tauri::State<AppState>,
        offset: usize,
        limit: usize,
    ) -> Result<storage::sqlite::PaginatedResult<Workflow>, String> {
        state.auth.require_role("workflow.list")?;
        state.db.list_workflows_paginated(offset, limit)
    }

    #[tauri::command]
    pub fn save_workflow(
        state: tauri::State<AppState>,
        workflow: Workflow,
    ) -> Result<Workflow, String> {
        state.auth.require_role("workflow.save")?;
        let mut wf = workflow;
        let now = Utc::now().timestamp_millis();
        wf.updated_at = now;

        if wf.created_at == 0 {
            // 尝试从数据库读取已有的 created_at，避免 INSERT OR REPLACE 覆盖
            if let Ok(Some(existing)) = state.db.get_workflow(&wf.id) {
                wf.created_at = existing.created_at;
            } else {
                wf.created_at = now;
            }
        }

        // 使用 ON CONFLICT DO UPDATE 原子化 upsert，消除 TOCTOU 竞态条件
        state.db.insert_or_replace_workflow(&wf)?;
        tracing::info!(action = "save_workflow", workflow_id = %wf.id, workflow_name = %wf.name, "保存工作流");
        Ok(wf)
    }

    #[tauri::command]
    pub fn remove_workflow(state: tauri::State<AppState>, id: String) -> Result<(), String> {
        state.auth.require_role("workflow.remove")?;
        state.db.delete_workflow(&id)?;
        tracing::info!(action = "remove_workflow", workflow_id = %id, "删除工作流");
        Ok(())
    }

    #[tauri::command]
    pub fn get_workflow(
        state: tauri::State<AppState>,
        id: String,
    ) -> Result<Option<Workflow>, String> {
        state.auth.require_role("workflow.get")?;
        state.db.get_workflow(&id)
    }

    #[tauri::command]
    pub async fn execute_workflow(
        app: tauri::AppHandle,
        state: tauri::State<'_, AppState>,
        id: String,
        idempotency_key: Option<String>,
        resume_from: Option<String>,
    ) -> Result<orchestrator::OrchestrationResult, String> {
        // 权限检查
        state.auth.require_role("workflow.execute")?;
        // 速率限制
        state.rate_limiter.check("execute_workflow")?;

        let workflow = state
            .db
            .get_workflow(&id)?
            .ok_or_else(|| format!("Workflow not found: {id}"))?;

        // 输入验证
        if workflow.nodes.is_empty() {
            return Err("工作流没有节点，无法执行".to_string());
        }
        if workflow.nodes.len() > 200 {
            return Err(format!(
                "工作流节点数量 ({}) 超过上限 (200)",
                workflow.nodes.len()
            ));
        }
        let valid_modes = ["manual", "auto", "intent"];
        if !valid_modes.contains(&workflow.mode.as_str()) {
            return Err(format!("无效的工作流模式: {}", workflow.mode));
        }

        let now = Utc::now().timestamp_millis();

        // ============================================================
        // 幂等检查：相同 workflow_id + idempotency_key 只执行一次
        // ============================================================
        let ikey = idempotency_key.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if let Some(existing) = state
            .db
            .find_execution_by_idempotency_key(&id, &ikey)
            .map_err(|e| format!("幂等检查失败: {e}"))?
        {
            tracing::info!(
                workflow_id = %id,
                idempotency_key = %ikey,
                existing_execution = %existing.id,
                "幂等命中：返回已有执行结果"
            );
            return Ok(orchestrator::OrchestrationResult {
                workflow_id: existing.workflow_id,
                execution_id: existing.id,
                status: existing.status,
                node_results: serde_json::from_value(existing.node_results)
                    .unwrap_or_default(),
                error: existing.error,
            });
        }

        // ============================================================
        // 执行锁：防止同一工作流并发执行
        // ============================================================
        if !state
            .db
            .acquire_workflow_lock(&id)
            .map_err(|e| format!("获取执行锁失败: {e}"))?
        {
            return Err(format!(
                "工作流 {} 正在执行中，请等待当前执行完成后再试",
                workflow.name
            ));
        }

        // 解析断点续传的已完成节点列表和输出
        let completed_node_ids: Vec<String>;
        let completed_outputs: HashMap<String, serde_json::Value>;
        if let Some(ref prev_exec_id) = resume_from {
            match state.db.get_execution(prev_exec_id) {
                Ok(Some(prev_exec)) => {
                    if prev_exec.status == "failed" || prev_exec.status == "timeout" {
                        let nodes: Vec<String> = serde_json::from_value(prev_exec.completed_nodes)
                            .unwrap_or_default();
                        // 提取已完成节点的输出，供下游节点引用
                        let outputs: HashMap<String, serde_json::Value> = prev_exec
                            .node_results
                            .as_object()
                            .map(|obj| {
                                obj.iter()
                                    .filter_map(|(k, v)| {
                                        v.get("output").cloned().map(|o| (k.clone(), o))
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();
                        tracing::info!(
                            workflow_id = %id,
                            completed_count = nodes.len(),
                            output_count = outputs.len(),
                            "断点续传：跳过已完成的 {} 个节点，注入 {} 个输出",
                            nodes.len(),
                            outputs.len()
                        );
                        completed_node_ids = nodes;
                        completed_outputs = outputs;
                    } else {
                        completed_node_ids = Vec::new();
                        completed_outputs = HashMap::new();
                    }
                }
                Ok(None) => {
                    completed_node_ids = Vec::new();
                    completed_outputs = HashMap::new();
                }
                Err(e) => {
                    tracing::error!(error = %e, "查询历史执行记录失败");
                    completed_node_ids = Vec::new();
                    completed_outputs = HashMap::new();
                }
            }
        } else {
            completed_node_ids = Vec::new();
            completed_outputs = HashMap::new();
        };

        tracing::info!(
            "执行工作流: {} ({} 节点, {} 边)",
            workflow.name,
            workflow.nodes.len(),
            workflow.edges.len()
        );

        // 审计日志：记录执行开始
        let execution_id = uuid::Uuid::new_v4().to_string();
        if let Err(e) = state.db.insert_audit_log(&new_audit_log(
            "workflow.execute",
            &format!("workflow:{}", id),
            serde_json::json!({
                "execution_id": execution_id,
                "idempotency_key": ikey,
                "workflow_name": workflow.name,
                "node_count": workflow.nodes.len(),
                "resume": resume_from.is_some(),
            })
            .to_string(),
        )) {
            tracing::error!(action = "audit_log_write", error = %e, "写入审计日志失败");
        }

        // 创建执行记录（含幂等键和已完成节点）
        let completed_nodes_json = serde_json::json!(completed_node_ids);
        if let Err(e) = state.db.insert_execution(&WorkflowExecution {
            id: execution_id.clone(),
            workflow_id: id.clone(),
            status: "running".to_string(),
            idempotency_key: Some(ikey),
            completed_nodes: completed_nodes_json,
            started_at: now,
            finished_at: None,
            node_results: serde_json::json!({}),
            error: None,
        }) {
            tracing::error!(action = "execution_insert", error = %e, "创建执行记录失败");
        }

        let window = app.get_webview_window("main");

        let config = orchestrator::SchedulerConfig::default();
        let scheduler = orchestrator::Scheduler::new(config);

        let result = scheduler
            .execute(&state.db, &workflow, window.as_ref(), &completed_node_ids, &completed_outputs)
            .await;

        let node_results: Vec<orchestrator::NodeResult> = result
            .node_results
            .into_iter()
            .map(|r| orchestrator::NodeResult {
                node_id: r.node_id,
                status: match r.state {
                    orchestrator::NodeState::Success => "success".to_string(),
                    orchestrator::NodeState::Failed => "failed".to_string(),
                    orchestrator::NodeState::Skipped => "skipped".to_string(),
                    orchestrator::NodeState::Timeout => "failed".to_string(),
                    _ => "idle".to_string(),
                },
                output: r.output,
                error: r.error,
            })
            .collect();

        // 更新执行记录（含已完成节点列表用于后续断点续传）
        let final_status = match result.state {
            orchestrator::NodeState::Success => "success",
            orchestrator::NodeState::Failed => "failed",
            orchestrator::NodeState::Timeout => "timeout",
            _ => "failed",
        };
        let ns_json: serde_json::Value = serde_json::json!(node_results);
        let finished_at = Utc::now().timestamp_millis();

        // 为断点续传：记录所有成功执行的节点 ID
        let completed_for_checkpoint: Vec<String> = node_results
            .iter()
            .filter(|r| r.status == "success")
            .map(|r| r.node_id.clone())
            .collect();
        let checkpoint_json = serde_json::json!(completed_for_checkpoint);

        // 计算最终错误信息（复用，避免 move）
        let final_error = result.error.or_else(|| {
            if final_status == "failed" {
                Some("工作流执行失败".to_string())
            } else {
                None
            }
        });

        if let Err(e) = state.db.update_execution(&WorkflowExecution {
            id: execution_id.clone(),
            workflow_id: id.clone(),
            status: final_status.to_string(),
            idempotency_key: None,
            completed_nodes: checkpoint_json,
            started_at: now,
            finished_at: Some(finished_at),
            node_results: ns_json,
            error: final_error.clone(),
        }) {
            tracing::error!(action = "execution_update", execution_id = %execution_id, error = %e, "更新执行记录失败");
        }

        // ============================================================
        // 释放执行锁
        // ============================================================
        if let Err(e) = state.db.release_workflow_lock(&id) {
            tracing::error!(action = "lock_release", workflow_id = %id, error = %e, "释放执行锁失败");
        }

        Ok(orchestrator::OrchestrationResult {
            workflow_id: result.workflow_id,
            execution_id,
            status: match result.state {
                orchestrator::NodeState::Success => "success".to_string(),
                _ => "failed".to_string(),
            },
            node_results,
            error: final_error,
        })
    }

    #[tauri::command]
    pub async fn list_tools(
        state: tauri::State<'_, AppState>,
        server_id: String,
    ) -> Result<Vec<serde_json::Value>, String> {
        state.auth.require_role("server.list_tools")?;
        state.rate_limiter.check("list_tools")?;
        if !state.circuit_breaker.allow_request(&server_id) {
            return Err(format!("服务 {} 已被熔断保护，请稍后重试", server_id));
        }

        let server = state
            .db
            .get_server(&server_id)?
            .ok_or_else(|| format!("MCP server not found: {server_id}"))?;
        // 解密敏感字段
        let mut server = server;
        decrypt_server_fields(&mut server, &state.encrypt_passphrase);

        // 从共享连接池获取或创建客户端（按协议分发）
        let client = {
            let cache = state.client_cache.lock().await;
            cache.get(&server_id).cloned()
        };

        let client = if let Some(c) = client {
            c
        } else {
            let new_client = crate::gateway::create_mcp_client(
                    &server.protocol,
                    &server.command,
                    &server.args,
                    &server.env,
                    &server.endpoint,
                )
                .await
                .map_err(|e| {
                    state.circuit_breaker.record_failure(&server_id);
                    e
                })?;

            let client_arc =
                std::sync::Arc::new(tokio::sync::Mutex::new(new_client));
            let mut cache = state.client_cache.lock().await;
            cache
                .entry(server_id.clone())
                .or_insert(client_arc)
                .clone()
        };

        let tools = client.lock().await.list_tools().await.map_err(|e| {
            state.circuit_breaker.record_failure(&server_id);
            e.to_string()
        })?;

        state.circuit_breaker.record_success(&server_id);

        let result: Vec<serde_json::Value> = tools
            .into_iter()
            .map(|t| {
                serde_json::json!({
                    "name": t.name,
                    "description": t.description,
                    "inputSchema": t.input_schema,
                })
            })
            .collect();

        Ok(result)
    }

    #[tauri::command]
    pub async fn ping_server(
        state: tauri::State<'_, AppState>,
        server_id: String,
    ) -> Result<serde_json::Value, String> {
        state.auth.require_role("server.ping")?;
        let server = state
            .db
            .get_server(&server_id)?
            .ok_or_else(|| format!("MCP server not found: {server_id}"))?;
        // 解密敏感字段
        let mut server = server;
        decrypt_server_fields(&mut server, &state.encrypt_passphrase);

        // 尝试连接并列出工具，检测服务器是否可达
        let start = std::time::Instant::now();
        match crate::gateway::create_mcp_client(
            &server.protocol,
            &server.command,
            &server.args,
            &server.env,
            &server.endpoint,
        )
        .await
        {
            Ok(mut client) => {
                let latency_ms = start.elapsed().as_millis() as u64;
                let tools = client.list_tools().await.map_err(|e| e.to_string())?;
                Ok(serde_json::json!({
                    "status": "connected",
                    "latency_ms": latency_ms,
                    "tool_count": tools.len(),
                }))
            }
            Err(e) => {
                let latency_ms = start.elapsed().as_millis() as u64;
                Ok(serde_json::json!({
                    "status": "error",
                    "latency_ms": latency_ms,
                    "error": e,
                }))
            }
        }
    }

    #[tauri::command]
    pub async fn execute_tool(
        state: tauri::State<'_, AppState>,
        server_id: String,
        tool_name: String,
        inputs: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        state.auth.require_role("server.execute_tool")?;
        state.rate_limiter.check("execute_tool")?;
        if !state.circuit_breaker.allow_request(&server_id) {
            return Err(format!("服务 {} 已被熔断保护，请稍后重试", server_id));
        }

        // 输入验证
        if server_id.trim().is_empty() {
            return Err("server_id 不能为空".to_string());
        }
        if tool_name.trim().is_empty() {
            return Err("tool_name 不能为空".to_string());
        }
        if !inputs.is_object() {
            return Err("inputs 必须是 JSON 对象".to_string());
        }
        if let Ok(s) = serde_json::to_string(&inputs) {
            if s.len() > 1024 * 1024 {
                return Err("inputs 大小超过 1MB 限制".to_string());
            }
        }

        let server = state
            .db
            .get_server(&server_id)?
            .ok_or_else(|| format!("MCP server not found: {server_id}"))?;
        // 解密敏感字段
        let mut server = server;
        decrypt_server_fields(&mut server, &state.encrypt_passphrase);

        // 从共享连接池获取或创建客户端（按协议分发）
        let client = {
            let cache = state.client_cache.lock().await;
            cache.get(&server_id).cloned()
        };

        let client = if let Some(c) = client {
            c
        } else {
            let new_client = crate::gateway::create_mcp_client(
                    &server.protocol,
                    &server.command,
                    &server.args,
                    &server.env,
                    &server.endpoint,
                )
                .await
                .map_err(|e| {
                    state.circuit_breaker.record_failure(&server_id);
                    e
                })?;

            let client_arc =
                std::sync::Arc::new(tokio::sync::Mutex::new(new_client));
            let mut cache = state.client_cache.lock().await;
            cache
                .entry(server_id.clone())
                .or_insert(client_arc)
                .clone()
        };

        let result = client
            .lock()
            .await
            .call_tool(&tool_name, inputs)
            .await
            .map_err(|e| {
                state.circuit_breaker.record_failure(&server_id);
                e.to_string()
            })?;

        state.circuit_breaker.record_success(&server_id);

        Ok(serde_json::json!({
            "content": result.content,
            "isError": result.is_error.unwrap_or(false),
        }))
    }

    #[tauri::command]
    pub async fn intent_parse(
        state: tauri::State<'_, AppState>,
        text: String,
    ) -> Result<Workflow, String> {
        // 权限检查
        state.auth.require_role("intent_parse")?;

        let now = Utc::now().timestamp_millis();
        let workflow_id = uuid::Uuid::new_v4().to_string();

        // 收集可用的 MCP 工具信息
        let available_tools = gather_available_tools(&state);

        // 尝试 LLM 解析
        let llm_config = llm::LlmConfig::from_env();
        if llm_config.is_configured() {
            match llm::parse_intent_with_llm(&text, &available_tools).await {
                Ok(parsed) => {
                    return convert_parsed_to_workflow(parsed, workflow_id, now)
                        .map_err(|e| format!("LLM 解析结果转换失败: {e}"));
                }
                Err(e) => {
                    tracing::warn!("LLM 意图解析失败，回退到关键词匹配: {e}");
                }
            }
        }

        // 回退到关键词匹配
        intent_parse_keyword(&text, workflow_id, now)
    }

    /// 使用 LLM 进行意图解析（强制使用 LLM，不降级）
    #[tauri::command]
    pub async fn intent_parse_llm(
        state: tauri::State<'_, AppState>,
        text: String,
    ) -> Result<Workflow, String> {
        state.auth.require_role("intent_parse")?;

        let now = Utc::now().timestamp_millis();
        let workflow_id = uuid::Uuid::new_v4().to_string();
        let available_tools = gather_available_tools(&state);

        let parsed = llm::parse_intent_with_llm(&text, &available_tools).await?;

        convert_parsed_to_workflow(parsed, workflow_id, now)
            .map_err(|e| format!("LLM 解析结果转换失败: {e}"))
    }

    /// 多轮对话细化工作流
    #[tauri::command]
    pub async fn refine_workflow(
        state: tauri::State<'_, AppState>,
        current_workflow_json: String,
        conversation_history: Vec<Vec<String>>,
        refinement_text: String,
    ) -> Result<Workflow, String> {
        state.auth.require_role("intent_parse")?;

        let now = Utc::now().timestamp_millis();
        let workflow_id = uuid::Uuid::new_v4().to_string();
        let available_tools = gather_available_tools(&state);

        // 构建对话会话
        let mut session = llm::ConversationSession::new();
        for turn in &conversation_history {
            if turn.len() >= 2 {
                session.add_user_message(&turn[0]);
                session.add_assistant_message(&turn[1]);
            }
        }

        // 解析当前工作流
        if let Ok(parsed) = serde_json::from_str::<llm::ParsedWorkflow>(&current_workflow_json) {
            session.current_workflow = Some(parsed);
        }

        let parsed = llm::refine_workflow_with_llm(&session, &refinement_text, &available_tools).await?;

        convert_parsed_to_workflow(parsed, workflow_id, now)
            .map_err(|e| format!("工作流细化失败: {e}"))
    }

    /// 根据意图推荐 MCP 工具
    #[tauri::command]
    pub async fn recommend_tools(
        state: tauri::State<'_, AppState>,
        text: String,
    ) -> Result<Vec<String>, String> {
        state.auth.require_role("intent_parse")?;
        let available_tools = gather_available_tools(&state);
        llm::recommend_tools(&text, &available_tools).await
    }

    /// 收集所有已配置的 MCP 工具信息
    fn gather_available_tools(state: &AppState) -> Vec<llm::ToolInfo> {
        let mut tools = Vec::new();
        if let Ok(servers) = state.db.list_servers() {
            for server in servers {
                if server.enabled {
                    // 从服务器配置中提取工具信息
                    // 实际的工具列表可以从 MCP 服务器 ping 结果中获取
                    // 这里使用静态的常见工具信息作为基础
                    tools.push(llm::ToolInfo {
                        server_id: server.id.clone(),
                        name: server.name.clone(),
                        description: server.description.clone(),
                    });
                }
            }
        }

        // 如果没有配置服务器，返回通用工具模板
        if tools.is_empty() {
            tools = vec![
                llm::ToolInfo { server_id: "mcp-http".to_string(), name: "http_request".to_string(), description: "发送 HTTP 请求（GET/POST/PUT/DELETE）".to_string() },
                llm::ToolInfo { server_id: "mcp-fs".to_string(), name: "file_reader".to_string(), description: "读取文件内容".to_string() },
                llm::ToolInfo { server_id: "mcp-fs".to_string(), name: "file_writer".to_string(), description: "写入内容到文件".to_string() },
                llm::ToolInfo { server_id: "mcp-data".to_string(), name: "json_transform".to_string(), description: "JSON 数据转换与映射".to_string() },
                llm::ToolInfo { server_id: "mcp-data".to_string(), name: "csv_parser".to_string(), description: "解析 CSV 数据".to_string() },
                llm::ToolInfo { server_id: "mcp-ai".to_string(), name: "text_ai".to_string(), description: "AI 文本生成与处理".to_string() },
                llm::ToolInfo { server_id: "mcp-notify".to_string(), name: "webhook_send".to_string(), description: "发送 Webhook 通知".to_string() },
                llm::ToolInfo { server_id: "mcp-db".to_string(), name: "db_query".to_string(), description: "执行数据库查询".to_string() },
            ];
        }

        tools
    }

    /// 将 LLM 解析结果转换为 Workflow 实体
    fn convert_parsed_to_workflow(
        parsed: llm::ParsedWorkflow,
        workflow_id: String,
        now: i64,
    ) -> Result<Workflow, String> {
        let nodes: Vec<WorkflowNode> = parsed
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| WorkflowNode {
                id: format!("node_{i}"),
                node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition {
                    x: n.position_x,
                    y: n.position_y,
                },
                data: WorkflowNodeData {
                    label: n.label.clone(),
                    tool: None,
                    inputs: n.inputs.clone(),
                    outputs: serde_json::json!({}),
                    config: serde_json::json!({
                        "toolName": n.tool_name,
                        "serverId": n.server_id,
                    }),
                },
            })
            .collect();

        let edges: Vec<WorkflowEdge> = parsed
            .edges
            .iter()
            .enumerate()
            .map(|(i, e)| WorkflowEdge {
                id: format!("edge_{i}"),
                source: format!("node_{}", e.source_index),
                target: format!("node_{}", e.target_index),
                source_handle: None,
                target_handle: None,
                edge_type: Some("smoothstep".to_string()),
                animated: Some(true),
            })
            .collect();

        tracing::info!(
            action = "intent_parse",
            text_len = parsed.description.len(),
            node_count = nodes.len(),
            edge_count = edges.len(),
            source = "llm",
            "LLM 意图解析完成"
        );

        Ok(Workflow {
            id: workflow_id,
            name: parsed.name,
            description: parsed.description,
            mode: "intent".to_string(),
            status: "idle".to_string(),
            nodes,
            edges,
            locked: false,
            locked_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// 关键词匹配的意图解析（LLM 不可用时的回退方案）
    fn intent_parse_keyword(
        text: &str,
        workflow_id: String,
        now: i64,
    ) -> Result<Workflow, String> {
        let text_lower = text.to_lowercase();

        // 检测关键词
        let has_http = [
            "request", "api", "http", "fetch", "get", "post", "url", "web", "rest",
        ]
        .iter()
        .any(|kw| text_lower.contains(kw));
        let has_data = [
            "csv",
            "json",
            "transform",
            "parse",
            "data",
            "filter",
            "aggregate",
            "excel",
            "convert",
            "extract",
        ]
        .iter()
        .any(|kw| text_lower.contains(kw));
        let has_ai = [
            "ai",
            "gpt",
            "generate",
            "translate",
            "summarize",
            "text",
            "model",
            "prompt",
            "llm",
            "chat",
        ]
        .iter()
        .any(|kw| text_lower.contains(kw));
        let has_file = [
            "file", "read", "write", "save", "load", "download", "export", "import",
        ]
        .iter()
        .any(|kw| text_lower.contains(kw));
        let has_notify = [
            "notify", "webhook", "send", "message", "alert", "email", "push",
        ]
        .iter()
        .any(|kw| text_lower.contains(kw));
        let has_db = [
            "db", "sql", "query", "database", "table", "select", "insert", "update", "delete",
        ]
        .iter()
        .any(|kw| text_lower.contains(kw));

        let mut nodes: Vec<WorkflowNode> = Vec::new();
        let mut edges: Vec<WorkflowEdge> = Vec::new();
        let mut node_idx = 0u32;

        let mut add_node =
            |tool_name: &str, server_id: &str, label: &str, x: f64, y: f64| -> String {
                let id = format!("node_{node_idx}");
                node_idx += 1;
                nodes.push(WorkflowNode {
                    id: id.clone(),
                    node_type: "mcpTool".to_string(),
                    position: WorkflowNodePosition { x, y },
                    data: WorkflowNodeData {
                        label: label.to_string(),
                        tool: None,
                        inputs: serde_json::json!({}),
                        outputs: serde_json::json!({}),
                        config: serde_json::json!({ "toolName": tool_name, "serverId": server_id }),
                    },
                });
                id
            };

        let mut add_edge = |source: &str, target: &str| {
            edges.push(WorkflowEdge {
                id: uuid::Uuid::new_v4().to_string(),
                source: source.to_string(),
                target: target.to_string(),
                source_handle: None,
                target_handle: None,
                edge_type: Some("smoothstep".to_string()),
                animated: Some(true),
            });
        };

        match (has_http, has_data, has_ai, has_file, has_db, has_notify) {
            (true, true, true, true, _, true) => {
                let n1 = add_node("http_request", "mcp-http", "HTTP 请求", 100.0, 100.0);
                let n2 = add_node("json_transform", "mcp-data", "数据转换", 400.0, 100.0);
                let n3 = add_node("text_ai", "mcp-ai", "AI 文本生成", 700.0, 100.0);
                let n4 = add_node("file_writer", "mcp-fs", "保存文件", 1000.0, 100.0);
                let n5 = add_node("webhook_send", "mcp-notify", "发送通知", 1300.0, 100.0);
                add_edge(&n1, &n2);
                add_edge(&n2, &n3);
                add_edge(&n3, &n4);
                add_edge(&n4, &n5);
            }
            (true, true, true, true, _, _) => {
                let n1 = add_node("http_request", "mcp-http", "HTTP 请求", 100.0, 100.0);
                let n2 = add_node("json_transform", "mcp-data", "数据转换", 400.0, 100.0);
                let n3 = add_node("text_ai", "mcp-ai", "AI 文本生成", 700.0, 100.0);
                let n4 = add_node("file_writer", "mcp-fs", "保存文件", 1000.0, 100.0);
                add_edge(&n1, &n2);
                add_edge(&n2, &n3);
                add_edge(&n3, &n4);
            }
            (true, true, _, true, _, _) => {
                let n1 = add_node("http_request", "mcp-http", "HTTP 请求", 100.0, 100.0);
                let n2 = add_node("csv_parser", "mcp-data", "CSV 解析", 400.0, 100.0);
                let n3 = add_node("file_writer", "mcp-fs", "保存文件", 700.0, 100.0);
                add_edge(&n1, &n2);
                add_edge(&n2, &n3);
            }
            (true, true, _, _, _, _) => {
                let n1 = add_node("http_request", "mcp-http", "HTTP 请求", 100.0, 100.0);
                let n2 = add_node("json_transform", "mcp-data", "数据转换", 400.0, 100.0);
                add_edge(&n1, &n2);
            }
            (true, _, _, true, _, _) => {
                let n1 = add_node("http_request", "mcp-http", "HTTP 请求", 100.0, 100.0);
                let n2 = add_node("file_writer", "mcp-fs", "保存文件", 400.0, 100.0);
                add_edge(&n1, &n2);
            }
            (_, true, _, true, true, _) => {
                let n1 = add_node("db_query", "mcp-db", "数据库查询", 100.0, 100.0);
                let n2 = add_node("csv_parser", "mcp-data", "CSV 解析", 400.0, 100.0);
                let n3 = add_node("file_writer", "mcp-fs", "保存文件", 700.0, 100.0);
                add_edge(&n1, &n2);
                add_edge(&n2, &n3);
            }
            (_, _, _, _, true, true) => {
                let n1 = add_node("db_query", "mcp-db", "数据库查询", 100.0, 100.0);
                let n2 = add_node("webhook_send", "mcp-notify", "发送通知", 400.0, 100.0);
                add_edge(&n1, &n2);
            }
            (_, _, true, true, _, _) => {
                let n1 = add_node("file_reader", "mcp-fs", "读取文件", 100.0, 100.0);
                let n2 = add_node("text_ai", "mcp-ai", "AI 文本生成", 400.0, 100.0);
                let n3 = add_node("file_writer", "mcp-fs", "保存文件", 700.0, 100.0);
                add_edge(&n1, &n2);
                add_edge(&n2, &n3);
            }
            (true, _, _, _, _, _) => {
                add_node("http_request", "mcp-http", "HTTP 请求", 100.0, 100.0);
            }
            (_, _, _, _, true, _) => {
                add_node("db_query", "mcp-db", "数据库查询", 100.0, 100.0);
            }
            (_, _, true, _, _, _) => {
                add_node("text_ai", "mcp-ai", "AI 文本生成", 100.0, 100.0);
            }
            (_, _, _, true, _, _) => {
                add_node("file_reader", "mcp-fs", "读取文件", 100.0, 100.0);
            }
            (_, _, _, _, _, true) => {
                add_node("webhook_send", "mcp-notify", "发送通知", 100.0, 100.0);
            }
            _ => {
                add_node("http_request", "mcp-http", "HTTP 请求", 100.0, 100.0);
            }
        }

        tracing::info!(
            action = "intent_parse",
            text_len = text.len(),
            node_count = nodes.len(),
            edge_count = edges.len(),
            source = "keyword",
            "关键词意图解析完成"
        );

        Ok(Workflow {
            id: workflow_id,
            name: format!("Intent: {}", text.chars().take(40).collect::<String>().trim()),
            description: text.to_string(),
            mode: "intent".to_string(),
            status: "idle".to_string(),
            nodes,
            edges,
            locked: false,
            locked_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    #[tauri::command]
    pub fn runtime_status() -> Result<serde_json::Value, String> {
        let running = orchestrator::scheduler::is_running();
        Ok(serde_json::json!({
            "status": if running { "running" } else { "idle" },
            "message": if running { "工作流执行中" } else { "就绪" },
        }))
    }

    #[tauri::command]
    pub fn runtime_stop(workflow_id: Option<String>) -> Result<(), String> {
        // runtime_stop 无 state 参数，权限检查在调用方
        match workflow_id {
            Some(id) => orchestrator::scheduler::request_abort(&id),
            None => orchestrator::scheduler::request_abort_all(),
        }
        Ok(())
    }

    // ============================================================
    // 插件市场命令
    // ============================================================

    /// 列出市场模板（优先远程仓库，失败时回退内置模板）
    #[tauri::command]
    pub async fn list_marketplace_templates(
        state: tauri::State<'_, AppState>,
        category: Option<String>,
        search: Option<String>,
    ) -> Result<Vec<marketplace::MarketplaceTemplate>, String> {
        state.auth.require_role("workflow.list")?;

        let config = marketplace::RegistryConfig::from_env();
        let registry = marketplace::TemplateRegistry::new(config);

        match registry.list_templates(category.as_deref(), search.as_deref()).await {
            Ok(templates) => Ok(templates),
            Err(e) => {
                tracing::warn!("远程模板仓库不可用: {e}，回退到内置模板");
                let mut templates = marketplace::builtin_templates();
                if let Some(cat) = &category {
                    if !cat.is_empty() && cat != "all" {
                        templates.retain(|t| t.category == *cat);
                    }
                }
                if let Some(q) = &search {
                    if !q.is_empty() {
                        let q_lower = q.to_lowercase();
                        templates.retain(|t| {
                            t.name.to_lowercase().contains(&q_lower)
                                || t.description.to_lowercase().contains(&q_lower)
                                || t.tags.iter().any(|tag| tag.to_lowercase().contains(&q_lower))
                        });
                    }
                }
                Ok(templates)
            }
        }
    }

    /// 获取模板详情（含工作流定义）
    #[tauri::command]
    pub async fn get_marketplace_template(
        state: tauri::State<'_, AppState>,
        template_id: String,
        version: Option<String>,
    ) -> Result<marketplace::TemplateDetail, String> {
        state.auth.require_role("workflow.list")?;

        let version = version.unwrap_or_else(|| "1.0.0".to_string());
        let config = marketplace::RegistryConfig::from_env();
        let registry = marketplace::TemplateRegistry::new(config);

        match registry.get_template_detail(&template_id, &version).await {
            Ok(detail) => Ok(detail),
            Err(e) => {
                tracing::warn!("远程模板详情获取失败: {e}，回退到内置模板");
                marketplace::builtin_template_detail(&template_id)
                    .ok_or_else(|| format!("模板不存在: {}", template_id))
            }
        }
    }

    /// 一键安装模板：从模板创建新的工作流并保存到数据库
    #[tauri::command]
    pub async fn install_template(
        state: tauri::State<'_, AppState>,
        template_id: String,
        version: Option<String>,
    ) -> Result<super::storage::sqlite::Workflow, String> {
        state.auth.require_role("workflow.save")?;

        let version = version.unwrap_or_else(|| "1.0.0".to_string());
        let config = marketplace::RegistryConfig::from_env();
        let registry = marketplace::TemplateRegistry::new(config);

        // 获取模板详情
        let detail = match registry.get_template_detail(&template_id, &version).await {
            Ok(d) => d,
            Err(e) => {
                tracing::warn!("远程模板详情获取失败: {e}，回退到内置模板");
                marketplace::builtin_template_detail(&template_id)
                    .ok_or_else(|| format!("模板不存在: {}", template_id))?
            }
        };

        let workflow_id = uuid::Uuid::new_v4().to_string();
        let workflow = marketplace::template_to_workflow(&detail, &workflow_id);

        // 保存到数据库
        state.db.insert_or_replace_workflow(&workflow)?;

        // 记录安装
        marketplace::record_install(&state.db, &template_id, &workflow_id, &version)?;

        // 审计日志
        let _ = state.db.insert_audit_log(&new_audit_log(
            "marketplace.install",
            &format!("template:{}", template_id),
            serde_json::json!({
                "template_id": template_id,
                "workflow_id": workflow_id,
                "version": version,
            })
            .to_string(),
        ));

        tracing::info!(
            template_id = %template_id,
            workflow_id = %workflow_id,
            version = %version,
            "模板安装成功"
        );

        Ok(workflow)
    }

    /// 检查已安装模板是否有更新
    #[tauri::command]
    pub async fn check_template_updates(
        state: tauri::State<'_, AppState>,
    ) -> Result<Vec<marketplace::TemplateUpdate>, String> {
        state.auth.require_role("workflow.list")?;

        let config = marketplace::RegistryConfig::from_env();
        let registry = marketplace::TemplateRegistry::new(config);

        marketplace::check_updates(&state.db, &registry).await
    }

    /// 强制释放工作流执行锁（管理员操作，用于解锁死锁）。
    #[tauri::command]
    pub fn force_release_lock(
        state: tauri::State<AppState>,
        workflow_id: String,
    ) -> Result<(), String> {
        state.auth.require_role("workflow.force_unlock")?;
        state
            .db
            .force_release_workflow_lock(&workflow_id)
            .map_err(|e| format!("释放锁失败: {e}"))?;

        tracing::warn!(workflow_id = %workflow_id, "管理员强制释放了工作流执行锁");

        // 审计日志
        if let Err(e) = state.db.insert_audit_log(&new_audit_log(
            "workflow.force_unlock",
            &format!("workflow:{}", workflow_id),
            serde_json::json!({"reason": "manual"}).to_string(),
        )) {
            tracing::error!(action = "audit_log_write", error = %e, "写入审计日志失败");
        }

        Ok(())
    }

    /// 断点续传：从上次失败的执行记录恢复执行。
    #[tauri::command]
    pub async fn retry_workflow(
        app: tauri::AppHandle,
        state: tauri::State<'_, AppState>,
        id: String,
        resume_from_execution_id: String,
    ) -> Result<orchestrator::OrchestrationResult, String> {
        state.auth.require_role("workflow.retry")?;
        execute_workflow(app, state, id, None, Some(resume_from_execution_id)).await
    }

    #[tauri::command]
    pub fn health_check(state: tauri::State<AppState>) -> Result<serde_json::Value, String> {
        state.auth.require_role("health")?;
        state.rate_limiter.check("health_check")?;
        // 数据库连通性检查
        let db_ok = state.db.list_servers().is_ok();
        let running = orchestrator::scheduler::is_running();

        Ok(serde_json::json!({
            "status": "ok",
            "database": db_ok,
            "runtime": if running { "running" } else { "idle" },
            "version": env!("CARGO_PKG_VERSION"),
            "timestamp": Utc::now().timestamp_millis(),
        }))
    }

    /// 备份数据库到指定路径
    #[tauri::command]
    pub fn backup_database(
        state: tauri::State<AppState>,
        backup_path: Option<String>,
    ) -> Result<String, String> {
        state.auth.require_role("backup")?;
        let src = &state.db_path;
        if src == ":memory:" {
            return Err("内存数据库不支持备份".to_string());
        }
        let dst = backup_path.unwrap_or_else(|| {
            let ts = Utc::now().format("%Y%m%d_%H%M%S");
            format!("{src}.backup.{ts}")
        });
        // 路径安全校验：只允许在备份目录内的相对路径，禁止绝对路径和路径遍历
        let backup_dir = std::path::Path::new(&dirs_db_path())
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("backups");
        let dst_path = backup_dir.join(&dst);
        // 规范化后检查是否仍在 backup_dir 内
        let canonical_dst = dst_path.canonicalize().unwrap_or_else(|_| dst_path.clone());
        let canonical_dir = backup_dir.canonicalize().unwrap_or_else(|_| backup_dir.clone());
        if !canonical_dst.starts_with(&canonical_dir) {
            return Err("备份路径不允许超出备份目录".to_string());
        }
        // 确保备份目录存在
        std::fs::create_dir_all(&backup_dir)
            .map_err(|e| format!("创建备份目录失败: {e}"))?;
        std::fs::copy(src, &canonical_dst).map_err(|e| format!("备份失败: {e}"))?;
        tracing::info!(action = "backup_database", src = %src, dst = %canonical_dst.display(), "数据库备份完成");
        Ok(canonical_dst.display().to_string())
    }

    /// 从备份文件恢复数据库
    #[tauri::command]
    pub fn restore_database(
        state: tauri::State<AppState>,
        backup_path: String,
    ) -> Result<(), String> {
        state.auth.require_role("restore")?;
        let dst = &state.db_path;
        if dst == ":memory:" {
            return Err("内存数据库不支持恢复".to_string());
        }
        // 路径安全校验：只允许从备份目录内恢复
        let backup_dir = std::path::Path::new(&dirs_db_path())
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("backups");
        let src_path = backup_dir.join(&backup_path);
        let canonical_src = src_path.canonicalize().unwrap_or_else(|_| src_path.clone());
        let canonical_dir = backup_dir.canonicalize().unwrap_or_else(|_| backup_dir.clone());
        if !canonical_src.starts_with(&canonical_dir) {
            return Err("恢复路径不允许超出备份目录".to_string());
        }
        if !canonical_src.exists() {
            return Err(format!("备份文件不存在: {}", canonical_src.display()));
        }
        tracing::warn!(action = "restore_database", src = %canonical_src.display(), dst = %dst, "从备份恢复数据库");
        std::fs::copy(&canonical_src, dst).map_err(|e| format!("恢复失败: {e}"))?;
        tracing::info!(action = "restore_database", "数据库恢复完成，请重启应用");
        Ok(())
    }

    // ============================================================
    // 审计日志查询
    // ============================================================

    #[tauri::command]
    pub fn list_audit_logs(
        state: tauri::State<AppState>,
        offset: usize,
        limit: usize,
    ) -> Result<storage::sqlite::PaginatedResult<storage::sqlite::AuditLog>, String> {
        state.auth.require_role("execution.list")?;
        state.db.list_audit_logs(offset, limit)
    }

    #[tauri::command]
    pub fn search_audit_logs(
        state: tauri::State<AppState>,
        action: Option<String>,
        resource: Option<String>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        offset: usize,
        limit: usize,
    ) -> Result<storage::sqlite::PaginatedResult<storage::sqlite::AuditLog>, String> {
        state.auth.require_role("execution.list")?;
        state.db.search_audit_logs(
            action.as_deref(),
            resource.as_deref(),
            start_time,
            end_time,
            offset,
            limit,
        )
    }

    // ============================================================
    // 工作流执行记录查询
    // ============================================================

    #[tauri::command]
    pub fn list_executions(
        state: tauri::State<AppState>,
        workflow_id: String,
        offset: usize,
        limit: usize,
    ) -> Result<storage::sqlite::PaginatedResult<storage::sqlite::WorkflowExecution>, String> {
        state.auth.require_role("execution.list")?;
        state.db.list_executions_by_workflow(&workflow_id, offset, limit)
    }

    // ============================================================
    // 认证与授权命令
    // ============================================================

    #[tauri::command]
    pub fn auth_init(state: tauri::State<AppState>) -> Result<serde_json::Value, String> {
        // 从数据库加载持久化的 API Key
        if let Some(token) = state.db.get_auth_token("default")? {
            state.auth.set_key_hash(&token.key_hash);
            tracing::info!("从数据库加载了持久化的 API Key");
        }
        Ok(serde_json::json!({
            "configured": state.auth.is_set(),
        }))
    }

    #[tauri::command]
    pub fn auth_generate_key(state: tauri::State<AppState>) -> Result<String, String> {
        let key = state.auth.generate_key();
        // 持久化到数据库
        let hash = {
            let guard = state.auth.api_key_hash.lock().unwrap();
            guard.clone().unwrap_or_default()
        };
        let role = state.auth.get_role();
        let token = storage::sqlite::AuthToken {
            id: "default".to_string(),
            key_hash: hash,
            label: "default".to_string(),
            role: role.as_str().to_string(),
            created_at: Utc::now().timestamp_millis(),
        };
        state.db.save_auth_token(&token)?;
        // 审计日志
        if let Err(e) = state.db.insert_audit_log(&new_audit_log(
            "auth.generate_key",
            "auth",
            serde_json::json!({"role": role.as_str()}).to_string(),
        )) {
            tracing::error!(action = "audit_log_write", error = %e, "写入审计日志失败");
        }
        tracing::info!(action = "auth_generate_key", "API Key 已生成并持久化");
        Ok(key)
    }

    #[tauri::command]
    pub fn auth_set_key(state: tauri::State<AppState>, key: String) -> Result<(), String> {
        if key.trim().is_empty() {
            return Err("API Key 不能为空".to_string());
        }
        state.auth.set_key(&key);
        // 持久化到数据库
        let hash = {
            let guard = state.auth.api_key_hash.lock().unwrap();
            guard.clone().unwrap_or_default()
        };
        let role = state.auth.get_role();
        let token = storage::sqlite::AuthToken {
            id: "default".to_string(),
            key_hash: hash,
            label: "default".to_string(),
            role: role.as_str().to_string(),
            created_at: Utc::now().timestamp_millis(),
        };
        state.db.save_auth_token(&token)?;
        // 审计日志
        if let Err(e) = state.db.insert_audit_log(&new_audit_log(
            "auth.set_key",
            "auth",
            serde_json::json!({"role": role.as_str()}).to_string(),
        )) {
            tracing::error!(action = "audit_log_write", error = %e, "写入审计日志失败");
        }
        tracing::info!(action = "auth_set_key", "API Key 已设置并持久化");
        Ok(())
    }

    #[tauri::command]
    pub fn auth_verify_key(state: tauri::State<AppState>, key: String) -> Result<bool, String> {
        state.rate_limiter.check("auth_verify_key")?;
        Ok(state.auth.verify(&key))
    }

    #[tauri::command]
    pub fn auth_status(state: tauri::State<AppState>) -> Result<serde_json::Value, String> {
        let role = state.auth.get_role();
        Ok(serde_json::json!({
            "configured": state.auth.is_set(),
            "role": role.as_str(),
        }))
    }

    #[tauri::command]
    pub fn auth_clear_key(state: tauri::State<AppState>) -> Result<(), String> {
        state.auth.clear();
        state.db.delete_auth_token("default")?;
        // 审计日志
        if let Err(e) = state.db.insert_audit_log(&new_audit_log(
            "auth.clear_key",
            "auth",
            "{}".to_string(),
        )) {
            tracing::error!(action = "audit_log_write", error = %e, "写入审计日志失败");
        }
        tracing::info!(action = "auth_clear_key", "API Key 已清除");
        Ok(())
    }

    // ============================================================
    // RBAC 角色管理命令
    // ============================================================

    /// 设置当前用户角色（仅 Admin 可操作）
    #[tauri::command]
    pub fn auth_set_role(
        state: tauri::State<AppState>,
        role: String,
    ) -> Result<(), String> {
        // 只有 Admin 可以切换角色
        let current_role = state.auth.get_role();
        if current_role != UserRole::Admin {
            return Err(format!(
                "权限不足：只有 Admin 可以切换角色，当前角色为 '{}'",
                current_role.as_str()
            ));
        }
        let valid_roles = ["admin", "developer", "viewer"];
        if !valid_roles.contains(&role.as_str()) {
            return Err(format!(
                "无效的角色 '{}'，有效角色: {}",
                role,
                valid_roles.join(", ")
            ));
        }
        state.auth.set_role(&role);
        // 审计日志
        if let Err(e) = state.db.insert_audit_log(&new_audit_log(
            "auth.set_role",
            "auth",
            serde_json::json!({"role": role}).to_string(),
        )) {
            tracing::error!(action = "audit_log_write", error = %e, "写入审计日志失败");
        }
        tracing::info!(action = "auth_set_role", role = %role, "角色已切换");
        Ok(())
    }

    /// 获取当前用户角色
    #[tauri::command]
    pub fn auth_get_role(state: tauri::State<AppState>) -> Result<serde_json::Value, String> {
        let role = state.auth.get_role();
        Ok(serde_json::json!({
            "role": role.as_str(),
            "permissions": {
                "servers": role.can("server.list"),
                "servers_manage": role.can("server.add"),
                "workflows": role.can("workflow.list"),
                "workflows_manage": role.can("workflow.save"),
                "workflows_execute": role.can("workflow.execute"),
                "executions": role.can("execution.list"),
                "backup": role.can("backup"),
                "restore": role.can("restore"),
            }
        }))
    }

    /// 验证审计日志哈希链完整性（合规审计）
    #[tauri::command]
    pub fn verify_audit_chain(
        state: tauri::State<AppState>,
    ) -> Result<serde_json::Value, String> {
        state.auth.require_role("execution.list")?;
        let (total, valid, invalid_details) = state.db.verify_audit_chain()?;
        Ok(serde_json::json!({
            "total": total,
            "valid": valid,
            "invalid": total - valid,
            "intact": total == valid,
            "details": invalid_details,
        }))
    }

    /// 导出 Prometheus 格式的指标数据（监控端点，需要 admin 权限）
    #[tauri::command]
    pub fn metrics(state: tauri::State<AppState>) -> Result<String, String> {
        state.auth.require_role("metrics.read")?;
        Ok(crate::metrics::gather_metrics())
    }
}

// ============================================================
// 启动（仅在 tauri-runtime feature 启用时编译）
// ============================================================

#[cfg(feature = "tauri-runtime")]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统（必须在持有 _log_guards 期间有效）
    let _log_guards = init_logging();

    let db_path = dirs_db_path();

    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let db = Database::open(&db_path).unwrap_or_else(|e| {
        tracing::error!("无法打开数据库 {db_path}: {e}. 使用内存数据库降级运行.");
        Database::open(":memory:").expect("无法打开内存数据库")
    });

    // 运行数据库迁移
    if let Err(e) = db.run_migrations() {
        tracing::error!("数据库迁移失败: {e}");
    }

    use commands::*;

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(AppState {
            db,
            db_path: db_path.clone(),
            auth: AuthManager::new(),
            rate_limiter: RateLimiter::new(),
            circuit_breaker: CircuitBreaker::new(),
            encrypt_passphrase: derive_encryption_passphrase(),
            client_cache: tokio::sync::Mutex::new(std::collections::HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![
            list_servers,
            list_servers_paginated,
            add_server,
            update_server,
            remove_server,
            get_server,
            list_workflows,
            list_workflows_paginated,
            save_workflow,
            remove_workflow,
            get_workflow,
            execute_workflow,
            list_tools,
            execute_tool,
            ping_server,
            intent_parse,
            intent_parse_llm,
            refine_workflow,
            recommend_tools,
            runtime_status,
            runtime_stop,
            health_check,
            backup_database,
            restore_database,
            auth_generate_key,
            auth_set_key,
            auth_verify_key,
            auth_status,
            auth_clear_key,
            auth_init,
            auth_set_role,
            auth_get_role,
            verify_audit_chain,
            metrics,
            list_audit_logs,
            search_audit_logs,
            list_executions,
            force_release_lock,
            retry_workflow,
            list_marketplace_templates,
            get_marketplace_template,
            install_template,
            check_template_updates,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    tracing::info!("MCP Fusion 启动完成");

    app.run(|_handle, event| {
        if let tauri::RunEvent::Exit = event {
            tracing::info!("MCP Fusion 正在关闭，清理资源...");
            // 请求中止所有正在运行的工作流
            orchestrator::scheduler::request_abort_all();
            // 刷新 OpenTelemetry 追踪数据
            tracing_otel::shutdown_tracer();
            // 清理子进程在 StdioClient::Drop 中自动执行
            tracing::info!("MCP Fusion 已关闭");
        }
    });
}

fn dirs_db_path() -> String {
    if let Ok(dir) = std::env::var("APPDATA") {
        format!("{dir}/mcp-fusion/mcp_fusion.db")
    } else if let Ok(dir) = std::env::var("HOME") {
        format!("{dir}/.local/share/mcp-fusion/mcp_fusion.db")
    } else {
        "mcp_fusion.db".to_string()
    }
}

/// 获取加密密钥：优先使用环境变量 MCP_FUSION_ENCRYPTION_KEY，
/// 否则使用默认值（生产环境必须设置环境变量）
fn derive_encryption_passphrase() -> String {
    std::env::var("MCP_FUSION_ENCRYPTION_KEY").unwrap_or_else(|_| {
        tracing::warn!(
            "MCP_FUSION_ENCRYPTION_KEY 未设置，使用默认加密密钥。生产环境请务必设置此环境变量！"
        );
        "mcp-fusion-default-key-change-in-production".to_string()
    })
}

/// 加密服务器敏感字段（env、endpoint、command）
#[cfg(feature = "tauri-runtime")]
fn encrypt_server_fields(server: &mut McpServer, passphrase: &str) {
    let env_json = serde_json::to_string(&server.env).unwrap_or_default();
    if !env_json.is_empty() && env_json != "{}" {
        if let Ok(encrypted) = crate::crypto::encrypt(&env_json, passphrase) {
            server.env = {
                let mut m = std::collections::HashMap::new();
                m.insert("__encrypted__".to_string(), encrypted);
                m
            };
        }
    }
}

/// 解密服务器敏感字段
#[cfg(feature = "tauri-runtime")]
fn decrypt_server_fields(server: &mut McpServer, passphrase: &str) {
    if let Some(encrypted) = server.env.get("__encrypted__") {
        if let Ok(decrypted) = crate::crypto::decrypt(encrypted, passphrase) {
            if let Ok(env) = serde_json::from_str(&decrypted) {
                server.env = env;
            }
        }
    }
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allow_requests_within_limit() {
        let rl = RateLimiter::new();
        // 连续 5 次都应通过（execute_workflow 限制 5/min）
        for _ in 0..5 {
            assert!(rl.check("execute_workflow").is_ok(), "前 5 次请求应通过");
        }
    }

    #[test]
    fn test_rate_limiter_blocks_exceeding_requests() {
        let rl = RateLimiter::new();
        for _ in 0..5 {
            assert!(rl.check("execute_workflow").is_ok());
        }
        // 第 6 次应被拒绝
        assert!(rl.check("execute_workflow").is_err(), "第 6 次请求应被限流");
    }

    #[test]
    fn test_rate_limiter_different_operations() {
        let rl = RateLimiter::new();
        // 不同操作独立计数
        assert!(rl.check("execute_workflow").is_ok());
        assert!(rl.check("execute_tool").is_ok());
        assert!(rl.check("list_tools").is_ok());
    }

    #[test]
    fn test_rate_limiter_fallback_rule() {
        let rl = RateLimiter::new();
        // 未定义规则的操作使用默认 10/min
        for _ in 0..10 {
            assert!(rl.check("unknown_operation").is_ok());
        }
        assert!(rl.check("unknown_operation").is_err());
    }

    #[test]
    fn test_circuit_breaker_initially_closed() {
        let cb = CircuitBreaker::new();
        assert!(cb.allow_request("test_service"), "初始状态应允许请求");
    }

    #[test]
    fn test_circuit_breaker_opens_after_failures() {
        let cb = CircuitBreaker::new();
        // 连续 3 次失败 → 熔断打开
        cb.record_failure("test_service");
        cb.record_failure("test_service");
        cb.record_failure("test_service");
        assert!(!cb.allow_request("test_service"), "熔断器应打开，拒绝请求");
    }

    #[test]
    fn test_circuit_breaker_resets_on_success() {
        let cb = CircuitBreaker::new();
        cb.record_failure("test_service");
        cb.record_failure("test_service");
        cb.record_success("test_service");
        // 成功重置后，失败计数归零
        assert!(cb.allow_request("test_service"), "成功重置后应允许请求");
    }

    #[test]
    fn test_circuit_breaker_isolated_services() {
        let cb = CircuitBreaker::new();
        // 服务 A 熔断，不影响服务 B
        cb.record_failure("service_a");
        cb.record_failure("service_a");
        cb.record_failure("service_a");
        assert!(!cb.allow_request("service_a"));
        assert!(cb.allow_request("service_b"), "独立服务不应受影响");
    }
}
