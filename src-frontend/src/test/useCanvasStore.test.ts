import { describe, it, expect, beforeEach } from "vitest";
import { useCanvasStore, getToolCategories, type CanvasNodeState } from "@/pages/CanvasMode/useCanvasStore";
import { type MCPTool, RunStatus } from "@shared/types";

// ============================================================
// 测试辅助工具
// ============================================================

function makeTool(overrides: Partial<MCPTool> = {}): MCPTool {
    return {
        name: "test_tool",
        description: "测试工具",
        serverId: "test-server",
        inputSchema: { type: "object", properties: {}, required: [] },
        outputSchema: {},
        ...overrides,
    };
}

function makeNodeState(id: string, overrides: Partial<CanvasNodeState> = {}): CanvasNodeState {
    return {
        id,
        tool: makeTool(),
        status: RunStatus.Idle,
        inputs: {},
        outputs: {},
        ...overrides,
    };
}

// ============================================================
// useCanvasStore 单元测试
// ============================================================

describe("useCanvasStore", () => {
    beforeEach(() => {
        useCanvasStore.setState({
            nodeStates: {},
            tools: [],
            toolsLoading: false,
            toolsError: null,
            selectedNodeId: null,
            panelOpen: false,
        });
    });

    // --- 初始状态 ---
    it("初始状态应为空", () => {
        const state = useCanvasStore.getState();
        expect(state.nodeStates).toEqual({});
        expect(state.tools).toEqual([]);
        expect(state.toolsLoading).toBe(false);
        expect(state.toolsError).toBeNull();
        expect(state.selectedNodeId).toBeNull();
        expect(state.panelOpen).toBe(false);
    });

    // --- addNodeState ---
    it("addNodeState 应添加节点状态", () => {
        const node = makeNodeState("node-1");
        useCanvasStore.getState().addNodeState(node);
        expect(useCanvasStore.getState().nodeStates["node-1"]).toEqual(node);
    });

    it("addNodeState 应覆盖同 ID 节点", () => {
        const node1 = makeNodeState("node-1", { status: RunStatus.Idle });
        const node2 = makeNodeState("node-1", { status: RunStatus.Running });
        useCanvasStore.getState().addNodeState(node1);
        useCanvasStore.getState().addNodeState(node2);
        expect(useCanvasStore.getState().nodeStates["node-1"].status).toBe(RunStatus.Running);
    });

    // --- setNodeState ---
    it("setNodeState 应更新已有节点的部分状态", () => {
        useCanvasStore.getState().addNodeState(makeNodeState("node-1"));
        useCanvasStore.getState().setNodeState("node-1", { status: RunStatus.Success });
        const node = useCanvasStore.getState().nodeStates["node-1"];
        expect(node.status).toBe(RunStatus.Success);
        expect(node.tool.name).toBe("test_tool"); // 其他字段不变
    });

    it("setNodeState 对不存在的节点应创建新状态", () => {
        useCanvasStore.getState().setNodeState("no-exist", { status: RunStatus.Running });
        const node = useCanvasStore.getState().nodeStates["no-exist"];
        expect(node).toBeDefined();
        expect(node.status).toBe(RunStatus.Running);
    });

    // --- removeNodeState ---
    it("removeNodeState 应删除节点", () => {
        useCanvasStore.getState().addNodeState(makeNodeState("node-1"));
        useCanvasStore.getState().removeNodeState("node-1");
        expect(useCanvasStore.getState().nodeStates["node-1"]).toBeUndefined();
    });

    it("removeNodeState 删除选中节点时应清空选中和面板", () => {
        useCanvasStore.getState().addNodeState(makeNodeState("node-1"));
        useCanvasStore.getState().setSelectedNodeId("node-1");
        useCanvasStore.getState().removeNodeState("node-1");
        expect(useCanvasStore.getState().selectedNodeId).toBeNull();
        expect(useCanvasStore.getState().panelOpen).toBe(false);
    });

    it("removeNodeState 删除非选中节点时不影响选中", () => {
        useCanvasStore.getState().addNodeState(makeNodeState("node-1"));
        useCanvasStore.getState().addNodeState(makeNodeState("node-2"));
        useCanvasStore.getState().setSelectedNodeId("node-1");
        useCanvasStore.getState().removeNodeState("node-2");
        expect(useCanvasStore.getState().selectedNodeId).toBe("node-1");
        expect(useCanvasStore.getState().panelOpen).toBe(true);
    });

    // --- setSelectedNodeId ---
    it("setSelectedNodeId 应同时设置选中和面板打开", () => {
        useCanvasStore.getState().setSelectedNodeId("node-1");
        expect(useCanvasStore.getState().selectedNodeId).toBe("node-1");
        expect(useCanvasStore.getState().panelOpen).toBe(true);
    });

    it("setSelectedNodeId(null) 应关闭面板", () => {
        useCanvasStore.getState().setSelectedNodeId("node-1");
        useCanvasStore.getState().setSelectedNodeId(null);
        expect(useCanvasStore.getState().selectedNodeId).toBeNull();
        expect(useCanvasStore.getState().panelOpen).toBe(false);
    });

    // --- setPanelOpen ---
    it("setPanelOpen(false) 应仅关闭面板不取消选中", () => {
        useCanvasStore.getState().setSelectedNodeId("node-1");
        useCanvasStore.getState().setPanelOpen(false);
        expect(useCanvasStore.getState().panelOpen).toBe(false);
        expect(useCanvasStore.getState().selectedNodeId).toBe("node-1");
    });

    // --- clearAll ---
    it("clearAll 应清空节点、选中和面板", () => {
        useCanvasStore.getState().addNodeState(makeNodeState("node-1"));
        useCanvasStore.getState().addNodeState(makeNodeState("node-2"));
        useCanvasStore.getState().setSelectedNodeId("node-1");
        useCanvasStore.getState().clearAll();
        expect(useCanvasStore.getState().nodeStates).toEqual({});
        expect(useCanvasStore.getState().selectedNodeId).toBeNull();
        expect(useCanvasStore.getState().panelOpen).toBe(false);
    });

    // --- updateNodeInput ---
    it("updateNodeInput 应更新节点输入", () => {
        useCanvasStore.getState().addNodeState(makeNodeState("node-1"));
        useCanvasStore.getState().updateNodeInput("node-1", "url", "https://example.com");
        expect(useCanvasStore.getState().nodeStates["node-1"].inputs.url).toBe("https://example.com");
    });

    it("updateNodeInput 对不存在的节点应无副作用", () => {
        const before = useCanvasStore.getState();
        useCanvasStore.getState().updateNodeInput("no-exist", "key", "value");
        expect(useCanvasStore.getState().nodeStates).toEqual(before.nodeStates);
    });

    it("updateNodeInput 应保留已有的其他输入", () => {
        useCanvasStore.getState().addNodeState({
            ...makeNodeState("node-1"),
            inputs: { existing: "old" },
        });
        useCanvasStore.getState().updateNodeInput("node-1", "new", "value");
        const inputs = useCanvasStore.getState().nodeStates["node-1"].inputs;
        expect(inputs.existing).toBe("old");
        expect(inputs.new).toBe("value");
    });
});

// ============================================================
// getToolCategories 测试
// ============================================================

describe("getToolCategories", () => {
    beforeEach(() => {
        useCanvasStore.setState({ tools: [] });
    });

    it("空工具列表应返回空数组", () => {
        expect(getToolCategories()).toEqual([]);
    });

    it("应按 serverId 分组工具", () => {
        useCanvasStore.setState({
            tools: [
                makeTool({ name: "tool-a", serverId: "server-1" }),
                makeTool({ name: "tool-b", serverId: "server-1" }),
                makeTool({ name: "tool-c", serverId: "server-2" }),
            ],
        });
        const categories = getToolCategories();
        expect(categories).toHaveLength(2);
        expect(categories[0].label).toBe("server-1");
        expect(categories[0].tools).toHaveLength(2);
        expect(categories[1].label).toBe("server-2");
        expect(categories[1].tools).toHaveLength(1);
    });
});