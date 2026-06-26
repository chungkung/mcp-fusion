pub mod sqlite;

// ============================================================
// Storage trait — 数据库抽象层
// ============================================================
// 定义所有持久化操作的统一接口，解耦业务逻辑与具体数据库实现。
// 当前仅实现 SQLite，未来可新增 PostgreSQL 等实现。

use sqlite::{
    AuditLog, AuthToken, McpServer, PaginatedResult, Workflow, WorkflowExecution,
};

/// 数据库抽象接口，封装所有 CRUD 操作。
/// 调用方不直接依赖 SQLite，可通过此 trait 切换后端。
#[allow(dead_code)]
pub trait Storage: Send + Sync {
    // ---- Server ----
    fn insert_server(&self, server: &McpServer) -> Result<(), String>;
    fn update_server(&self, server: &McpServer) -> Result<(), String>;
    fn delete_server(&self, id: &str) -> Result<(), String>;
    fn get_server(&self, id: &str) -> Result<Option<McpServer>, String>;
    fn list_servers(&self) -> Result<Vec<McpServer>, String>;
    fn list_servers_paginated(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<McpServer>, String>;

    // ---- Workflow ----
    fn insert_or_replace_workflow(&self, workflow: &Workflow) -> Result<(), String>;
    fn delete_workflow(&self, id: &str) -> Result<(), String>;
    fn get_workflow(&self, id: &str) -> Result<Option<Workflow>, String>;
    fn list_workflows(&self) -> Result<Vec<Workflow>, String>;
    fn list_workflows_paginated(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<Workflow>, String>;

    // ---- Execution Lock ----
    fn acquire_workflow_lock(&self, workflow_id: &str) -> Result<bool, String>;
    fn release_workflow_lock(&self, workflow_id: &str) -> Result<(), String>;
    fn force_release_workflow_lock(&self, workflow_id: &str) -> Result<(), String>;

    // ---- Audit Log ----
    fn insert_audit_log(&self, log: &AuditLog) -> Result<(), String>;
    fn list_audit_logs(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<AuditLog>, String>;
    fn search_audit_logs(
        &self,
        action: Option<&str>,
        resource: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<AuditLog>, String>;
    fn verify_audit_chain(&self) -> Result<(usize, usize, Vec<String>), String>;

    // ---- Execution ----
    fn insert_execution(&self, exec: &WorkflowExecution) -> Result<(), String>;
    fn update_execution(&self, exec: &WorkflowExecution) -> Result<(), String>;
    fn get_execution(&self, id: &str) -> Result<Option<WorkflowExecution>, String>;
    fn find_execution_by_idempotency_key(
        &self,
        workflow_id: &str,
        idempotency_key: &str,
    ) -> Result<Option<WorkflowExecution>, String>;
    fn list_executions_by_workflow(
        &self,
        workflow_id: &str,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<WorkflowExecution>, String>;

    // ---- Auth ----
    fn save_auth_token(&self, token: &AuthToken) -> Result<(), String>;
    fn get_auth_token(&self, id: &str) -> Result<Option<AuthToken>, String>;
    fn delete_auth_token(&self, id: &str) -> Result<(), String>;
}

// ============================================================
// Storage trait 自动实现给 Database
// ============================================================
// Database 的方法签名与 trait 一致，无需额外 impl 块。
// 若未来新增 PostgreSQL 实现，只需为其实现 Storage trait 即可。