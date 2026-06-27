import { lazy, Suspense, type FC } from "react";
import {
    Outlet,
    useLocation,
    createBrowserRouter,
    Navigate,
    type RouteObject,
} from "react-router-dom";
import { AnimatePresence } from "framer-motion";

import MainLayout from "@/components/layout/MainLayout";

// ============================================================
// 懒加载页面
// ============================================================

const IntentPage = lazy(() => import("@/pages/IntentPage"));
const CanvasMode = lazy(() => import("@/pages/CanvasMode"));
const CodeMode = lazy(() => import("@/pages/CodeMode"));
const Marketplace = lazy(() => import("@/pages/Marketplace"));
const Settings = lazy(() => import("@/pages/Settings"));
const AuditLogs = lazy(() => import("@/pages/AuditLogs"));
const ExecutionHistory = lazy(() => import("@/pages/ExecutionHistory"));

// ============================================================
// 路由布局（集成 AnimatePresence 页面切换动画）
// ============================================================

const RouteLayout: FC = () => {
    const location = useLocation();

    return (
        <div className="flex-1 min-h-0 flex flex-col">
            <AnimatePresence mode="wait">
                <Suspense
                    fallback={
                        <div className="flex items-center justify-center flex-1">
                            <div className="h-8 w-8 animate-spin rounded-full border-[3px] border-muted border-t-primary" />
                        </div>
                    }
                >
                    <Outlet key={location.pathname} />
                </Suspense>
            </AnimatePresence>
        </div>
    );
};

// ============================================================
// 路由配置
// ============================================================

export const routeConfig: RouteObject[] = [
    {
        // MainLayout: 侧边栏 + 内容区域
        element: <MainLayout />,
        children: [
            {
                // RouteLayout: AnimatePresence 页面切换动画
                element: <RouteLayout />,
                children: [
                    {
                        index: true,
                        element: <Navigate to="/intent" replace />,
                    },
                    {
                        path: "/intent",
                        element: <IntentPage />,
                    },
                    {
                        path: "/canvas",
                        element: <CanvasMode />,
                    },
                    {
                        path: "/code",
                        element: <CodeMode />,
                    },
                    {
                        path: "/marketplace",
                        element: <Marketplace />,
                    },
                    {
                        path: "/settings",
                        element: <Settings />,
                    },
                    {
                        path: "/audit-logs",
                        element: <AuditLogs />,
                    },
                    {
                        path: "/executions",
                        element: <ExecutionHistory />,
                    },
                ],
            },
        ],
    },
];

export const router = createBrowserRouter(routeConfig);