import { type FC, useState, useEffect, useCallback, useRef } from "react";
import { motion } from "framer-motion";
import PageTransition from "@/components/animations/PageTransition";
import { cn } from "@/lib/utils";
import { auditService } from "@/services/ipc";
import { type AuditLog, type PaginatedResult, type IpcResult } from "@shared/types";
import {
    Calendar,
    Filter,
    X,
    ChevronLeft,
    ChevronRight,
    Loader2,
    FileText,
    Clock,
    Server,
    Workflow,
    Shield,
    Activity,
    RefreshCw,
} from "lucide-react";

// ============================================================
// 操作类型图标映射
// ============================================================

const ACTION_ICON: Record<string, typeof Activity> = {
    "workflow.execute": Workflow,
    "server.add": Server,
    "server.remove": Server,
    "auth.generate_key": Shield,
    "auth.set_key": Shield,
    "auth.clear_key": Shield,
};

const ACTION_LABEL: Record<string, string> = {
    "workflow.execute": "执行工作流",
    "server.add": "添加服务器",
    "server.remove": "删除服务器",
    "auth.generate_key": "生成密钥",
    "auth.set_key": "设置密钥",
    "auth.clear_key": "清除密钥",
};

function getActionIcon(action: string) {
    return ACTION_ICON[action] ?? Activity;
}

function getActionLabel(action: string) {
    return ACTION_LABEL[action] ?? action;
}

function formatTime(ts: number): string {
    return new Date(ts).toLocaleString("zh-CN", {
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
    });
}

// ============================================================
// 审计日志页面
// ============================================================

const PAGE_SIZE = 20;

