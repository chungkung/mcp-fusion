import { RunStatus, WorkflowMode } from "./common";
import { MCPTool } from "./mcp";

// ============================================================
// 接口定义
// ============================================================

/** 工作流节点位置 */
export interface WorkflowNodePosition {
    x: number;
    y: number;
}

/** 工作流节点数据 */
export interface WorkflowNodeData {
    label: string;
    tool?: MCPTool;
    inputs: Record<string, unknown>;
    outputs: Record<string, unknown>;
    config: Record<string, unknown>;
}

/** 工作流节点（匹配 React Flow Node 结构） */
export interface WorkflowNode {
    id: string;
    type: string;
    position: WorkflowNodePosition;
    data: WorkflowNodeData;
}

/** 工作流连线（匹配 React Flow Edge 结构） */
export interface WorkflowEdge {
    id: string;
    source: string;
    target: string;
    sourceHandle?: string;
    targetHandle?: string;
    type?: string;
    animated?: boolean;
}

/** 工作流实体 */
export interface Workflow {
    id: string;
    name: string;
    description: string;
    mode: WorkflowMode;
    status: RunStatus;
    nodes: WorkflowNode[];
    edges: WorkflowEdge[];
    createdAt: number;
    updatedAt: number;
}