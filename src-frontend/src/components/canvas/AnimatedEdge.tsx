import { memo, type FC } from "react";
import {
    BaseEdge,
    getBezierPath,
    type EdgeProps,
} from "@xyflow/react";
import { cn } from "@/lib/utils";
import { RunStatus } from "@shared/types";

// ============================================================
// 连线数据接口
// ============================================================

export interface AnimatedEdgeData {
    /** 运行状态 */
    status?: RunStatus;
}

// ============================================================
// 状态 → 颜色映射
// ============================================================

const statusColors: Record<RunStatus, string> = {
    [RunStatus.Idle]: "hsl(var(--border))",
    [RunStatus.Running]: "#8b5cf6",
    [RunStatus.Success]: "#10b981",
    [RunStatus.Failed]: "#ef4444",
    [RunStatus.Timeout]: "#f59e0b",
};

const statusSelectedColors: Record<RunStatus, string> = {
    [RunStatus.Idle]: "hsl(var(--primary))",
    [RunStatus.Running]: "#a78bfa",
    [RunStatus.Success]: "#34d399",
    [RunStatus.Failed]: "#f87171",
    [RunStatus.Timeout]: "#fbbf24",
};

// ============================================================
// 组件
// ============================================================

const AnimatedEdge: FC<EdgeProps> = ({
    sourceX,
    sourceY,
    targetX,
    targetY,
    sourcePosition,
    targetPosition,
    style = {},
    markerEnd,
    data,
    selected,
}) => {
    const edgeData = data as AnimatedEdgeData | undefined;
    const status = edgeData?.status ?? RunStatus.Idle;
    const isRunning = status === RunStatus.Running;

    const color = selected ? statusSelectedColors[status] : statusColors[status];

    const [edgePath] = getBezierPath({
        sourceX,
        sourceY,
        sourcePosition,
        targetX,
        targetY,
        targetPosition,
    });

    const edgeStyle: React.CSSProperties = {
        ...style,
        stroke: color,
        strokeWidth: selected ? 2.5 : 1.5,
        transition: "stroke 0.3s ease, stroke-width 0.3s ease",
    };

    return (
        <>
            {/* 底层阴影路径（选中时） */}
            {selected && (
                <BaseEdge
                    path={edgePath}
                    style={{
                        stroke: color,
                        strokeWidth: 6,
                        opacity: 0.15,
                        fill: "none",
                    }}
                />
            )}

            {/* 主路径 */}
            <BaseEdge
                path={edgePath}
                markerEnd={markerEnd}
                style={edgeStyle}
            />

            {/* 流动动画路径（虚线循环滚动） */}
            <path
                d={edgePath}
                fill="none"
                stroke={color}
                strokeWidth={isRunning ? 2 : 1.5}
                strokeDasharray={isRunning ? "8 6" : "4 8"}
                strokeLinecap="round"
                className={cn(
                    "pointer-events-none",
                    isRunning && "animate-flow-edge-running",
                    !isRunning && status !== RunStatus.Idle && "animate-flow-edge",
                )}
                style={{
                    opacity: isRunning ? 0.7 : 0.35,
                    filter: isRunning
                        ? `drop-shadow(0 0 4px ${color}80)`
                        : undefined,
                    transition: "opacity 0.3s ease",
                }}
            />

            {/* 运行中状态：流动光点（stroke-dasharray 虚化圆点 + offset 动画） */}
            {isRunning && (
                <path
                    d={edgePath}
                    fill="none"
                    stroke={color}
                    strokeWidth={3}
                    strokeDasharray="0 14"
                    strokeLinecap="round"
                    className="animate-flow-edge-running"
                    style={{
                        filter: `drop-shadow(0 0 6px ${color})`,
                    }}
                />
            )}
        </>
    );
};

export default memo(AnimatedEdge);