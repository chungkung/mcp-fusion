import { create } from "zustand";
import { type MCPTool, RunStatus } from "@shared/types";
import { mcpService } from "@/services/ipc";

// ============================================================
// 画布节点运行时状态
// ============================================================

export interface CanvasNodeState {
    id: string;
    tool: MCPTool;
    status: RunStatus;
    inputs: Record<string, string>;
    outputs: Record<string, string>;
}

// ============================================================
// 画布 Store
// ============================================================

interface CanvasState {
    nodeStates: Record<string, CanvasNodeState>;
    tools: MCPTool[];
    toolsLoading: boolean;
    toolsError: string | null;
    selectedNodeId: string | null;
    panelOpen: boolean;
    setNodeState: (id: string, state: Partial<CanvasNodeState>) => void;
    addNodeState: (state: CanvasNodeState) => void;
    removeNodeState: (id: string) => void;
    setSelectedNodeId: (id: string | null) => void;
    setPanelOpen: (open: boolean) => void;
    clearAll: () => void;
    updateNodeInput: (nodeId: string, key: string, value: string) => void;
    fetchTools: () => Promise<void>;
}

export const useCanvasStore = create<CanvasState>((set) => ({
    nodeStates: {},
    tools: [],
    toolsLoading: false,
    toolsError: null,
    selectedNodeId: null,
    panelOpen: false,

    setNodeState: (id, state) =>
        set((s) => ({
            nodeStates: {
                ...s.nodeStates,
                [id]: { ...s.nodeStates[id], ...state },
            },
        })),

    addNodeState: (state) =>
        set((s) => ({
            nodeStates: { ...s.nodeStates, [state.id]: state },
        })),

    removeNodeState: (id) =>
        set((s) => {
            const next = { ...s.nodeStates };
            delete next[id];
            return {
                nodeStates: next,
                selectedNodeId: s.selectedNodeId === id ? null : s.selectedNodeId,
                panelOpen: s.selectedNodeId === id ? false : s.panelOpen,
            };
        }),

    setSelectedNodeId: (id) =>
        set({ selectedNodeId: id, panelOpen: id !== null }),

    setPanelOpen: (open) => set({ panelOpen: open }),

    clearAll: () =>
        set({ nodeStates: {}, selectedNodeId: null, panelOpen: false }),

    updateNodeInput: (nodeId, key, value) =>
        set((s) => {
            const node = s.nodeStates[nodeId];
            if (!node) return s;
            return {
                nodeStates: {
                    ...s.nodeStates,
                    [nodeId]: { ...node, inputs: { ...node.inputs, [key]: value } },
                },
            };
        }),

    fetchTools: async () => {
        set({ toolsLoading: true, toolsError: null });
        try {
            const serversResult = await mcpService.listServers();
            if (!serversResult.success || !serversResult.data) {
                set({ toolsError: serversResult.error ?? "获取服务器列表失败", toolsLoading: false });
                return;
            }
            const allTools: MCPTool[] = [];
            const enabledServers = serversResult.data.filter((s) => s.enabled);
            const toolResults = await Promise.all(
                enabledServers.map(async (server) => {
                    const result = await mcpService.listTools(server.id);
                    return result.success && result.data ? result.data : [];
                }),
            );
            toolResults.forEach((tools) => allTools.push(...tools));
            if (allTools.length === 0) {
                set({ tools: MOCK_TOOLS, toolsLoading: false });
                return;
            }
            set({ tools: allTools, toolsLoading: false });
        } catch {
            set({ tools: MOCK_TOOLS, toolsError: null, toolsLoading: false });
        }
    },
}));

// ============================================================
// 模拟 MCP 工具列表（开发/降级用）
// ============================================================

const MOCK_TOOLS: MCPTool[] = [
    {
        name: "http_request",
        description: "发送 HTTP 请求并返回响应数据",
        serverId: "mcp-http",
        inputSchema: {
            type: "object",
            properties: {
                url: { type: "string", description: "请求 URL" },
                method: { type: "string", enum: ["GET", "POST", "PUT", "DELETE"], default: "GET" },
                headers: { type: "string", description: "JSON 格式的请求头" },
                body: { type: "string", description: "请求体 (JSON)" },
            },
            required: ["url"],
        },
        outputSchema: {},
    },
    {
        name: "csv_parser",
        description: "解析 CSV 数据并转换为结构化对象",
        serverId: "mcp-data",
        inputSchema: {
            type: "object",
            properties: {
                data: { type: "string", description: "CSV 原始数据" },
                delimiter: { type: "string", default: "," },
                hasHeader: { type: "boolean", default: true },
            },
            required: ["data"],
        },
        outputSchema: {},
    },
    {
        name: "json_transform",
        description: "对 JSON 数据进行过滤、映射、聚合等变换操作",
        serverId: "mcp-data",
        inputSchema: {
            type: "object",
            properties: {
                data: { type: "string", description: "输入 JSON 数据" },
                expression: { type: "string", description: "JMESPath 表达式" },
            },
            required: ["data", "expression"],
        },
        outputSchema: {},
    },
    {
        name: "text_ai",
        description: "调用 AI 大模型进行文本生成、翻译、摘要等",
        serverId: "mcp-ai",
        inputSchema: {
            type: "object",
            properties: {
                prompt: { type: "string", description: "提示词" },
                model: { type: "string", default: "gpt-4" },
                max_tokens: { type: "number", default: 1024 },
            },
            required: ["prompt"],
        },
        outputSchema: {},
    },
    {
        name: "file_writer",
        description: "将数据写入本地文件系统",
        serverId: "mcp-fs",
        inputSchema: {
            type: "object",
            properties: {
                path: { type: "string", description: "文件路径" },
                content: { type: "string", description: "文件内容" },
            },
            required: ["path", "content"],
        },
        outputSchema: {},
    },
    {
        name: "file_reader",
        description: "从本地文件系统读取文件内容",
        serverId: "mcp-fs",
        inputSchema: {
            type: "object",
            properties: {
                path: { type: "string", description: "文件路径" },
            },
            required: ["path"],
        },
        outputSchema: {},
    },
    {
        name: "webhook_send",
        description: "向指定 Webhook 地址发送通知消息",
        serverId: "mcp-notify",
        inputSchema: {
            type: "object",
            properties: {
                url: { type: "string", description: "Webhook URL" },
                message: { type: "string", description: "消息内容" },
            },
            required: ["url", "message"],
        },
        outputSchema: {},
    },
    {
        name: "db_query",
        description: "执行 SQL 查询并返回结果集",
        serverId: "mcp-db",
        inputSchema: {
            type: "object",
            properties: {
                query: { type: "string", description: "SQL 查询语句" },
                connection: { type: "string", description: "数据库连接名称" },
            },
            required: ["query", "connection"],
        },
        outputSchema: {},
    },
];

// ============================================================
// 从 store 获取工具分类（按 serverId 分组）
// ============================================================

export function getToolCategories(): { label: string; tools: MCPTool[] }[] {
    const tools = useCanvasStore.getState().tools;
    if (tools.length === 0) return [];
    const groups = new Map<string, MCPTool[]>();
    for (const tool of tools) {
        const list = groups.get(tool.serverId) ?? [];
        list.push(tool);
        groups.set(tool.serverId, list);
    }
    return Array.from(groups.entries()).map(([serverId, serverTools]) => ({
        label: serverId,
        tools: serverTools,
    }));
}