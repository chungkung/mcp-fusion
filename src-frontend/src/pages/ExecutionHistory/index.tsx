import { type FC, useState, useEffect, useCallback, useRef } from "react";
import PageTransition from "@/components/animations/PageTransition";
import { cn } from "@/lib/utils";
import toast from "react-hot-toast";
import { executionService, workflowService, systemService } from "@/services/ipc";
import {
    type WorkflowExecution,
    type Workflow,
} from "@shared/types";
import {
    Loader2,
    History,
    Clock,
    CheckCircle,
    XCircle,
    AlertTriangle,
    Circle,
    Timer,
    RefreshCw,
    ChevronLeft,
    ChevronRight,
    Filter,
    BarChart3,
    RotateCcw,
    Unlock,
} from "lucide-react";

// ============================================================
// 状态配置
// ============================================================

const STATUS_CONFIG: Record<string, { icon: typeof Circle; color: string; label: string }> = {
    running: { icon: Timer, color: "text-yellow-500", label: "运行中" },
    success: { icon: CheckCircle, color: "text-green-500", label: "成功" },
    failed: { icon: XCircle, color: "text-red-500", label: "失败" },
    timeout: { icon: AlertTriangle, color: "text-orange-500", label: "超时" },
    aborted: { icon: XCircle, color: "text-muted-foreground/50", label: "已中断" },
};

function formatTime(ts: number): string {
    return new Date(ts).toLocaleString("zh-CN", {
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
    });
}

function formatDuration(start: number, end: number | null): string {
    if (!end) return "进行中";
    const ms = end - start;
    if (ms < 1000) return `${ms}ms`;
    if (ms < 60_000) return `${(ms / 1000).toFixed(1)}s`;
    return `${Math.floor(ms / 60_000)}m ${Math.floor((ms % 60_000) / 1000)}s`;
}

// ============================================================
// 执行历史页面
// ============================================================

const PAGE_SIZE = 20;

