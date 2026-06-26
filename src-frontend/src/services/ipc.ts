import { type IpcResult } from "@shared/types";
import { IPC_CHANNELS } from "@shared/constants";

// ============================================================
// Tauri 环境检测
// ============================================================

function isTauri(): boolean {
    if (typeof window === "undefined") return false;
    // __TAURI_INTERNALS__ 是 Tauri IPC 实际运行所需的内部对象；
    // 仅检查 __TAURI__ 不够，因为在某些异常情况下它可能存在但 IPC 未就绪。
    return (
        "__TAURI_INTERNALS__" in window &&
        typeof (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ === "object" &&
        (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ !== null
    );
}

// ============================================================
// 统一 IPC 调用封装（含浏览器降级 Mock）
// ============================================================

/**
 * 调用 Tauri Rust 命令，统一错误处理。
 * 浏览器 dev 模式下自动降级返回 Mock 数据。
 */
export async function invokeIPC<T = unknown>(
    channel: string,
    args?: Record<string, unknown>,
): Promise<IpcResult<T>> {
    if (!isTauri()) {
        const mock = getMock(channel, args);
        if (mock !== undefined) return mock as IpcResult<T>;
        return {
            success: false,
            error: "Tauri 环境未就绪，请在 Tauri 应用中运行",
        };
    }

    try {
        const { invoke } = await import("@tauri-apps/api/core");
        const data = await invoke<T>(channel, args);
        return { success: true, data };
    } catch (error: unknown) {
        const message =
            error instanceof Error ? error.message : String(error);
        return { success: false, error: message };
    }
}

/**
 * 注册事件监听。浏览器模式下返回空退订函数。
 */
export async function listenIPC<T = unknown>(
    event: string,
    handler: (payload: T) => void,
): Promise<() => void> {
    if (!isTauri()) {
        console.debug(`[IPC Mock] 监听事件: ${event}`);
        return () => {};
    }

    const { listen } = await import("@tauri-apps/api/event");
    const unlisten = await listen<T>(event, (e) => handler(e.payload));
    return unlisten;
}

// ============================================================
// 浏览器 Mock 数据（开发调试用）
// ============================================================

import {
    type MCPServer,
    type MCPTool,
    type Workflow,
    type OrchestrationResult,
    MCPProtocol,
    WorkflowMode,
    type RunStatus,
} from "@shared/types";

let mockWorkflowId = 0;

function getMock(
    channel: string,
    args?: Record<string, unknown>,
): IpcResult<unknown> | undefined {
    switch (channel) {
        case IPC_CHANNELS.MCP_LIST_SERVERS: {
            const servers: MCPServer[] = [
                {
                    id: "mock-1",
                    name: "Demo MCP Server",
                    description: "浏览器调试用模拟服务端",
                    protocol: MCPProtocol.Http,
                    endpoint: "http://localhost:3000/mcp",
                    command: "",
                    args: [],
                    env: {},
                    enabled: true,
                    connectionStatus: "connected",
                    createdAt: Date.now(),
                    updatedAt: Date.now(),
                },
            ];
            return { success: true, data: servers };
        }

        case IPC_CHANNELS.MCP_LIST_TOOLS: {
            const tools: MCPTool[] = [
                {
                    name: "mock_tool",
                    description: "模拟工具",
                    inputSchema: {},
                    outputSchema: {},
                    serverId:
                        (args?.serverId as string) ?? "mock-1",
                },
            ];
            return { success: true, data: tools };
        }

        case IPC_CHANNELS.MCP_EXECUTE_TOOL:
            return {
                success: true,
                data: { result: "mock_execution_ok" },
            };

        case IPC_CHANNELS.MCP_ADD_SERVER: {
            const server = args?.server as Partial<MCPServer>;
            return {
                success: true,
                data: {
                    ...server,
                    id: "mock-new",
                    createdAt: Date.now(),
                    updatedAt: Date.now(),
                },
            };
        }

        case IPC_CHANNELS.MCP_REMOVE_SERVER:
            return { success: true, data: undefined };

        case IPC_CHANNELS.MCP_PING_SERVER:
            return {
                success: true,
                data: { status: "connected", latency_ms: 5, tool_count: 1 },
            };

        case IPC_CHANNELS.FLOW_LOAD: {
            const workflows: Workflow[] = [
                {
                    id: "flow-mock-1",
                    name: "示例工作流",
                    description: "浏览器调试用模拟工作流",
                    mode: WorkflowMode.Canvas,
                    status: "idle" as RunStatus,
                    nodes: [],
                    edges: [],
                    createdAt: Date.now(),
                    updatedAt: Date.now(),
                },
            ];
            return { success: true, data: workflows };
        }

        case IPC_CHANNELS.FLOW_SAVE: {
            const workflow = args?.workflow as Workflow;
            return {
                success: true,
                data: { ...workflow, updatedAt: Date.now() },
            };
        }

        case IPC_CHANNELS.FLOW_DELETE:
            return { success: true, data: undefined };

        case IPC_CHANNELS.FLOW_EXECUTE: {
            const id = (args?.id as string) ?? "mock";
            return {
                success: true,
                data: {
                    workflow_id: id,
                    status: "success",
                    node_results: [],
                },
            };
        }

        case IPC_CHANNELS.INTENT_PARSE: {
            mockWorkflowId++;
            const text = (args?.text as string)?.slice(0, 30) ?? "";
            const parsed: Workflow = {
                id: `flow-${mockWorkflowId}`,
                name: `工作流 ${mockWorkflowId}`,
                description: `由意图解析生成: ${text}`,
                mode: WorkflowMode.Intent,
                status: "idle" as RunStatus,
                nodes: [
                    {
                        id: "node-1",
                        type: "default",
                        position: { x: 100, y: 100 },
                        data: {
                            label: "API 请求",
                            tool: undefined,
                            inputs: {
                                url: "https://api.example.com/data",
                            },
                            outputs: {},
                            config: {},
                        },
                    },
                    {
                        id: "node-2",
                        type: "default",
                        position: { x: 400, y: 100 },
                        data: {
                            label: "保存文件",
                            tool: undefined,
                            inputs: { path: "output.csv" },
                            outputs: {},
                            config: {},
                        },
                    },
                ],
                edges: [
                    {
                        id: "edge-1",
                        source: "node-1",
                        target: "node-2",
                    },
                ],
                createdAt: Date.now(),
                updatedAt: Date.now(),
            };
            return { success: true, data: parsed };
        }

        case IPC_CHANNELS.RUNTIME_STATUS:
            return {
                success: true,
                data: {
                    status: "idle" as RunStatus,
                    message: "就绪",
                },
            };

        case IPC_CHANNELS.RUNTIME_STOP:
            return { success: true, data: undefined };

        case IPC_CHANNELS.MARKETPLACE_LIST: {
            const templates: MarketplaceTemplate[] = [
                { id: "api-data-aggregation", name: "API 数据聚合", description: "从多个 API 获取数据并合并输出", category: "数据处理", version: "1.0.0", author: "MCP Fusion", stars: 128, downloads: 2400, icon: "🔗", node_count: 3, edge_count: 2, tags: ["api", "http", "data"], source_url: "", file_url: "", updated_at: "2025-01-01" },
                { id: "file-batch-process", name: "文件批量处理", description: "批量重命名、转换格式、压缩文件", category: "文件操作", version: "1.0.0", author: "MCP Fusion", stars: 96, downloads: 1800, icon: "📁", node_count: 2, edge_count: 1, tags: ["file", "batch", "convert"], source_url: "", file_url: "", updated_at: "2025-01-01" },
                { id: "cron-scheduler", name: "定时任务调度", description: "基于 Cron 表达式的定时工作流", category: "自动化", version: "1.0.0", author: "MCP Fusion", stars: 75, downloads: 3200, icon: "⏰", node_count: 2, edge_count: 1, tags: ["cron", "schedule", "timer"], source_url: "", file_url: "", updated_at: "2025-01-01" },
                { id: "git-pipeline", name: "Git 操作流水线", description: "自动 clone、commit、push 操作", category: "DevOps", version: "1.0.0", author: "MCP Fusion", stars: 210, downloads: 5600, icon: "🔧", node_count: 3, edge_count: 2, tags: ["git", "ci", "devops"], source_url: "", file_url: "", updated_at: "2025-01-01" },
                { id: "db-migration", name: "数据库迁移", description: "跨数据库的数据迁移与同步", category: "数据库", version: "1.0.0", author: "MCP Fusion", stars: 64, downloads: 1200, icon: "🗄️", node_count: 2, edge_count: 1, tags: ["database", "migration", "sql"], source_url: "", file_url: "", updated_at: "2025-01-01" },
                { id: "image-process", name: "图片处理流水线", description: "裁剪、压缩、加水印一键完成", category: "媒体处理", version: "1.0.0", author: "MCP Fusion", stars: 89, downloads: 1500, icon: "🖼️", node_count: 3, edge_count: 2, tags: ["image", "compress", "watermark"], source_url: "", file_url: "", updated_at: "2025-01-01" },
                { id: "webhook-listener", name: "Webhook 监听", description: "接收 Webhook 并触发后续流程", category: "自动化", version: "1.0.0", author: "MCP Fusion", stars: 156, downloads: 4100, icon: "🪝", node_count: 2, edge_count: 1, tags: ["webhook", "trigger", "event"], source_url: "", file_url: "", updated_at: "2025-01-01" },
                { id: "log-analyzer", name: "日志分析", description: "收集、解析、可视化日志数据", category: "监控", version: "1.0.0", author: "MCP Fusion", stars: 43, downloads: 900, icon: "📊", node_count: 2, edge_count: 1, tags: ["log", "analyze", "monitor"], source_url: "", file_url: "", updated_at: "2025-01-01" },
            ];
            const category = (args?.category as string) ?? "all";
            const search = (args?.search as string) ?? "";
            let filtered = templates;
            if (category && category !== "all") {
                filtered = filtered.filter((t) => t.category === category);
            }
            if (search) {
                const q = search.toLowerCase();
                filtered = filtered.filter(
                    (t) =>
                        t.name.toLowerCase().includes(q) ||
                        t.description.toLowerCase().includes(q) ||
                        t.tags.some((tag) => tag.toLowerCase().includes(q)),
                );
            }
            return { success: true, data: filtered };
        }

        case IPC_CHANNELS.MARKETPLACE_INSTALL: {
            const templateId = (args?.templateId as string) ?? "api-data-aggregation";
            const templateNames: Record<string, string> = {
                "api-data-aggregation": "API 数据聚合",
                "file-batch-process": "文件批量处理",
                "cron-scheduler": "定时任务调度",
                "git-pipeline": "Git 操作流水线",
                "db-migration": "数据库迁移",
                "image-process": "图片处理流水线",
                "webhook-listener": "Webhook 监听",
                "log-analyzer": "日志分析",
            };
            const workflow: Workflow = {
                id: `installed-${templateId}`,
                name: templateNames[templateId] ?? templateId,
                description: `从模板 "${templateId}" 安装的工作流`,
                mode: WorkflowMode.Intent,
                status: "idle" as RunStatus,
                nodes: [
                    {
                        id: "node-1",
                        type: "mcpTool",
                        position: { x: 100, y: 100 },
                        data: { label: "Step 1", tool: undefined, inputs: {}, outputs: {}, config: {} },
                    },
                ],
                edges: [],
                createdAt: Date.now(),
                updatedAt: Date.now(),
            };
            return { success: true, data: workflow };
        }

        case IPC_CHANNELS.MARKETPLACE_CHECK_UPDATES:
            return { success: true, data: [] };

        default:
            return undefined;
    }
}

// ============================================================
// MCP 服务
// ============================================================

export const mcpService = {
    async listServers(): Promise<IpcResult<MCPServer[]>> {
        return invokeIPC<MCPServer[]>(IPC_CHANNELS.MCP_LIST_SERVERS);
    },

    async addServer(
        server: Omit<MCPServer, "id" | "createdAt" | "updatedAt" | "connectionStatus">,
    ): Promise<IpcResult<MCPServer>> {
        return invokeIPC<MCPServer>(IPC_CHANNELS.MCP_ADD_SERVER, {
            server,
        });
    },

    async removeServer(id: string): Promise<IpcResult<void>> {
        return invokeIPC<void>(IPC_CHANNELS.MCP_REMOVE_SERVER, { id });
    },

    async pingServer(
        serverId: string,
    ): Promise<
        IpcResult<{
            status: string;
            latency_ms: number;
            tool_count?: number;
            error?: string;
        }>
    > {
        return invokeIPC(IPC_CHANNELS.MCP_PING_SERVER, { serverId });
    },

    async listTools(serverId: string): Promise<IpcResult<MCPTool[]>> {
        return invokeIPC<MCPTool[]>(IPC_CHANNELS.MCP_LIST_TOOLS, {
            serverId,
        });
    },

    async executeTool(
        serverId: string,
        toolName: string,
        inputs: Record<string, unknown>,
    ): Promise<IpcResult<Record<string, unknown>>> {
        return invokeIPC<Record<string, unknown>>(
            IPC_CHANNELS.MCP_EXECUTE_TOOL,
            { serverId, toolName, inputs },
        );
    },
};

// ============================================================
// 工作流服务
// ============================================================

export const workflowService = {
    async list(): Promise<IpcResult<Workflow[]>> {
        return invokeIPC<Workflow[]>(IPC_CHANNELS.FLOW_LOAD);
    },

    async save(workflow: Workflow): Promise<IpcResult<Workflow>> {
        return invokeIPC<Workflow>(IPC_CHANNELS.FLOW_SAVE, {
            workflow,
        });
    },

    async delete(id: string): Promise<IpcResult<void>> {
        return invokeIPC<void>(IPC_CHANNELS.FLOW_DELETE, { id });
    },

    async execute(id: string): Promise<IpcResult<OrchestrationResult>> {
        return invokeIPC<OrchestrationResult>(IPC_CHANNELS.FLOW_EXECUTE, { id });
    },
};

// ============================================================
// 意图解析服务
// ============================================================

export const intentService = {
    async parse(text: string): Promise<IpcResult<Workflow>> {
        return invokeIPC<Workflow>(IPC_CHANNELS.INTENT_PARSE, { text });
    },

    async parseWithLlm(text: string): Promise<IpcResult<Workflow>> {
        return invokeIPC<Workflow>(IPC_CHANNELS.INTENT_PARSE_LLM, { text });
    },

    async refine(
        currentWorkflowJson: string,
        conversationHistory: string[][],
        refinementText: string,
    ): Promise<IpcResult<Workflow>> {
        return invokeIPC<Workflow>(IPC_CHANNELS.REFINE_WORKFLOW, {
            currentWorkflowJson,
            conversationHistory,
            refinementText,
        });
    },

    async recommendTools(text: string): Promise<IpcResult<string[]>> {
        return invokeIPC<string[]>(IPC_CHANNELS.RECOMMEND_TOOLS, { text });
    },
};

// ============================================================
// 运行时服务
// ============================================================

export const runtimeService = {
    async getStatus(): Promise<
        IpcResult<{ status: RunStatus; message: string }>
    > {
        return invokeIPC<{ status: RunStatus; message: string }>(
            IPC_CHANNELS.RUNTIME_STATUS,
        );
    },

    async stop(): Promise<IpcResult<void>> {
        return invokeIPC<void>(IPC_CHANNELS.RUNTIME_STOP);
    },
};

// ============================================================
// 审计日志服务
// ============================================================

import { type AuditLog, type PaginatedResult, type WorkflowExecution } from "@shared/types";
import { type MarketplaceTemplate, type TemplateDetail, type TemplateUpdate } from "@shared/types";

export const auditService = {
    async list(
        offset: number = 0,
        limit: number = 50,
    ): Promise<IpcResult<PaginatedResult<AuditLog>>> {
        return invokeIPC<PaginatedResult<AuditLog>>(IPC_CHANNELS.AUDIT_LIST, {
            offset,
            limit,
        });
    },

    async search(params: {
        action?: string;
        resource?: string;
        startTime?: number;
        endTime?: number;
        offset?: number;
        limit?: number;
    }): Promise<IpcResult<PaginatedResult<AuditLog>>> {
        return invokeIPC<PaginatedResult<AuditLog>>(
            IPC_CHANNELS.AUDIT_SEARCH,
            {
                action: params.action ?? null,
                resource: params.resource ?? null,
                start_time: params.startTime ?? null,
                end_time: params.endTime ?? null,
                offset: params.offset ?? 0,
                limit: params.limit ?? 50,
            },
        );
    },
};

// ============================================================
// 执行记录服务
// ============================================================

export const executionService = {
    async listByWorkflow(
        workflowId: string,
        offset: number = 0,
        limit: number = 50,
    ): Promise<IpcResult<PaginatedResult<WorkflowExecution>>> {
        return invokeIPC<PaginatedResult<WorkflowExecution>>(
            IPC_CHANNELS.EXECUTIONS_LIST,
            {
                workflowId,
                offset,
                limit,
            },
        );
    },
};

// ============================================================
// 插件市场服务
// ============================================================

export const marketplaceService = {
    async list(params?: {
        category?: string;
        search?: string;
    }): Promise<IpcResult<MarketplaceTemplate[]>> {
        return invokeIPC<MarketplaceTemplate[]>(
            IPC_CHANNELS.MARKETPLACE_LIST,
            {
                category: params?.category ?? null,
                search: params?.search ?? null,
            },
        );
    },

    async getTemplate(
        templateId: string,
        version?: string,
    ): Promise<IpcResult<TemplateDetail>> {
        return invokeIPC<TemplateDetail>(
            IPC_CHANNELS.MARKETPLACE_GET_TEMPLATE,
            {
                templateId,
                version: version ?? null,
            },
        );
    },

    async install(
        templateId: string,
        version?: string,
    ): Promise<IpcResult<Workflow>> {
        return invokeIPC<Workflow>(
            IPC_CHANNELS.MARKETPLACE_INSTALL,
            {
                templateId,
                version: version ?? null,
            },
        );
    },

    async checkUpdates(): Promise<IpcResult<TemplateUpdate[]>> {
        return invokeIPC<TemplateUpdate[]>(
            IPC_CHANNELS.MARKETPLACE_CHECK_UPDATES,
        );
    },
};

// ============================================================
// 认证与权限服务
// ============================================================

export const authService = {
    /** 初始化认证（从数据库加载持久化的 API Key） */
    async init(): Promise<IpcResult<{ configured: boolean }>> {
        return invokeIPC(IPC_CHANNELS.AUTH_INIT);
    },

    /** 生成新的 API Key 并持久化 */
    async generateKey(): Promise<IpcResult<string>> {
        return invokeIPC(IPC_CHANNELS.AUTH_GENERATE_KEY);
    },

    /** 手动设置 API Key */
    async setKey(key: string): Promise<IpcResult<void>> {
        return invokeIPC(IPC_CHANNELS.AUTH_SET_KEY, { key });
    },

    /** 验证 API Key 是否有效 */
    async verifyKey(key: string): Promise<IpcResult<boolean>> {
        return invokeIPC(IPC_CHANNELS.AUTH_VERIFY_KEY, { key });
    },

    /** 获取认证状态（是否已配置 Key、当前角色） */
    async status(): Promise<IpcResult<{ configured: boolean; role: string }>> {
        return invokeIPC(IPC_CHANNELS.AUTH_STATUS);
    },

    /** 清除 API Key */
    async clearKey(): Promise<IpcResult<void>> {
        return invokeIPC(IPC_CHANNELS.AUTH_CLEAR_KEY);
    },

    /** 设置当前用户角色（仅 Admin 可操作） */
    async setRole(role: string): Promise<IpcResult<void>> {
        return invokeIPC(IPC_CHANNELS.AUTH_SET_ROLE, { role });
    },

    /** 获取当前角色及权限详情 */
    async getRole(): Promise<
        IpcResult<{
            role: string;
            permissions: Record<string, boolean>;
        }>
    > {
        return invokeIPC(IPC_CHANNELS.AUTH_GET_ROLE);
    },
};

// ============================================================
// 系统管理服务
// ============================================================

export const systemService = {
    /** 健康检查 */
    async healthCheck(): Promise<
        IpcResult<{
            status: string;
            database: boolean;
            runtime: string;
            version: string;
            timestamp: number;
        }>
    > {
        return invokeIPC(IPC_CHANNELS.HEALTH_CHECK);
    },

    /** 备份数据库 */
    async backup(backupPath?: string): Promise<IpcResult<string>> {
        return invokeIPC(IPC_CHANNELS.BACKUP_DATABASE, {
            backupPath: backupPath ?? null,
        });
    },

    /** 恢复数据库 */
    async restore(backupPath: string): Promise<IpcResult<void>> {
        return invokeIPC(IPC_CHANNELS.RESTORE_DATABASE, { backupPath });
    },

    /** 强制释放工作流锁 */
    async forceReleaseLock(workflowId: string): Promise<IpcResult<void>> {
        return invokeIPC(IPC_CHANNELS.FORCE_RELEASE_LOCK, { workflowId });
    },

    /** 重试失败的工作流（断点续传） */
    async retryWorkflow(
        id: string,
        resumeFromExecutionId: string,
    ): Promise<IpcResult<unknown>> {
        return invokeIPC(IPC_CHANNELS.RETRY_WORKFLOW, {
            id,
            resumeFromExecutionId,
        });
    },

    /** 导出 Prometheus 指标 */
    async metrics(): Promise<IpcResult<string>> {
        return invokeIPC(IPC_CHANNELS.METRICS);
    },

    /** 验证审计链完整性 */
    async verifyAuditChain(): Promise<
        IpcResult<{
            total: number;
            valid: number;
            invalid: number;
            intact: boolean;
            details: unknown[];
        }>
    > {
        return invokeIPC(IPC_CHANNELS.VERIFY_AUDIT_CHAIN);
    },
};

// ============================================================
// 自动更新服务
// ============================================================

export const updaterService = {
    /**
     * 检查是否有可用更新
     * 返回更新信息，无更新时返回 null
     */
    async check(): Promise<IpcResult<{
        version: string;
        body: string;
        date: string;
    } | null>> {
        if (!isTauri()) {
            return { success: true, data: null };
        }
        try {
            const { check } = await import("@tauri-apps/plugin-updater");
            const update = await check();
            if (!update) {
                return { success: true, data: null };
            }
            return {
                success: true,
                data: {
                    version: update.version,
                    body: update.body ?? "",
                    date: update.date ?? "",
                },
            };
        } catch (error: unknown) {
            const message =
                error instanceof Error ? error.message : String(error);
            return { success: false, error: message };
        }
    },

    /**
     * 下载并安装更新
     */
    async downloadAndInstall(
        onProgress?: (progress: number) => void,
    ): Promise<IpcResult<void>> {
        if (!isTauri()) {
            return { success: false, error: "仅支持 Tauri 环境" };
        }
        try {
            const { check } = await import("@tauri-apps/plugin-updater");
            const update = await check();
            if (!update) {
                return { success: false, error: "无可用更新" };
            }

            let downloaded = 0;
            let contentLength = 0;

            await update.downloadAndInstall((event) => {
                switch (event.event) {
                    case "Started":
                        contentLength = event.data.contentLength ?? 0;
                        onProgress?.(0);
                        break;
                    case "Progress":
                        downloaded += event.data.chunkLength;
                        if (contentLength > 0) {
                            onProgress?.(downloaded / contentLength);
                        }
                        break;
                    case "Finished":
                        onProgress?.(1);
                        break;
                }
            });

            return { success: true, data: undefined };
        } catch (error: unknown) {
            const message =
                error instanceof Error ? error.message : String(error);
            return { success: false, error: message };
        }
    },
};