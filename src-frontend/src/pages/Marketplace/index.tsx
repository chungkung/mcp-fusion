import { type FC, useState, useCallback, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { motion } from "framer-motion";
import toast from "react-hot-toast";
import PageTransition from "@/components/animations/PageTransition";
import LoadingSpinner from "@/components/animations/LoadingSpinner";
import { cn } from "@/lib/utils";
import { Search, Star, Download, ArrowRight, RefreshCw } from "lucide-react";
import { marketplaceService } from "@/services/ipc";
import { ROUTES } from "@shared/constants";
import { useWorkflowStore } from "@/stores/useWorkflowStore";
import { type MarketplaceTemplate } from "@shared/types";

// ============================================================
// 分类列表
// ============================================================

const categories = [
    { key: "all", label: "全部" },
    { key: "自动化", label: "自动化" },
    { key: "数据处理", label: "数据处理" },
    { key: "DevOps", label: "DevOps" },
    { key: "文件操作", label: "文件操作" },
    { key: "媒体处理", label: "媒体处理" },
    { key: "数据库", label: "数据库" },
    { key: "监控", label: "监控" },
];

// ============================================================
// 动画配置
// ============================================================

const containerVariants = {
    hidden: {},
    visible: {
        transition: { staggerChildren: 0.06 },
    },
};

const cardVariants = {
    hidden: { opacity: 0, y: 16 },
    visible: {
        opacity: 1,
        y: 0,
        transition: { duration: 0.3, ease: "easeOut" },
    },
};

// ============================================================
// 组件
// ============================================================

const Marketplace: FC = () => {
    const navigate = useNavigate();
    const setCurrentWorkflow = useWorkflowStore((s) => s.setCurrentWorkflow);

    const [searchQuery, setSearchQuery] = useState("");
    const [activeCategory, setActiveCategory] = useState("all");
    const [templates, setTemplates] = useState<MarketplaceTemplate[]>([]);
    const [loading, setLoading] = useState(true);
    const [installing, setInstalling] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);

    // 加载模板列表
    const loadTemplates = useCallback(async () => {
        setLoading(true);
        setError(null);
        try {
            const result = await marketplaceService.list({
                category: activeCategory !== "all" ? activeCategory : undefined,
                search: searchQuery || undefined,
            });
            if (result.success && result.data) {
                setTemplates(result.data);
            } else {
                setError(result.error ?? "加载模板列表失败");
            }
        } catch {
            setError("网络异常，请检查连接后重试");
        } finally {
            setLoading(false);
        }
    }, [activeCategory, searchQuery]);

    useEffect(() => {
        loadTemplates();
    }, [loadTemplates]);

    // 搜索防抖
    const handleSearchChange = useCallback(
        (e: React.ChangeEvent<HTMLInputElement>) => {
            setSearchQuery(e.target.value);
        },
        [],
    );

    // 一键安装模板
    const handleInstall = useCallback(
        async (template: MarketplaceTemplate) => {
            setInstalling(template.id);
            try {
                const result = await marketplaceService.install(
                    template.id,
                    template.version,
                );
                if (result.success && result.data) {
                    setCurrentWorkflow(result.data);
                    toast.success(`模板 "${template.name}" 安装成功！`);
                    navigate(ROUTES.CANVAS);
                } else {
                    toast.error(result.error ?? "安装失败，请重试");
                }
            } catch {
                toast.error("网络异常，请检查连接后重试");
            } finally {
                setInstalling(null);
            }
        },
        [navigate, setCurrentWorkflow],
    );

    return (
        <PageTransition className="h-full">
            <div className="flex flex-col h-full">
                {/* 顶部搜索栏 */}
                <div className="shrink-0 px-6 py-5 border-b border-border bg-background/70 backdrop-blur-xl">
                    <div className="flex items-center gap-4">
                        <h1 className="text-lg font-bold text-foreground shrink-0">
                            插件市场
                        </h1>

                        {/* 搜索框 */}
                        <div className="relative flex-1 max-w-md">
                            <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground/50" />
                            <input
                                type="text"
                                value={searchQuery}
                                onChange={handleSearchChange}
                                placeholder="搜索模板..."
                                className="w-full h-9 pl-9 pr-4 rounded-lg border border-border bg-background/60 text-sm text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:ring-2 focus:ring-primary/30 transition-shadow"
                            />
                        </div>

                        {/* 刷新按钮 */}
                        <button
                            onClick={loadTemplates}
                            disabled={loading}
                            className="shrink-0 p-2 rounded-lg text-muted-foreground hover:text-foreground hover:bg-accent transition-colors disabled:opacity-40"
                            title="刷新"
                        >
                            <RefreshCw
                                className={cn(
                                    "h-4 w-4",
                                    loading && "animate-spin",
                                )}
                            />
                        </button>
                    </div>
                </div>

                {/* 分类筛选 */}
                <div className="shrink-0 px-6 py-2 border-b border-border/50 bg-background/40">
                    <div className="flex items-center gap-1 overflow-x-auto">
                        {categories.map((cat) => (
                            <button
                                key={cat.key}
                                onClick={() => setActiveCategory(cat.key)}
                                className={cn(
                                    "px-3 py-1.5 rounded-lg text-xs font-medium whitespace-nowrap transition-colors",
                                    activeCategory === cat.key
                                        ? "bg-primary/10 text-primary"
                                        : "text-muted-foreground hover:text-foreground hover:bg-accent",
                                )}
                            >
                                {cat.label}
                            </button>
                        ))}
                    </div>
                </div>

                {/* 模板卡片网格 */}
                <div className="flex-1 overflow-y-auto px-6 py-4">
                    {/* 加载状态 */}
                    {loading && (
                        <div className="flex items-center justify-center py-20">
                            <LoadingSpinner size="md" />
                            <span className="ml-3 text-sm text-muted-foreground">
                                加载模板中...
                            </span>
                        </div>
                    )}

                    {/* 错误状态 */}
                    {!loading && error && (
                        <div className="flex flex-col items-center justify-center py-20">
                            <div className="h-16 w-16 rounded-full bg-destructive/10 flex items-center justify-center mb-4">
                                <Search className="h-6 w-6 text-destructive/40" />
                            </div>
                            <p className="text-sm text-destructive">{error}</p>
                            <button
                                onClick={loadTemplates}
                                className="mt-3 text-xs text-primary hover:underline"
                            >
                                点击重试
                            </button>
                        </div>
                    )}

                    {/* 模板列表 */}
                    {!loading && !error && (
                        <motion.div
                            className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
                            variants={containerVariants}
                            initial="hidden"
                            animate="visible"
                            key={activeCategory + searchQuery}
                        >
                            {templates.map((template) => (
                                <motion.div
                                    key={template.id}
                                    variants={cardVariants}
                                    className="group rounded-xl border border-border bg-background/60 backdrop-blur-sm p-4 hover:border-primary/30 hover:shadow-md transition-all"
                                >
                                    {/* 图标 + 标题 */}
                                    <div className="flex items-start gap-3 mb-3">
                                        <div className="h-10 w-10 rounded-lg bg-muted flex items-center justify-center text-xl shrink-0">
                                            {template.icon}
                                        </div>
                                        <div className="min-w-0">
                                            <h3 className="text-sm font-semibold text-foreground truncate">
                                                {template.name}
                                            </h3>
                                            <span className="text-[11px] text-muted-foreground/70">
                                                {template.category}
                                            </span>
                                        </div>
                                    </div>

                                    {/* 描述 */}
                                    <p className="text-xs text-muted-foreground/80 leading-relaxed mb-4 line-clamp-2">
                                        {template.description}
                                    </p>

                                    {/* 底部：统计 + 操作 */}
                                    <div className="flex items-center justify-between">
                                        <div className="flex items-center gap-3">
                                            <span className="flex items-center gap-1 text-[11px] text-muted-foreground/60">
                                                <Star className="h-3 w-3" />
                                                {template.stars}
                                            </span>
                                            <span className="flex items-center gap-1 text-[11px] text-muted-foreground/60">
                                                <Download className="h-3 w-3" />
                                                {template.downloads}
                                            </span>
                                            {template.version && (
                                                <span className="text-[10px] text-muted-foreground/40">
                                                    v{template.version}
                                                </span>
                                            )}
                                        </div>
                                        <button
                                            onClick={() => handleInstall(template)}
                                            disabled={installing === template.id}
                                            className="flex items-center gap-1 text-[11px] font-medium text-primary opacity-0 group-hover:opacity-100 transition-opacity disabled:opacity-40"
                                        >
                                            {installing === template.id ? (
                                                <LoadingSpinner size="sm" />
                                            ) : (
                                                <>
                                                    <span>使用</span>
                                                    <ArrowRight className="h-3 w-3" />
                                                </>
                                            )}
                                        </button>
                                    </div>
                                </motion.div>
                            ))}
                        </motion.div>
                    )}

                    {/* 空状态 */}
                    {!loading && !error && templates.length === 0 && (
                        <div className="flex flex-col items-center justify-center py-20">
                            <div className="h-16 w-16 rounded-full bg-muted flex items-center justify-center mb-4">
                                <Search className="h-6 w-6 text-muted-foreground/30" />
                            </div>
                            <p className="text-sm text-muted-foreground/60">
                                没有找到匹配的模板
                            </p>
                            <p className="text-xs text-muted-foreground/40 mt-1">
                                尝试更换关键词或筛选条件
                            </p>
                        </div>
                    )}
                </div>
            </div>
        </PageTransition>
    );
};

export default Marketplace;