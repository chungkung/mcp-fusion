use super::*;
use rusqlite::params;
use tempfile::tempdir;

// ============================================================
// 测试辅助
// ============================================================

fn open_test_db() -> Database {
    let dir = tempdir().expect("创建临时目录失败");
    let path = dir.path().join("test.db");
    Database::open(path).expect("打开数据库失败")
}

fn insert_test_server(db: &Database, id: &str, name: &str) {
    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO mcp_servers (id, name, description, protocol, endpoint, command, args, env, enabled, created_at, updated_at)
         VALUES (?1, ?2, '', 'stdio', '', '', '[]', '{}', 1, 1000, 1000)",
        params![id, name],
    )
    .unwrap_or_else(|e| panic!("插入服务器 {} 失败: {}", id, e));
}

fn insert_test_workflow(db: &Database, id: &str, name: &str) {
    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO workflows (id, name, description, mode, status, nodes, edges, locked, locked_at, created_at, updated_at)
         VALUES (?1, ?2, '', 'canvas', 'idle', '[]', '[]', 0, NULL, 1000, 1000)",
        params![id, name],
    )
    .unwrap();
}

// ============================================================
// 数据库初始化测试
// ============================================================

#[test]
fn test_database_open_creates_tables() {
    let db = open_test_db();
    let conn = db.conn().lock().unwrap();

    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    assert!(tables.contains(&"mcp_servers".to_string()));
    assert!(tables.contains(&"workflows".to_string()));
    assert!(tables.contains(&"audit_logs".to_string()));
    assert!(tables.contains(&"workflow_executions".to_string()));
    assert!(tables.contains(&"auth_tokens".to_string()));
    assert!(tables.contains(&"installed_templates".to_string()));
}

#[test]
fn test_database_open_is_idempotent() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("test.db");

    Database::open(&path).expect("第一次打开失败");
    Database::open(&path).expect("第二次打开应成功");
}

// ============================================================
// MCP Server CRUD 测试
// ============================================================

