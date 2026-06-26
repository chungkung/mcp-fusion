import { type FC, useState, useEffect, useCallback } from "react";
import { motion } from "framer-motion";
import PageTransition from "@/components/animations/PageTransition";
import { cn } from "@/lib/utils";
import toast from "react-hot-toast";
import { useGlobalStore } from "@/stores/useGlobalStore";
import { useMCPStore } from "@/stores/useMCPStore";
import { authService, systemService } from "@/services/ipc";
import {
    Settings2,
    Server,
    Shield,
    Info,
    Sun,
    Moon,
    Plus,
    Trash2,
    Wifi,
    WifiOff,
    Loader2,
    RefreshCw,
    Plug,
    AlertCircle,
    X,
    Key,
    Copy,
    Eye,
    EyeOff,
    Check,
    Database,
    Download,
    Upload,
    HardDrive,
} from "lucide-react";
import { type ConnectionStatus, MCPProtocol } from "@shared/types";

// ============================================================
// 标签页配置
// ============================================================

interface Tab {
    key: string;
    label: string;
    icon: typeof Settings2;
}

const TABS: Tab[] = [
    { key: "general", label: "通用", icon: Settings2 },
    { key: "mcp", label: "MCP 配置", icon: Server },
    { key: "permissions", label: "权限", icon: Shield },
    { key: "system", label: "系统", icon: HardDrive },
    { key: "about", label: "关于", icon: Info },
];

// ============================================================
// 通用设置面板
// ============================================================

const GeneralSettings: FC = () => {
    const theme = useGlobalStore((s) => s.theme);
    const toggleTheme = useGlobalStore((s) => s.toggleTheme);

    return (
        <div className="space-y-6">
            {/* 主题设置 */}
            <div>
                <h3 className="text-sm font-semibold text-foreground mb-3">
                    主题
                </h3>
                <div className="flex items-center gap-2">
                    <button
                        onClick={() => theme !== "light" && toggleTheme()}
                        className={cn(
                            "flex items-center gap-2 px-4 py-2.5 rounded-xl border text-sm font-medium transition-all",
                            theme === "light"
                                ? "border-primary bg-primary/5 text-primary"
                                : "border-border text-muted-foreground hover:border-primary/30",
                        )}
                    >
                        <Sun className="h-4 w-4" />
                        <span>浅色</span>
                    </button>
                    <button
                        onClick={() => theme !== "dark" && toggleTheme()}
                        className={cn(
                            "flex items-center gap-2 px-4 py-2.5 rounded-xl border text-sm font-medium transition-all",
                            theme === "dark"
                                ? "border-primary bg-primary/5 text-primary"
                                : "border-border text-muted-foreground hover:border-primary/30",
                        )}
                    >
                        <Moon className="h-4 w-4" />
                        <span>深色</span>
                    </button>
                </div>
            </div>

            {/* 语言设置 */}
            <div>
                <h3 className="text-sm font-semibold text-foreground mb-3">
                    语言
                </h3>
                <select
                    className="w-48 h-9 rounded-lg border border-border bg-background px-3 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/30"
                    defaultValue="zh-CN"
                >
                    <option value="zh-CN">简体中文</option>
                    <option value="en-US">English</option>
                </select>
            </div>

            {/* 自动保存 */}
            <div>
                <h3 className="text-sm font-semibold text-foreground mb-3">
                    自动保存
                </h3>
                <div className="flex items-center gap-3">
                    <label className="relative inline-flex items-center cursor-pointer">
                        <input
                            type="checkbox"
                            defaultChecked
                            className="sr-only peer"
                        />
                        <div className="w-9 h-5 rounded-full bg-muted peer-checked:bg-primary peer-focus:ring-2 peer-focus:ring-primary/30 transition-colors after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:after:translate-x-full" />
                    </label>
                    <span className="text-sm text-muted-foreground">
                        每 30 秒自动保存工作流
                    </span>
                </div>
            </div>
        </div>
    );
};

// ============================================================
// 连接状态指示器
// ============================================================

const STATUS_CONFIG: Record<
    ConnectionStatus,
    { icon: typeof Wifi; color: string; label: string }
> = {
    connected: { icon: Wifi, color: "text-green-500", label: "已连接" },
    disconnected: { icon: WifiOff, color: "text-muted-foreground/40", label: "未连接" },
    connecting: { icon: Loader2, color: "text-yellow-500", label: "连接中..." },
    error: { icon: AlertCircle, color: "text-red-500", label: "连接失败" },
};

