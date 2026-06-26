use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod scheduler;

// 从 scheduler 重新导出（仅在 tauri-runtime 下被 lib.rs 使用）
#[cfg(feature = "tauri-runtime")]
pub use scheduler::{NodeState, Scheduler, SchedulerConfig};

// ============================================================
// 执行结果（Tauri 命令返回值，兼容前端接口）
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResult {
    pub node_id: String,
    pub status: String, // "success" | "failed" | "skipped"
    pub output: Option<Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationResult {
    pub workflow_id: String,
    pub execution_id: String,
    pub status: String,
    pub node_results: Vec<NodeResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