#[test]
fn test_insert_and_get_server() {
    let db = open_test_db();
    insert_test_server(&db, "srv-1", "Test Server");

    let conn = db.conn().lock().unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM mcp_servers WHERE id='srv-1'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn test_update_server() {
    let db = open_test_db();
    insert_test_server(&db, "srv-1", "Original");

    let conn = db.conn().lock().unwrap();
    conn.execute(
        "UPDATE mcp_servers SET name=?1, updated_at=?2 WHERE id='srv-1'",
        params!["Updated", 2000],
    )
    .unwrap();

    let name: String = conn
        .query_row("SELECT name FROM mcp_servers WHERE id='srv-1'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(name, "Updated");
}

#[test]
fn test_delete_server() {
    let db = open_test_db();
    insert_test_server(&db, "srv-1", "To Delete");

    let conn = db.conn().lock().unwrap();
    conn.execute("DELETE FROM mcp_servers WHERE id='srv-1'", [])
        .unwrap();

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM mcp_servers WHERE id='srv-1'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_list_servers() {
    let db = open_test_db();
    insert_test_server(&db, "srv-1", "Server A");
    insert_test_server(&db, "srv-2", "Server B");

    let conn = db.conn().lock().unwrap();
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM mcp_servers", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 2);
}

// ============================================================
// 工作流执行测试
// ============================================================

#[test]
fn test_insert_execution() {
    let db = open_test_db();
    insert_test_workflow(&db, "wf-1", "Test Workflow");

    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO workflow_executions (id, workflow_id, status, started_at, node_results)
         VALUES ('exec-1', 'wf-1', 'running', 2000, '{}')",
        [],
    )
    .unwrap();

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_executions WHERE workflow_id='wf-1'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn test_update_execution_status() {
    let db = open_test_db();
    insert_test_workflow(&db, "wf-1", "Test Workflow");

    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO workflow_executions (id, workflow_id, status, started_at, node_results)
         VALUES ('exec-1', 'wf-1', 'running', 2000, '{}')",
        [],
    )
    .unwrap();

    conn.execute(
        "UPDATE workflow_executions SET status='success', finished_at=3000, node_results='{\"n1\":{\"status\":\"success\"}}' WHERE id='exec-1'",
        [],
    )
    .unwrap();

    let status: String = conn
        .query_row("SELECT status FROM workflow_executions WHERE id='exec-1'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(status, "success");
}

#[test]
fn test_execution_idempotency_key() {
    let db = open_test_db();
    insert_test_workflow(&db, "wf-1", "Test Workflow");

    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO workflow_executions (id, workflow_id, status, idempotency_key, started_at, node_results)
         VALUES ('exec-1', 'wf-1', 'running', 'ik-abc', 2000, '{}')",
        [],
    )
    .unwrap();

    let key: String = conn
        .query_row("SELECT idempotency_key FROM workflow_executions WHERE id='exec-1'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(key, "ik-abc");
}

#[test]
fn test_execution_foreign_key_cascade() {
    let db = open_test_db();
    insert_test_workflow(&db, "wf-1", "Test Workflow");

    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO workflow_executions (id, workflow_id, status, started_at, node_results)
         VALUES ('exec-1', 'wf-1', 'running', 2000, '{}')",
        [],
    )
    .unwrap();

    // 删除工作流应级联删除执行记录
    conn.execute("DELETE FROM workflows WHERE id='wf-1'", [])
        .unwrap();

    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_executions WHERE id='exec-1'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 0);
}

// ============================================================
// 审计日志测试
// ============================================================

#[test]
fn test_insert_audit_log() {
    let db = open_test_db();

    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO audit_logs (id, action, resource, detail, prev_hash, chain_hash, created_at)
         VALUES ('audit-1', 'workflow.execute', 'workflow:wf-1', '{}', '', 'test-hash', 3000)",
        [],
    )
    .unwrap();

    let action: String = conn
        .query_row("SELECT action FROM audit_logs WHERE id='audit-1'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(action, "workflow.execute");
}

#[test]
fn test_audit_log_chain_hash() {
    let db = open_test_db();

    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO audit_logs (id, action, resource, detail, prev_hash, chain_hash, created_at)
         VALUES ('audit-1', 'server.add', 'server:srv-1', '{}', '', 'hash-1', 1000)",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO audit_logs (id, action, resource, detail, prev_hash, chain_hash, created_at)
         VALUES ('audit-2', 'workflow.create', 'workflow:wf-1', '{}', 'hash-1', 'hash-2', 2000)",
        [],
    )
    .unwrap();

    let prev_hash: String = conn
        .query_row("SELECT prev_hash FROM audit_logs WHERE id='audit-2'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(prev_hash, "hash-1");
}

// ============================================================
// Auth Token 测试
// ============================================================

#[test]
fn test_insert_and_query_auth_token() {
    let db = open_test_db();

    let conn = db.conn().lock().unwrap();
    conn.execute(
        "INSERT INTO auth_tokens (id, key_hash, label, role, created_at)
         VALUES ('token-1', 'abc123hash', 'default', 'admin', 1000)",
        [],
    )
    .unwrap();

    let role: String = conn
        .query_row("SELECT role FROM auth_tokens WHERE id='token-1'", [], |r| r.get(0))
        .unwrap();
    assert_eq!(role, "admin");
}

// ============================================================
// 数据库迁移测试
// ============================================================

#[test]
fn test_migrations_create_version_table() {
    let db = open_test_db();

    let conn = db.conn().lock().unwrap();
    let has_table: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='schema_version'",
            [],
            |r| r.get(0),
        )
        .unwrap();

    // 迁移后应有 schema_version 表
    assert!(has_table);
}

#[test]
fn test_migration_idempotent() {
    let db = open_test_db();

    // 重复调用 init_tables 不会出错
    let result = db.init_tables();
    assert!(result.is_ok());
}