// ============================================================
// MCP 配置面板
// ============================================================

const MCPSettings: FC = () => {
    const {
        servers,
        loading,
        error,
        connectionStatus,
        fetchServers,
        removeServer,
        addServer,
        pingServer,
        pingAllServers,
        clearError,
    } = useMCPStore();

    const [pingingAll, setPingingAll] = useState(false);
    const [showAddDialog, setShowAddDialog] = useState(false);
    const [formData, setFormData] = useState({
        name: "",
        description: "",
        protocol: MCPProtocol.Stdio,
        endpoint: "",
        command: "",
        args: "",
        env: "",
    });
    const [submitting, setSubmitting] = useState(false);

    useEffect(() => {
        fetchServers();
    }, [fetchServers]);

    const handlePingAll = useCallback(async () => {
        setPingingAll(true);
        await pingAllServers();
        setPingingAll(false);
    }, [pingAllServers]);

    const handleAddServer = useCallback(async () => {
        if (!formData.name.trim()) return;
        setSubmitting(true);
        try {
            const parsedArgs: string[] = formData.args
                ? formData.args.split(",").map((s) => s.trim()).filter(Boolean)
                : [];
            const parsedEnv: Record<string, string> = {};
            if (formData.env) {
                for (const line of formData.env.split("\n")) {
                    const eqIdx = line.indexOf("=");
                    if (eqIdx > 0) {
                        parsedEnv[line.slice(0, eqIdx).trim()] = line.slice(eqIdx + 1).trim();
                    }
                }
            }
            await addServer({
                name: formData.name.trim(),
                description: formData.description.trim(),
                protocol: formData.protocol,
                endpoint: formData.endpoint.trim(),
                command: formData.command.trim(),
                args: parsedArgs,
                env: parsedEnv,
                enabled: true,
            });
            setShowAddDialog(false);
            setFormData({
                name: "",
                description: "",
                protocol: MCPProtocol.Stdio,
                endpoint: "",
                command: "",
                args: "",
                env: "",
            });
        } catch (e) {
            console.error("添加服务器失败:", e);
            toast.error(e instanceof Error ? e.message : "添加服务器失败");
        } finally {
            setSubmitting(false);
        }
    }, [formData, addServer]);

    return (
        <>
        <div className="space-y-6">
            {/* 头部 */}
            <div className="flex items-center justify-between">
                <div>
                    <h3 className="text-sm font-semibold text-foreground mb-1">
                        MCP 服务器
                    </h3>
                    <p className="text-sm text-muted-foreground/70">
                        管理已连接的 MCP 服务器
                    </p>
                </div>
                <div className="flex items-center gap-2">
                    <button
                        onClick={handlePingAll}
                        disabled={pingingAll || servers.length === 0}
                        className="inline-flex items-center gap-1.5 rounded-lg border border-border px-3 py-1.5 text-xs font-medium text-muted-foreground hover:bg-accent transition-colors disabled:opacity-50"
                    >
                        {pingingAll ? (
                            <Loader2 className="h-3.5 w-3.5 animate-spin" />
                        ) : (
                            <RefreshCw className="h-3.5 w-3.5" />
                        )}
                        <span>检测全部</span>
                    </button>
                    <button
                        onClick={() => setShowAddDialog(true)}
                        className="inline-flex items-center gap-1.5 rounded-lg bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 transition-colors">
                        <Plus className="h-3.5 w-3.5" />
                        <span>添加服务器</span>
                    </button>
                </div>
            </div>

            {/* 错误提示 */}
            {error && (
                <div className="flex items-center gap-2 rounded-lg bg-red-500/10 border border-red-500/20 px-3 py-2 text-sm text-red-600">
                    <AlertCircle className="h-4 w-4 shrink-0" />
                    <span className="flex-1">{error}</span>
                    <button
                        onClick={clearError}
                        className="text-xs underline hover:no-underline"
                    >
                        关闭
                    </button>
                </div>
            )}

            {/* 加载状态 */}
            {loading && servers.length === 0 && (
                <div className="flex items-center justify-center py-8">
                    <Loader2 className="h-5 w-5 animate-spin text-muted-foreground/40" />
                </div>
            )}

            {/* 空状态 */}
            {!loading && servers.length === 0 && (
                <div className="rounded-xl border border-border bg-muted/30 p-8 text-center">
                    <div className="h-12 w-12 mx-auto mb-3 rounded-full bg-muted flex items-center justify-center">
                        <Plug className="h-5 w-5 text-muted-foreground/40" />
                    </div>
                    <p className="text-sm text-muted-foreground/60">
                        暂无已连接的 MCP 服务器
                    </p>
                    <p className="text-xs text-muted-foreground/40 mt-1">
                        点击"添加服务器"按钮创建第一个 MCP 连接
                    </p>
                </div>
            )}

            {/* 服务器列表 */}
            {servers.length > 0 && (
                <div className="space-y-2">
                    {servers.map((server) => {
                        const status = connectionStatus[server.id] ?? "disconnected";
                        const StatusIcon = STATUS_CONFIG[status].icon;
                        const isConnecting = status === "connecting";

                        return (
                            <div
                                key={server.id}
                                className="flex items-center gap-4 rounded-xl border border-border bg-background/60 px-4 py-3 transition-colors hover:border-primary/20"
                            >
                                {/* 状态指示器 */}
                                <button
                                    onClick={() => pingServer(server.id)}
                                    disabled={isConnecting}
                                    className="shrink-0"
                                    title="点击检测连接"
                                >
                                    <StatusIcon
                                        className={cn(
                                            "h-4 w-4",
                                            STATUS_CONFIG[status].color,
                                            isConnecting && "animate-spin",
                                        )}
                                    />
                                </button>

                                {/* 服务器信息 */}
                                <div className="flex-1 min-w-0">
                                    <div className="flex items-center gap-2">
                                        <p className="text-sm font-medium text-foreground truncate">
                                            {server.name}
                                        </p>
                                        <span className="shrink-0 rounded-md bg-muted px-1.5 py-0.5 text-[10px] font-medium text-muted-foreground uppercase">
                                            {server.protocol}
                                        </span>
                                        {!server.enabled && (
                                            <span className="shrink-0 rounded-md bg-yellow-500/10 px-1.5 py-0.5 text-[10px] font-medium text-yellow-600">
                                                已禁用
                                            </span>
                                        )}
                                    </div>
                                    <p className="text-xs text-muted-foreground/60 truncate mt-0.5">
                                        {server.description || server.endpoint || server.command}
                                    </p>
                                </div>

                                {/* 连接状态文字 */}
                                <span
                                    className={cn(
                                        "shrink-0 text-xs",
                                        STATUS_CONFIG[status].color,
                                    )}
                                >
                                    {STATUS_CONFIG[status].label}
                                </span>

                                {/* 删除按钮 */}
                                <button
                                    onClick={() => removeServer(server.id)}
                                    className="shrink-0 rounded-lg p-1.5 text-muted-foreground/40 hover:text-red-500 hover:bg-red-500/10 transition-colors"
                                    title="删除服务器"
                                >
                                    <Trash2 className="h-4 w-4" />
                                </button>
                            </div>
                        );
                    })}
                </div>)}
            </div>

        {/* 添加服务器对话框 */}
        {showAddDialog && (
            <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
                <motion.div
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    className="w-full max-w-md rounded-2xl border border-border bg-background p-6 shadow-2xl"
                >
                    <div className="flex items-center justify-between mb-4">
                        <h3 className="text-base font-semibold text-foreground">
                            添加 MCP 服务器
                        </h3>
                        <button
                            onClick={() => setShowAddDialog(false)}
                            className="rounded-lg p-1 text-muted-foreground hover:bg-accent transition-colors"
                        >
                            <X className="h-4 w-4" />
                        </button>
                    </div>

                    <div className="space-y-3">
                        <div>
                            <label className="block text-xs font-medium text-muted-foreground mb-1">
                                名称 *
                            </label>
                            <input
                                type="text"
                                value={formData.name}
                                onChange={(e) =>
                                    setFormData((f) => ({ ...f, name: e.target.value }))
                                }
                                placeholder="例如: My Server"
                                className="w-full h-9 rounded-lg border border-border bg-background px-3 text-sm text-foreground placeholder:text-muted-foreground/40 focus:outline-none focus:ring-2 focus:ring-primary/30"
                            />
                        </div>

                        <div>
                            <label className="block text-xs font-medium text-muted-foreground mb-1">
                                描述
                            </label>
                            <input
                                type="text"
                                value={formData.description}
                                onChange={(e) =>
                                    setFormData((f) => ({ ...f, description: e.target.value }))
                                }
                                placeholder="可选描述"
                                className="w-full h-9 rounded-lg border border-border bg-background px-3 text-sm text-foreground placeholder:text-muted-foreground/40 focus:outline-none focus:ring-2 focus:ring-primary/30"
                            />
                        </div>

                        <div>
                            <label className="block text-xs font-medium text-muted-foreground mb-1">
                                协议
                            </label>
                            <select
                                value={formData.protocol}
                                onChange={(e) =>
                                    setFormData((f) => ({
                                        ...f,
                                        protocol: e.target.value as MCPProtocol,
                                    }))
                                }
                                className="w-full h-9 rounded-lg border border-border bg-background px-3 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-primary/30"
                            >
                                <option value="stdio">stdio (本地进程)</option>
                                <option value="sse">SSE</option>
                                <option value="streamable-http">Streamable HTTP</option>
                            </select>
                        </div>

                        {(formData.protocol === "sse" ||
                            formData.protocol === "streamable-http") && (
                            <div>
                                <label className="block text-xs font-medium text-muted-foreground mb-1">
                                    Endpoint *
                                </label>
                                <input
                                    type="text"
                                    value={formData.endpoint}
                                    onChange={(e) =>
                                        setFormData((f) => ({
                                            ...f,
                                            endpoint: e.target.value,
                                        }))
                                    }
                                    placeholder="例如: http://localhost:8080"
                                    className="w-full h-9 rounded-lg border border-border bg-background px-3 text-sm text-foreground placeholder:text-muted-foreground/40 focus:outline-none focus:ring-2 focus:ring-primary/30"
                                />
                            </div>
                        )}

                        {formData.protocol === "stdio" && (
                            <div>
                                <label className="block text-xs font-medium text-muted-foreground mb-1">
                                    命令 *
                                </label>
                                <input
                                    type="text"
                                    value={formData.command}
                                    onChange={(e) =>
                                        setFormData((f) => ({
                                            ...f,
                                            command: e.target.value,
                                        }))
                                    }
                                    placeholder="例如: npx -y @modelcontextprotocol/server-filesystem"
                                    className="w-full h-9 rounded-lg border border-border bg-background px-3 text-sm text-foreground placeholder:text-muted-foreground/40 focus:outline-none focus:ring-2 focus:ring-primary/30"
                                />
                            </div>
                        )}

                        {formData.protocol === "stdio" && (
                            <div>
                                <label className="block text-xs font-medium text-muted-foreground mb-1">
                                    参数 (逗号分隔)
                                </label>
                                <input
                                    type="text"
                                    value={formData.args}
                                    onChange={(e) =>
                                        setFormData((f) => ({ ...f, args: e.target.value }))
                                    }
                                    placeholder="例如: arg1, arg2"
                                    className="w-full h-9 rounded-lg border border-border bg-background px-3 text-sm text-foreground placeholder:text-muted-foreground/40 focus:outline-none focus:ring-2 focus:ring-primary/30"
                                />
                            </div>
                        )}

                        <div>
                            <label className="block text-xs font-medium text-muted-foreground mb-1">
                                环境变量 (每行 KEY=VALUE)
                            </label>
                            <textarea
                                value={formData.env}
                                onChange={(e) =>
                                    setFormData((f) => ({ ...f, env: e.target.value }))
                                }
                                placeholder="KEY1=value1&#10;KEY2=value2"
                                rows={3}
                                className="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm text-foreground placeholder:text-muted-foreground/40 focus:outline-none focus:ring-2 focus:ring-primary/30 resize-none"
                            />
                        </div>
                    </div>

                    <div className="flex items-center justify-end gap-2 mt-5">
                        <button
                            onClick={() => setShowAddDialog(false)}
                            className="rounded-lg border border-border px-4 py-2 text-sm text-muted-foreground hover:bg-accent transition-colors"
                        >
                            取消
                        </button>
                        <button
                            onClick={handleAddServer}
                            disabled={!formData.name.trim() || submitting}
                            className="inline-flex items-center gap-1.5 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50 transition-colors"
                        >
                            {submitting && (
                                <Loader2 className="h-3.5 w-3.5 animate-spin" />
                            )}
                            <span>添加</span>
                        </button>
                    </div>
                </motion.div>
            </div>
        )}
        </>
    );
};

