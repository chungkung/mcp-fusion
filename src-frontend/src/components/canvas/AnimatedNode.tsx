import { memo, type FC } from "react";
import { Handle, Position, type NodeProps } from "@xyflow/react";
import { motion, type Variants } from "framer-motion";
import { CheckCircle, XCircle, Loader2, Zap, AlertTriangle } from "lucide-react";
import { cn } from "@/lib/utils";
import { RunStatus } from "@shared/types";

// ============================================================
// 节点数据接口
// ============================================================

export interface AnimatedNodeData {
    /** 工具名称 */
    label: string;
    /** 工具名称（备用） */
    toolName?: string;
    /** 工具描述 */
    description?: string;
    /** 所属 MCP 服务器 ID */
    serverId?: string;
    /** 所属 MCP 服务器名称 */
    serverName?: string;
    /** 运行状态 */
    status?: RunStatus;
}

// ============================================================
// 动画配置
// ============================================================

/** 节点创建弹入动画 */
const entryVariants: Variants = {
    hidden: {
        scale: 0.7,
        opacity: 0,
        y: 8,
    },
    visible: {
        scale: 1,
        opacity: 1,
        y: 0,
        transition: {
            type: "spring",
            stiffness: 400,
            damping: 25,
            mass: 0.8,
        },
    },
    exit: {
        scale: 0.7,
        opacity: 0,
        transition: { duration: 0.2, ease: "easeIn" },
    },
};

/** 状态内容切换动画 */
const contentVariants: Variants = {
    initial: { opacity: 0, scale: 0.85 },
    animate: { opacity: 1, scale: 1 },
    exit: { opacity: 0, scale: 0.85 },
};

// ============================================================
// 状态样式映射
// ============================================================

/** 节点边框/背景样式 */
const statusCardStyles: Record<RunStatus, string> = {
    [RunStatus.Idle]:
        "border-border/60 bg-card/80 backdrop-blur-md",
    [RunStatus.Running]:
        "border-blue-400/60 bg-blue-50/60 dark:bg-blue-950/20 backdrop-blur-md",
    [RunStatus.Success]:
        "border-emerald-400/60 bg-emerald-50/60 dark:bg-emerald-950/20 backdrop-blur-md",
    [RunStatus.Failed]:
        "border-red-400/60 bg-red-50/60 dark:bg-red-950/20 backdrop-blur-md",
    [RunStatus.Timeout]:
        "border-amber-400/60 bg-amber-50/60 dark:bg-amber-950/20 backdrop-blur-md",
};

/** 状态图标配置 */
const statusConfig: Record<
    RunStatus,
    { icon: typeof Zap; className: string; label: string }
> = {
    [RunStatus.Idle]: {
        icon: Zap,
        className: "text-muted-foreground/50",
        label: "空闲",
    },
    [RunStatus.Running]: {
        icon: Loader2,
        className: "text-blue-500 animate-spin",
        label: "运行中",
    },
    [RunStatus.Success]: {
        icon: CheckCircle,
        className: "text-emerald-500",
        label: "成功",
    },
    [RunStatus.Failed]: {
        icon: XCircle,
        className: "text-red-500",
        label: "失败",
    },
    [RunStatus.Timeout]: {
        icon: AlertTriangle,
        className: "text-amber-500",
        label: "超时",
    },
};

// ============================================================
// 组件
// ============================================================

