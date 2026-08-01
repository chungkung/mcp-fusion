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
                set({ tools: [], toolsError: null, toolsLoading: false });
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
            set({ tools: allTools, toolsLoading: false });
        } catch {
            set({ tools: [], toolsError: null, toolsLoading: false });
        }
    },
}));

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