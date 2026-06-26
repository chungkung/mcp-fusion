// ============================================================
// 插件市场模块
//
// 功能：
// - 从远程模板仓库（GitHub/GitLab）获取工作流模板
// - 搜索与分类浏览
// - 一键安装/导入工作流
// - 版本管理与更新通知
// ============================================================

use serde::{Deserialize, Serialize};
use crate::storage::sqlite::{Workflow, WorkflowNode, WorkflowEdge, WorkflowNodePosition, WorkflowNodeData};

// ============================================================
// 模板数据结构
// ============================================================

/// 市场模板元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub author: String,
    pub stars: u32,
    pub downloads: u32,
    pub icon: String,
    /// 模板包含的节点数
    pub node_count: usize,
    /// 模板包含的连线数
    pub edge_count: usize,
    /// 模板标签
    pub tags: Vec<String>,
    /// 模板来源仓库 URL
    pub source_url: String,
    /// 模板文件的原始 URL
    pub file_url: String,
    /// 更新时间
    pub updated_at: String,
}

/// 模板详情（包含完整工作流定义）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDetail {
    pub template: MarketplaceTemplate,
    /// 工作流定义 JSON
    pub workflow: WorkflowTemplate,
}

/// 工作流模板定义（与 Workflow 对应，但去掉 id 等运行时字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    pub name: String,
    pub description: String,
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
}

/// 已安装模板记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledTemplate {
    pub template_id: String,
    pub workflow_id: String,
    pub version: String,
    pub installed_at: i64,
}

/// 模板更新信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateUpdate {
    pub template_id: String,
    pub workflow_id: String,
    pub current_version: String,
    pub latest_version: String,
    pub has_update: bool,
}

// ============================================================
// 模板仓库客户端
// ============================================================

/// 模板仓库配置
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// 仓库索引 URL（JSON 文件），包含所有模板的元数据列表
    pub index_url: String,
    /// 模板详情基础 URL，拼接 {id}/{version}/template.json 获取详情
    pub base_url: String,
    /// GitHub API token（可选，用于私有仓库或提高速率限制）
    pub token: Option<String>,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            index_url: "https://raw.githubusercontent.com/mcp-fusion/templates/main/index.json".to_string(),
            base_url: "https://raw.githubusercontent.com/mcp-fusion/templates/main".to_string(),
            token: None,
        }
    }
}

impl RegistryConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        let index_url = std::env::var("MCP_FUSION_TEMPLATE_INDEX_URL")
            .unwrap_or_else(|_| Self::default().index_url);
        let base_url = std::env::var("MCP_FUSION_TEMPLATE_BASE_URL")
            .unwrap_or_else(|_| Self::default().base_url);
        let token = std::env::var("GITHUB_TOKEN").ok()
            .or_else(|| std::env::var("GITLAB_TOKEN").ok());

        Self { index_url, base_url, token }
    }
}

/// 模板仓库客户端
pub struct TemplateRegistry {
    config: RegistryConfig,
    client: reqwest::Client,
}

impl TemplateRegistry {
    pub fn new(config: RegistryConfig) -> Self {
        let mut client_builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .user_agent("mcp-fusion-marketplace/1.0");

        if let Some(ref token) = config.token {
            let mut headers = reqwest::header::HeaderMap::new();
            let auth_value = format!("Bearer {}", token);
            match reqwest::header::HeaderValue::from_str(&auth_value) {
                Ok(mut auth_header) => {
                    auth_header.set_sensitive(true);
                    headers.insert(reqwest::header::AUTHORIZATION, auth_header);
                    client_builder = client_builder.default_headers(headers);
                }
                Err(_) => {
                    tracing::warn!("Token 格式无效，跳过认证头");
                }
            }
        }

        let client = client_builder.build().unwrap_or_default();
        Self { config, client }
    }

