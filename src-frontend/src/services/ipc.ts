import { type IpcResult } from "@shared/types";
import { IPC_CHANNELS } from "@shared/constants";
import { type MCPServer, type MCPTool, type Workflow, type OrchestrationResult, type RunStatus } from "@shared/types";

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
// 统一 IPC 调用封装
// ============================================================

/**
 * 调用 Tauri Rust 命令，统一错误处理。
 */
export async function invokeIPC<T = unknown>(
    channel: string,
    args?: Record<string, unknown>,
): Promise<IpcResult<T>> {
    if (!isTauri()) {
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