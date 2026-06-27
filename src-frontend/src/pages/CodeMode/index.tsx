import { type FC, useState, useCallback, useRef } from "react";
import { motion } from "framer-motion";
import PageTransition from "@/components/animations/PageTransition";
import { cn } from "@/lib/utils";
import { Play, Copy, Download, Code, Eye, Check, Terminal, Loader2 } from "lucide-react";
import { workflowService, listenIPC } from "@/services/ipc";
import { WorkflowMode, RunStatus, type Workflow, type WorkflowNode, type WorkflowEdge } from "@shared/types";
import toast from "react-hot-toast";

// ============================================================
// 轻量级 YAML 解析器（无外部依赖）
// ============================================================

interface YamlLine {
    indent: number;
    isListItem: boolean;
    key: string | null;
    value: string;
}

function parseYamlLines(text: string): YamlLine[] {
    const lines = text.split("\n");
    const result: YamlLine[] = [];

    for (const rawLine of lines) {
        const line = rawLine.replace(/\r$/, "");
        // 跳过空行和纯注释行
        const trimmed = line.trimStart();
        if (trimmed === "" || trimmed.startsWith("#")) continue;

        const indent = line.length - trimmed.length;
        const isListItem = trimmed.startsWith("- ");

        let content = isListItem ? trimmed.slice(2).trimStart() : trimmed;
        const colonIdx = content.indexOf(":");
        const isMapping = colonIdx !== -1;

        if (isListItem && !isMapping) {
            // 纯列表项值，如 "- value"
            result.push({ indent, isListItem: true, key: null, value: content });
        } else if (isMapping) {
            const key = content.slice(0, colonIdx).trim();
            let value = content.slice(colonIdx + 1).trim();
            // 处理引号字符串
            if ((value.startsWith('"') && value.endsWith('"')) ||
                (value.startsWith("'") && value.endsWith("'"))) {
                value = value.slice(1, -1);
            }
            result.push({ indent, isListItem, key, value });
        } else {
            result.push({ indent, isListItem: false, key: null, value: content });
        }
    }
    return result;
}

interface YamlNode {
    [key: string]: YamlValue;
}

type YamlValue = string | YamlNode | YamlNode[];

function buildYamlTree(lines: YamlLine[]): YamlNode[] {
    const roots: YamlNode[] = [];
    const stack: { indent: number; node: YamlNode; listKey: string | null }[] = [];
    let currentListNode: YamlNode[] | null = null;
    let currentListKey: string | null = null;
    let currentListIndent: number = -1;

    for (const line of lines) {
        // Pop stack until we find a parent
        while (stack.length > 0 && stack[stack.length - 1].indent >= line.indent) {
            stack.pop();
        }

        // 只有当缩进小于当前列表缩进时（回到父级），才结束列表；
        // 相同缩进的列表项（如第二个 "- id: node-2"）仍属于同一列表。
        if (currentListNode !== null && line.indent < currentListIndent) {
            if (stack.length > 0 && currentListKey !== null) {
                stack[stack.length - 1].node[currentListKey] = currentListNode;
            }
            currentListNode = null;
            currentListKey = null;
            currentListIndent = -1;
        }

        if (line.isListItem) {
            if (line.key !== null) {
                // List item with key: value (mapping in list)
                const item: YamlNode = {};
                const val = parseScalarValue(line.value);
                item[line.key] = val;
                if (currentListNode === null) {
                    currentListNode = [];
                    currentListKey = stack.length > 0 ? stack[stack.length - 1].listKey : null;
                    currentListIndent = line.indent;
                }
                currentListNode.push(item);
                stack.push({ indent: line.indent, node: item, listKey: line.key });
            } else {
                // Pure list item value
                const val = parseScalarValue(line.value);
                if (currentListNode === null) {
                    currentListNode = [];
                    currentListKey = stack.length > 0 ? stack[stack.length - 1].listKey : null;
                    currentListIndent = line.indent;
                }
                currentListNode.push(val as unknown as YamlNode);
            }
        } else if (line.key !== null) {
            const node: YamlNode = {};
            const val = parseScalarValue(line.value);

            if (val === "") {
                // Empty value means this is a parent node for children
                if (stack.length === 0) {
                    roots.push(node);
                } else {
                    const parent = stack[stack.length - 1].node;
                    parent[line.key] = node;
                }
                stack.push({ indent: line.indent, node, listKey: line.key });
            } else {
                node[line.key] = val;
                if (stack.length === 0) {
                    roots.push(node);
                } else {
                    const parent = stack[stack.length - 1].node;
                    parent[line.key] = val;
                }
            }
        }
    }

    // Finalize remaining list
    if (currentListNode !== null && currentListKey !== null && stack.length > 0) {
        stack[stack.length - 1].node[currentListKey] = currentListNode;
    }

    return roots;
}