const ExecutionHistory: FC = () => {
    const [workflows, setWorkflows] = useState<Workflow[]>([]);
    const [selectedWorkflowId, setSelectedWorkflowId] = useState<string>("");
    const [executions, setExecutions] = useState<WorkflowExecution[]>([]);
    const [total, setTotal] = useState(0);
    const [page, setPage] = useState(0);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const cancelledRef = useRef(false);

    // 加载工作流列表
    useEffect(() => {
        cancelledRef.current = false;
        const loadWorkflows = async () => {
            const result = await workflowService.list();
            if (cancelledRef.current) return;
            if (result.success && result.data) {
                setWorkflows(result.data);
                if (result.data.length > 0 && !selectedWorkflowId) {
                    setSelectedWorkflowId(result.data[0].id);
                }
            }
        };
        loadWorkflows();
        return () => {
            cancelledRef.current = true;
        };
    }, []);

    const fetchExecutions = useCallback(async () => {
        if (!selectedWorkflowId) {
            setLoading(false);
            setError(null);
            return;
        }
        setLoading(true);
        setError(null);
        try {
            const result = await executionService.listByWorkflow(
                selectedWorkflowId,
                page * PAGE_SIZE,
                PAGE_SIZE,
            );
            if (cancelledRef.current) return;
            if (result.success && result.data) {
                setExecutions(result.data.items);
                setTotal(result.data.total);
            } else {
                setError(result.error ?? "加载失败");
            }
        } catch (e) {
            if (cancelledRef.current) return;
            setError(e instanceof Error ? e.message : "未知错误");
        } finally {
            if (!cancelledRef.current) setLoading(false);
        }
    }, [selectedWorkflowId, page]);

    useEffect(() => {
        fetchExecutions();
    }, [fetchExecutions]);

    const handleRetry = useCallback(
        async (execution: WorkflowExecution) => {
            toast.loading("正在重试执行...");
            try {
                const res = await systemService.retryWorkflow(
                    execution.workflowId,
                    execution.id,
                );
                if (res.success) {
                    toast.dismiss();
                    toast.success("重试执行已启动");
                    fetchExecutions();
                } else {
                    toast.dismiss();
                    toast.error(res.error ?? "重试失败");
                }
            } catch (e) {
                toast.dismiss();
                toast.error(e instanceof Error ? e.message : "重试失败");
            }
        },
        [fetchExecutions],
    );

    const handleUnlock = useCallback(async () => {
        if (!selectedWorkflowId) return;
        toast.loading("正在释放锁...");
        try {
            const res = await systemService.forceReleaseLock(selectedWorkflowId);
            if (res.success) {
                toast.dismiss();
                toast.success("工作流锁已释放");
            } else {
                toast.dismiss();
                toast.error(res.error ?? "释放锁失败");
            }
        } catch (e) {
            toast.dismiss();
            toast.error(e instanceof Error ? e.message : "释放锁失败");
        }
    }, [selectedWorkflowId]);

    const totalPages = Math.max(1, Math.ceil(total / PAGE_SIZE));
    const selectedWorkflow = workflows.find((w) => w.id === selectedWorkflowId);

    return (
        <PageTransition className="h-full">
            <div className="flex flex-col h-full">
                {/* 头部 */}
                <div className="flex items-center justify-between h-12 px-4 border-b border-border bg-background/70 backdrop-blur-xl shrink-0">
                    <div className="flex items-center gap-2">
                        <History className="h-4 w-4 text-muted-foreground" />
                        <h2 className="text-sm font-semibold text-foreground">
                            执行历史
                        </h2>
                        <span className="text-xs text-muted-foreground/60">
                            ({total} 条记录)
                        </span>
                    </div>
                    <div className="flex items-center gap-2">
                        <button
                            onClick={fetchExecutions}
                            disabled={loading}
                            className="inline-flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-xs font-medium text-muted-foreground hover:bg-accent transition-colors"
                        >
                            <RefreshCw
                                className={cn(
                                    "h-3.5 w-3.5",
                                    loading && "animate-spin",
                                )}
                            />
                            <span>刷新</span>
                        </button>
                        <button
                            onClick={handleUnlock}
                            disabled={!selectedWorkflowId}
                            className="inline-flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-xs font-medium text-orange-500 hover:bg-orange-500/10 transition-colors disabled:opacity-30"
                            title="强制释放工作流执行锁"
                        >
                            <Unlock className="h-3.5 w-3.5" />
                            <span>解锁</span>
                        </button>
                    </div>
                </div>

                {/* 工作流选择器 */}
                <div className="flex items-center gap-2 px-4 py-2 border-b border-border bg-background/40 shrink-0">
                    <Filter className="h-3.5 w-3.5 text-muted-foreground" />
                    <select
                        value={selectedWorkflowId}
                        onChange={(e) => {
                            setSelectedWorkflowId(e.target.value);
                            setPage(0);
                        }}
                        className="h-7 rounded-md border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary/30"
                    >
                        {workflows.length === 0 && (
                            <option value="">暂无工作流</option>
                        )}
                        {workflows.map((w) => (
                            <option key={w.id} value={w.id}>
                                {w.name}
                            </option>
                        ))}
                    </select>
                    {selectedWorkflow && (
                        <span className="text-xs text-muted-foreground/60">
                            {selectedWorkflow.description}
                        </span>
                    )}
                </div>

                {/* 统计概览 */}
                {executions.length > 0 && (
                    <div className="flex items-center gap-4 px-4 py-2 border-b border-border bg-background/30 shrink-0">
                        <BarChart3 className="h-3.5 w-3.5 text-muted-foreground" />
                        <span className="text-xs text-green-500">
                            本页成功: {executions.filter((e) => e.status === "success").length}
                        </span>
                        <span className="text-xs text-red-500">
                            本页失败: {executions.filter((e) => e.status === "failed").length}
                        </span>
                        <span className="text-xs text-yellow-500">
                            本页运行中: {executions.filter((e) => e.status === "running").length}
                        </span>
                    </div>
                )}

                {/* 执行列表 */}
                <div className="flex-1 overflow-auto">
                    {loading && executions.length === 0 ? (
                        <div className="flex items-center justify-center h-full">
                            <Loader2 className="h-6 w-6 animate-spin text-muted-foreground/40" />
                        </div>
                    ) : error ? (
                        <div className="flex items-center justify-center h-full">
                            <p className="text-sm text-red-500">{error}</p>
                        </div>
                    ) : executions.length === 0 ? (
                        <div className="flex flex-col items-center justify-center h-full">
                            <div className="h-12 w-12 rounded-full bg-muted flex items-center justify-center mb-3">
                                <History className="h-5 w-5 text-muted-foreground/40" />
                            </div>
                            <p className="text-sm text-muted-foreground/60">
                                {selectedWorkflowId
                                    ? "该工作流暂无执行记录"
                                    : "请选择一个工作流"}
                            </p>
                        </div>
                    ) : (
                        <div className="divide-y divide-border">
                            {executions.map((exec) => {
                                const statusInfo =
                                    STATUS_CONFIG[exec.status] ?? {
                                        icon: Circle,
                                        color: "text-muted-foreground/50",
                                        label: exec.status,
                                    };
                                const StatusIcon = statusInfo.icon;
                                return (
                                    <div
                                        key={exec.id}
                                        className="flex items-start gap-4 px-4 py-3 hover:bg-accent/30 transition-colors"
                                    >
                                        <div className="shrink-0 mt-0.5">
                                            <StatusIcon
                                                className={cn(
                                                    "h-5 w-5",
                                                    statusInfo.color,
                                                    exec.status === "running" &&
                                                        "animate-pulse",
                                                )}
                                            />
                                        </div>
                                        <div className="flex-1 min-w-0">
                                            <div className="flex items-center gap-2">
                                                <span className="text-sm font-medium text-foreground">
                                                    {statusInfo.label}
                                                </span>
                                                <span className="text-[10px] text-muted-foreground/40 font-mono">
                                                    {exec.id.slice(0, 8)}
                                                </span>
                                            </div>
                                            <div className="flex items-center gap-3 mt-1">
                                                <span className="text-xs text-muted-foreground/70 flex items-center gap-1">
                                                    <Clock className="h-3 w-3" />
                                                    {formatTime(exec.startedAt)}
                                                </span>
                                                <span className="text-xs text-muted-foreground/50">
                                                    耗时: {formatDuration(exec.startedAt, exec.finishedAt)}
                                                </span>
                                            </div>
                                            {exec.error && (
                                                <p className="text-xs text-red-500/80 mt-1 truncate">
                                                    错误: {exec.error}
                                                </p>
                                            )}
                                        </div>
                                        <div className="shrink-0 flex items-center gap-1.5">
                                            <span className="text-[10px] text-muted-foreground/40">
                                                {exec.nodeResults
                                                    ? Object.keys(
                                                          exec.nodeResults as Record<
                                                              string,
                                                              unknown
                                                          >,
                                                      ).length
                                                    : 0}{" "}
                                                节点
                                            </span>
                                            {(exec.status === "failed" ||
                                                exec.status === "timeout" ||
                                                exec.status === "aborted") && (
                                                <button
                                                    onClick={(e) => {
                                                        e.stopPropagation();
                                                        handleRetry(exec);
                                                    }}
                                                    className="inline-flex items-center gap-1 rounded-md px-2 py-1 text-[10px] font-medium text-blue-500 hover:bg-blue-500/10 transition-colors"
                                                    title="重试此执行"
                                                >
                                                    <RotateCcw className="h-3 w-3" />
                                                    <span>重试</span>
                                                </button>
                                            )}
                                        </div>
                                    </div>
                                );
                            })}
                        </div>
                    )}
                </div>

                {/* 分页 */}
                {total > PAGE_SIZE && (
                    <div className="flex items-center justify-between h-10 px-4 border-t border-border bg-background/50 shrink-0">
                        <span className="text-xs text-muted-foreground">
                            共 {total} 条，第 {page + 1}/{totalPages} 页
                        </span>
                        <div className="flex items-center gap-1">
                            <button
                                onClick={() =>
                                    setPage((p) => Math.max(0, p - 1))
                                }
                                disabled={page === 0}
                                className="rounded-md p-1 text-muted-foreground hover:bg-accent disabled:opacity-30 transition-colors"
                            >
                                <ChevronLeft className="h-4 w-4" />
                            </button>
                            <span className="text-xs text-muted-foreground px-2">
                                {page + 1} / {totalPages}
                            </span>
                            <button
                                onClick={() =>
                                    setPage((p) =>
                                        Math.min(totalPages - 1, p + 1),
                                    )
                                }
                                disabled={page >= totalPages - 1}
                                className="rounded-md p-1 text-muted-foreground hover:bg-accent disabled:opacity-30 transition-colors"
                            >
                                <ChevronRight className="h-4 w-4" />
                            </button>
                        </div>
                    </div>
                )}
            </div>
        </PageTransition>
    );
};

export default ExecutionHistory;