import { create } from "zustand";

type Theme = "light" | "dark";
type SidebarState = "expanded" | "collapsed";

// ============================================================
// 主题初始化：读取 localStorage 并同步到 DOM
// ============================================================

function resolveTheme(): Theme {
    if (typeof window === "undefined") return "light";
    const stored = localStorage.getItem("theme") as Theme | null;
    const theme = stored ?? "light";
    // 首次加载时同步应用到 <html> 元素
    document.documentElement.classList.remove("light", "dark");
    document.documentElement.classList.add(theme);
    return theme;
}

// ============================================================
// Store 接口
// ============================================================

interface GlobalState {
    // ---- 主题 ----
    theme: Theme;
    setTheme: (theme: Theme) => void;
    toggleTheme: () => void;

    // ---- 侧边栏 ----
    sidebar: SidebarState;
    sidebarWidth: number;
    setSidebar: (state: SidebarState) => void;
    toggleSidebar: () => void;
    setSidebarWidth: (width: number) => void;

    // ---- 加载 ----
    globalLoading: boolean;
    setGlobalLoading: (loading: boolean) => void;

    // ---- 错误 ----
    globalError: string | null;
    setGlobalError: (error: string | null) => void;
}

// ============================================================
// Store
// ============================================================

export const useGlobalStore = create<GlobalState>((set, get) => ({
    // ---- 主题 ----
    theme: resolveTheme(),

    setTheme: (theme) => {
        localStorage.setItem("theme", theme);
        document.documentElement.classList.remove("light", "dark");
        document.documentElement.classList.add(theme);
        set({ theme });
    },

    toggleTheme: () => {
        const next = get().theme === "light" ? "dark" : "light";
        get().setTheme(next);
    },

    // ---- 侧边栏 ----
    sidebar: "expanded",
    sidebarWidth: 240,

    setSidebar: (sidebar) => set({ sidebar }),

    toggleSidebar: () => {
        const next = get().sidebar === "expanded" ? "collapsed" : "expanded";
        set({ sidebar: next });
    },

    setSidebarWidth: (sidebarWidth) => set({ sidebarWidth }),

    // ---- 加载 ----
    globalLoading: false,
    setGlobalLoading: (globalLoading) => set({ globalLoading }),

    // ---- 错误 ----
    globalError: null,
    setGlobalError: (globalError) => set({ globalError }),
}));