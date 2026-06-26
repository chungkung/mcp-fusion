// 前后端共享常量

// ============================================================
// 应用信息
// ============================================================

export const APP_NAME = "MCP Fusion";

export const APP_VERSION = "0.1.0";

// ============================================================
// 路由
// ============================================================

export const ROUTES = {
    HOME: "/",
    INTENT_MODE: "/intent-mode",
    INTENT: "/intent",
    CANVAS: "/canvas",
    CODE: "/code",
    MARKETPLACE: "/marketplace",
    SETTINGS: "/settings",
    EXECUTIONS: "/executions",
    AUDIT: "/audit-logs",
} as const;

// ============================================================
// IPC 通道
// ============================================================

export const IPC_CHANNELS = {
    // 工作流
    FLOW_LOAD: "list_workflows",
    FLOW_SAVE: "save_workflow",
    FLOW_DELETE: "remove_workflow",
    FLOW_EXECUTE: "execute_workflow",
    FLOW_GET: "get_workflow",
    // MCP 服务器
    MCP_LIST_SERVERS: "list_servers",
    MCP_ADD_SERVER: "add_server",
    MCP_REMOVE_SERVER: "remove_server",
    MCP_GET_SERVER: "get_server",
    MCP_PING_SERVER: "ping_server",
    // MCP 工具
    MCP_LIST_TOOLS: "list_tools",
    MCP_EXECUTE_TOOL: "execute_tool",
    // 意图解析
    INTENT_PARSE: "intent_parse",
    INTENT_PARSE_LLM: "intent_parse_llm",
    REFINE_WORKFLOW: "refine_workflow",
    RECOMMEND_TOOLS: "recommend_tools",
    // 运行时
    RUNTIME_STATUS: "runtime_status",
    RUNTIME_STOP: "runtime_stop",
    // 审计日志
    AUDIT_LIST: "list_audit_logs",
    AUDIT_SEARCH: "search_audit_logs",
    // 执行记录
    EXECUTIONS_LIST: "list_executions",
    // 插件市场
    MARKETPLACE_LIST: "list_marketplace_templates",
    MARKETPLACE_GET_TEMPLATE: "get_marketplace_template",
    MARKETPLACE_INSTALL: "install_template",
    MARKETPLACE_CHECK_UPDATES: "check_template_updates",
    // 认证与权限
    AUTH_INIT: "auth_init",
    AUTH_GENERATE_KEY: "auth_generate_key",
    AUTH_SET_KEY: "auth_set_key",
    AUTH_VERIFY_KEY: "auth_verify_key",
    AUTH_STATUS: "auth_status",
    AUTH_CLEAR_KEY: "auth_clear_key",
    AUTH_SET_ROLE: "auth_set_role",
    AUTH_GET_ROLE: "auth_get_role",
    // 系统管理
    METRICS: "metrics",
    HEALTH_CHECK: "health_check",
    BACKUP_DATABASE: "backup_database",
    RESTORE_DATABASE: "restore_database",
    FORCE_RELEASE_LOCK: "force_release_lock",
    RETRY_WORKFLOW: "retry_workflow",
    VERIFY_AUDIT_CHAIN: "verify_audit_chain",
} as const;

// ============================================================
// 默认配置
// ============================================================

export const DEFAULT_CONFIG = {
    pageSize: 20,
    maxPageSize: 100,
    requestTimeout: 30_000,
    flowMaxNodes: 200,
    canvasGridSize: 20,
    autosaveInterval: 30_000,
} as const;

// ============================================================
// 通用枚举常量
// ============================================================

/** 排序方向 */
export const SortOrder = {
    Asc: "asc",
    Desc: "desc",
} as const;

export type SortOrder = (typeof SortOrder)[keyof typeof SortOrder];