import { memo, type FC } from "react";
import { Handle, Position, type NodeProps } from "@xyflow/react";
import { motion } from "framer-motion";
import { cn } from "@/lib/utils";
import { RunStatus } from "@shared/types";
import { useCanvasStore } from "./useCanvasStore";

// ============================================================
// 状态颜色映射
// ============================================================

const STATUS_COLORS: Record<RunStatus, string> = {
    [RunStatus.Idle]: "border-muted-foreground/30 bg-card",
    [RunStatus.Running]: "border-blue-400 bg-blue-50 dark:bg-blue-950/30",
    [RunStatus.Success]: "border-emerald-400 bg-emerald-50 dark:bg-emerald-950/30",
    [RunStatus.Failed]: "border-red-400 bg-red-50 dark:bg-red-950/30",
    [RunStatus.Timeout]: "border-amber-400 bg-amber-50 dark:bg-amber-950/30",
};

const STATUS_DOT: Record<RunStatus, string> = {
    [RunStatus.Idle]: "bg-muted-foreground/40",
    [RunStatus.Running]: "bg-blue-500 animate-pulse",
    [RunStatus.Success]: "bg-emerald-500",
    [RunStatus.Failed]: "bg-red-500",
    [RunStatus.Timeout]: "bg-amber-500",
};

// ============================================================
// 节点数据
// ============================================================

export interface CanvasNodeData {
    label: string;
    toolName: string;
    description: string;
    category?:string;
}

// ============================================================
// 自定义节点
// ============================================================

const CanvasNode: FC<NodeProps> = ({ id, data, selected }) => {
    const nodeData = data as unknown as CanvasNodeData;
    const nodeState = useCanvasStore((s) => s.nodeStates[id]);
    const status = nodeState?.status ?? RunStatus.Idle;

    return (
        <motion.div
            initial={{ scale: 0.8, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            exit={{ scale: 0.8, opacity: 0 }}
            transition={{ duration: 0.25, ease: "easeOut" }}
            className={cn(
                "relative rounded-xl border-2 shadow-lg cursor-pointer transition-shadow duration-200 min-w-[180px]",
                STATUS_COLORS[status],
                selected
                    ? "ring-2 ring-primary ring-offset-2 ring-offset-background shadow-xl"
                    : "hover:shadow-md",
            )}
        >
            {/* 输入连接点 */}
            <Handle
                type="target"
                position={Position.Left}
                className="!w-3 !h-3 !border-2 !border-muted-foreground !bg-background hover:!border-primary transition-colors"
            />

            {/* 节点内容 */}
            <div className="px-4 py-3">
                <div className="flex items-center gap-2 mb-1.5">
                    <span
                        className={cn(
                            "inline-block h-2 w-2 rounded-full shrink-0",
                            STATUS_DOT[status],
                        )}
                    />
                    <span className="text-xs font-semibold text-foreground truncate">
                        {nodeData.label}
                    </span>
                </div>
                <p className="text-[11px] text-muted-foreground leading-tight line-clamp-2">
                    {nodeData.description}
                </p>
                <span className="inline-block mt-1.5 text-[10px] px-1.5 py-0.5 rounded-full bg-muted text-muted-foreground font-medium">
                    {nodeData.category}
                </span>
            </div>

            {/* 输出连接点 */}
            <Handle
                type="source"
                position={Position.Right}
                className="!w-3 !h-3 !border-2 !border-muted-foreground !bg-background hover:!border-primary transition-colors"
            />
        </motion.div>
    );
};

export default memo(CanvasNode);