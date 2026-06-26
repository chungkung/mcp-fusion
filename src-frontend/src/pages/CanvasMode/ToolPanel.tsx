import { type FC, useCallback } from "react";
import { cn } from "@/lib/utils";
import { useCanvasStore, getToolCategories } from "./useCanvasStore";

const ToolIcons: Record<string, string> = {
    http_request: "\u{1F310}",
    webhook_send: "\u{1F4E1}",
    csv_parser: "\u{1F4CA}",
    json_transform: "\u{1F504}",
    text_ai: "\u{1F916}",
    file_writer: "\u{1F4BE}",
    file_reader: "\u{1F4C2}",
    db_query: "\u{1F5C4}\u{FE0F}",
};

interface ToolPanelProps {
    className?: string;
}

const ToolPanel: FC<ToolPanelProps> = ({ className }) => {
    const tools = useCanvasStore((s) => s.tools);
    const toolsLoading = useCanvasStore((s) => s.toolsLoading);
    const categories = getToolCategories();

    const onDragStart = useCallback(
        (event: React.DragEvent<HTMLDivElement>, toolName: string) => {
            event.dataTransfer.setData("application/reactflow-tool", toolName);
            event.dataTransfer.effectAllowed = "move";
        },
        [],
    );

    return (
        <div className={cn("flex flex-col h-full", className)}>
            <div className="px-4 py-3 border-b border-border shrink-0">
                <h2 className="text-sm font-semibold text-foreground">MCP 工具</h2>
                <p className="text-[11px] text-muted-foreground mt-0.5">
                    拖拽工具到画布添加节点
                </p>
            </div>
            <div className="flex-1 overflow-y-auto p-3 space-y-4">
                {toolsLoading && (
                    <p className="text-xs text-muted-foreground text-center py-4">
                        加载工具列表中...
                    </p>
                )}
                {!toolsLoading && tools.length === 0 && (
                    <p className="text-xs text-muted-foreground text-center py-4">
                        暂无可用工具，请先添加 MCP 服务器
                    </p>
                )}
                {categories.map((category) => (
                    <div key={category.label}>
                        <h3 className="text-[11px] font-semibold text-muted-foreground uppercase tracking-wider mb-2 px-1">
                            {category.label}
                        </h3>
                        <div className="space-y-1">
                            {category.tools.map((tool) => (
                                <div
                                    key={tool.name}
                                    draggable
                                    onDragStart={(e) => onDragStart(e, tool.name)}
                                    className="flex items-center gap-2.5 rounded-lg border border-border bg-card hover:bg-accent/50 hover:border-primary/30 hover:scale-[1.02] active:scale-[0.98] cursor-grab active:cursor-grabbing px-3 py-2.5 transition-all duration-150"
                                >
                                    <span className="text-base shrink-0">
                                        {ToolIcons[tool.name] ?? "\u{1F527}"}
                                    </span>
                                    <div className="min-w-0">
                                        <p className="text-xs font-medium text-foreground truncate">
                                            {tool.name}
                                        </p>
                                        <p className="text-[10px] text-muted-foreground truncate leading-tight">
                                            {tool.description}
                                        </p>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default ToolPanel;