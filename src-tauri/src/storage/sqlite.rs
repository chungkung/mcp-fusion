use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

// ============================================================
// 数据模型（Rust 侧，对应前端 shared/types）
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub id: String,
    pub name: String,
    pub description: String,
    pub protocol: String, // "stdio" | "http" | "websocket"
    pub endpoint: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
    pub enabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: serde_json::Value,
    #[serde(rename = "outputSchema")]
    pub output_schema: serde_json::Value,
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNodePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNodeData {
    pub label: String,
    pub tool: Option<McpTool>,
    pub inputs: serde_json::Value,
    pub outputs: serde_json::Value,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub position: WorkflowNodePosition,
    pub data: WorkflowNodeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    #[serde(rename = "sourceHandle")]
    pub source_handle: Option<String>,
    #[serde(rename = "targetHandle")]
    pub target_handle: Option<String>,
    #[serde(rename = "type")]
    pub edge_type: Option<String>,
    pub animated: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub mode: String,   // "intent" | "canvas" | "code"
    pub status: String, // "idle" | "running" | "success" | "failed" | "timeout"
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
    /// 执行锁：防止同一工作流并发执行
    #[serde(default)]
    pub locked: bool,
    /// 锁定时间戳（毫秒），用于检测死锁
    #[serde(default)]
    #[serde(rename = "lockedAt")]
    pub locked_at: Option<i64>,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

// ============================================================
// 审计日志
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub action: String, // "workflow.execute" | "server.add" | "auth.generate" | ...
    pub resource: String, // "workflow:xxx" | "server:xxx" | "auth" | ...
    pub detail: String, // JSON 格式的额外信息
    /// 哈希链：上一条日志的 SHA-256 哈希值，构成防篡改链
    #[serde(default)]
    #[serde(rename = "prevHash")]
    pub prev_hash: String,
    /// 当前日志的哈希值 = SHA-256(id + action + resource + detail + prev_hash + created_at)
    #[serde(default)]
    #[serde(rename = "chainHash")]
    pub chain_hash: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

// ============================================================
// 工作流执行记录
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: String,
    #[serde(rename = "workflowId")]
    pub workflow_id: String,
    pub status: String, // "running" | "success" | "failed" | "timeout" | "aborted"
    /// 幂等键：相同 workflow_id + idempotency_key 只执行一次
    #[serde(default)]
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Option<String>,
    /// 已完成节点 ID 列表（JSON 数组），用于断点续传
    #[serde(default)]
    #[serde(rename = "completedNodes")]
    pub completed_nodes: serde_json::Value,
    #[serde(rename = "startedAt")]
    pub started_at: i64,
    #[serde(rename = "finishedAt")]
    pub finished_at: Option<i64>,
    #[serde(rename = "nodeResults")]
    pub node_results: serde_json::Value, // JSON: { node_id: { status, output, error, duration_ms } }
    pub error: Option<String>,
}

// ============================================================
// Auth Token 持久化
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub id: String,
    #[serde(rename = "keyHash")]
    pub key_hash: String,
    pub label: String, // "default" | "admin" | ...
    pub role: String,  // "admin" | "developer" | "viewer"
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

// ============================================================
// 分页结果
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub offset: usize,
    pub limit: usize,
}

