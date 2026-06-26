import { useCallback, useEffect, useRef, useState, type FC, type DragEvent } from "react";
import {
    ReactFlow,
    Background,
    Controls,
    MiniMap,
    addEdge,
    useNodesState,
    useEdgesState,
    type Connection,
    type Node,
    type Edge,
    type ReactFlowInstance,
    type NodeChange,
    type EdgeChange,
    BackgroundVariant,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { motion } from "framer-motion";
import { RunStatus } from "@shared/types";
import PageTransition from "@/components/animations/PageTransition";
import CanvasNode from "./CanvasNode";
import AnimatedNode from "@/components/canvas/AnimatedNode";
import AnimatedEdge from "@/components/canvas/AnimatedEdge";
import ToolPanel from "./ToolPanel";
import PropertyPanel from "./PropertyPanel";
import Toolbar from "./Toolbar";
import { useCanvasStore, type CanvasNodeState } from "./useCanvasStore";
import { useWorkflowStore } from "@/stores/useWorkflowStore";
import { workflowService } from "@/services/ipc";
import { listenIPC } from "@/services/ipc";
import toast from "react-hot-toast";
import { WorkflowMode, type Workflow, type WorkflowEdge, type WorkflowNode } from "@shared/types";

// ============================================================
// 节点类型注册
// ============================================================

const nodeTypes = {
    mcpTool: CanvasNode,
    animatedMCP: AnimatedNode,
};

// ============================================================
// 连线类型注册
// ============================================================

const edgeTypes = {
    animated: AnimatedEdge,
};

// ============================================================
// 初始空状态
// ============================================================

const initialNodes: Node[] = [];
const initialEdges: Edge[] = [];

// ============================================================
// 撤销/重做 快照
// ============================================================

interface CanvasSnapshot {
    nodes: Node[];
    edges: Edge[];
    nodeStates: Record<string, CanvasNodeState>;
}

const MAX_HISTORY = 50;

// ============================================================
// CanvasMode 主页面
// ============================================================

const CanvasMode: FC = () => {
    const reactFlowWrapper = useRef<HTMLDivElement>(null);
    const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
    const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
    const rfInstance = useRef<ReactFlowInstance | null>(null);

    // ---- 撤销/重做 ----
    const [undoStack, setUndoStack] = useState<CanvasSnapshot[]>([]);
    const [redoStack, setRedoStack] = useState<CanvasSnapshot[]>([]);

    const pushSnapshot = useCallback(
        (snapshotNodes: Node[], snapshotEdges: Edge[]) => {
            const nodeStates = useCanvasStore.getState().nodeStates;
            setUndoStack((prev) => {
                const next = [...prev, { nodes: snapshotNodes, edges: snapshotEdges, nodeStates }];
                return next.length > MAX_HISTORY ? next.slice(1) : next;
            });
            setRedoStack([]);
        },
        [],
    );

    const undo = useCallback(() => {
        setUndoStack((prevUndo) => {
            if (prevUndo.length === 0) return prevUndo;
            const prev = prevUndo[prevUndo.length - 1];
            // 当前状态推入 redo
            setRedoStack((prevRedo) => [
                ...prevRedo,
                { nodes, edges, nodeStates: useCanvasStore.getState().nodeStates },
            ]);
            setNodes(prev.nodes);
            setEdges(prev.edges);
            // 恢复 nodeStates（通过直接设置整个 store）
            useCanvasStore.setState({ nodeStates: prev.nodeStates });
            return prevUndo.slice(0, -1);
        });
    }, [nodes, edges, setNodes, setEdges]);

    const redo = useCallback(() => {
        setRedoStack((prevRedo) => {
            if (prevRedo.length === 0) return prevRedo;
            const next = prevRedo[prevRedo.length - 1];
            // 当前状态推入 undo
            setUndoStack((prevUndo) => [
                ...prevUndo,
                { nodes, edges, nodeStates: useCanvasStore.getState().nodeStates },
            ]);
            setNodes(next.nodes);
            setEdges(next.edges);
            useCanvasStore.setState({ nodeStates: next.nodeStates });
            return prevRedo.slice(0, -1);
        });
    }, [nodes, edges, setNodes, setEdges]);

    const clearHistory = useCallback(() => {
        setUndoStack([]);
        setRedoStack([]);
    }, []);

    // ---- Store ----
    const addNodeState = useCanvasStore((s) => s.addNodeState);
    const removeNodeState = useCanvasStore((s) => s.removeNodeState);
    const setSelectedNodeId = useCanvasStore((s) => s.setSelectedNodeId);
    const clearAll = useCanvasStore((s) => s.clearAll);
    const setNodeState = useCanvasStore((s) => s.setNodeState);
    const fetchTools = useCanvasStore((s) => s.fetchTools);

    // 组件挂载时获取工具列表
    useEffect(() => {
        fetchTools();
    }, [fetchTools]);

    const workflowLoading = useWorkflowStore((s) => s.loading);

    // ============================================================
    // 连线处理
    // ============================================================

    const onConnect = useCallback(
        (connection: Connection) => {
            pushSnapshot(nodes, edges);
            setEdges((eds: Edge[]) =>
                addEdge(
                    { ...connection, type: "animated", animated: true },
                    eds,
                ),
            );
        },
        [setEdges, nodes, edges, pushSnapshot],
    );

    // ============================================================
    // 节点点击
    // ============================================================

    const onNodeClick = useCallback(
        (_event: React.MouseEvent, node: Node) => {
            setSelectedNodeId(node.id);
        },
        [setSelectedNodeId],
    );

    // ============================================================
    // 画布空白处点击 - 取消选择
    // ============================================================

    const onPaneClick = useCallback(() => {
        setSelectedNodeId(null);
    }, [setSelectedNodeId]);

    // ============================================================
    // 节点删除处理
    // ============================================================

    const onNodesDelete = useCallback(
        (deleted: Node[]) => {
            deleted.forEach((n) => removeNodeState(n.id));
        },
        [removeNodeState],
    );

    // ============================================================
    // 拖拽添加节点
    // ============================================================

    const onDragOver = useCallback((event: DragEvent) => {
        event.preventDefault();
        event.dataTransfer.dropEffect = "move";
    }, []);

    const onDrop = useCallback(
        (event: DragEvent) => {
            event.preventDefault();

            const toolName = event.dataTransfer.getData("application/reactflow-tool");
            if (!toolName || !reactFlowWrapper.current || !rfInstance.current) return;

            const tool = useCanvasStore.getState().tools.find((t) => t.name === toolName);
            if (!tool) return;

            const position = rfInstance.current.screenToFlowPosition({
                x: event.clientX,
                y: event.clientY,
            });

            const nodeId = `${tool.name}-${Date.now()}`;

            pushSnapshot(nodes, edges);

            const newNode: Node = {
                id: nodeId,
                type: "animatedMCP",
                position,
                data: {
                    label: tool.name,
                    toolName: tool.name,
                    description: tool.description,
                    serverId:tool.serverId,
                    status: RunStatus.Idle,
                },
            };

            const newState: CanvasNodeState = {
                id: nodeId,
                tool,
                status: RunStatus.Idle,
                inputs: {},
                outputs: {},
            };

            setNodes((nds: Node[]) => [...nds, newNode]);
            addNodeState(newState);
        },
        [setNodes, addNodeState, nodes, edges, pushSnapshot],
    );

    // ============================================================
    // React Flow 节点变化（含拖拽、删除前拍快照）
    // ============================================================

    const handleNodesChange = useCallback(
        (changes: NodeChange[]) => {
            // 即将删除节点时拍快照
            const hasRemove = changes.some((c) => c.type === "remove");
            if (hasRemove) {
                pushSnapshot(nodes, edges);
            }
            onNodesChange(changes);
        },
        [onNodesChange, nodes, edges, pushSnapshot],
    );

    const handleEdgesChange = useCallback(
        (changes: EdgeChange[]) => {
            const hasRemove = changes.some((c) => c.type === "remove");
            if (hasRemove) {
                pushSnapshot(nodes, edges);
            }
            onEdgesChange(changes);
        },
        [onEdgesChange, nodes, edges, pushSnapshot],
    );

    // ============================================================
    // React Flow 初始化
    // ============================================================

    const onInit = useCallback((instance: ReactFlowInstance) => {
        rfInstance.current = instance;
    }, []);

    // ============================================================
    // 画布操作
    // ============================================================

    const onClearCanvas = useCallback(() => {
        setNodes([]);
        setEdges([]);
        clearAll();
        clearHistory();
    }, [setNodes, setEdges, clearAll, clearHistory]);

    const onExport = useCallback(() => {
        const workflow = {
            nodes: nodes.map((n) => ({
                id: n.id,
                type: n.type,
                position: n.position,
                data: n.data,
            })),
            edges: edges.map((e) => ({
                id: e.id,
                source: e.source,
                target: e.target,
                sourceHandle: e.sourceHandle ?? null,
                targetHandle: e.targetHandle ?? null,
                type: e.type ?? null,
                animated: e.animated ?? null,
            })),
        };
        const blob = new Blob([JSON.stringify(workflow, null, 2)], {
            type: "application/json",
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `workflow-${Date.now()}.json`;
        a.click();
        URL.revokeObjectURL(url);
    }, [nodes, edges]);

    // ============================================================
    // 执行工作流（调用后端 execute_workflow + 监听 node-state-change 事件）
    // ============================================================

    const handleRun = useCallback(async () => {
        const nodeIds = nodes.map((n) => n.id);
        if (nodeIds.length === 0) return;

        const nodeStates = useCanvasStore.getState().nodeStates;
        const now = Date.now();

        // 1. 构建工作流并保存
        const workflowNodes: WorkflowNode[] = Object.values(nodeStates).map((n) => {
            const rfNode = nodes.find((rn) => rn.id === n.id);
            return {
                id: n.id,
                type: "mcpTool",
                position: rfNode?.position ?? { x: 0, y: 0 },
                data: {
                    label: n.tool.name,
                    tool: n.tool,
                    inputs: n.inputs,
                    outputs: n.outputs,
                    config: {},
                },
            };
        });
        const workflowEdges: WorkflowEdge[] = edges.map((e) => ({
            id: e.id,
            source: e.source,
            target: e.target,
            sourceHandle: e.sourceHandle ?? undefined,
            targetHandle: e.targetHandle ?? undefined,
            type: e.type ?? undefined,
            animated: e.animated ?? undefined,
        }));

        const workflow: Workflow = {
            id: `wf-${now}`,
            name: "画布工作流",
            description: "",
            mode: WorkflowMode.Canvas,
            status: RunStatus.Idle,
            nodes: workflowNodes,
            edges: workflowEdges,
            createdAt: now,
            updatedAt: now,
        };

        // 2. 保存工作流
        const saveResult = await workflowService.save(workflow);
        if (!saveResult.success) {
            toast.error(saveResult.error ?? "保存工作流失败");
            return;
        }
        const persistedWorkflowId = saveResult.data?.id ?? workflow.id;

        // 3. 将所有节点设为 Idle
        setNodes((nds) =>
            nds.map((n) => ({ ...n, data: { ...n.data, status: RunStatus.Idle } })),
        );
        nodeIds.forEach((id) => setNodeState(id, { status: RunStatus.Idle }));

        // 4. 注册 node-state-change 事件监听（校验 workflow_id 防止串扰）
        const completedNodes = new Set<string>();
        let cancelled = false;
        const unlisten = await listenIPC<{
            workflow_id: string;
            node_id: string;
            state: string;
            output?: unknown;
            error?: string;
        }>("node-state-change", (payload) => {
            // 只处理当前工作流的事件
            if (payload.workflow_id !== persistedWorkflowId) return;
            if (cancelled) return;

            const stateMap: Record<string, string> = {
                idle: "idle",
                running: "running",
                success: "success",
                failed: "failed",
                skipped: "idle",
                timeout: "timeout",
            };
            const mappedStatus = (stateMap[payload.state] ?? "idle") as string;
            setNodeState(payload.node_id, {
                status: mappedStatus as RunStatus,
            });
            setNodes((nds) =>
                nds.map((n) =>
                    n.id === payload.node_id
                        ? { ...n, data: { ...n.data, status: mappedStatus } }
                        : n,
                ),
            );
            if (payload.state !== "running" && payload.state !== "idle") {
                completedNodes.add(payload.node_id);
            }
        });

        // 5. 执行工作流（try/finally 确保事件监听器始终被清理）
        try {
            const execResult = await workflowService.execute(persistedWorkflowId);

            if (execResult.success) {
                toast.success("工作流执行完成");
            } else {
                toast.error(execResult.error ?? "工作流执行失败");
            }
        } catch (e) {
            toast.error(e instanceof Error ? e.message : "工作流执行异常");
        } finally {
            // 等待所有节点完成，然后清理监听器
            const startWait = Date.now();
            const maxWait = 10000;
            while (completedNodes.size < nodeIds.length && Date.now() - startWait < maxWait) {
                await new Promise((r) => setTimeout(r, 100));
            }
            cancelled = true;
            unlisten();
        }
    }, [nodes, edges, setNodeState, setNodes]);

    // ============================================================
    // 渲染
    // ============================================================

    const nodeCount = nodes.length;

    return (
        <PageTransition className="relative h-full w-full">
            <div className="flex flex-col h-full">
                {/* 顶部工具栏 */}
                <Toolbar
                    nodes={nodes}
                    edges={edges}
                    canUndo={undoStack.length > 0}
                    canRedo={redoStack.length > 0}
                    onUndo={undo}
                    onRedo={redo}
                    onClear={onClearCanvas}
                    onExport={onExport}
                    onRun={handleRun}
                    nodeCount={nodeCount}
                    loading={workflowLoading}
                />

                {/* 三栏布局 */}
                <div className="flex-1 flex min-h-0">
                    {/* 左侧工具面板 */}
                    <div className="w-56 border-r border-border bg-background shrink-0">
                        <ToolPanel />
                    </div>

                    {/* 中间画布 */}
                    <div
                        ref={reactFlowWrapper}
                        className="flex-1 relative"
                        onDragOver={onDragOver}
                        onDrop={onDrop}
                    >
                        <ReactFlow
                            nodes={nodes}
                            edges={edges}
                            onNodesChange={handleNodesChange}
                            onEdgesChange={handleEdgesChange}
                            onConnect={onConnect}
                            onNodeClick={onNodeClick}
                            onPaneClick={onPaneClick}
                            onNodesDelete={onNodesDelete}
                            onInit={onInit}
                            nodeTypes={nodeTypes}
                            edgeTypes={edgeTypes}
                            fitView
                            deleteKeyCode={["Backspace", "Delete"]}
                            multiSelectionKeyCode="Shift"
                            snapToGrid
                            snapGrid={[16, 16]}
                            className="bg-background"
                            defaultEdgeOptions={{
                                type: "animated",
                                animated: true,
                            }}
                        >
                            <Background
                                variant={BackgroundVariant.Dots}
                                gap={20}
                                size={1}
                                color="hsl(var(--border))"
                            />
                            <Controls
                                className="!rounded-lg !border !border-border !bg-card !shadow-md"
                            />
                            <MiniMap
                                nodeStrokeWidth={3}
                                className="!rounded-lg !border !border-border !shadow-md"
                                maskColor="hsl(var(--background))"
                            />
                        </ReactFlow>

                        {/* 空状态提示 */}
                        {nodes.length === 0 && (
                            <motion.div
                                initial={{ opacity: 0 }}
                                animate={{ opacity: 1 }}
                                transition={{ delay: 0.5 }}
                                className="absolute inset-0 flex items-center justify-center pointer-events-none"
                            >
                                <div className="text-center">
                                    <div className="h-16 w-16 mx-auto mb-4 rounded-2xl bg-muted/50 flex items-center justify-center">
                                        <svg
                                            width="32"
                                            height="32"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="currentColor"
                                            strokeWidth="1.5"
                                            strokeLinecap="round"
                                            strokeLinejoin="round"
                                            className="text-muted-foreground/40"
                                        >
                                            <rect x="3" y="3" width="18" height="18" rx="2" />
                                            <path d="M12 8v8" />
                                            <path d="M8 12h8" />
                                        </svg>
                                    </div>
                                    <p className="text-sm text-muted-foreground/60">
                                        从左侧拖拽工具到画布开始构建工作流
                                    </p>
                                    <p className="text-xs text-muted-foreground/40 mt-1">
                                        Ctrl+Z 撤销 · Ctrl+Shift+Z 重做
                                    </p>
                                </div>
                            </motion.div>
                        )}
                    </div>

                    {/* 右侧属性面板 */}
                    <PropertyPanel />
                </div>
            </div>
        </PageTransition>
    );
};

export default CanvasMode;