import { WorkflowNode, WorkflowEdge } from "./workflow";

// ============================================================
// 插件市场类型定义（前后端共享）
// ============================================================

/** 市场模板元数据 */
export interface MarketplaceTemplate {
    id: string;
    name: string;
    description: string;
    category: string;
    version: string;
    author: string;
    stars: number;
    downloads: number;
    icon: string;
    node_count: number;
    edge_count: number;
    tags: string[];
    source_url: string;
    file_url: string;
    updated_at: string;
}

/** 工作流模板定义 */
export interface WorkflowTemplate {
    name: string;
    description: string;
    nodes: WorkflowNode[];
    edges: WorkflowEdge[];
}

/** 模板详情 */
export interface TemplateDetail {
    template: MarketplaceTemplate;
    workflow: WorkflowTemplate;
}

/** 已安装模板记录 */
export interface InstalledTemplate {
    template_id: string;
    workflow_id: string;
    version: string;
    installed_at: number;
}

/** 模板更新信息 */
export interface TemplateUpdate {
    template_id: string;
    workflow_id: string;
    current_version: string;
    latest_version: string;
    has_update: boolean;
}