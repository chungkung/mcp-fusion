import { type FC, useCallback } from "react";
import { Outlet, useNavigate, useLocation } from "react-router-dom";
import { motion } from "framer-motion";
import { cn } from "@/lib/utils";
import { useGlobalStore } from "@/stores/useGlobalStore";
import { ROUTES, APP_NAME } from "@shared/constants";
import {
    Sun,
    Moon,
    Settings,
    ChevronLeft,
    ChevronRight,
    Sparkles,
    GitBranch,
    Code,
    ShoppingBag,
    FileText,
    History,
} from "lucide-react";

// ============================================================
// 导航项配置
// ============================================================

interface NavItem {
    path: string;
    label: string;
    icon: typeof Sparkles;
}

const NAV_ITEMS: NavItem[] = [
    {
        path: ROUTES.INTENT,
        label: "意图模式",
        icon: Sparkles,
    },
    {
        path: ROUTES.CANVAS,
        label: "工作流画布",
        icon: GitBranch,
    },
    {
        path: ROUTES.CODE,
        label: "代码视图",
        icon: Code,
    },
    {
        path: ROUTES.MARKETPLACE,
        label: "插件市场",
        icon: ShoppingBag,
    },
    {
        path: ROUTES.SETTINGS,
        label: "设置",
        icon: Settings,
    },
    {
        path: ROUTES.AUDIT,
        label: "审计日志",
        icon: FileText,
    },
    {
        path: ROUTES.EXECUTIONS,
        label: "执行历史",
        icon: History,
    },
];

// ============================================================
// 动画配置
// ============================================================

const sidebarVariants = {
    expanded: { width: 240, opacity: 1 },
    collapsed: { width: 64, opacity: 1 },
};

const sidebarTransition = {
    duration: 0.3,
    ease: "easeInOut",
};

// ============================================================
// 顶部栏
// ============================================================

const TopBar: FC = () => {
    const navigate = useNavigate();
    const theme = useGlobalStore((s) => s.theme);
    const toggleTheme = useGlobalStore((s) => s.toggleTheme);

    const handleSettings = useCallback(() => {
        navigate(ROUTES.SETTINGS);
    }, [navigate]);

    return (
        <header className="flex items-center justify-between h-16 px-6 border-b border-border bg-background/70 backdrop-blur-xl shrink-0">
            {/* 左侧：项目标题 */}
            <div className="flex items-center gap-2">
                <span className="text-base font-bold tracking-tight text-foreground">
                    {APP_NAME}
                </span>
                <span className="text-[10px] px-1.5 py-0.5 rounded-full bg-primary/10 text-primary font-medium">
                    Beta
                </span>
            </div>

            {/* 右侧：深色模式切换 + 设置 */}
            <div className="flex items-center gap-1">
                {/* 深色模式切换 */}
                <button
                    onClick={toggleTheme}
                    className="rounded-lg p-2 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
                    aria-label={theme === "dark" ? "切换浅色模式" : "切换深色模式"}
                >
                    {theme === "dark" ? (
                        <Sun className="h-[18px] w-[18px]" />
                    ) : (
                        <Moon className="h-[18px] w-[18px]" />
                    )}
                </button>

                {/* 设置按钮 */}
                <button
                    onClick={handleSettings}
                    className="rounded-lg p-2 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
                    aria-label="设置"
                >
                    <Settings className="h-[18px] w-[18px]" />
                </button>
            </div>
        </header>
    );
};

// ============================================================
// 侧边栏导航
// ============================================================

interface SidebarNavProps {
    collapsed: boolean;
    onToggle: () => void;
}

const SidebarNav: FC<SidebarNavProps> = ({ collapsed, onToggle }) => {
    const navigate = useNavigate();
    const location = useLocation();

    const handleNav = useCallback(
        (path: string) => {
            navigate(path);
        },
        [navigate],
    );

    return (
        <div className="flex flex-col h-full">
            {/* 侧边栏头部 */}
            <div
                className={cn(
                    "flex items-center border-b border-border shrink-0 h-16",
                    collapsed ? "justify-center px-2" : "justify-between px-4",
                )}
            >
                {!collapsed && (
                    <div className="flex items-center gap-2">
                        <div className="h-7 w-7 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center">
                            <Sparkles className="h-4 w-4 text-white" />
                        </div>
                        <span className="text-sm font-bold tracking-tight text-foreground truncate">
                            {APP_NAME}
                        </span>
                    </div>
                )}
                <button
                    onClick={onToggle}
                    className="rounded-lg p-1.5 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors shrink-0"
                    aria-label={collapsed ? "展开侧边栏" : "收起侧边栏"}
                >
                    {collapsed ? (
                        <ChevronRight className="h-[18px] w-[18px]" />
                    ) : (
                        <ChevronLeft className="h-[18px] w-[18px]" />
                    )}
                </button>
            </div>

            {/* 导航菜单 */}
            <nav
                className={cn(
                    "flex-1 overflow-y-auto",
                    collapsed ? "p-2 space-y-1" : "p-3 space-y-1",
                )}
            >
                {NAV_ITEMS.map((item) => {
                    const isActive = location.pathname === item.path;
                    const Icon = item.icon;

                    return (
                        <button
                            key={item.path}
                            onClick={() => handleNav(item.path)}
                            title={collapsed ? item.label : undefined}
                            className={cn(
                                "flex items-center w-full rounded-lg text-sm font-medium transition-all duration-200",
                                collapsed
                                    ? "justify-center p-2.5"
                                    : "gap-3 px-3 py-2.5",
                                isActive
                                    ? "bg-primary/10 text-primary"
                                    : "text-muted-foreground hover:bg-accent hover:text-accent-foreground",
                            )}
                        >
                            <Icon
                                className={cn(
                                    "h-5 w-5 shrink-0",
                                    isActive
                                        ? "text-primary"
                                        : "text-muted-foreground",
                                )}
                            />
                            {!collapsed && (
                                <span className="truncate">{item.label}</span>
                            )}
                        </button>
                    );
                })}
            </nav>

            {/* 侧边栏底部：版本信息 */}
            {!collapsed && (
                <div className="p-3 border-t border-border shrink-0">
                    <p className="text-[11px] text-muted-foreground text-center">
                        MCP Fusion v0.1.0
                    </p>
                </div>
            )}
        </div>
    );
};

// ============================================================
// MainLayout
// ============================================================

const MainLayout: FC = () => {
    const sidebar = useGlobalStore((s) => s.sidebar);
    const toggleSidebar = useGlobalStore((s) => s.toggleSidebar);

    const isExpanded = sidebar === "expanded";

    return (
        <div className="flex h-screen overflow-hidden">
            {/* 侧边栏 */}
            <motion.aside
                initial={false}
                animate={isExpanded ? "expanded" : "collapsed"}
                variants={sidebarVariants}
                transition={sidebarTransition}
                className="h-full border-r border-border bg-background/80 backdrop-blur-xl overflow-hidden shrink-0"
            >
                <SidebarNav
                    collapsed={!isExpanded}
                    onToggle={toggleSidebar}
                />
            </motion.aside>

            {/* 右侧：顶部栏 + 内容区域 */}
            <div className="flex-1 flex flex-col min-w-0">
                <TopBar />
                <main className="flex-1 overflow-auto relative">
                    <Outlet />
                </main>
            </div>
        </div>
    );
};

export default MainLayout;