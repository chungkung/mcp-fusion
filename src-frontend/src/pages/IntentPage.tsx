import { useState, useCallback, useRef, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { motion } from "framer-motion";
import toast from "react-hot-toast";

import PageTransition from "@/components/animations/PageTransition";
import ParticleBackground from "@/components/animations/ParticleBackground";
import LoadingSpinner from "@/components/animations/LoadingSpinner";

import { intentService } from "@/services/ipc";
import { useWorkflowStore } from "@/stores/useWorkflowStore";

import { ROUTES } from "@shared/constants";

// ============================================================
// 示例提示词
// ============================================================

const EXAMPLES = [
    "帮我查询 GitHub trending 仓库，并保存到本地文件",
    "读取 config.json，将其中所有 API 地址替换为生产环境地址",
    "批量下载指定 URL 列表中的图片，并压缩为 WebP 格式",
    "监听指定目录的文件变化，自动执行 ESLint 检查和格式化",
];

// ============================================================
// 动画配置
// ============================================================

const containerVariants = {
    hidden: {},
    visible: {
        transition: {
            staggerChildren: 0.12,
        },
    },
};

const itemVariants = {
    hidden: { opacity: 0, y: 20 },
    visible: {
        opacity: 1,
        y: 0,
        transition: {
            duration: 0.4,
            ease: [0.25, 0.46, 0.45, 0.94],
        },
    },
};

// ============================================================
// 组件
// ============================================================

function IntentPage(): JSX.Element {
    const navigate = useNavigate();
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    const setCurrentWorkflow = useWorkflowStore(
        (s) => s.setCurrentWorkflow,
    );

    const [input, setInput] = useState("");
    const [loading, setLoading] = useState(false);

    // 自动聚焦
    useEffect(() => {
        textareaRef.current?.focus();
    }, []);

    // 点击示例填充输入框
    const handleExampleClick = useCallback((example: string) => {
        setInput(example);
        textareaRef.current?.focus();
    }, []);

    // 提交意图
    const handleSubmit = useCallback(async () => {
        const trimmed = input.trim();
        if (!trimmed) {
            toast.error("请输入您的意图描述");
            return;
        }

        setLoading(true);
        try {
            const result = await intentService.parse(trimmed);

            if (result.success && result.data) {
                setCurrentWorkflow(result.data);
                toast.success("工作流已生成，正在跳转...");
                navigate(ROUTES.CANVAS);
            } else {
                toast.error(result.error ?? "意图解析失败，请重试");
            }
        } catch {
            toast.error("网络异常，请检查连接后重试");
        } finally {
            setLoading(false);
        }
    }, [input, navigate, setCurrentWorkflow]);

    // Ctrl+Enter 提交
    const handleKeyDown = useCallback(
        (e: React.KeyboardEvent) => {
            if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
                e.preventDefault();
                handleSubmit();
            }
        },
        [handleSubmit],
    );

    const canSubmit = input.trim().length > 0 && !loading;

    return (
        <PageTransition className="relative h-full w-full">
            {/* 粒子背景 */}
            <ParticleBackground className="absolute inset-0" />

            <motion.div
                className="relative z-10 flex flex-col h-full items-center overflow-y-auto py-8 px-6"
                variants={containerVariants}
                initial="hidden"
                animate="visible"
            >
                {/* 标题 */}
                <motion.h1
                    className="text-3xl font-bold tracking-tight text-foreground mb-2 shrink-0"
                    variants={itemVariants}
                >
                    MCP Fusion
                </motion.h1>
                <motion.p
                    className="text-muted-foreground mb-4 text-center max-w-lg shrink-0"
                    variants={itemVariants}
                >
                    用自然语言描述您的需求，AI 将自动编排多协议工作流
                </motion.p>

                {/* 输入卡片（玻璃拟态） */}
                <motion.div
                    className="w-full max-w-2xl rounded-2xl border border-border bg-background/60 backdrop-blur-xl p-5 shadow-lg shrink-0"
                    variants={itemVariants}
                >
                    <textarea
                        ref={textareaRef}
                        className="w-full min-h-[80px] resize-none rounded-lg border border-input bg-transparent px-4 py-3 text-foreground placeholder:text-muted-foreground/60 focus:outline-none focus:ring-2 focus:ring-primary/30 transition-shadow"
                        placeholder="例如：查询天气 API 数据并保存为 CSV 文件..."
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        onKeyDown={handleKeyDown}
                        disabled={loading}
                    />

                    <div className="flex items-center justify-between mt-4">
                        <span className="text-xs text-muted-foreground">
                            Ctrl + Enter 快速提交
                        </span>

                        <motion.button
                            whileHover={canSubmit ? { scale: 1.03 } : {}}
                            whileTap={canSubmit ? { scale: 0.97 } : {}}
                            disabled={!canSubmit}
                            onClick={handleSubmit}
                            className="inline-flex items-center gap-2 rounded-lg bg-primary px-6 py-2.5 text-sm font-medium text-primary-foreground transition-all hover:bg-primary/90 disabled:opacity-40 disabled:cursor-not-allowed"
                        >
                            {loading ? (
                                <>
                                    <LoadingSpinner size="sm" />
                                    <span>解析中...</span>
                                </>
                            ) : (
                                <span>生成工作流</span>
                            )}
                        </motion.button>
                    </div>
                </motion.div>

                {/* 示例提示词 */}
                <motion.div className="w-full max-w-2xl mt-4 shrink-0" variants={itemVariants}>
                    <p className="text-xs font-medium text-muted-foreground mb-2 uppercase tracking-wider">
                        试试这样说
                    </p>
                    <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
                        {EXAMPLES.map((example, i) => (
                            <motion.button
                                key={i}
                                whileHover={{ scale: 1.02 }}
                                whileTap={{ scale: 0.98 }}
                                onClick={() => handleExampleClick(example)}
                                disabled={loading}
                                className="text-left rounded-xl border border-border bg-background/50 backdrop-blur-sm px-3 py-2.5 text-sm text-foreground/80 hover:bg-accent hover:text-accent-foreground transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
                            >
                                {example}
                            </motion.button>
                        ))}
                    </div>
                </motion.div>
            </motion.div>
        </PageTransition>
    );
}

export default IntentPage;