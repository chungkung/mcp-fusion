// src-shared 统一类型导出
// 外部通过 @shared/types 一次性导入所有类型

// ---- IpcResult ----
export type { IpcResult } from "./ipc";

// ---- Common ----
export { RunStatus, MCPProtocol, WorkflowMode } from "./common";
export type { AppError, PageParams, PaginatedResult, AuditLog, WorkflowExecution, OrchestrationResult, NodeResult } from "./common";

// ---- MCP ----
export type { MCPServer, MCPTool, ConnectionStatus } from "./mcp";

// ---- Workflow ----
export type {
    WorkflowNodePosition,
    WorkflowNodeData,
    WorkflowNode,
    WorkflowEdge,
    Workflow,
} from "./workflow";

// ---- Marketplace ----
export type {
    MarketplaceTemplate,
    TemplateDetail,
    WorkflowTemplate,
    InstalledTemplate,
    TemplateUpdate,
} from "./marketplace";