// ============================================================
// 权限设置面板
// ============================================================

const AUTH_PERMISSION_LABELS: Record<string, string> = {
    servers: "查看 MCP 服务器",
    servers_manage: "管理 MCP 服务器",
    workflows: "查看工作流",
    workflows_manage: "管理工作流",
    workflows_execute: "执行工作流",
    executions: "查看执行记录",
    backup: "数据库备份",
    restore: "数据库恢复",
};

const PermissionsSettings: FC = () => {
    const [role, setRole] = useState<string>("admin");
    const [permissions, setPermissions] = useState<Record<string, boolean>>({});
    const [loading, setLoading] = useState(true);
    const [apiKey, setApiKey] = useState<string | null>(null);
    const [showKey, setShowKey] = useState(false);
    const [newKeyInput, setNewKeyInput] = useState("");
    const [authConfigured, setAuthConfigured] = useState(false);
    const [submitting, setSubmitting] = useState(false);

    const loadAuth = useCallback(async () => {
        setLoading(true);
        try {
            const [roleRes, statusRes] = await Promise.all([
                authService.getRole(),
                authService.status(),
            ]);
            if (roleRes.success && roleRes.data) {
                setRole(roleRes.data.role);
                setPermissions(roleRes.data.permissions);
            }
            if (statusRes.success && statusRes.data) {
                setAuthConfigured(statusRes.data.configured);
            }
        } catch (e) {
            console.error("加载认证信息失败:", e);
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        loadAuth();
    }, [loadAuth]);

    const handleGenerateKey = async () => {
        setSubmitting(true);
        try {
            const res = await authService.generateKey();
            if (res.success && res.data) {
                setApiKey(res.data);
                setAuthConfigured(true);
                toast.success("API Key 已生成");
            } else {
                toast.error(res.error ?? "生成失败");
            }
        } catch (e) {
            toast.error(e instanceof Error ? e.message : "生成失败");
        } finally {
            setSubmitting(false);
        }
    };

    const handleSetKey = async () => {
        if (!newKeyInput.trim()) {
            toast.error("请输入 API Key");
            return;
        }
        setSubmitting(true);
        try {
            const res = await authService.setKey(newKeyInput.trim());
            if (res.success) {
                setAuthConfigured(true);
                setNewKeyInput("");
                toast.success("API Key 已设置");
            } else {
                toast.error(res.error ?? "设置失败");
            }
        } catch (e) {
            toast.error(e instanceof Error ? e.message : "设置失败");
        } finally {
            setSubmitting(false);
        }
    };

    const handleClearKey = async () => {
        setSubmitting(true);
        try {
            const res = await authService.clearKey();
            if (res.success) {
                setApiKey(null);
                setAuthConfigured(false);
                toast.success("API Key 已清除");
            } else {
                toast.error(res.error ?? "清除失败");
            }
        } catch (e) {
            toast.error(e instanceof Error ? e.message : "清除失败");
        } finally {
            setSubmitting(false);
        }
    };

    const handleRoleChange = async (newRole: string) => {
        setSubmitting(true);
        try {
            const res = await authService.setRole(newRole);
            if (res.success) {
                setRole(newRole);
                loadAuth();
                toast.success(`角色已切换为 ${newRole}`);
            } else {
                toast.error(res.error ?? "切换失败");
            }
        } catch (e) {
            toast.error(e instanceof Error ? e.message : "切换失败");
        } finally {
            setSubmitting(false);
        }
    };

    const handleCopyKey = () => {
        if (apiKey) {
            navigator.clipboard.writeText(apiKey);
            toast.success("已复制到剪贴板");
        }
    };

    if (loading) {
        return (
            <div className="flex items-center justify-center py-8">
                <Loader2 className="h-5 w-5 animate-spin text-muted-foreground/40" />
            </div>
        );
    }

    return (
        <div className="space-y-6">
            {/* API Key 管理 */}
            <div>
                <h3 className="text-sm font-semibold text-foreground mb-2">
                    API Key
                </h3>
                <p className="text-sm text-muted-foreground/70 mb-4">
                    管理 API Key 用于外部程序访问 MCP Fusion
                </p>

                <div className="space-y-3">
                    {apiKey ? (
                        <div className="flex items-center gap-2 rounded-xl border border-border bg-background/60 px-4 py-3">
                            <Key className="h-4 w-4 text-muted-foreground shrink-0" />
                            <input
                                type={showKey ? "text" : "password"}
                                value={apiKey}
                                readOnly
                                className="flex-1 bg-transparent text-sm font-mono text-foreground outline-none"
                            />
                            <button
                                onClick={() => setShowKey(!showKey)}
                                className="shrink-0 rounded-lg p-1 text-muted-foreground/60 hover:text-foreground transition-colors"
                                title={showKey ? "隐藏" : "显示"}
                            >
                                {showKey ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                            </button>
                            <button
                                onClick={handleCopyKey}
                                className="shrink-0 rounded-lg p-1 text-muted-foreground/60 hover:text-foreground transition-colors"
                                title="复制"
                            >
                                <Copy className="h-4 w-4" />
                            </button>
                        </div>
                    ) : null}

                    <div className="flex items-center gap-2">
                        <button
                            onClick={handleGenerateKey}
                            disabled={submitting}
                            className="inline-flex items-center gap-1.5 rounded-lg bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 transition-colors disabled:opacity-50"
                        >
                            {submitting ? <Loader2 className="h-3.5 w-3.5 animate-spin" /> : <Key className="h-3.5 w-3.5" />}
                            <span>生成新 Key</span>
                        </button>
                        {authConfigured ? (
                            <button
                                onClick={handleClearKey}
                                disabled={submitting}
                                className="inline-flex items-center gap-1.5 rounded-lg border border-red-500/20 px-3 py-1.5 text-xs font-medium text-red-500 hover:bg-red-500/10 transition-colors disabled:opacity-50"
                            >
                                <Trash2 className="h-3.5 w-3.5" />
                                <span>清除 Key</span>
                            </button>
                        ) : null}
                    </div>

                    <div className="flex items-center gap-2">
                        <input
                            type="text"
                            value={newKeyInput}
                            onChange={(e) => setNewKeyInput(e.target.value)}
                            placeholder="或粘贴已有 API Key..."
                            className="flex-1 h-9 rounded-lg border border-border bg-background px-3 text-sm text-foreground placeholder:text-muted-foreground/40 focus:outline-none focus:ring-2 focus:ring-primary/30"
                        />
                        <button
                            onClick={handleSetKey}
                            disabled={submitting || !newKeyInput.trim()}
                            className="inline-flex items-center gap-1 rounded-lg border border-border px-3 py-1.5 text-xs font-medium text-muted-foreground hover:bg-accent transition-colors disabled:opacity-50"
                        >
                            <Check className="h-3.5 w-3.5" />
                            <span>设置</span>
                        </button>
                    </div>
                </div>
            </div>

            {/* 角色切换 */}
            <div>
                <h3 className="text-sm font-semibold text-foreground mb-2">
                    角色管理
                </h3>
                <p className="text-sm text-muted-foreground/70 mb-4">
                    当前角色：<span className="font-medium text-foreground">{role}</span>
                </p>

                <div className="flex gap-2">
                    {["admin", "developer", "viewer"].map((r) => (
                        <button
                            key={r}
                            onClick={() => handleRoleChange(r)}
                            disabled={submitting || role === r}
                            className={cn(
                                "rounded-lg px-4 py-2 text-xs font-medium transition-colors",
                                role === r
                                    ? "bg-primary text-primary-foreground"
                                    : "border border-border text-muted-foreground hover:bg-accent",
                            )}
                        >
                            {r === "admin" ? "管理员" : r === "developer" ? "开发者" : "观察者"}
                        </button>
                    ))}
                </div>
            </div>

            {/* 权限清单 */}
            <div>
                <h3 className="text-sm font-semibold text-foreground mb-2">
                    权限清单
                </h3>
                <p className="text-sm text-muted-foreground/70 mb-4">
                    当前角色可执行的操作
                </p>
                <div className="space-y-2">
                    {Object.entries(AUTH_PERMISSION_LABELS).map(([key, label]) => (
                        <div
                            key={key}
                            className="flex items-center justify-between rounded-xl border border-border bg-background/60 px-4 py-3"
                        >
                            <span className="text-sm text-foreground">{label}</span>
                            <span
                                className={cn(
                                    "text-xs font-medium",
                                    permissions[key] ? "text-green-500" : "text-muted-foreground/40",
                                )}
                            >
                                {permissions[key] ? "已授权" : "未授权"}
                            </span>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

// ============================================================
// 系统设置面板
// ============================================================

const SystemSettings: FC = () => {
    const [backingUp, setBackingUp] = useState(false);
    const [restoring, setRestoring] = useState(false);
    const [restorePath, setRestorePath] = useState("");
    const [health, setHealth] = useState<{
        status: string;
        database: boolean;
        runtime: string;
        version: string;
    } | null>(null);
    const [healthLoading, setHealthLoading] = useState(false);

    const checkHealth = useCallback(async () => {
        setHealthLoading(true);
        try {
            const res = await systemService.healthCheck();
            if (res.success && res.data) {
                setHealth(res.data);
            } else {
                toast.error(res.error ?? "健康检查失败");
            }
        } catch (e) {
            toast.error(e instanceof Error ? e.message : "健康检查失败");
        } finally {
            setHealthLoading(false);
        }
    }, []);

    useEffect(() => {
        checkHealth();
    }, [checkHealth]);

    const handleBackup = async () => {
        setBackingUp(true);
        try {
            const res = await systemService.backup();
            if (res.success && res.data) {
                toast.success(`备份成功: ${res.data}`);
            } else {
                toast.error(res.error ?? "备份失败");
            }
        } catch (e) {
            toast.error(e instanceof Error ? e.message : "备份失败");
        } finally {
            setBackingUp(false);
        }
    };

    const handleRestore = async () => {
        if (!restorePath.trim()) {
            toast.error("请输入备份文件路径");
            return;
        }
        setRestoring(true);
        try {
            const res = await systemService.restore(restorePath.trim());
            if (res.success) {
                toast.success("数据库恢复成功");
                setRestorePath("");
            } else {
                toast.error(res.error ?? "恢复失败");
            }
        } catch (e) {
            toast.error(e instanceof Error ? e.message : "恢复失败");
        } finally {
            setRestoring(false);
        }
    };

    return (
        <div className="space-y-6">
            {/* 健康状态 */}
            <div>
                <h3 className="text-sm font-semibold text-foreground mb-2">
                    系统状态
                </h3>
                <div className="rounded-xl border border-border bg-background/60 p-4 space-y-2">
                    {healthLoading ? (
                        <div className="flex items-center justify-center py-2">
                            <Loader2 className="h-4 w-4 animate-spin text-muted-foreground/40" />
                        </div>
                    ) : health ? (
                        <>
                            <div className="flex items-center justify-between">
                                <span className="text-sm text-muted-foreground">状态</span>
                                <span className="text-sm font-medium text-green-500">
                                    {health.status}
                                </span>
                            </div>
                            <div className="flex items-center justify-between">
                                <span className="text-sm text-muted-foreground">数据库</span>
                                <span
                                    className={cn(
                                        "text-sm font-medium",
                                        health.database ? "text-green-500" : "text-red-500",
                                    )}
                                >
                                    {health.database ? "正常" : "异常"}
                                </span>
                            </div>
                            <div className="flex items-center justify-between">
                                <span className="text-sm text-muted-foreground">运行时</span>
                                <span className="text-sm font-medium text-foreground">
                                    {health.runtime === "running" ? "运行中" : "空闲"}
                                </span>
                            </div>
                            <div className="flex items-center justify-between">
                                <span className="text-sm text-muted-foreground">版本</span>
                                <span className="text-sm font-medium text-foreground">
                                    v{health.version}
                                </span>
                            </div>
                        </>
                    ) : (
                        <p className="text-sm text-muted-foreground/60">
                            无法获取系统状态
                        </p>
                    )}
                    <button
                        onClick={checkHealth}
                        disabled={healthLoading}
                        className="inline-flex items-center gap-1.5 rounded-lg border border-border px-2.5 py-1 text-xs text-muted-foreground hover:bg-accent transition-colors"
                    >
                        <RefreshCw
                            className={cn(
                                "h-3 w-3",
                                healthLoading && "animate-spin",
                            )}
                        />
                        <span>刷新</span>
                    </button>
                </div>
            </div>

            {/* 数据库备份 */}
            <div>
                <h3 className="text-sm font-semibold text-foreground mb-2">
                    <Database className="h-4 w-4 inline mr-1.5" />
                    数据库备份
                </h3>
                <p className="text-sm text-muted-foreground/70 mb-4">
                    备份当前数据库到本地文件，用于灾难恢复
                </p>
                <button
                    onClick={handleBackup}
                    disabled={backingUp}
                    className="inline-flex items-center gap-1.5 rounded-lg bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 transition-colors disabled:opacity-50"
                >
                    {backingUp ? (
                        <Loader2 className="h-3.5 w-3.5 animate-spin" />
                    ) : (
                        <Download className="h-3.5 w-3.5" />
                    )}
                    <span>立即备份</span>
                </button>
            </div>

            {/* 数据库恢复 */}
            <div>
                <h3 className="text-sm font-semibold text-foreground mb-2">
                    数据库恢复
                </h3>
                <p className="text-sm text-muted-foreground/70 mb-4">
                    从备份文件恢复数据库（将覆盖当前数据）
                </p>
                <div className="flex items-center gap-2">
                    <input
                        type="text"
                        value={restorePath}
                        onChange={(e) => setRestorePath(e.target.value)}
                        placeholder="输入备份文件路径..."
                        className="flex-1 h-9 rounded-lg border border-border bg-background px-3 text-sm text-foreground placeholder:text-muted-foreground/40 focus:outline-none focus:ring-2 focus:ring-primary/30"
                    />
                    <button
                        onClick={handleRestore}
                        disabled={restoring || !restorePath.trim()}
                        className="inline-flex items-center gap-1.5 rounded-lg border border-red-500/20 px-3 py-1.5 text-xs font-medium text-red-500 hover:bg-red-500/10 transition-colors disabled:opacity-50"
                    >
                        {restoring ? (
                            <Loader2 className="h-3.5 w-3.5 animate-spin" />
                        ) : (
                            <Upload className="h-3.5 w-3.5" />
                        )}
                        <span>恢复</span>
                    </button>
                </div>
            </div>
        </div>
    );
};

// ============================================================
// 关于面板
// ============================================================

const AboutSettings: FC = () => {
    return (
        <div className="space-y-6">
            <div className="flex items-center gap-4 pb-6 border-b border-border">
                <div className="h-16 w-16 rounded-2xl bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center">
                    <span className="text-2xl font-bold text-white">MF</span>
                </div>
                <div>
                    <h2 className="text-lg font-bold text-foreground">
                        MCP Fusion
                    </h2>
                    <p className="text-sm text-muted-foreground">v0.1.0</p>
                    <p className="text-xs text-muted-foreground/60 mt-1">
                        多协议工作流编排平台
                    </p>
                </div>
            </div>

            <div className="space-y-2">
                <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">技术栈</span>
                    <span className="text-foreground">Tauri + React + Rust</span>
                </div>
                <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">React Flow</span>
                    <span className="text-foreground">v12.x</span>
                </div>
                <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Tauri</span>
                    <span className="text-foreground">v2.x</span>
                </div>
                <div className="flex justify-between text-sm">
                    <span className="text-muted-foreground">Rust</span>
                    <span className="text-foreground">Stable</span>
                </div>
            </div>
        </div>
    );
};

// ============================================================
// 设置页面
// ============================================================

const Settings: FC = () => {
    const [activeTab, setActiveTab] = useState("general");

    const renderPanel = () => {
        switch (activeTab) {
            case "general":
                return <GeneralSettings />;
            case "mcp":
                return <MCPSettings />;
            case "permissions":
                return <PermissionsSettings />;
            case "system":
                return <SystemSettings />;
            case "about":
                return <AboutSettings />;
            default:
                return null;
        }
    };

    return (
        <PageTransition className="h-full">
            <div className="flex h-full">
                {/* 左侧标签栏 */}
                <div className="w-48 shrink-0 border-r border-border bg-background/40 px-3 py-6">
                    <h2 className="px-3 mb-4 text-xs font-semibold text-muted-foreground/60 uppercase tracking-wider">
                        设置
                    </h2>
                    <nav className="space-y-1">
                        {TABS.map((tab) => {
                            const Icon = tab.icon;
                            const isActive = activeTab === tab.key;

                            return (
                                <button
                                    key={tab.key}
                                    onClick={() => setActiveTab(tab.key)}
                                    className={cn(
                                        "flex items-center gap-3 w-full rounded-lg px-3 py-2.5 text-sm font-medium transition-all",
                                        isActive
                                            ? "bg-primary/10 text-primary"
                                            : "text-muted-foreground hover:bg-accent hover:text-accent-foreground",
                                    )}
                                >
                                    <Icon
                                        className={cn(
                                            "h-4 w-4",
                                            isActive
                                                ? "text-primary"
                                                : "text-muted-foreground",
                                        )}
                                    />
                                    <span>{tab.label}</span>
                                </button>
                            );
                        })}
                    </nav>
                </div>

                {/* 右侧内容面板 */}
                <div className="flex-1 overflow-y-auto px-8 py-6">
                    <motion.div
                        key={activeTab}
                        initial={{ opacity: 0, x: 8 }}
                        animate={{ opacity: 1, x: 0 }}
                        transition={{ duration: 0.2, ease: "easeOut" }}
                    >
                        {renderPanel()}
                    </motion.div>
                </div>
            </div>
        </PageTransition>
    );
};

export default Settings;