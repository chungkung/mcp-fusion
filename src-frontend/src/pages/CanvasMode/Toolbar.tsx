import { type FC, useCallback, useRef, useState } from "react";
import { motion } from "framer-motion";
import toast from "react-hot-toast";
import { cn } from "@/lib/utils";
import { Save, Play, Trash2, Download, Undo, Redo, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useCanvasStore } from "./useCanvasStore";
import { workflowService } from "@/services/ipc";
import { WorkflowMode, RunStatus } from "@shared/types";
import type { Workflow, WorkflowNode, WorkflowEdge } from "@shared/types";
import type { Edge, Node } from "@xyflow/react";

// ============================================================
// 顶部工具栏
// ============================================================

interface ToolbarProps {
    className?: string;
    nodes?: Node[];
    edges?: Edge[];
    canUndo?: boolean;
    canRedo?: boolean;
    onUndo?: () => void;
    onRedo?: () => void;
    onClear?: () => void;
    onExport?: () => void;
    onRun?: () => void;
    nodeCount?: number;
    loading?: boolean;
}

const Toolbar: FC<ToolbarProps> = ({
    className,
    nodes = [],
    edges = [],
    canUndo = false,
    canRedo = false,
    onUndo,
    onRedo,
    onClear,
    onExport,
    onRun,
    nodeCount = 0,
    loading = false,
}) => {
    const nodeStates = useCanvasStore((s) => s.nodeStates);
    const savedWorkflowId = useRef<string | null>(null);
    const [saving, setSaving] = useState(false);

    const handleSave = useCallback(async () => {
        if (saving) return;
        setSaving(true);
        try {
            const now = Date.now();
            const workflowNodes: WorkflowNode[] = Object.values(nodeStates).map(
                (n) => {
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
                },
            );
            const workflowEdges: WorkflowEdge[] = edges.map((e) => ({
                id: e.id,
                source: e.source,
                target: e.target,
                sourceHandle: e.sourceHandle ?? undefined,
                targetHandle: e.targetHandle ?? undefined,
                type: e.type ?? undefined,
                animated: e.animated ?? undefined,
            }));

            // 复用已保存的工作流 ID，避免重复创建记录
            const existingId = savedWorkflowId.current;
            const workflow: Workflow = {
                id: existingId ?? `wf-${now}`,
                name: "画布工作流",
                description: "",
                mode: WorkflowMode.Canvas,
                status: RunStatus.Idle,
                nodes: workflowNodes,
                edges: workflowEdges,
                createdAt: existingId ? 0 : now, // 已存在的工作流保留服务端 created_at
                updatedAt: now,
            };

            const result = await workflowService.save(workflow);
            if (result.success) {
                // 记录服务端返回的 ID，后续保存复用
                if (result.data?.id) {
                    savedWorkflowId.current = result.data.id;
                }
                toast.success("工作流已保存");
            } else {
                toast.error(result.error ?? "保存失败");
            }
        } catch (e) {
            toast.error(e instanceof Error ? e.message : "保存异常");
        } finally {
            setSaving(false);
        }
    }, [nodeStates, nodes, edges, saving]);

    return (
        <div
            className={cn(
                "flex items-center justify-between h-11 px-4 border-b border-border bg-background/80 backdrop-blur-sm shrink-0",
                className,
            )}
        >
            {/* 左侧：标题 + 节点计数 */}
            <div className="flex items-center gap-3">
                <h2 className="text-sm font-semibold text-foreground">
                    工作流画布
                </h2>
                <span className="text-[11px] text-muted-foreground px-2 py-0.5 rounded-full bg-muted">
                    {nodeCount} 个节点
                </span>
                {loading && (
                    <Loader2 className="h-3.5 w-3.5 text-primary animate-spin" />
                )}
            </div>

            {/* 右侧：操作按钮 */}
            <div className="flex items-center gap-1">
                {/* 撤销 */}
                <motion.div whileHover={{ scale: 1.05 }} whileTap={{ scale: 0.95 }}>
                    <Button
                        variant="ghost"
                        size="sm"
                        onClick={onUndo}
                        disabled={!canUndo}
                        className="h-8 px-2 text-xs gap-1"
                        aria-label="撤销"
                    >
                        <Undo className="h-3.5 w-3.5" />
                        <span className="hidden sm:inline">撤销</span>
                    </Button>
                </motion.div>

                {/* 重做 */}
                <motion.div whileHover={{ scale: 1.05 }} whileTap={{ scale: 0.95 }}>
                    <Button
                        variant="ghost"
                        size="sm"
                        onClick={onRedo}
                        disabled={!canRedo}
                        className="h-8 px-2 text-xs gap-1"
                        aria-label="重做"
                    >
                        <Redo className="h-3.5 w-3.5" />
                        <span className="hidden sm:inline">重做</span>
                    </Button>
                </motion.div>

                <span className="w-px h-5 bg-border mx-1" />

                {/* 保存 */}
                <motion.div whileHover={{ scale: 1.05 }} whileTap={{ scale: 0.95 }}>
                    <Button
                        variant="secondary"
                        size="sm"
                        onClick={handleSave}
                        disabled={saving}
                        className="h-8 px-2.5 text-xs gap-1.5"
                        aria-label="保存工作流"
                    >
                        {saving ? (
                            <Loader2 className="h-3.5 w-3.5 animate-spin" />
                        ) : (
                            <Save className="h-3.5 w-3.5" />
                        )}
                        <span className="hidden sm:inline">{saving ? "保存中..." : "保存"}</span>
                    </Button>
                </motion.div>

                {/* 运行 */}
                <motion.div whileHover={{ scale: 1.05 }} whileTap={{ scale: 0.95 }}>
                    <Button
                        size="sm"
                        onClick={onRun}
                        disabled={nodeCount === 0 || loading}
                        className="h-8 px-2.5 text-xs gap-1.5 bg-gradient-to-r from-indigo-500 to-purple-500 hover:from-indigo-600 hover:to-purple-600 text-white"
                        aria-label="执行工作流"
                    >
                        {loading ? (
                            <Loader2 className="h-3.5 w-3.5 animate-spin" />
                        ) : (
                            <Play className="h-3.5 w-3.5" />
                        )}
                        <span className="hidden sm:inline">运行</span>
                    </Button>
                </motion.div>

                <span className="w-px h-5 bg-border mx-1" />

                {/* 导出 */}
                <motion.div whileHover={{ scale: 1.05 }} whileTap={{ scale: 0.95 }}>
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={onExport}
                        disabled={nodeCount === 0}
                        className="h-8 px-2 text-xs gap-1"
                        aria-label="导出工作流"
                    >
                        <Download className="h-3.5 w-3.5" />
                    </Button>
                </motion.div>

                {/* 清空 */}
                <motion.div whileHover={{ scale: 1.05 }} whileTap={{ scale: 0.95 }}>
                    <Button
                        variant="outline"
                        size="sm"
                        onClick={onClear}
                        disabled={nodeCount === 0}
                        className="h-8 px-2 text-xs gap-1 text-destructive border-destructive/30 hover:bg-destructive/10"
                        aria-label="清空画布"
                    >
                        <Trash2 className="h-3.5 w-3.5" />
                    </Button>
                </motion.div>
            </div>
        </div>
    );
};

export default Toolbar;