const AnimatedNode: FC<NodeProps> = ({ data, selected }) => {
    const nodeData = data as unknown as AnimatedNodeData;
    const status = nodeData.status ?? RunStatus.Idle;
    const isRunning = status === RunStatus.Running;
    const isFailed = status === RunStatus.Failed || status === RunStatus.Timeout;

    const displayName = nodeData.label || nodeData.toolName || "未命名工具";
    const serverLabel = nodeData.serverName || nodeData.serverId || "MCP Server";

    const StatusIcon = statusConfig[status].icon;

    return (
        <motion.div
            variants={entryVariants}
            initial="hidden"
            animate="visible"
            exit="exit"
            transition={{ type: "spring", stiffness: 400, damping: 20 }}
            className={cn(
                // 基础样式：毛玻璃卡片
                "relative rounded-xl border-2 shadow-lg",
                "transition-shadow duration-300",
                "min-w-[200px] max-w-[280px]",
                // 悬停效果（CSS 替代 framer-motion whileHover，避免干扰 ReactFlow 拖拽）
                "hover:shadow-xl hover:border-primary/30",
                // 状态样式
                statusCardStyles[status],
                // 选中高亮
                selected &&
                    "ring-2 ring-primary ring-offset-2 ring-offset-background shadow-xl border-primary/60",
            )}
        >
            {/* 呼吸边框动画（运行中） */}
            {isRunning && (
                <motion.div
                    className="absolute inset-0 rounded-xl border-2 border-blue-400/40 pointer-events-none"
                    animate={{
                        opacity: [0.4, 1, 0.4],
                        scale: [1, 1.02, 1],
                    }}
                    transition={{
                        duration: 2,
                        repeat: Infinity,
                        ease: "easeInOut",
                    }}
                />
            )}

            {/* 失败状态闪烁边框 */}
            {isFailed && (
                <motion.div
                    className="absolute inset-0 rounded-xl border-2 border-red-400/20 pointer-events-none"
                    animate={{ opacity: [0.2, 0.5, 0.2] }}
                    transition={{
                        duration: 1.5,
                        repeat: Infinity,
                        ease: "easeInOut",
                    }}
                />
            )}

            {/* 输入连接点 */}
            <Handle
                type="target"
                position={Position.Left}
                className={cn(
                    "!w-3 !h-3 !border-2 !bg-background transition-all duration-300",
                    "!border-muted-foreground/60 hover:!border-primary hover:!scale-125",
                    isRunning && "!border-blue-400",
                )}
            />

            {/* 节点内容 */}
            <div className="px-4 py-3">
                {/* 头部：状态图标 + 工具名 */}
                <div className="flex items-center gap-2.5 mb-2">
                    <motion.span
                        key={status}
                        variants={contentVariants}
                        initial="initial"
                        animate="animate"
                        exit="exit"
                        transition={{ duration: 0.2 }}
                    >
                        <StatusIcon
                            className={cn(
                                "h-4 w-4 shrink-0",
                                statusConfig[status].className,
                            )}
                        />
                    </motion.span>

                    <span className="text-sm font-semibold text-foreground truncate">
                        {displayName}
                    </span>
                </div>

                {/* 描述 */}
                {nodeData.description && (
                    <p className="text-[11px] text-muted-foreground/70 leading-tight mb-2 line-clamp-2">
                        {nodeData.description}
                    </p>
                )}

                {/* 底部：服务器标签 + 状态文字 */}
                <div className="flex items-center justify-between">
                    <span className="inline-flex items-center gap-1 text-[10px] px-1.5 py-0.5 rounded-md bg-muted/50 text-muted-foreground font-medium">
                        <span className="h-1.5 w-1.5 rounded-full bg-muted-foreground/40" />
                        {serverLabel}
                    </span>

                    <motion.span
                        key={status}
                        variants={contentVariants}
                        initial="initial"
                        animate="animate"
                        exit="exit"
                        transition={{ duration: 0.2 }}
                        className={cn(
                            "text-[10px] font-medium",
                            status === RunStatus.Idle && "text-muted-foreground/60",
                            status === RunStatus.Running && "text-blue-500",
                            status === RunStatus.Success && "text-emerald-500",
                            status === RunStatus.Failed && "text-red-500",
                            status === RunStatus.Timeout && "text-amber-500",
                        )}
                    >
                        {statusConfig[status].label}
                    </motion.span>
                </div>
            </div>

            {/* 输出连接点 */}
            <Handle
                type="source"
                position={Position.Right}
                className={cn(
                    "!w-3 !h-3 !border-2 !bg-background transition-all duration-300",
                    "!border-muted-foreground/60 hover:!border-primary hover:!scale-125",
                    isRunning && "!border-blue-400",
                )}
            />
        </motion.div>
    );
};

export default memo(AnimatedNode);