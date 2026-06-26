// ============================================================
// 枚举定义
// ============================================================

/** 运行状态 */
export enum RunStatus {
    Idle = "idle",
    Running = "running",
    Success = "success",
    Failed = "failed",
    Timeout = "timeout",
}

/** MCP 协议类型 */
export enum MCPProtocol {
    Stdio = "stdio",
    Sse = "sse",
    StreamableHttp = "streamable-http",
    Http = "http",
    WebSocket = "websocket",
}

/** 工作流模式 */
export enum WorkflowMode {
    Intent = "intent",
    Canvas = "canvas",
    Code = "code",
}

// ============================================================
// 接口定义
// ============================================================

/** 应用错误 */
export interface AppError {
    code: string;
    message: string;
    module: string;
    timestamp: number;
}

/** 分页参数 */
export interface PageParams {
    page: number;
    pageSize: number;
}

/** 分页结果 */
export interface PaginatedResult<T> {
    items: T[];
    total: number;
    offset: number;
    limit: number;
}

/** 审计日志 */
export interface AuditLog {
    id: string;
    action: string;
    resource: string;
    detail: string;
    createdAt: number;
}

/** 工作流执行记录 */
export interface WorkflowExecution {
    id: string;
    workflowId: string;
    status: string;
    startedAt: number;
    finishedAt: number | null;
    nodeResults: Record<string, unknown>;
    error: string | null;
}

/** 工作流编排执行结果（与后端 OrchestrationResult 对应） */
export interface OrchestrationResult {
    workflow_id: string;
    status: string;
    node_results: NodeResult[];
}

/** 单个节点执行结果 */
export interface NodeResult {
    node_id: string;
    status: string;
    output: unknown;
    error: string | null;
}