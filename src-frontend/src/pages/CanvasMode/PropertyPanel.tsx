import { type FC, useCallback } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { cn } from "@/lib/utils";
import { RunStatus } from "@shared/types";
import { useCanvasStore } from "./useCanvasStore";

// ============================================================
// 属性面板
// ============================================================

interface PropertyPanelProps {
    className?: string;
}

const PropertyPanel: FC<PropertyPanelProps> = ({ className }) => {
    const selectedNodeId = useCanvasStore((s) => s.selectedNodeId);
    const nodeStates = useCanvasStore((s) => s.nodeStates);
    const panelOpen = useCanvasStore((s) => s.panelOpen);
    const setSelectedNodeId = useCanvasStore((s) => s.setSelectedNodeId);
    const setPanelOpen = useCanvasStore((s) => s.setPanelOpen);
    const updateNodeInput = useCanvasStore((s) => s.updateNodeInput);

    const selectedNode = selectedNodeId ? nodeStates[selectedNodeId] : null;
    const schema = selectedNode?.tool.inputSchema as Record<string, unknown> | undefined;
    const properties = (schema?.properties as Record<string, { type: string; description: string; default?: string; enum?: string[] }>) ?? {};

    const handleClose = useCallback(() => {
        setSelectedNodeId(null);
        setPanelOpen(false);
    }, [setSelectedNodeId, setPanelOpen]);

    const handleInputChange = useCallback(
        (key: string, value: string) => {
            if (selectedNodeId) {
                updateNodeInput(selectedNodeId, key, value);
            }
        },
        [selectedNodeId, updateNodeInput],
    );

    return (
        <AnimatePresence>
            {panelOpen && (
                <motion.div
                    initial={{ width: 0, opacity: 0 }}
                    animate={{ width: 300, opacity: 1 }}
                    exit={{ width: 0, opacity: 0 }}
                    transition={{ duration: 0.25, ease: "easeInOut" }}
                    className={cn(
                        "h-full border-l border-border bg-background overflow-hidden shrink-0",
                        className,
                    )}
                >
                    <div className="flex flex-col h-full w-[300px]">
                        {/* 面板头部 */}
                        <div className="flex items-center justify-between px-4 py-3 border-b border-border shrink-0">
                            <h2 className="text-sm font-semibold text-foreground">
                                属性
                            </h2>
                            <button
                                onClick={handleClose}
                                className="rounded-md p-1 text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
                                aria-label="关闭属性面板"
                            >
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                                    <path d="M18 6 6 18" />
                                    <path d="m6 6 12 12" />
                                </svg>
                            </button>
                        </div>

                        {/* 面板内容 */}
                        {selectedNode ? (
                            <div className="flex-1 overflow-y-auto p-4 space-y-4">
                                {/* 工具信息 */}
                                <div>
                                    <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">
                                        工具信息
                                    </h3>
                                    <div className="rounded-lg border border-border bg-card p-3 space-y-2">
                                        <div>
                                            <span className="text-[10px] text-muted-foreground">名称</span>
                                            <p className="text-sm font-medium text-foreground">
                                                {selectedNode.tool.name}
                                            </p>
                                        </div>
                                        <div>
                                            <span className="text-[10px] text-muted-foreground">描述</span>
                                            <p className="text-xs text-foreground/80">
                                                {selectedNode.tool.description}
                                            </p>
                                        </div>
                                        <div>
                                            <span className="text-[10px] text-muted-foreground">状态</span>
                                            <div className="flex items-center gap-1.5 mt-0.5">
                                                <span
                                                    className={cn(
                                                        "inline-block h-2 w-2 rounded-full",
                                                        selectedNode.status === RunStatus.Running && "bg-blue-500 animate-pulse",
                                                        selectedNode.status === RunStatus.Success && "bg-emerald-500",
                                                        selectedNode.status === RunStatus.Failed && "bg-red-500",
                                                        selectedNode.status === RunStatus.Idle && "bg-muted-foreground/40",
                                                        selectedNode.status === RunStatus.Timeout && "bg-amber-500",
                                                    )}
                                                />
                                                <span className="text-xs text-foreground/80">
                                                    {selectedNode.status}
                                                </span>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                {/* 输入参数 */}
                                {Object.keys(properties).length > 0 && (
                                    <div>
                                        <h3 className="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">
                                            输入参数
                                        </h3>
                                        <div className="space-y-3">
                                            {Object.entries(properties).map(([key, prop]) => (
                                                <div key={key} className="space-y-1">
                                                    <label className="text-[11px] font-medium text-foreground">
                                                        {key}
                                                    </label>
                                                    {prop.enum ? (
                                                        <select
                                                            value={selectedNode.inputs[key] ?? prop.default ?? ""}
                                                            onChange={(e) =>
                                                                handleInputChange(key, e.target.value)
                                                            }
                                                            className="w-full rounded-md border border-border bg-background px-2.5 py-1.5 text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
                                                        >
                                                            <option value="" disabled>
                                                                请选择
                                                            </option>
                                                            {prop.enum.map((v) => (
                                                                <option key={v} value={v}>
                                                                    {v}
                                                                </option>
                                                            ))}
                                                        </select>
                                                    ) : (
                                                        <input
                                                            type="text"
                                                            value={selectedNode.inputs[key] ?? prop.default ?? ""}
                                                            onChange={(e) =>
                                                                handleInputChange(key, e.target.value)
                                                            }
                                                            placeholder={prop.description}
                                                            className="w-full rounded-md border border-border bg-background px-2.5 py-1.5 text-xs text-foreground placeholder:text-muted-foreground/50 focus:outline-none focus:ring-1 focus:ring-primary"
                                                        />
                                                    )}
                                                    <p className="text-[10px] text-muted-foreground">
                                                        {prop.description}
                                                    </p>
                                                </div>
                                            ))}
                                        </div>
                                    </div>
                                )}
                            </div>
                        ) : (
                            <div className="flex-1 flex items-center justify-center">
                                <p className="text-xs text-muted-foreground">
                                    点击画布中的节点查看详情
                                </p>
                            </div>
                        )}
                    </div>
                </motion.div>
            )}
        </AnimatePresence>
    );
};

export default PropertyPanel;