    /// 获取模板索引列表
    pub async fn list_templates(&self, category: Option<&str>, search: Option<&str>) -> Result<Vec<MarketplaceTemplate>, String> {
        let resp = self.client
            .get(&self.config.index_url)
            .send()
            .await
            .map_err(|e| format!("获取模板列表失败: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("模板仓库返回错误: HTTP {}", resp.status()));
        }

        let templates: Vec<MarketplaceTemplate> = resp
            .json()
            .await
            .map_err(|e| format!("解析模板列表失败: {e}"))?;

        // 过滤
        let mut result = templates;
        if let Some(cat) = category {
            if !cat.is_empty() && cat != "all" {
                result.retain(|t| t.category == cat);
            }
        }
        if let Some(q) = search {
            if !q.is_empty() {
                let q_lower = q.to_lowercase();
                result.retain(|t| {
                    t.name.to_lowercase().contains(&q_lower)
                        || t.description.to_lowercase().contains(&q_lower)
                        || t.tags.iter().any(|tag| tag.to_lowercase().contains(&q_lower))
                });
            }
        }

        Ok(result)
    }

    /// 获取模板详情（含完整工作流定义）
    pub async fn get_template_detail(&self, template_id: &str, version: &str) -> Result<TemplateDetail, String> {
        // 远端路径: {base_url}/{template_id}/{version}/template.json
        let url = format!("{}/{}/{}/template.json", self.config.base_url, template_id, version);

        let resp = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("获取模板详情失败: {e}"))?;

        if !resp.status().is_success() {
            return Err(format!("模板详情不存在: HTTP {} (id={}, version={})", resp.status(), template_id, version));
        }

        let detail: TemplateDetail = resp
            .json()
            .await
            .map_err(|e| format!("解析模板详情失败: {e}"))?;

        Ok(detail)
    }
}

// ============================================================
// 模板安装器
// ============================================================

/// 将模板详情转换为工作流实体
pub fn template_to_workflow(template: &TemplateDetail, workflow_id: &str) -> Workflow {
    let now = chrono::Utc::now().timestamp_millis();

    Workflow {
        id: workflow_id.to_string(),
        name: template.workflow.name.clone(),
        description: template.workflow.description.clone(),
        mode: "intent".to_string(),
        status: "idle".to_string(),
        nodes: template.workflow.nodes.clone(),
        edges: template.workflow.edges.clone(),
        locked: false,
        locked_at: None,
        created_at: now,
        updated_at: now,
    }
}

// ============================================================
// 本地模板缓存（使用 SQLite 存储已安装记录）
// ============================================================

use crate::storage::sqlite::Database;

/// 记录模板安装
pub fn record_install(db: &Database, template_id: &str, workflow_id: &str, version: &str) -> Result<(), String> {
    let conn = db.conn().lock().map_err(|e| format!("获取锁失败: {e}"))?;
    let now = chrono::Utc::now().timestamp_millis();

    conn.execute(
        "INSERT OR REPLACE INTO installed_templates (template_id, workflow_id, version, installed_at)
         VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![template_id, workflow_id, version, now],
    )
    .map_err(|e| format!("记录模板安装失败: {e}"))?;

    Ok(())
}

/// 获取已安装模板列表
pub fn list_installed(db: &Database) -> Result<Vec<InstalledTemplate>, String> {
    let conn = db.conn().lock().map_err(|e| format!("获取锁失败: {e}"))?;

    let mut stmt = conn
        .prepare("SELECT template_id, workflow_id, version, installed_at FROM installed_templates ORDER BY installed_at DESC")
        .map_err(|e| format!("查询已安装模板失败: {e}"))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(InstalledTemplate {
                template_id: row.get(0)?,
                workflow_id: row.get(1)?,
                version: row.get(2)?,
                installed_at: row.get(3)?,
            })
        })
        .map_err(|e| format!("查询已安装模板失败: {e}"))?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| format!("读取已安装模板记录失败: {e}"))?);
    }
    Ok(result)
}

/// 检查已安装模板是否有更新
pub async fn check_updates(
    db: &Database,
    registry: &TemplateRegistry,
) -> Result<Vec<TemplateUpdate>, String> {
    let installed = list_installed(db)?;
    let mut updates = Vec::new();

    // 获取所有模板的最新版本
    let all_templates = match registry.list_templates(None, None).await {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!("无法获取模板列表，跳过更新检查: {e}");
            return Ok(updates);
        }
    };

    for inst in &installed {
        if let Some(latest) = all_templates.iter().find(|t| t.id == inst.template_id) {
            if latest.version != inst.version {
                updates.push(TemplateUpdate {
                    template_id: inst.template_id.clone(),
                    workflow_id: inst.workflow_id.clone(),
                    current_version: inst.version.clone(),
                    latest_version: latest.version.clone(),
                    has_update: true,
                });
            }
        }
    }

    Ok(updates)
}