const AuditLogs: FC = () => {
    const [logs, setLogs] = useState<AuditLog[]>([]);
    const [total, setTotal] = useState(0);
    const [page, setPage] = useState(0);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    // 搜索状态
    const [showFilters, setShowFilters] = useState(false);
    const [searchAction, setSearchAction] = useState("");
    const [searchResource, setSearchResource] = useState("");
    const [startTime, setStartTime] = useState("");
    const [endTime, setEndTime] = useState("");

    // 搜索防抖
    const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);
    const [debouncedResource, setDebouncedResource] = useState("");

    const fetchLogs = useCallback(
        async (forceSearch: boolean = false) => {
            setLoading(true);
            setError(null);
            try {
                const hasFilters =
                    searchAction || debouncedResource || startTime || endTime;
                let result: IpcResult<PaginatedResult<AuditLog>>;

                if (hasFilters || forceSearch) {
                    result = await auditService.search({
                        action: searchAction || undefined,
                        resource: debouncedResource || undefined,
                        startTime: startTime
                            ? new Date(startTime).getTime()
                            : undefined,
                        endTime: endTime
                            ? new Date(endTime).getTime()
                            : undefined,
                        offset: page * PAGE_SIZE,
                        limit: PAGE_SIZE,
                    });
                } else {
                    result = await auditService.list(
                        page * PAGE_SIZE,
                        PAGE_SIZE,
                    );
                }

                if (result.success && result.data) {
                    setLogs(result.data.items);
                    setTotal(result.data.total);
                } else {
                    setError(result.error ?? "加载失败");
                }
            } catch (e) {
                setError(e instanceof Error ? e.message : "未知错误");
            } finally {
                setLoading(false);
            }
        },
        [page, searchAction, debouncedResource, startTime, endTime],
    );

    useEffect(() => {
        fetchLogs();
    }, [fetchLogs]);

    // 搜索资源防抖
    useEffect(() => {
        if (debounceRef.current) clearTimeout(debounceRef.current);
        debounceRef.current = setTimeout(() => {
            setDebouncedResource(searchResource);
            setPage(0);
        }, 300);
        return () => {
            if (debounceRef.current) clearTimeout(debounceRef.current);
        };
    }, [searchResource]);

    const totalPages = Math.max(1, Math.ceil(total / PAGE_SIZE));

    const handleClearFilters = useCallback(() => {
        setSearchAction("");
        setSearchResource("");
        setDebouncedResource("");
        setStartTime("");
        setEndTime("");
        setPage(0);
    }, []);

    return (
        <PageTransition className="h-full">
            <div className="flex flex-col h-full">
                {/* 头部 */}
                <div className="flex items-center justify-between h-12 px-4 border-b border-border bg-background/70 backdrop-blur-xl shrink-0">
                    <div className="flex items-center gap-2">
                        <FileText className="h-4 w-4 text-muted-foreground" />
                        <h2 className="text-sm font-semibold text-foreground">
                            审计日志
                        </h2>
                        <span className="text-xs text-muted-foreground/60">
                            ({total} 条记录)
                        </span>
                    </div>
                    <div className="flex items-center gap-2">
                        <button
                            onClick={() => {
                                fetchLogs(true);
                            }}
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
                            onClick={() => setShowFilters(!showFilters)}
                            className={cn(
                                "inline-flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-xs font-medium transition-colors",
                                showFilters
                                    ? "bg-primary/10 text-primary"
                                    : "text-muted-foreground hover:bg-accent",
                            )}
                        >
                            <Filter className="h-3.5 w-3.5" />
                            <span>筛选</span>
                        </button>
                    </div>
                </div>

                {/* 筛选面板 */}
                {showFilters && (
                    <motion.div
                        initial={{ height: 0, opacity: 0 }}
                        animate={{ height: "auto", opacity: 1 }}
                        className="border-b border-border bg-background/40 px-4 py-3 shrink-0 overflow-hidden"
                    >
                        <div className="flex items-center gap-3 flex-wrap">
                            <div className="flex items-center gap-1.5">
                                <label className="text-[11px] text-muted-foreground whitespace-nowrap">
                                    操作:
                                </label>
                                <select
                                    value={searchAction}
                                    onChange={(e) => {
                                        setSearchAction(e.target.value);
                                        setPage(0);
                                    }}
                                    className="h-7 rounded-md border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary/30"
                                >
                                    <option value="">全部</option>
                                    <option value="workflow.execute">
                                        执行工作流
                                    </option>
                                    <option value="server.add">
                                        添加服务器
                                    </option>
                                    <option value="server.remove">
                                        删除服务器
                                    </option>
                                    <option value="auth.generate_key">
                                        生成密钥
                                    </option>
                                    <option value="auth.set_key">
                                        设置密钥
                                    </option>
                                    <option value="auth.clear_key">
                                        清除密钥
                                    </option>
                                </select>
                            </div>

                            <div className="flex items-center gap-1.5">
                                <label className="text-[11px] text-muted-foreground whitespace-nowrap">
                                    资源:
                                </label>
                                <input
                                    type="text"
                                    value={searchResource}
                                    onChange={(e) =>
                                        setSearchResource(e.target.value)
                                    }
                                    placeholder="workflow:xxx"
                                    className="h-7 w-36 rounded-md border border-border bg-background px-2 text-xs text-foreground placeholder:text-muted-foreground/40 focus:outline-none focus:ring-1 focus:ring-primary/30"
                                />
                            </div>

                            <div className="flex items-center gap-1.5">
                                <Calendar className="h-3.5 w-3.5 text-muted-foreground" />
                                <input
                                    type="date"
                                    value={startTime}
                                    onChange={(e) => {
                                        setStartTime(e.target.value);
                                        setPage(0);
                                    }}
                                    className="h-7 rounded-md border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary/30"
                                />
                                <span className="text-xs text-muted-foreground">
                                    -
                                </span>
                                <input
                                    type="date"
                                    value={endTime}
                                    onChange={(e) => {
                                        setEndTime(e.target.value);
                                        setPage(0);
                                    }}
                                    className="h-7 rounded-md border border-border bg-background px-2 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary/30"
                                />
                            </div>

                            <button
                                onClick={handleClearFilters}
                                className="inline-flex items-center gap-1 rounded-md px-2 py-1 text-[11px] text-muted-foreground hover:bg-accent transition-colors"
                            >
                                <X className="h-3 w-3" />
                                <span>清除</span>
                            </button>
                        </div>
                    </motion.div>
                )}

                {/* 日志列表 */}
                <div className="flex-1 overflow-auto">
                    {loading && logs.length === 0 ? (
                        <div className="flex items-center justify-center h-full">
                            <Loader2 className="h-6 w-6 animate-spin text-muted-foreground/40" />
                        </div>
                    ) : error ? (
                        <div className="flex items-center justify-center h-full">
                            <p className="text-sm text-red-500">{error}</p>
                        </div>
                    ) : logs.length === 0 ? (
                        <div className="flex flex-col items-center justify-center h-full">
                            <div className="h-12 w-12 rounded-full bg-muted flex items-center justify-center mb-3">
                                <FileText className="h-5 w-5 text-muted-foreground/40" />
                            </div>
                            <p className="text-sm text-muted-foreground/60">
                                暂无审计日志
                            </p>
                        </div>
                    ) : (
                        <div className="divide-y divide-border">
                            {logs.map((log) => {
                                const Icon = getActionIcon(log.action);
                                return (
                                    <div
                                        key={log.id}
                                        className="flex items-start gap-4 px-4 py-3 hover:bg-accent/30 transition-colors"
                                    >
                                        <div className="shrink-0 mt-0.5">
                                            <div className="h-8 w-8 rounded-lg bg-muted flex items-center justify-center">
                                                <Icon className="h-4 w-4 text-muted-foreground" />
                                            </div>
                                        </div>
                                        <div className="flex-1 min-w-0">
                                            <div className="flex items-center gap-2">
                                                <span className="text-sm font-medium text-foreground">
                                                    {getActionLabel(log.action)}
                                                </span>
                                                <span className="text-xs text-muted-foreground/50">
                                                    {log.action}
                                                </span>
                                            </div>
                                            <p className="text-xs text-muted-foreground/70 mt-0.5 truncate">
                                                {log.resource}
                                            </p>
                                            {log.detail &&
                                                log.detail !== "{}" && (
                                                    <p className="text-xs text-muted-foreground/50 mt-0.5 font-mono truncate">
                                                        {log.detail}
                                                    </p>
                                                )}
                                        </div>
                                        <div className="shrink-0 flex items-center gap-1.5 text-xs text-muted-foreground/50">
                                            <Clock className="h-3 w-3" />
                                            <span>{formatTime(log.createdAt)}</span>
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
                                onClick={() => setPage((p) => Math.max(0, p - 1))}
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

export default AuditLogs;