// ============================================================
// 数据库
// ============================================================

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    /// 提供对内部连接的只读访问（用于外部模块执行自定义查询）
    pub fn conn(&self) -> &Mutex<Connection> {
        &self.conn
    }

    /// 打开（或创建）SQLite 数据库，初始化表结构
    pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| format!("无法打开数据库: {e}"))?;

        // 启用 WAL 模式提升并发性能
        conn.execute_batch("PRAGMA journal_mode=WAL;")
            .map_err(|e| format!("set WAL mode failed: {e}"))?;
        conn.execute_batch("PRAGMA busy_timeout=5000;")
            .map_err(|e| format!("set busy timeout failed: {e}"))?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|e| format!("set foreign_keys failed: {e}"))?;

        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_tables()?;
        Ok(db)
    }

    /// 创建表
    fn init_tables(&self) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS mcp_servers (
                id          TEXT PRIMARY KEY,
                name        TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                protocol    TEXT NOT NULL DEFAULT 'stdio',
                endpoint    TEXT NOT NULL DEFAULT '',
                command     TEXT NOT NULL DEFAULT '',
                args        TEXT NOT NULL DEFAULT '[]',
                env         TEXT NOT NULL DEFAULT '{}',
                enabled     INTEGER NOT NULL DEFAULT 1,
                created_at  INTEGER NOT NULL,
                updated_at  INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS workflows (
                id          TEXT PRIMARY KEY,
                name        TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                mode        TEXT NOT NULL DEFAULT 'canvas',
                status      TEXT NOT NULL DEFAULT 'idle',
                nodes       TEXT NOT NULL DEFAULT '[]',
                edges       TEXT NOT NULL DEFAULT '[]',
                locked      INTEGER NOT NULL DEFAULT 0,
                locked_at   INTEGER,
                created_at  INTEGER NOT NULL,
                updated_at  INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS audit_logs (
                id          TEXT PRIMARY KEY,
                action      TEXT NOT NULL,
                resource    TEXT NOT NULL DEFAULT '',
                detail      TEXT NOT NULL DEFAULT '{}',
                prev_hash   TEXT NOT NULL DEFAULT '',
                chain_hash  TEXT NOT NULL DEFAULT '',
                created_at  INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS workflow_executions (
                id                TEXT PRIMARY KEY,
                workflow_id       TEXT NOT NULL,
                status            TEXT NOT NULL DEFAULT 'running',
                idempotency_key   TEXT,
                completed_nodes   TEXT NOT NULL DEFAULT '[]',
                started_at        INTEGER NOT NULL,
                finished_at       INTEGER,
                node_results      TEXT NOT NULL DEFAULT '{}',
                error             TEXT,
                FOREIGN KEY (workflow_id) REFERENCES workflows(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS auth_tokens (
                id          TEXT PRIMARY KEY,
                key_hash    TEXT NOT NULL,
                label       TEXT NOT NULL DEFAULT 'default',
                role        TEXT NOT NULL DEFAULT 'admin',
                created_at  INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS installed_templates (
                template_id  TEXT NOT NULL,
                workflow_id  TEXT NOT NULL,
                version      TEXT NOT NULL,
                installed_at INTEGER NOT NULL,
                PRIMARY KEY (template_id, workflow_id)
            );
            ",
        )
        .map_err(|e| format!("初始化表失败: {e}"))
    }

    /// 运行数据库迁移，确保 schema 版本兼容
    pub fn run_migrations(&self) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;

        // 创建版本表（如果不存在）
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
            [],
        )
        .map_err(|e| format!("创建版本表失败: {e}"))?;

        // 获取当前版本
        let current_version: i32 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // 迁移 1: 初始表结构（如果 version < 1，说明表结构需要更新）
        if current_version < 1 {
            conn.execute(
                "INSERT OR IGNORE INTO schema_version (version) VALUES (1)",
                [],
            )
            .map_err(|e| format!("记录迁移版本 v1 失败: {e}"))?;
            tracing::info!("数据库迁移 v1 完成");
        }

        // 后续迁移可在此处添加，例如：
        // if current_version < 2 { ... }

        // 迁移 2: 添加执行锁和幂等/断点续传字段
        if current_version < 2 {
            conn.execute_batch(
                "ALTER TABLE workflows ADD COLUMN locked INTEGER NOT NULL DEFAULT 0;
                 ALTER TABLE workflows ADD COLUMN locked_at INTEGER;
                 ALTER TABLE workflow_executions ADD COLUMN idempotency_key TEXT;
                 ALTER TABLE workflow_executions ADD COLUMN completed_nodes TEXT NOT NULL DEFAULT '[]';",
            )
            .map_err(|e| format!("执行迁移 v2 失败: {e}"))?;
            conn.execute(
                "INSERT OR IGNORE INTO schema_version (version) VALUES (2)",
                [],
            )
            .map_err(|e| format!("记录迁移版本 v2 失败: {e}"))?;
            tracing::info!("数据库迁移 v2 完成（执行锁 + 幂等/断点续传）");
        }

        // 迁移 3: 添加 RBAC 角色字段
        if current_version < 3 {
            conn.execute_batch(
                "ALTER TABLE auth_tokens ADD COLUMN role TEXT NOT NULL DEFAULT 'admin';",
            )
            .map_err(|e| format!("执行迁移 v3 失败: {e}"))?;
            conn.execute(
                "INSERT OR IGNORE INTO schema_version (version) VALUES (3)",
                [],
            )
            .map_err(|e| format!("记录迁移版本 v3 失败: {e}"))?;
            tracing::info!("数据库迁移 v3 完成（RBAC 角色）");
        }

        // 迁移 4: 添加审计日志哈希链字段
        if current_version < 4 {
            conn.execute_batch(
                "ALTER TABLE audit_logs ADD COLUMN prev_hash TEXT NOT NULL DEFAULT '';
                 ALTER TABLE audit_logs ADD COLUMN chain_hash TEXT NOT NULL DEFAULT '';",
            )
            .map_err(|e| format!("执行迁移 v4 失败: {e}"))?;
            conn.execute(
                "INSERT OR IGNORE INTO schema_version (version) VALUES (4)",
                [],
            )
            .map_err(|e| format!("记录迁移版本 v4 失败: {e}"))?;
            tracing::info!("数据库迁移 v4 完成（审计日志哈希链）");
        }

        Ok(())
    }

    // ============================================================
    // MCP Server CRUD
    // ============================================================

    pub fn insert_server(&self, server: &McpServer) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let args_json = serde_json::to_string(&server.args).unwrap_or_default();
        let env_json = serde_json::to_string(&server.env).unwrap_or_default();

        conn.execute(
            "INSERT INTO mcp_servers (id, name, description, protocol, endpoint, command, args, env, enabled, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                server.id,
                server.name,
                server.description,
                server.protocol,
                server.endpoint,
                server.command,
                args_json,
                env_json,
                server.enabled as i32,
                server.created_at,
                server.updated_at,
            ],
        )
        .map_err(|e| format!("插入服务器失败: {e}"))?;

        Ok(())
    }

    pub fn update_server(&self, server: &McpServer) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let args_json = serde_json::to_string(&server.args).unwrap_or_default();
        let env_json = serde_json::to_string(&server.env).unwrap_or_default();

        let affected = conn
            .execute(
                "UPDATE mcp_servers SET name=?2, description=?3, protocol=?4, endpoint=?5, command=?6, args=?7, env=?8, enabled=?9, updated_at=?10 WHERE id=?1",
                params![
                    server.id,
                    server.name,
                    server.description,
                    server.protocol,
                    server.endpoint,
                    server.command,
                    args_json,
                    env_json,
                    server.enabled as i32,
                    server.updated_at,
                ],
            )
            .map_err(|e| format!("更新服务器失败: {e}"))?;

        if affected == 0 {
            return Err(format!("服务器不存在: {}", server.id));
        }
        Ok(())
    }

    pub fn delete_server(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let affected = conn
            .execute("DELETE FROM mcp_servers WHERE id = ?1", params![id])
            .map_err(|e| format!("删除服务器失败: {e}"))?;

        if affected == 0 {
            return Err(format!("服务器不存在: {id}"));
        }
        Ok(())
    }

    pub fn get_server(&self, id: &str) -> Result<Option<McpServer>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let mut stmt = conn
            .prepare("SELECT id, name, description, protocol, endpoint, command, args, env, enabled, created_at, updated_at FROM mcp_servers WHERE id = ?1")
            .map_err(|e| format!("准备查询失败: {e}"))?;

        let result = stmt.query_row(params![id], |row| {
            let args_str: String = row.get(6)?;
            let env_str: String = row.get(7)?;
            Ok(McpServer {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                protocol: row.get(3)?,
                endpoint: row.get(4)?,
                command: row.get(5)?,
                args: serde_json::from_str(&args_str).unwrap_or_else(|e| {
                    tracing::error!("failed to parse args JSON: {e}");
                    Default::default()
                }),
                env: serde_json::from_str(&env_str).unwrap_or_else(|e| {
                    tracing::error!("failed to parse env JSON: {e}");
                    Default::default()
                }),
                enabled: row.get::<_, i32>(8)? != 0,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        });

        match result {
            Ok(server) => Ok(Some(server)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("查询服务器失败: {e}")),
        }
    }

    pub fn list_servers(&self) -> Result<Vec<McpServer>, String> {
        self.list_servers_paginated(0, usize::MAX).map(|p| p.items)
    }

    /// 分页查询服务器列表
    pub fn list_servers_paginated(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<McpServer>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;

        let total: i64 = conn
            .query_row("SELECT COUNT(*) FROM mcp_servers", [], |row| row.get(0))
            .map_err(|e| format!("查询总数失败: {e}"))?;

        let mut stmt = conn
            .prepare("SELECT id, name, description, protocol, endpoint, command, args, env, enabled, created_at, updated_at FROM mcp_servers ORDER BY created_at DESC LIMIT ? OFFSET ?")
            .map_err(|e| format!("准备查询失败: {e}"))?;

        let limit_i64 = limit as i64;
        let offset_i64 = offset as i64;
        let rows = stmt
            .query_map([limit_i64, offset_i64], |row| {
                let args_str: String = row.get(6)?;
                let env_str: String = row.get(7)?;
                Ok(McpServer {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    protocol: row.get(3)?,
                    endpoint: row.get(4)?,
                    command: row.get(5)?,
                    args: serde_json::from_str(&args_str).unwrap_or_else(|e| {
                        tracing::error!("failed to parse args JSON: {e}");
                        Default::default()
                    }),
                    env: serde_json::from_str(&env_str).unwrap_or_else(|e| {
                        tracing::error!("failed to parse env JSON: {e}");
                        Default::default()
                    }),
                    enabled: row.get::<_, i32>(8)? != 0,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            })
            .map_err(|e| format!("查询服务器列表失败: {e}"))?;

        let mut servers = Vec::new();
        for row in rows {
            servers.push(row.map_err(|e| format!("读取行失败: {e}"))?);
        }
        Ok(PaginatedResult {
            items: servers,
            total: total as usize,
            offset,
            limit,
        })
    }

    // ============================================================
    // Workflow CRUD
    // ============================================================

    pub fn insert_or_replace_workflow(&self, workflow: &Workflow) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let nodes_json = serde_json::to_string(&workflow.nodes).unwrap_or_default();
        let edges_json = serde_json::to_string(&workflow.edges).unwrap_or_default();

        // 使用 ON CONFLICT DO UPDATE 替代 INSERT OR REPLACE，
        // 避免 SQLite 先 DELETE 再 INSERT 触发级联删除执行记录。
        conn.execute(
            "INSERT INTO workflows (id, name, description, mode, status, nodes, edges, locked, locked_at, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                description = excluded.description,
                mode = excluded.mode,
                status = excluded.status,
                nodes = excluded.nodes,
                edges = excluded.edges,
                locked = excluded.locked,
                locked_at = excluded.locked_at,
                updated_at = excluded.updated_at",
            params![
                workflow.id,
                workflow.name,
                workflow.description,
                workflow.mode,
                workflow.status,
                nodes_json,
                edges_json,
                workflow.locked as i32,
                workflow.locked_at,
                workflow.created_at,
                workflow.updated_at,
            ],
        )
        .map_err(|e| format!("插入或替换工作流失败: {e}"))?;

        Ok(())
    }

    pub fn delete_workflow(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let affected = conn
            .execute("DELETE FROM workflows WHERE id = ?1", params![id])
            .map_err(|e| format!("删除工作流失败: {e}"))?;

        if affected == 0 {
            return Err(format!("工作流不存在: {id}"));
        }
        Ok(())
    }

    pub fn get_workflow(&self, id: &str) -> Result<Option<Workflow>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let mut stmt = conn
            .prepare("SELECT id, name, description, mode, status, nodes, edges, locked, locked_at, created_at, updated_at FROM workflows WHERE id = ?1")
            .map_err(|e| format!("准备查询失败: {e}"))?;

        let result = stmt.query_row(params![id], |row| {
            let nodes_str: String = row.get(5)?;
            let edges_str: String = row.get(6)?;
            Ok(Workflow {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                mode: row.get(3)?,
                status: row.get(4)?,
                nodes: serde_json::from_str(&nodes_str).unwrap_or_else(|e| {
                    tracing::error!("failed to parse nodes JSON: {e}");
                    Default::default()
                }),
                edges: serde_json::from_str(&edges_str).unwrap_or_else(|e| {
                    tracing::error!("failed to parse edges JSON: {e}");
                    Default::default()
                }),
                locked: row.get::<_, i32>(7)? != 0,
                locked_at: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        });

        match result {
            Ok(wf) => Ok(Some(wf)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("查询工作流失败: {e}")),
        }
    }

    pub fn list_workflows(&self) -> Result<Vec<Workflow>, String> {
        self.list_workflows_paginated(0, usize::MAX)
            .map(|p| p.items)
    }

    /// 分页查询工作流列表
    pub fn list_workflows_paginated(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<Workflow>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;

        let total: i64 = conn
            .query_row("SELECT COUNT(*) FROM workflows", [], |row| row.get(0))
            .map_err(|e| format!("查询总数失败: {e}"))?;

        let mut stmt = conn
            .prepare("SELECT id, name, description, mode, status, nodes, edges, locked, locked_at, created_at, updated_at FROM workflows ORDER BY updated_at DESC LIMIT ? OFFSET ?")
            .map_err(|e| format!("准备查询失败: {e}"))?;

        let limit_i64 = limit as i64;
        let offset_i64 = offset as i64;
        let rows = stmt
            .query_map([limit_i64, offset_i64], |row| {
                let nodes_str: String = row.get(5)?;
                let edges_str: String = row.get(6)?;
                Ok(Workflow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    mode: row.get(3)?,
                    status: row.get(4)?,
                    nodes: serde_json::from_str(&nodes_str).unwrap_or_else(|e| {
                        tracing::error!("failed to parse nodes JSON: {e}");
                        Default::default()
                    }),
                    edges: serde_json::from_str(&edges_str).unwrap_or_else(|e| {
                        tracing::error!("failed to parse edges JSON: {e}");
                        Default::default()
                    }),
                    locked: row.get::<_, i32>(7)? != 0,
                    locked_at: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            })
            .map_err(|e| format!("查询工作流列表失败: {e}"))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(row.map_err(|e| format!("读取行失败: {e}"))?);
        }
        Ok(PaginatedResult {
            items: workflows,
            total: total as usize,
            offset,
            limit,
        })
    }

    // ============================================================
    // 执行锁（防止同一工作流并发执行）
    // ============================================================

    /// 尝试获取工作流执行锁。
    /// 返回 true 表示获取成功，false 表示已被其他执行实例锁定。
    /// 锁超时时间默认 5 分钟，超时后自动释放（防止死锁）。
    pub fn acquire_workflow_lock(&self, workflow_id: &str) -> Result<bool, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let now = chrono::Utc::now().timestamp_millis();
        let lock_timeout_ms = 5 * 60 * 1000; // 5 分钟超时

        // 原子操作：仅在 locked=0 或锁已超时时获取锁
        let affected = conn
            .execute(
                "UPDATE workflows SET locked = 1, locked_at = ?2
                 WHERE id = ?1 AND (locked = 0 OR locked_at IS NULL OR (?2 - locked_at) > ?3)",
                params![workflow_id, now, lock_timeout_ms],
            )
            .map_err(|e| format!("获取执行锁失败: {e}"))?;

        Ok(affected > 0)
    }

    /// 释放工作流执行锁。
    pub fn release_workflow_lock(&self, workflow_id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        conn.execute(
            "UPDATE workflows SET locked = 0, locked_at = NULL WHERE id = ?1",
            params![workflow_id],
        )
        .map_err(|e| format!("释放执行锁失败: {e}"))?;
        Ok(())
    }

    /// 强制释放工作流执行锁（管理员操作）。
    pub fn force_release_workflow_lock(&self, workflow_id: &str) -> Result<(), String> {
        self.release_workflow_lock(workflow_id)
    }

    // ============================================================
    // 审计日志
    // ============================================================

    pub fn insert_audit_log(&self, log: &AuditLog) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;

        // 获取上一条日志的 chain_hash 作为 prev_hash
        let prev_hash: String = conn
            .query_row(
                "SELECT COALESCE(chain_hash, '') FROM audit_logs ORDER BY created_at DESC, rowid DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap_or_default();

        // 计算当前日志的哈希链: SHA-256(prev_hash + id + action + resource + detail + created_at)
        let chain_input = format!(
            "{}|{}|{}|{}|{}|{}",
            prev_hash, log.id, log.action, log.resource, log.detail, log.created_at
        );
        let chain_hash = crate::crypto::sha256_hex(&chain_input);

        conn.execute(
            "INSERT INTO audit_logs (id, action, resource, detail, prev_hash, chain_hash, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![log.id, log.action, log.resource, log.detail, prev_hash, chain_hash, log.created_at],
        )
        .map_err(|e| format!("插入审计日志失败: {e}"))?;
        Ok(())
    }

    pub fn list_audit_logs(
        &self,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<AuditLog>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let total: i64 = conn
            .query_row("SELECT COUNT(*) FROM audit_logs", [], |row| row.get(0))
            .map_err(|e| format!("查询总数失败: {e}"))?;

        let mut stmt = conn
            .prepare("SELECT id, action, resource, detail, prev_hash, chain_hash, created_at FROM audit_logs ORDER BY created_at DESC LIMIT ? OFFSET ?")
            .map_err(|e| format!("准备查询失败: {e}"))?;

        let limit_i64 = limit as i64;
        let offset_i64 = offset as i64;
        let rows = stmt
            .query_map([limit_i64, offset_i64], |row| {
                Ok(AuditLog {
                    id: row.get(0)?,
                    action: row.get(1)?,
                    resource: row.get(2)?,
                    detail: row.get(3)?,
                    prev_hash: row.get(4)?,
                    chain_hash: row.get(5)?,
                    created_at: row.get(6)?,
                })
            })
            .map_err(|e| format!("查询审计日志失败: {e}"))?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(row.map_err(|e| format!("读取行失败: {e}"))?);
        }
        Ok(PaginatedResult {
            items: logs,
            total: total as usize,
            offset,
            limit,
        })
    }

    /// 搜索审计日志，支持按 action、resource、时间范围过滤
    pub fn search_audit_logs(
        &self,
        action: Option<&str>,
        resource: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<AuditLog>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;

        // 动态构建查询条件
        let mut conditions = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(a) = action {
            if !a.is_empty() {
                conditions.push(format!("action LIKE ?{}", params.len() + 1));
                params.push(Box::new(format!("%{a}%")));
            }
        }
        if let Some(r) = resource {
            if !r.is_empty() {
                conditions.push(format!("resource LIKE ?{}", params.len() + 1));
                params.push(Box::new(format!("%{r}%")));
            }
        }
        if let Some(st) = start_time {
            conditions.push(format!("created_at >= ?{}", params.len() + 1));
            params.push(Box::new(st));
        }
        if let Some(et) = end_time {
            conditions.push(format!("created_at <= ?{}", params.len() + 1));
            params.push(Box::new(et));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // 查询总数
        let count_sql = format!("SELECT COUNT(*) FROM audit_logs {where_clause}");
        let total: i64 = {
            let param_refs: Vec<&dyn rusqlite::types::ToSql> =
                params.iter().map(|p| p.as_ref()).collect();
            conn.query_row(&count_sql, param_refs.as_slice(), |row| row.get(0))
                .map_err(|e| format!("查询审计日志总数失败: {e}"))?
        };

        // 查询数据
        let query_sql = format!(
            "SELECT id, action, resource, detail, prev_hash, chain_hash, created_at FROM audit_logs {where_clause} ORDER BY created_at DESC LIMIT ?{} OFFSET ?{}",
            params.len() + 1,
            params.len() + 2,
        );
        params.push(Box::new(limit as i64));
        params.push(Box::new(offset as i64));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            params.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn
            .prepare(&query_sql)
            .map_err(|e| format!("准备搜索查询失败: {e}"))?;

        let rows = stmt
            .query_map(param_refs.as_slice(), |row| {
                Ok(AuditLog {
                    id: row.get(0)?,
                    action: row.get(1)?,
                    resource: row.get(2)?,
                    detail: row.get(3)?,
                    prev_hash: row.get(4)?,
                    chain_hash: row.get(5)?,
                    created_at: row.get(6)?,
                })
            })
            .map_err(|e| format!("搜索审计日志失败: {e}"))?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(row.map_err(|e| format!("读取行失败: {e}"))?);
        }
        Ok(PaginatedResult {
            items: logs,
            total: total as usize,
            offset,
            limit,
        })
    }

    // ============================================================
    // 工作流执行记录
    // ============================================================

    pub fn insert_execution(&self, exec: &WorkflowExecution) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let node_results = serde_json::to_string(&exec.node_results).unwrap_or_default();
        let completed_nodes = serde_json::to_string(&exec.completed_nodes).unwrap_or_default();
        conn.execute(
            "INSERT INTO workflow_executions (id, workflow_id, status, idempotency_key, completed_nodes, started_at, finished_at, node_results, error) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                exec.id,
                exec.workflow_id,
                exec.status,
                exec.idempotency_key,
                completed_nodes,
                exec.started_at,
                exec.finished_at,
                node_results,
                exec.error,
            ],
        )
        .map_err(|e| format!("插入执行记录失败: {e}"))?;
        Ok(())
    }

    pub fn update_execution(&self, exec: &WorkflowExecution) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let node_results = serde_json::to_string(&exec.node_results).unwrap_or_default();
        let completed_nodes = serde_json::to_string(&exec.completed_nodes).unwrap_or_default();
        let affected = conn
            .execute(
                "UPDATE workflow_executions SET status=?2, finished_at=?3, node_results=?4, completed_nodes=?5, error=?6 WHERE id=?1",
                params![exec.id, exec.status, exec.finished_at, node_results, completed_nodes, exec.error],
            )
            .map_err(|e| format!("更新执行记录失败: {e}"))?;
        if affected == 0 {
            return Err(format!("执行记录不存在: {}", exec.id));
        }
        Ok(())
    }

    /// 根据 idempotency_key 查找已有的执行记录（幂等检查）。
    /// 返回 Some 表示已有执行记录，应直接返回该结果，不重复执行。
    pub fn find_execution_by_idempotency_key(
        &self,
        workflow_id: &str,
        idempotency_key: &str,
    ) -> Result<Option<WorkflowExecution>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let mut stmt = conn
            .prepare("SELECT id, workflow_id, status, idempotency_key, completed_nodes, started_at, finished_at, node_results, error FROM workflow_executions WHERE workflow_id = ?1 AND idempotency_key = ?2 ORDER BY started_at DESC LIMIT 1")
            .map_err(|e| format!("准备查询失败: {e}"))?;

        let result = stmt.query_row(params![workflow_id, idempotency_key], |row| {
            let nr_str: String = row.get(7)?;
            let cn_str: String = row.get(4)?;
            Ok(WorkflowExecution {
                id: row.get(0)?,
                workflow_id: row.get(1)?,
                status: row.get(2)?,
                idempotency_key: row.get(3)?,
                completed_nodes: serde_json::from_str(&cn_str).unwrap_or_default(),
                started_at: row.get(5)?,
                finished_at: row.get(6)?,
                node_results: serde_json::from_str(&nr_str).unwrap_or_default(),
                error: row.get(8)?,
            })
        });

        match result {
            Ok(exec) => Ok(Some(exec)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("查询幂等执行记录失败: {e}")),
        }
    }

    /// 根据 ID 查询执行记录（用于断点续传）。
    pub fn get_execution(&self, id: &str) -> Result<Option<WorkflowExecution>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let mut stmt = conn
            .prepare("SELECT id, workflow_id, status, idempotency_key, completed_nodes, started_at, finished_at, node_results, error FROM workflow_executions WHERE id = ?1")
            .map_err(|e| format!("准备查询失败: {e}"))?;

        let result = stmt.query_row(params![id], |row| {
            let nr_str: String = row.get(7)?;
            let cn_str: String = row.get(4)?;
            Ok(WorkflowExecution {
                id: row.get(0)?,
                workflow_id: row.get(1)?,
                status: row.get(2)?,
                idempotency_key: row.get(3)?,
                completed_nodes: serde_json::from_str(&cn_str).unwrap_or_default(),
                started_at: row.get(5)?,
                finished_at: row.get(6)?,
                node_results: serde_json::from_str(&nr_str).unwrap_or_default(),
                error: row.get(8)?,
            })
        });

        match result {
            Ok(exec) => Ok(Some(exec)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("查询执行记录失败: {e}")),
        }
    }

    pub fn list_executions_by_workflow(
        &self,
        workflow_id: &str,
        offset: usize,
        limit: usize,
    ) -> Result<PaginatedResult<WorkflowExecution>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let total: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM workflow_executions WHERE workflow_id = ?1",
                params![workflow_id],
                |row| row.get(0),
            )
            .map_err(|e| format!("查询总数失败: {e}"))?;

        let mut stmt = conn
            .prepare("SELECT id, workflow_id, status, idempotency_key, completed_nodes, started_at, finished_at, node_results, error FROM workflow_executions WHERE workflow_id = ?1 ORDER BY started_at DESC LIMIT ?2 OFFSET ?3")
            .map_err(|e| format!("准备查询失败: {e}"))?;

        let limit_i64 = limit as i64;
        let offset_i64 = offset as i64;
        let rows = stmt
            .query_map(params![workflow_id, limit_i64, offset_i64], |row| {
                let nr_str: String = row.get(7)?;
                let cn_str: String = row.get(4)?;
                Ok(WorkflowExecution {
                    id: row.get(0)?,
                    workflow_id: row.get(1)?,
                    status: row.get(2)?,
                    idempotency_key: row.get(3)?,
                    completed_nodes: serde_json::from_str(&cn_str).unwrap_or_default(),
                    started_at: row.get(5)?,
                    finished_at: row.get(6)?,
                    node_results: serde_json::from_str(&nr_str).unwrap_or_default(),
                    error: row.get(8)?,
                })
            })
            .map_err(|e| format!("查询执行记录失败: {e}"))?;

        let mut execs = Vec::new();
        for row in rows {
            execs.push(row.map_err(|e| format!("读取行失败: {e}"))?);
        }
        Ok(PaginatedResult {
            items: execs,
            total: total as usize,
            offset,
            limit,
        })
    }

    // ============================================================
    // Auth Token
    // ============================================================

    pub fn save_auth_token(&self, token: &AuthToken) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        conn.execute(
            "INSERT OR REPLACE INTO auth_tokens (id, key_hash, label, role, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![token.id, token.key_hash, token.label, token.role, token.created_at],
        )
        .map_err(|e| format!("保存 auth token 失败: {e}"))?;
        Ok(())
    }

    pub fn get_auth_token(&self, id: &str) -> Result<Option<AuthToken>, String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let mut stmt = conn
            .prepare("SELECT id, key_hash, label, role, created_at FROM auth_tokens WHERE id = ?1")
            .map_err(|e| format!("准备查询失败: {e}"))?;
        let result = stmt.query_row(params![id], |row| {
            Ok(AuthToken {
                id: row.get(0)?,
                key_hash: row.get(1)?,
                label: row.get(2)?,
                role: row.get(3)?,
                created_at: row.get(4)?,
            })
        });
        match result {
            Ok(t) => Ok(Some(t)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("查询 auth token 失败: {e}")),
        }
    }

    pub fn delete_auth_token(&self, id: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        conn.execute("DELETE FROM auth_tokens WHERE id = ?1", params![id])
            .map_err(|e| format!("删除 auth token 失败: {e}"))?;
        Ok(())
    }

    // ============================================================
    // 审计日志哈希链验证
    // ============================================================

    /// 验证审计日志哈希链完整性。
    /// 返回 (总数, 有效数, 无效条目详情列表)。
    pub fn verify_audit_chain(&self) -> Result<(usize, usize, Vec<String>), String> {
        let conn = self.conn.lock().map_err(|e| format!("获取锁失败: {e}"))?;
        let mut stmt = conn
            .prepare("SELECT id, action, resource, detail, prev_hash, chain_hash, created_at FROM audit_logs ORDER BY created_at ASC, rowid ASC")
            .map_err(|e| format!("准备验证查询失败: {e}"))?;

        let rows = stmt
            .query_map([], |row| {
                Ok(AuditLog {
                    id: row.get(0)?,
                    action: row.get(1)?,
                    resource: row.get(2)?,
                    detail: row.get(3)?,
                    prev_hash: row.get(4)?,
                    chain_hash: row.get(5)?,
                    created_at: row.get(6)?,
                })
            })
            .map_err(|e| format!("查询审计日志失败: {e}"))?;

        let mut total = 0usize;
        let mut valid = 0usize;
        let mut invalid_details = Vec::new();
        let mut prev_chain_hash = String::new();

        for row in rows {
            let log = row.map_err(|e| format!("读取行失败: {e}"))?;
            total += 1;

            // 验证 prev_hash 是否匹配上一条的 chain_hash
            if log.prev_hash != prev_chain_hash {
                invalid_details.push(format!(
                    "日志 {} 的 prev_hash 不匹配：期望 '{}'，实际 '{}'",
                    log.id,
                    &prev_chain_hash[..prev_chain_hash.len().min(16)],
                    &log.prev_hash[..log.prev_hash.len().min(16)]
                ));
            }

            // 重新计算 chain_hash 并比对
            let chain_input = format!(
                "{}|{}|{}|{}|{}|{}",
                log.prev_hash, log.id, log.action, log.resource, log.detail, log.created_at
            );
            let expected_chain = crate::crypto::sha256_hex(&chain_input);
            if log.chain_hash != expected_chain {
                invalid_details.push(format!(
                    "日志 {} 的 chain_hash 被篡改：期望 '{}'，实际 '{}'",
                    log.id,
                    &expected_chain[..16],
                    &log.chain_hash[..log.chain_hash.len().min(16)]
                ));
            }

            if log.prev_hash == prev_chain_hash && log.chain_hash == expected_chain {
                valid += 1;
            }

            prev_chain_hash = log.chain_hash.clone();
        }

        Ok((total, valid, invalid_details))
    }
}

#[cfg(test)]
#[path = "sqlite_tests.rs"]
mod tests;