// ============================================================
// 内置模板（离线回退方案）
// ============================================================

/// 内置模板列表，当远程仓库不可用时使用
pub fn builtin_templates() -> Vec<MarketplaceTemplate> {
    vec![
        MarketplaceTemplate {
            id: "api-data-aggregation".to_string(),
            name: "API 数据聚合".to_string(),
            description: "从多个 API 获取数据并合并输出".to_string(),
            category: "数据处理".to_string(),
            version: "1.0.0".to_string(),
            author: "MCP Fusion".to_string(),
            stars: 128,
            downloads: 2400,
            icon: "🔗".to_string(),
            node_count: 3,
            edge_count: 2,
            tags: vec!["api".to_string(), "http".to_string(), "data".to_string()],
            source_url: String::new(),
            file_url: String::new(),
            updated_at: "2025-01-01".to_string(),
        },
        MarketplaceTemplate {
            id: "file-batch-process".to_string(),
            name: "文件批量处理".to_string(),
            description: "批量重命名、转换格式、压缩文件".to_string(),
            category: "文件操作".to_string(),
            version: "1.0.0".to_string(),
            author: "MCP Fusion".to_string(),
            stars: 96,
            downloads: 1800,
            icon: "📁".to_string(),
            node_count: 2,
            edge_count: 1,
            tags: vec!["file".to_string(), "batch".to_string(), "convert".to_string()],
            source_url: String::new(),
            file_url: String::new(),
            updated_at: "2025-01-01".to_string(),
        },
        MarketplaceTemplate {
            id: "cron-scheduler".to_string(),
            name: "定时任务调度".to_string(),
            description: "基于 Cron 表达式的定时工作流".to_string(),
            category: "自动化".to_string(),
            version: "1.0.0".to_string(),
            author: "MCP Fusion".to_string(),
            stars: 75,
            downloads: 3200,
            icon: "⏰".to_string(),
            node_count: 2,
            edge_count: 1,
            tags: vec!["cron".to_string(), "schedule".to_string(), "timer".to_string()],
            source_url: String::new(),
            file_url: String::new(),
            updated_at: "2025-01-01".to_string(),
        },
        MarketplaceTemplate {
            id: "git-pipeline".to_string(),
            name: "Git 操作流水线".to_string(),
            description: "自动 clone、commit、push 操作".to_string(),
            category: "DevOps".to_string(),
            version: "1.0.0".to_string(),
            author: "MCP Fusion".to_string(),
            stars: 210,
            downloads: 5600,
            icon: "🔧".to_string(),
            node_count: 3,
            edge_count: 2,
            tags: vec!["git".to_string(), "ci".to_string(), "devops".to_string()],
            source_url: String::new(),
            file_url: String::new(),
            updated_at: "2025-01-01".to_string(),
        },
        MarketplaceTemplate {
            id: "db-migration".to_string(),
            name: "数据库迁移".to_string(),
            description: "跨数据库的数据迁移与同步".to_string(),
            category: "数据库".to_string(),
            version: "1.0.0".to_string(),
            author: "MCP Fusion".to_string(),
            stars: 64,
            downloads: 1200,
            icon: "🗄️".to_string(),
            node_count: 2,
            edge_count: 1,
            tags: vec!["database".to_string(), "migration".to_string(), "sql".to_string()],
            source_url: String::new(),
            file_url: String::new(),
            updated_at: "2025-01-01".to_string(),
        },
        MarketplaceTemplate {
            id: "image-process".to_string(),
            name: "图片处理流水线".to_string(),
            description: "裁剪、压缩、加水印一键完成".to_string(),
            category: "媒体处理".to_string(),
            version: "1.0.0".to_string(),
            author: "MCP Fusion".to_string(),
            stars: 89,
            downloads: 1500,
            icon: "🖼️".to_string(),
            node_count: 3,
            edge_count: 2,
            tags: vec!["image".to_string(), "compress".to_string(), "watermark".to_string()],
            source_url: String::new(),
            file_url: String::new(),
            updated_at: "2025-01-01".to_string(),
        },
        MarketplaceTemplate {
            id: "webhook-listener".to_string(),
            name: "Webhook 监听".to_string(),
            description: "接收 Webhook 并触发后续流程".to_string(),
            category: "自动化".to_string(),
            version: "1.0.0".to_string(),
            author: "MCP Fusion".to_string(),
            stars: 156,
            downloads: 4100,
            icon: "🪝".to_string(),
            node_count: 2,
            edge_count: 1,
            tags: vec!["webhook".to_string(), "trigger".to_string(), "event".to_string()],
            source_url: String::new(),
            file_url: String::new(),
            updated_at: "2025-01-01".to_string(),
        },
        MarketplaceTemplate {
            id: "log-analyzer".to_string(),
            name: "日志分析".to_string(),
            description: "收集、解析、可视化日志数据".to_string(),
            category: "监控".to_string(),
            version: "1.0.0".to_string(),
            author: "MCP Fusion".to_string(),
            stars: 43,
            downloads: 900,
            icon: "📊".to_string(),
            node_count: 2,
            edge_count: 1,
            tags: vec!["log".to_string(), "analyze".to_string(), "monitor".to_string()],
            source_url: String::new(),
            file_url: String::new(),
            updated_at: "2025-01-01".to_string(),
        },
    ]
}

