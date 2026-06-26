import { create } from "zustand";
import { type Workflow, type WorkflowMode, RunStatus } from "@shared/types";
import { workflowService } from "@/services/ipc";

interface WorkflowState {
    // ---- 数据 ----
    workflows: Workflow[];
    currentWorkflow: Workflow | null;

    // ---- 过滤 ----
    filterMode: WorkflowMode | null;
    filterStatus: RunStatus | null;

    // ---- 加载 ----
    loading: boolean;
    error: string | null;

    // ---- 操作 ----
    fetchWorkflows: () => Promise<void>;
    loadWorkflow: (id: string) => Promise<void>;
    createWorkflow: (workflow: Workflow) => Promise<void>;
    saveWorkflow: (workflow: Workflow) => Promise<void>;
    deleteWorkflow: (id: string) => Promise<void>;
    executeWorkflow: (id: string) => Promise<void>;
    setCurrentWorkflow: (workflow: Workflow | null) => void;
    setFilterMode: (mode: WorkflowMode | null) => void;
    setFilterStatus: (status: RunStatus | null) => void;
    clearError: () => void;
}

export const useWorkflowStore = create<WorkflowState>((set) => ({
    // ---- 数据 ----
    workflows: [],
    currentWorkflow: null,

    // ---- 过滤 ----
    filterMode: null,
    filterStatus: null,

    // ---- 加载 ----
    loading: false,
    error: null,

    // ---- 操作 ----
    fetchWorkflows: async () => {
        set({ loading: true, error: null });
        try {
            const result = await workflowService.list();
            if (result.success && result.data) {
                set({ workflows: result.data });
            } else {
                set({ error: result.error ?? "获取工作流列表失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        } finally {
            set({ loading: false });
        }
    },

    loadWorkflow: async (id) => {
        set({ loading: true, error: null });
        try {
            const result = await workflowService.list();
            if (result.success && result.data) {
                const found = result.data.find((w) => w.id === id) ?? null;
                set({ currentWorkflow: found });
                if (!found) {
                    set({ error: "工作流未找到" });
                }
            } else {
                set({ error: result.error ?? "加载工作流失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        } finally {
            set({ loading: false });
        }
    },

    createWorkflow: async (workflow) => {
        set({ loading: true, error: null });
        try {
            const result = await workflowService.save(workflow);
            if (result.success && result.data) {
                set((s) => ({
                    workflows: [...s.workflows, result.data!],
                    currentWorkflow: result.data!,
                }));
            } else {
                set({ error: result.error ?? "创建工作流失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        } finally {
            set({ loading: false });
        }
    },

    saveWorkflow: async (workflow) => {
        set({ loading: true, error: null });
        try {
            const result = await workflowService.save(workflow);
            if (result.success && result.data) {
                set((s) => ({
                    workflows: s.workflows.map((w) =>
                        w.id === result.data!.id ? result.data! : w,
                    ),
                    currentWorkflow: result.data!,
                }));
            } else {
                set({ error: result.error ?? "保存工作流失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        } finally {
            set({ loading: false });
        }
    },

    deleteWorkflow: async (id) => {
        set({ loading: true, error: null });
        try {
            const result = await workflowService.delete(id);
            if (result.success) {
                set((s) => ({
                    workflows: s.workflows.filter((w) => w.id !== id),
                    currentWorkflow:
                        s.currentWorkflow?.id === id
                            ? null
                            : s.currentWorkflow,
                }));
            } else {
                set({ error: result.error ?? "删除工作流失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        } finally {
            set({ loading: false });
        }
    },

    executeWorkflow: async (id) => {
        set({ loading: true, error: null });
        try {
            const result = await workflowService.execute(id);
            if (result.success && result.data) {
                // 更新工作流状态为执行结果
                set((s) => ({
                    workflows: s.workflows.map((w) =>
                        w.id === id
                            ? { ...w, status: (result.data!.status as RunStatus) ?? RunStatus.Idle }
                            : w,
                    ),
                }));
            } else {
                set({ error: result.error ?? "执行工作流失败" });
            }
        } catch (e) {
            set({ error: e instanceof Error ? e.message : "未知错误" });
        } finally {
            set({ loading: false });
        }
    },

    setCurrentWorkflow: (currentWorkflow) => set({ currentWorkflow }),

    setFilterMode: (filterMode) => set({ filterMode }),

    setFilterStatus: (filterStatus) => set({ filterStatus }),

    clearError: () => set({ error: null }),
}));