function parseScalarValue(raw: string): YamlValue {
    if (raw === "" || raw === "null" || raw === "~") return "";
    if (raw === "true") return "true";
    if (raw === "false") return "false";
    // 尝试解析为数字
    if (/^-?\d+(\.\d+)?$/.test(raw)) return raw;
    return raw;
}

function yamlToJson(text: string): Record<string, unknown> {
    const lines = parseYamlLines(text);
    const roots = buildYamlTree(lines);
    if (roots.length === 0) throw new Error("YAML 解析失败：未找到有效内容");
    return deepMerge(roots) as Record<string, unknown>;
}

function deepMerge(nodes: YamlNode[]): YamlValue {
    const merged: YamlNode = {};
    for (const node of nodes) {
        for (const [key, val] of Object.entries(node)) {
            if (key in merged && typeof merged[key] === "object" && typeof val === "object" &&
                !Array.isArray(merged[key]) && !Array.isArray(val)) {
                merged[key] = deepMerge([merged[key] as YamlNode, val as YamlNode]);
            } else {
                merged[key] = val;
            }
        }
    }
    // If there's only one top-level key, return the whole thing
    return merged;
}

// ============================================================
// 代码 → Workflow 转换
// ============================================================

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function parseCodeToWorkflow(code: string, lang: string): Workflow {
    const now = Date.now();

    if (lang === "json") {
        const parsed = JSON.parse(code) as Record<string, unknown>;
        return normalizeWorkflow(parsed, now);
    }

    if (lang === "yaml") {
        const parsed = yamlToJson(code);
        return normalizeWorkflow(parsed, now);
    }

    throw new Error(
        `"${lang}" 语言暂不支持直接执行，请切换到 JSON 或 YAML 标签页编写工作流定义`,
    );
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function normalizeWorkflow(raw: Record<string, any>, now: number): Workflow {
    const id = raw.id ?? `wf-code-${now}`;
    const name = raw.name ?? "代码工作流";
    const description = raw.description ?? "";
    const mode = raw.mode ?? WorkflowMode.Code;

    const rawNodes: unknown[] = raw.nodes ?? [];
    const nodes: WorkflowNode[] = rawNodes.map((n: any, i: number) => ({
        id: n.id ?? `node-${i}`,
        type: n.type ?? "mcpTool",
        position: n.position ?? { x: 100 + i * 300, y: 100 },
        data: {
            label: n.data?.label ?? (n.label ?? `节点 ${i + 1}`),
            tool: n.data?.tool ?? null,
            inputs: n.data?.inputs ?? {},
            outputs: n.data?.outputs ?? {},
            config: n.data?.config ?? { toolName: n.toolName ?? "", serverId: n.serverId ?? "" },
        },
    }));

    const rawEdges: unknown[] = raw.edges ?? [];
    const edges: WorkflowEdge[] = rawEdges.map((e: any, i: number) => ({
        id: e.id ?? `edge-${i}`,
        source: e.source,
        target: e.target,
        sourceHandle: e.sourceHandle ?? undefined,
        targetHandle: e.targetHandle ?? undefined,
        type: e.type ?? undefined,
        animated: e.animated ?? undefined,
    }));

    return {
        id,
        name,
        description,
        mode,
        status: RunStatus.Idle,
        nodes,
        edges,
        createdAt: now,
        updatedAt: now,
    };
}

const languages = [
    { id: "typescript", label: "TypeScript", ext: ".ts", available: false },
    { id: "javascript", label: "JavaScript", ext: ".js", available: false },
    { id: "python", label: "Python", ext: ".py", available: false },
    { id: "json", label: "JSON", ext: ".json", available: true },
    { id: "yaml", label: "YAML", ext: ".yaml", available: true },
] as const;

const PLACEHOLDER_CODE: Record<string, string> = {
    typescript: `// TypeScript 工作流定义
// 在此编写工作流编排代码

import { defineWorkflow, step } from "@mcp-fusion/sdk";

export default defineWorkflow({
    name: "my-workflow",
    description: "示例工作流",
    steps: [
        step("http_request", {
            url: "https://api.example.com/data",
            method: "GET",
        }),
        step("json_transform", {
            input: "\u0024{{ steps.http_request.output }}",
        }),
        step("file_writer", {
            path: "output.json",
            content: "\u0024{{ steps.json_transform.output }}",
        }),
    ],
});`,
    javascript: `// JavaScript 工作流定义
// 在此编写工作流编排代码

const { defineWorkflow, step } = require("@mcp-fusion/sdk");

module.exports = defineWorkflow({
    name: "my-workflow",
    description: "示例工作流",
    steps: [
        step("http_request", {
            url: "https://api.example.com/data",
            method: "GET",
        }),
        step("json_transform", {
            input: "\u0024{{ steps.http_request.output }}",
        }),
        step("file_writer", {
            path: "output.json",
            content: "\u0024{{ steps.json_transform.output }}",
        }),
    ],
});`,
    python: `# Python 工作流定义
# 在此编写工作流编排代码

from mcp_fusion import Workflow, step

workflow = Workflow(
    name="my-workflow",
    description="示例工作流",
    steps=[
        step("http_request", url="https://api.example.com/data", method="GET"),
        step("json_transform", input="\u0024{{ steps.http_request.output }}"),
        step("file_writer", path="output.json", content="\u0024{{ steps.json_transform.output }}"),
    ],
)`,
    json: `{
    "name": "my-workflow",
    "description": "示例工作流",
    "mode": "canvas",
    "nodes": [
        {
            "id": "node-1",
            "type": "mcpTool",
            "data": {
                "label": "HTTP 请求",
                "config": {
                    "toolName": "http_request",
                    "serverId": "mcp-http"
                },
                "inputs": {
                    "url": "https://api.example.com/data",
                    "method": "GET"
                }
            }
        },
        {
            "id": "node-2",
            "type": "mcpTool",
            "data": {
                "label": "保存文件",
                "config": {
                    "toolName": "file_writer",
                    "serverId": "mcp-fs"
                },
                "inputs": {
                    "path": "output.json",
                    "content": "\${node-1}"
                }
            }
        }
    ],
    "edges": [
        { "id": "edge-1", "source": "node-1", "target": "node-2" }
    ]
}`,
    yaml: `# YAML 工作流定义
name: my-workflow
description: 示例工作流
mode: canvas
nodes:
  - id: node-1
    type: mcpTool
    data:
      label: HTTP 请求
      config:
        toolName: http_request
        serverId: mcp-http
      inputs:
        url: https://api.example.com/data
        method: GET
  - id: node-2
    type: mcpTool
    data:
      label: 保存文件
      config:
        toolName: file_writer
        serverId: mcp-fs
      inputs:
        path: output.json
        content: "\${node-1}"
edges:
  - id: edge-1
    source: node-1
    target: node-2`,
};

const CodeMode: FC = () => {
    const [activeLang, setActiveLang] = useState<string>("typescript");
    const [viewMode, setViewMode] = useState<"code" | "preview">("code");
    const [codeContent, setCodeContent] = useState<Record<string, string>>(
        () => {
            const initial: Record<string, string> = {};
            for (const lang of languages) {
                initial[lang.id] = PLACEHOLDER_CODE[lang.id] ?? "";
            }
            return initial;
        },
    );
    const [copied, setCopied] = useState(false);
    const [running, setRunning] = useState(false);
    const [runOutput, setRunOutput] = useState<string | null>(null);
    const editorRef = useRef<HTMLTextAreaElement>(null);

    const currentLang = languages.find((l) => l.id === activeLang) ?? languages[0];
    const currentCode = codeContent[activeLang] ?? "";

    const lineCount = currentCode.split("\n").length;

    const handleRun = useCallback(async () => {
        setRunning(true);
        setRunOutput(null);
        setViewMode("preview");

        const startTime = Date.now();
        const outputLines: string[] = [];

        const appendOutput = (msg: string) => {
            outputLines.push(`[${new Date().toLocaleTimeString()}] ${msg}`);
            setRunOutput(outputLines.join("\n"));
        };

        try {
            // 1. 解析代码为 Workflow
            appendOutput(`解析 ${currentLang.label} 代码...`);
            let workflow: Workflow;
            try {
                workflow = parseCodeToWorkflow(currentCode, activeLang);
            } catch (parseErr) {
                appendOutput(`解析失败: ${parseErr instanceof Error ? parseErr.message : String(parseErr)}`);
                return;
            }
            appendOutput(`解析成功: ${workflow.name} (${workflow.nodes.length} 节点, ${workflow.edges.length} 边)`);

            // 2. 保存工作流
            appendOutput("保存工作流...");
            const saveResult = await workflowService.save(workflow);
            if (!saveResult.success) {
                appendOutput(`保存失败: ${saveResult.error ?? "未知错误"}`);
                return;
            }
            appendOutput(`保存成功: ${saveResult.data?.id ?? workflow.id}`);

            // 3. 注册 node-state-change 事件监听（校验 workflow_id 防止串扰）
            const nodeStates = new Map<string, string>();
            let cancelled = false;
            const unlisten = await listenIPC<{
                workflow_id: string;
                node_id: string;
                state: string;
                output?: unknown;
                error?: string;
            }>("node-state-change", (payload) => {
                if (payload.workflow_id !== (saveResult.data?.id ?? workflow.id)) return;
                if (cancelled) return;
                const stateMap: Record<string, string> = {
                    idle: "等待",
                    running: "运行中",
                    success: "成功",
                    failed: "失败",
                    skipped: "跳过",
                    timeout: "超时",
                };
                const stateLabel = stateMap[payload.state] ?? payload.state;
                nodeStates.set(payload.node_id, payload.state);
                appendOutput(`  [${payload.node_id}] ${stateLabel}${payload.error ? `: ${payload.error}` : ""}`);
            });

            // 4. 执行工作流（try/finally 确保事件监听器始终被清理）
            appendOutput("开始执行工作流...");
            let execResult: Awaited<ReturnType<typeof workflowService.execute>>;
            try {
                execResult = await workflowService.execute(saveResult.data?.id ?? workflow.id);
            } catch (e) {
                appendOutput(`执行异常: ${e instanceof Error ? e.message : String(e)}`);
                throw e;
            } finally {
                // 等待事件到达后清理监听器
                await new Promise((r) => setTimeout(r, 500));
                cancelled = true;
                unlisten();
            }

            const elapsed = ((Date.now() - startTime) / 1000).toFixed(1);
            if (execResult.success) {
                const successCount = [...nodeStates.values()].filter((s) => s === "success").length;
                appendOutput("");
                appendOutput(`执行完成 (${elapsed}s)`);
                appendOutput(`  状态: ${execResult.data?.status ?? "unknown"}`);
                appendOutput(`  成功节点: ${successCount}/${nodeStates.size}`);
                toast.success("工作流执行完成");
            } else {
                appendOutput(`执行失败: ${execResult.error ?? "未知错误"}`);
                toast.error(execResult.error ?? "工作流执行失败");
            }
        } catch (err) {
            appendOutput(`执行异常: ${err instanceof Error ? err.message : String(err)}`);
            toast.error("工作流执行异常");
        } finally {
            setRunning(false);
        }
    }, [activeLang, currentCode]);

    const handleCopy = useCallback(async () => {
        try {
            await navigator.clipboard.writeText(currentCode);
            setCopied(true);
            setTimeout(() => setCopied(false), 2000);
        } catch {
            // 降级方案：使用 textarea 选中复制
            if (editorRef.current) {
                editorRef.current.select();
                document.execCommand("copy");
                setCopied(true);
                setTimeout(() => setCopied(false), 2000);
            }
        }
    }, [currentCode]);

    const handleDownload = useCallback(() => {
        const blob = new Blob([currentCode], { type: "text/plain;charset=utf-8" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `workflow${currentLang.ext}`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }, [currentCode, currentLang.ext]);

    const handleCodeChange = useCallback(
        (value: string) => {
            setCodeContent((prev) => ({ ...prev, [activeLang]: value }));
        },
        [activeLang],
    );

    return (
        <PageTransition className="h-full">
            <div className="flex flex-col h-full">
                {/* 顶部操作栏 */}
                <div className="flex items-center justify-between h-12 px-4 border-b border-border bg-background/70 backdrop-blur-xl shrink-0">
                    {/* 左侧：语言选择 */}
                    <div className="flex items-center gap-1">
                        {languages.map((lang) => (
                            <button
                                key={lang.id}
                                onClick={() => setActiveLang(lang.id)}
                                className={cn(
                                    "px-3 py-1 rounded-md text-xs font-medium transition-colors relative",
                                    activeLang === lang.id
                                        ? "bg-primary/10 text-primary"
                                        : "text-muted-foreground hover:text-foreground hover:bg-accent",
                                )}
                                title={lang.available ? undefined : `"${lang.label}" 即将推出，当前请使用 JSON 或 YAML`}
                            >
                                {lang.label}
                                {!lang.available && (
                                    <span className="ml-1.5 rounded-full bg-yellow-500/10 px-1.5 py-0.5 text-[10px] font-medium text-yellow-600 dark:text-yellow-400">
                                        即将推出
                                    </span>
                                )}
                            </button>
                        ))}
                    </div>

                    {/* 右侧：操作按钮 */}
                    <div className="flex items-center gap-1">
                        {/* 视图切换 */}
                        <div className="flex items-center rounded-lg border border-border p-0.5 mr-2">
                            <button
                                onClick={() => setViewMode("code")}
                                className={cn(
                                    "flex items-center gap-1 px-2.5 py-1 rounded-md text-xs font-medium transition-colors",
                                    viewMode === "code"
                                        ? "bg-accent text-foreground"
                                        : "text-muted-foreground hover:text-foreground",
                                )}
                            >
                                <Code className="h-3.5 w-3.5" />
                                <span>代码</span>
                            </button>
                            <button
                                onClick={() => setViewMode("preview")}
                                className={cn(
                                    "flex items-center gap-1 px-2.5 py-1 rounded-md text-xs font-medium transition-colors",
                                    viewMode === "preview"
                                        ? "bg-accent text-foreground"
                                        : "text-muted-foreground hover:text-foreground",
                                )}
                            >
                                <Eye className="h-3.5 w-3.5" />
                                <span>预览</span>
                            </button>
                        </div>

                        <button
                            onClick={handleRun}
                            disabled={running || !currentLang.available}
                            title={!currentLang.available ? "当前语言暂不支持直接执行，请切换到 JSON 或 YAML" : undefined}
                            className="flex items-center gap-1.5 rounded-lg bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 transition-colors disabled:opacity-50"
                        >
                            <Play className={cn("h-3.5 w-3.5", running && "animate-pulse")} />
                            <span>{running ? "运行中..." : "运行"}</span>
                        </button>
                        <button
                            onClick={handleCopy}
                            className="flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-xs font-medium text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
                            title="复制到剪贴板"
                        >
                            {copied ? (
                                <Check className="h-3.5 w-3.5 text-green-500" />
                            ) : (
                                <Copy className="h-3.5 w-3.5" />
                            )}
                        </button>
                        <button
                            onClick={handleDownload}
                            className="flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-xs font-medium text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
                            title={`下载为 ${currentLang.ext} 文件`}
                        >
                            <Download className="h-3.5 w-3.5" />
                        </button>
                    </div>
                </div>

                {/* 主体区域：编辑器 + 预览 */}
                <div className="flex-1 flex min-h-0">
                    {/* 代码编辑器区域 */}
                    <motion.div
                        layout
                        className={cn(
                            "flex flex-col min-h-0",
                            viewMode === "code" ? "flex-1" : "flex-1 border-r border-border",
                        )}
                    >
                        <div className="flex-1 flex bg-background/50">
                            {/* 行号列 */}
                            <div className="w-12 shrink-0 border-r border-border bg-muted/30 py-3 select-none overflow-hidden">
                                {Array.from({ length: Math.max(lineCount, 1) }, (_, i) => (
                                    <div
                                        key={i}
                                        className="h-6 flex items-center justify-end pr-3 text-xs text-muted-foreground/50 leading-6"
                                    >
                                        {i + 1}
                                    </div>
                                ))}
                            </div>

                            {/* 代码编辑区 */}
                            <div className="flex-1 overflow-auto">
                                <textarea
                                    ref={editorRef}
                                    value={currentCode}
                                    onChange={(e) => handleCodeChange(e.target.value)}
                                    className="w-full h-full min-h-[400px] py-3 px-4 text-sm font-mono text-foreground bg-transparent border-none outline-none resize-none leading-6"
                                    spellCheck={false}
                                    placeholder={`在此输入 ${currentLang.label} 代码...`}
                                />
                            </div>
                        </div>

                        {/* 底部状态栏 */}
                        <div className="flex items-center justify-between h-7 px-4 border-t border-border bg-muted/30 shrink-0">
                            <span className="text-[11px] text-muted-foreground">
                                {currentLang.label} · UTF-8 · LF
                            </span>
                            <span className="text-[11px] text-muted-foreground">
                                行 {lineCount}
                            </span>
                        </div>
                    </motion.div>

                    {/* 预览区域 */}
                    {viewMode === "preview" && (
                        <motion.div
                            initial={{ opacity: 0, width: 0 }}
                            animate={{ opacity: 1, width: "50%" }}
                            exit={{ opacity: 0, width: 0 }}
                            className="flex flex-col min-h-0"
                        >
                            <div className="flex items-center h-8 px-4 border-b border-border bg-muted/30 shrink-0">
                                <Terminal className="h-3.5 w-3.5 text-muted-foreground mr-2" />
                                <span className="text-[11px] font-medium text-muted-foreground">
                                    输出
                                </span>
                                {running && (
                                    <Loader2 className="h-3 w-3 ml-2 animate-spin text-primary" />
                                )}
                            </div>
                            <div className="flex-1 overflow-auto bg-background/30 p-4">
                                {runOutput || running ? (
                                    <pre className="text-sm font-mono text-foreground/80 leading-relaxed whitespace-pre-wrap">
                                        {runOutput || "准备执行..."}
                                    </pre>
                                ) : (
                                    <div className="h-full flex items-center justify-center">
                                        <div className="text-center">
                                            <div className="h-12 w-12 mx-auto mb-3 rounded-full bg-muted flex items-center justify-center">
                                                <Eye className="h-5 w-5 text-muted-foreground/40" />
                                            </div>
                                            <p className="text-sm text-muted-foreground/60">
                                                点击"运行"查看解析结果
                                            </p>
                                        </div>
                                    </div>
                                )}
                            </div>
                        </motion.div>
                    )}
                </div>
            </div>
        </PageTransition>
    );
};

export default CodeMode;