/// 获取内置模板的详情（含工作流定义）
pub fn builtin_template_detail(template_id: &str) -> Option<TemplateDetail> {
    let templates = builtin_templates();
    let template = templates.into_iter().find(|t| t.id == template_id)?;
    let name = template.name.clone();
    let description = template.description.clone();

    let (nodes, edges) = match template_id {
        "api-data-aggregation" => {
            let n1 = WorkflowNode {
                id: "node_0".to_string(),
                node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 100.0, y: 100.0 },
                data: WorkflowNodeData {
                    label: "HTTP 请求".to_string(),
                    tool: None,
                    inputs: serde_json::json!({}),
                    outputs: serde_json::json!({}),
                    config: serde_json::json!({"toolName": "http_request", "serverId": "mcp-http"}),
                },
            };
            let n2 = WorkflowNode {
                id: "node_1".to_string(),
                node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 400.0, y: 100.0 },
                data: WorkflowNodeData {
                    label: "JSON 转换".to_string(),
                    tool: None,
                    inputs: serde_json::json!({}),
                    outputs: serde_json::json!({}),
                    config: serde_json::json!({"toolName": "json_transform", "serverId": "mcp-data"}),
                },
            };
            let n3 = WorkflowNode {
                id: "node_2".to_string(),
                node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 700.0, y: 100.0 },
                data: WorkflowNodeData {
                    label: "保存文件".to_string(),
                    tool: None,
                    inputs: serde_json::json!({}),
                    outputs: serde_json::json!({}),
                    config: serde_json::json!({"toolName": "file_writer", "serverId": "mcp-fs"}),
                },
            };
            let e1 = WorkflowEdge { id: "edge_0".to_string(), source: "node_0".to_string(), target: "node_1".to_string(), source_handle: None, target_handle: None, edge_type: Some("smoothstep".to_string()), animated: Some(true) };
            let e2 = WorkflowEdge { id: "edge_1".to_string(), source: "node_1".to_string(), target: "node_2".to_string(), source_handle: None, target_handle: None, edge_type: Some("smoothstep".to_string()), animated: Some(true) };
            (vec![n1, n2, n3], vec![e1, e2])
        }
        "file-batch-process" => {
            let n1 = WorkflowNode {
                id: "node_0".to_string(), node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 100.0, y: 100.0 },
                data: WorkflowNodeData { label: "读取文件".to_string(), tool: None, inputs: serde_json::json!({}), outputs: serde_json::json!({}), config: serde_json::json!({"toolName": "file_reader", "serverId": "mcp-fs"}) },
            };
            let n2 = WorkflowNode {
                id: "node_1".to_string(), node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 400.0, y: 100.0 },
                data: WorkflowNodeData { label: "格式转换".to_string(), tool: None, inputs: serde_json::json!({}), outputs: serde_json::json!({}), config: serde_json::json!({"toolName": "file_convert", "serverId": "mcp-fs"}) },
            };
            let e1 = WorkflowEdge { id: "edge_0".to_string(), source: "node_0".to_string(), target: "node_1".to_string(), source_handle: None, target_handle: None, edge_type: Some("smoothstep".to_string()), animated: Some(true) };
            (vec![n1, n2], vec![e1])
        }
        "cron-scheduler" => {
            let n1 = WorkflowNode {
                id: "node_0".to_string(), node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 100.0, y: 100.0 },
                data: WorkflowNodeData { label: "Cron 触发器".to_string(), tool: None, inputs: serde_json::json!({}), outputs: serde_json::json!({}), config: serde_json::json!({"toolName": "cron_trigger", "serverId": "mcp-scheduler"}) },
            };
            let n2 = WorkflowNode {
                id: "node_1".to_string(), node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 400.0, y: 100.0 },
                data: WorkflowNodeData { label: "执行任务".to_string(), tool: None, inputs: serde_json::json!({}), outputs: serde_json::json!({}), config: serde_json::json!({"toolName": "run_task", "serverId": "mcp-scheduler"}) },
            };
            let e1 = WorkflowEdge { id: "edge_0".to_string(), source: "node_0".to_string(), target: "node_1".to_string(), source_handle: None, target_handle: None, edge_type: Some("smoothstep".to_string()), animated: Some(true) };
            (vec![n1, n2], vec![e1])
        }
        "git-pipeline" => {
            let n1 = WorkflowNode {
                id: "node_0".to_string(), node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 100.0, y: 100.0 },
                data: WorkflowNodeData { label: "Git Clone".to_string(), tool: None, inputs: serde_json::json!({}), outputs: serde_json::json!({}), config: serde_json::json!({"toolName": "git_clone", "serverId": "mcp-git"}) },
            };
            let n2 = WorkflowNode {
                id: "node_1".to_string(), node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 400.0, y: 100.0 },
                data: WorkflowNodeData { label: "Git Commit".to_string(), tool: None, inputs: serde_json::json!({}), outputs: serde_json::json!({}), config: serde_json::json!({"toolName": "git_commit", "serverId": "mcp-git"}) },
            };
            let n3 = WorkflowNode {
                id: "node_2".to_string(), node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 700.0, y: 100.0 },
                data: WorkflowNodeData { label: "Git Push".to_string(), tool: None, inputs: serde_json::json!({}), outputs: serde_json::json!({}), config: serde_json::json!({"toolName": "git_push", "serverId": "mcp-git"}) },
            };
            let e1 = WorkflowEdge { id: "edge_0".to_string(), source: "node_0".to_string(), target: "node_1".to_string(), source_handle: None, target_handle: None, edge_type: Some("smoothstep".to_string()), animated: Some(true) };
            let e2 = WorkflowEdge { id: "edge_1".to_string(), source: "node_1".to_string(), target: "node_2".to_string(), source_handle: None, target_handle: None, edge_type: Some("smoothstep".to_string()), animated: Some(true) };
            (vec![n1, n2, n3], vec![e1, e2])
        }
        _ => {
            let n1 = WorkflowNode {
                id: "node_0".to_string(), node_type: "mcpTool".to_string(),
                position: WorkflowNodePosition { x: 100.0, y: 100.0 },
                data: WorkflowNodeData { label: name.clone(), tool: None, inputs: serde_json::json!({}), outputs: serde_json::json!({}), config: serde_json::json!({"toolName": "http_request", "serverId": "mcp-http"}) },
            };
            (vec![n1], vec![])
        }
    };

    Some(TemplateDetail {
        template,
        workflow: WorkflowTemplate {
            name,
            description,
            nodes,
            edges,
        },
    })
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_templates_not_empty() {
        let templates = builtin_templates();
        assert!(!templates.is_empty());
        assert!(templates.len() >= 8);
    }

    #[test]
    fn test_builtin_template_detail() {
        let detail = builtin_template_detail("api-data-aggregation");
        assert!(detail.is_some());
        let detail = detail.unwrap();
        assert_eq!(detail.workflow.nodes.len(), 3);
        assert_eq!(detail.workflow.edges.len(), 2);
    }

    #[test]
    fn test_builtin_template_detail_unknown() {
        let detail = builtin_template_detail("nonexistent");
        assert!(detail.is_none());
    }

    #[test]
    fn test_template_to_workflow() {
        let detail = builtin_template_detail("api-data-aggregation").unwrap();
        let workflow = template_to_workflow(&detail, "test-wf-1");
        assert_eq!(workflow.id, "test-wf-1");
        assert_eq!(workflow.nodes.len(), 3);
        assert_eq!(workflow.edges.len(), 2);
    }

    #[test]
    fn test_registry_config_default() {
        let config = RegistryConfig::default();
        assert!(config.index_url.contains("index.json"));
    }
}