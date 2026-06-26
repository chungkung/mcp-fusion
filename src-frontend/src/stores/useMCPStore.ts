import { create } from "zustand";
import {
    type MCPServer,
    type MCPTool,
    type ConnectionStatus,
} from "@shared/types";
import { mcpService } from "@/services/ipc";

interface MCPState {
    // ---- 数据 ----
    servers: MCPServer[];
    tools: MCPTool[];
    selectedServerId: string | null;
    selectedTool: MCPTool | null;

    // ---- 连接状态 ----
    connectionStatus: Record<string, ConnectionStatus>;

    // ---- 加载 ----
    loading: boolean;
    error: string | null;

    // ---- 操作 ----
    fetchServers: () => Promise<void>;
    addServer: (
        server: Omit<MCPServer, "id" | "createdAt" | "updatedAt" | "connectionStatus">,
    ) => Promise<void>;
    removeServer: (id: string) => Promise<void>;
    pingServer: (serverId: string) => Promise<ConnectionStatus>;
    pingAllServers: () => Promise<void>;
    fetchTools: (serverId: string) => Promise<void>;
    executeTool: (
        serverId: string,
        toolName: string,
        inputs: Record<string, unknown>,
    ) => Promise<Record<string, unknown> | null>;
    selectServer: (id: string | null) => void;
    selectTool: (tool: MCPTool | null) => void;
    clearError: () => void;
}

export const useMCPStore = create<MCPState>((set, get) => ({
    // ---- 数据 ----
    servers: [],
    tools: [],
    selectedServerId: null,
    selectedTool: null,

    // ---- 连接状态 ----
    connectionStatus: {},

    // ---- 加载 ----
    loading: false,
    error: null,

    // ---- 操作 ----
    fetchServers: async () => {
        set({ loading: true, error: null });
        try {
            const result = await mcpService.listServers();
            if (result.success && result.data) {
                // 初始化连接状态
                const statusMap: Record<string, ConnectionStatus> = {};
                for (const srv of result.data) {
                    statusMap[srv.id] = srv.connectionStatus ?? "disconnected";
                }
                set({ servers: result.data, connectionStatus: statusMap });
            } else {
                set({ error: result.error ?? "获取服务器列表失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        } finally {
            set({ loading: false });
        }
    },

    addServer: async (server) => {
        set({ loading: true, error: null });
        try {
            const result = await mcpService.addServer(server);
            if (result.success && result.data) {
                set((s) => ({
                    servers: [...s.servers, result.data!],
                    connectionStatus: {
                        ...s.connectionStatus,
                        [result.data!.id]: "disconnected",
                    },
                }));
            } else {
                set({ error: result.error ?? "添加服务器失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        } finally {
            set({ loading: false });
        }
    },

    removeServer: async (id) => {
        set({ loading: true, error: null });
        try {
            const result = await mcpService.removeServer(id);
            if (result.success) {
                set((s) => {
                    const { [id]: _, ...rest } = s.connectionStatus;
                    return {
                        servers: s.servers.filter((sv) => sv.id !== id),
                        selectedServerId:
                            s.selectedServerId === id ? null : s.selectedServerId,
                        connectionStatus: rest,
                    };
                });
            } else {
                set({ error: result.error ?? "删除服务器失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        } finally {
            set({ loading: false });
        }
    },

    pingServer: async (serverId) => {
        // 标记为 connecting
        set((s) => ({
            connectionStatus: { ...s.connectionStatus, [serverId]: "connecting" },
        }));
        try {
            const result = await mcpService.pingServer(serverId);
            if (result.success && result.data) {
                const status: ConnectionStatus =
                    result.data.status === "connected" ? "connected" : "error";
                set((s) => ({
                    connectionStatus: { ...s.connectionStatus, [serverId]: status },
                }));
                return status;
            }
            set((s) => ({
                connectionStatus: { ...s.connectionStatus, [serverId]: "error" },
            }));
            return "error";
        } catch {
            set((s) => ({
                connectionStatus: { ...s.connectionStatus, [serverId]: "error" },
            }));
            return "error";
        }
    },

    pingAllServers: async () => {
        const { servers } = get();
        await Promise.all(
            servers
                .filter((s) => s.enabled)
                .map((s) => get().pingServer(s.id)),
        );
    },

    fetchTools: async (serverId) => {
        set({ error: null });
        try {
            const result = await mcpService.listTools(serverId);
            if (result.success && result.data) {
                set({ tools: result.data });
            } else {
                set({ error: result.error ?? "获取工具列表失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        }
    },

    executeTool: async (serverId, toolName, inputs) => {
        set({ loading: true, error: null });
        try {
            const result = await mcpService.executeTool(
                serverId,
                toolName,
                inputs,
            );
            if (result.success && result.data) {
                return result.data;
            }
            set({ error: result.error ?? "工具执行失败" });
            return null;
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
            return null;
        } finally {
            set({ loading: false });
        }
    },

    selectServer: (id) => set({ selectedServerId: id }),

    selectTool: (tool) => set({ selectedTool: tool }),

    clearError: () => set({ error: null }),
}));