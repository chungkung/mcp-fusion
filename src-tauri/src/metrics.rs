// ============================================================
// Prometheus 指标监控模块
// 关键指标：工作流执行耗时、成功率、节点失败率、API 调用延迟
// ============================================================

use lazy_static::lazy_static;
use prometheus::{
    self, register_counter_vec, register_gauge, register_histogram_vec, register_int_gauge,
    CounterVec, Encoder, Gauge, HistogramVec, IntGauge, TextEncoder,
};

lazy_static! {
    /// 工作流执行计数（按状态分：success / failed / timeout）
    pub static ref WORKFLOW_EXECUTIONS_TOTAL: CounterVec = register_counter_vec!(
        "mcp_fusion_workflow_executions_total",
        "工作流执行总次数",
        &["status"]
    )
    .unwrap();

    /// 工作流执行耗时直方图（秒）
    pub static ref WORKFLOW_EXECUTION_DURATION: HistogramVec = register_histogram_vec!(
        "mcp_fusion_workflow_execution_duration_seconds",
        "工作流执行耗时（秒）",
        &[],
        vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0, 120.0]
    )
    .unwrap();

    /// 节点执行计数（按状态分：success / failed / skipped / timeout）
    pub static ref NODE_EXECUTIONS_TOTAL: CounterVec = register_counter_vec!(
        "mcp_fusion_node_executions_total",
        "节点执行总次数",
        &["status"]
    )
    .unwrap();

    /// 节点执行耗时直方图（秒）
    pub static ref NODE_EXECUTION_DURATION: HistogramVec = register_histogram_vec!(
        "mcp_fusion_node_execution_duration_seconds",
        "节点执行耗时（秒）",
        &[],
        vec![0.05, 0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0]
    )
    .unwrap();

    /// MCP 工具调用计数（按状态分：success / failed）
    pub static ref MCP_TOOL_CALLS_TOTAL: CounterVec = register_counter_vec!(
        "mcp_fusion_mcp_tool_calls_total",
        "MCP 工具调用总次数",
        &["status"]
    )
    .unwrap();

    /// MCP 工具调用耗时直方图（秒）
    pub static ref MCP_TOOL_CALL_DURATION: HistogramVec = register_histogram_vec!(
        "mcp_fusion_mcp_tool_call_duration_seconds",
        "MCP 工具调用耗时（秒）",
        &[],
        vec![0.01, 0.05, 0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0]
    )
    .unwrap();

    /// MCP 服务器连接状态（0=disconnected, 1=connected, 2=error）
    pub static ref MCP_SERVER_CONNECTION_STATUS: Gauge = register_gauge!(
        "mcp_fusion_mcp_server_connection_status",
        "MCP 服务器连接状态（0=disconnected, 1=connected, 2=error）"
    )
    .unwrap();

    /// 当前活跃工作流数量
    pub static ref ACTIVE_WORKFLOWS: IntGauge = register_int_gauge!(
        "mcp_fusion_active_workflows",
        "当前正在执行的工作流数量"
    )
    .unwrap();
}

/// 导出 Prometheus 文本格式的指标数据
pub fn gather_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap_or_else(|_| "# UTF-8 error\n".to_string())
}

// ============================================================
// 便捷记录函数
// ============================================================

/// 记录工作流执行结果
pub fn record_workflow_execution(status: &str, duration_secs: f64) {
    WORKFLOW_EXECUTIONS_TOTAL.with_label_values(&[status]).inc();
    WORKFLOW_EXECUTION_DURATION
        .with_label_values(&[])
        .observe(duration_secs);
}

/// 记录节点执行结果
pub fn record_node_execution(status: &str, duration_secs: f64) {
    NODE_EXECUTIONS_TOTAL.with_label_values(&[status]).inc();
    NODE_EXECUTION_DURATION
        .with_label_values(&[])
        .observe(duration_secs);
}

/// 记录 MCP 工具调用结果
pub fn record_tool_call(status: &str, duration_secs: f64) {
    MCP_TOOL_CALLS_TOTAL.with_label_values(&[status]).inc();
    MCP_TOOL_CALL_DURATION
        .with_label_values(&[])
        .observe(duration_secs);
}

/// 设置活跃工作流数量
pub fn set_active_workflows(count: i64) {
    ACTIVE_WORKFLOWS.set(count);
}
