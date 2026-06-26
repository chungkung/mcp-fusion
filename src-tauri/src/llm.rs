// ============================================================
// LLM 意图解析模块
//
// 功能：
// - 接入 OpenAI-compatible API 做语义理解
// - 支持多轮对话细化工作流
// - 自动推荐 MCP 工具
// - 离线回退到关键词匹配
// ============================================================

use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================
// LLM 配置
// ============================================================

/// LLM 提供商配置
#[derive(Debug, Clone)]
pub struct LlmConfig {
    /// API 端点（OpenAI-compatible），如 https://api.openai.com/v1/chat/completions
    pub api_url: String,
    /// API Key
    pub api_key: String,
    /// 模型名称，如 gpt-4o, claude-3-5-sonnet, qwen2.5 等
    pub model: String,
    /// 最大 Token 数
    pub max_tokens: u32,
    /// 温度（0.0 ~ 2.0）
    pub temperature: f32,
    /// 请求超时（秒）
    pub timeout_secs: u64,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            api_url: "https://api.openai.com/v1/chat/completions".to_string(),
            api_key: String::new(),
            model: "gpt-4o-mini".to_string(),
            max_tokens: 4096,
            temperature: 0.3,
            timeout_secs: 30,
        }
    }
}

impl LlmConfig {
    /// 从环境变量加载配置
    pub fn from_env() -> Self {
        let api_url = std::env::var("LLM_API_URL")
            .or_else(|_| std::env::var("OPENAI_API_BASE"))
            .unwrap_or_else(|_| Self::default().api_url);

        let api_key = std::env::var("LLM_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .unwrap_or_default();

        let model = std::env::var("LLM_MODEL").unwrap_or_else(|_| Self::default().model);

        let max_tokens = std::env::var("LLM_MAX_TOKENS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(Self::default().max_tokens);

        let temperature = std::env::var("LLM_TEMPERATURE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(Self::default().temperature);

        let timeout_secs = std::env::var("LLM_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(Self::default().timeout_secs);

        Self {
            api_url,
            api_key,
            model,
            max_tokens,
            temperature,
            timeout_secs,
        }
    }

    /// 是否已配置（有 API Key 即视为已配置）
    pub fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }
}

// ============================================================
// LLM 客户端
// ============================================================

/// OpenAI-compatible 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenAI-compatible 聊天请求
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<ResponseFormat>,
}

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

/// OpenAI-compatible 聊天响应
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

/// LLM 客户端
pub struct LlmClient {
    config: LlmConfig,
    client: reqwest::Client,
}

impl LlmClient {
    pub fn new(config: LlmConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .unwrap_or_default();
        Self { config, client }
    }

    /// 发送聊天请求
    pub async fn chat(&self, system_prompt: &str, user_message: &str) -> Result<String, String> {
        if !self.config.is_configured() {
            return Err("LLM 未配置 (缺少 API Key)".to_string());
        }

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_message.to_string(),
            },
        ];

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            response_format: None,
        };

        let resp = self
            .client
            .post(&self.config.api_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("LLM 请求失败: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("LLM API 返回错误 (HTTP {}): {}", status, body));
        }

        let response: ChatResponse = resp
            .json()
            .await
            .map_err(|e| format!("解析 LLM 响应失败: {e}"))?;

        response
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| "LLM 返回空响应".to_string())
    }

    /// 发送多轮对话请求
    pub async fn chat_with_history(
        &self,
        system_prompt: &str,
        history: &[(String, String)], // (role, content) pairs
        user_message: &str,
    ) -> Result<String, String> {
        if !self.config.is_configured() {
            return Err("LLM 未配置 (缺少 API Key)".to_string());
        }

        let mut messages = Vec::with_capacity(history.len() + 2);
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        });

        for (role, content) in history {
            messages.push(ChatMessage {
                role: role.clone(),
                content: content.clone(),
            });
        }

        messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
        });

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            response_format: None,
        };

        let resp = self
            .client
            .post(&self.config.api_url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("LLM 请求失败: {e}"))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("LLM API 返回错误 (HTTP {}): {}", status, body));
        }

        let response: ChatResponse = resp
            .json()
            .await
            .map_err(|e| format!("解析 LLM 响应失败: {e}"))?;

        response
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| "LLM 返回空响应".to_string())
    }
}

// ============================================================
// 意图解析：LLM 驱动的工作流生成
// ============================================================

/// 解析后的工作流节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedNode {
    pub tool_name: String,
    pub server_id: String,
    pub label: String,
    pub position_x: f64,
    pub position_y: f64,
    pub inputs: Value,
}

/// 解析后的工作流连线
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedEdge {
    pub source_index: usize,
    pub target_index: usize,
}

/// 解析后的工作流
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedWorkflow {
    pub name: String,
    pub description: String,
    pub nodes: Vec<ParsedNode>,
    pub edges: Vec<ParsedEdge>,
}

/// 构建 LLM 意图解析的系统提示词
pub fn build_intent_system_prompt(available_tools: &[ToolInfo]) -> String {
    let tools_desc: Vec<String> = available_tools
        .iter()
        .map(|t| {
            format!(
                "- {}.{} (server: {}, desc: {})",
                t.server_id, t.name, t.server_id, t.description
            )
        })
        .collect();

    format!(
        r#"你是一个工作流编排专家。根据用户的自然语言描述，生成一个工作流定义。

## 可用工具
{}

## 输出格式
请严格输出 JSON，格式如下：
```json
{{
  "name": "工作流名称",
  "description": "工作流描述",
  "nodes": [
    {{
      "tool_name": "工具名称",
      "server_id": "MCP服务器ID",
      "label": "节点显示名称",
      "position_x": 100.0,
      "position_y": 100.0,
      "inputs": {{}}
    }}
  ],
  "edges": [
    {{
      "source_index": 0,
      "target_index": 1
    }}
  ]
}}
```

## 规则
1. 每个节点必须使用可用工具列表中的工具
2. 节点的 position_x 按顺序递增（间隔 300），position_y 保持 100
3. 节点数不超过 5 个
4. 连线必须合法，source_index 和 target_index 指向 nodes 数组索引
5. 如果用户的描述中没有明确提到某个工具，不要凭空添加
6. 只输出 JSON，不要输出任何其他内容"#,
        tools_desc.join("\n")
    )
}

/// 可用的 MCP 工具信息
#[derive(Debug, Clone)]
pub struct ToolInfo {
    pub server_id: String,
    pub name: String,
    pub description: String,
}

/// 使用 LLM 解析用户意图，生成工作流定义
pub async fn parse_intent_with_llm(
    user_text: &str,
    available_tools: &[ToolInfo],
) -> Result<ParsedWorkflow, String> {
    let config = LlmConfig::from_env();
    if !config.is_configured() {
        return Err("LLM 未配置".to_string());
    }

    let client = LlmClient::new(config);
    let system_prompt = build_intent_system_prompt(available_tools);

    let response = client.chat(&system_prompt, user_text).await?;

    // 尝试从响应中提取 JSON（可能被 markdown 代码块包裹）
    let json_str = extract_json_from_response(&response);

    let parsed: ParsedWorkflow = serde_json::from_str(&json_str)
        .map_err(|e| format!("LLM 返回的工作流格式无效: {e}\n原始响应: {response}"))?;

    Ok(parsed)
}

/// 从 LLM 响应中提取 JSON（处理 markdown 代码块包裹的情况）
fn extract_json_from_response(response: &str) -> String {
    let trimmed = response.trim();

    // 尝试提取 ```json ... ``` 代码块
    if let Some(start) = trimmed.find("```json") {
        let after_start = &trimmed[start + 7..];
        if let Some(end) = after_start.find("```") {
            return after_start[..end].trim().to_string();
        }
    }

    // 尝试提取 ``` ... ``` 代码块
    if let Some(start) = trimmed.find("```") {
        let after_start = &trimmed[start + 3..];
        if let Some(end) = after_start.find("```") {
            return after_start[..end].trim().to_string();
        }
    }

    // 直接返回原始文本（可能是纯 JSON）
    trimmed.to_string()
}

// ============================================================
// 对话上下文管理
// ============================================================

/// 对话轮次
#[derive(Debug, Clone)]
pub struct ConversationTurn {
    pub role: String, // "user" | "assistant"
    pub content: String,
}

/// 对话会话
#[derive(Debug, Clone)]
pub struct ConversationSession {
    #[allow(dead_code)]
    pub session_id: String,
    pub turns: Vec<ConversationTurn>,
    pub current_workflow: Option<ParsedWorkflow>,
}

impl ConversationSession {
    pub fn new() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            turns: Vec::new(),
            current_workflow: None,
        }
    }

    pub fn add_user_message(&mut self, content: &str) {
        self.turns.push(ConversationTurn {
            role: "user".to_string(),
            content: content.to_string(),
        });
    }

    pub fn add_assistant_message(&mut self, content: &str) {
        self.turns.push(ConversationTurn {
            role: "assistant".to_string(),
            content: content.to_string(),
        });
    }

    pub fn to_history(&self) -> Vec<(String, String)> {
        self.turns
            .iter()
            .map(|t| (t.role.clone(), t.content.clone()))
            .collect()
    }
}

/// 使用多轮对话细化工作流
pub async fn refine_workflow_with_llm(
    session: &ConversationSession,
    refinement_text: &str,
    available_tools: &[ToolInfo],
) -> Result<ParsedWorkflow, String> {
    let config = LlmConfig::from_env();
    if !config.is_configured() {
        return Err("LLM 未配置".to_string());
    }

    let client = LlmClient::new(config);

    let system_prompt = format!(
        r#"你是一个工作流编排专家。用户正在通过多轮对话细化一个工作流。

## 当前工作流
{}

## 可用工具
{}

## 用户要求
{}

## 输出格式
请严格输出更新后的完整工作流 JSON（不是 diff），格式如下：
```json
{{
  "name": "工作流名称",
  "description": "工作流描述",
  "nodes": [...],
  "edges": [...]
}}
```

只输出 JSON，不要输出任何其他内容。"#,
        serde_json::to_string_pretty(&session.current_workflow).unwrap_or_default(),
        available_tools
            .iter()
            .map(|t| format!("- {}.{}", t.server_id, t.name))
            .collect::<Vec<_>>()
            .join("\n"),
        refinement_text,
    );

    let response = client
        .chat_with_history(&system_prompt, &session.to_history(), refinement_text)
        .await?;

    let json_str = extract_json_from_response(&response);
    let parsed: ParsedWorkflow = serde_json::from_str(&json_str)
        .map_err(|e| format!("LLM 返回的工作流格式无效: {e}\n原始响应: {response}"))?;

    Ok(parsed)
}

// ============================================================
// 工具推荐
// ============================================================

/// 根据用户意图推荐最相关的 MCP 工具
pub async fn recommend_tools(
    user_text: &str,
    available_tools: &[ToolInfo],
) -> Result<Vec<String>, String> {
    let config = LlmConfig::from_env();
    if !config.is_configured() {
        // 无 LLM 时使用关键词匹配推荐
        return Ok(keyword_recommend_tools(user_text, available_tools));
    }

    let client = LlmClient::new(config);

    let tools_desc: Vec<String> = available_tools
        .iter()
        .enumerate()
        .map(|(i, t)| format!("{}. {}.{} - {}", i, t.server_id, t.name, t.description))
        .collect();

    let system_prompt = format!(
        r#"你是一个 MCP 工具推荐专家。根据用户的意图描述，推荐最相关的工具。

## 可用工具
{}

## 输出格式
只输出推荐的工具索引列表（JSON 数组），如 [0, 3, 5]。推荐不超过 5 个工具。
只输出 JSON 数组，不要输出任何其他内容。"#,
        tools_desc.join("\n")
    );

    let response = client.chat(&system_prompt, user_text).await?;
    let json_str = extract_json_from_response(&response);

    let indices: Vec<usize> =
        serde_json::from_str(&json_str).map_err(|e| format!("工具推荐响应格式无效: {e}"))?;

    let recommended: Vec<String> = indices
        .into_iter()
        .filter_map(|i| {
            available_tools
                .get(i)
                .map(|t| format!("{}.{}", t.server_id, t.name))
        })
        .collect();

    Ok(recommended)
}

/// 关键词匹配的简单工具推荐（离线回退）
fn keyword_recommend_tools(user_text: &str, available_tools: &[ToolInfo]) -> Vec<String> {
    let text_lower = user_text.to_lowercase();
    let mut scored: Vec<(&ToolInfo, usize)> = available_tools
        .iter()
        .map(|t| {
            let mut score = 0usize;
            for keyword in t.name.split('_').chain(t.description.split_whitespace()) {
                if text_lower.contains(&keyword.to_lowercase()) {
                    score += 1;
                }
            }
            (t, score)
        })
        .collect();

    scored.sort_by(|a, b| b.1.cmp(&a.1));
    scored
        .into_iter()
        .filter(|(_, score)| *score > 0)
        .take(5)
        .map(|(t, _)| format!("{}.{}", t.server_id, t.name))
        .collect()
}

// ============================================================
// 测试
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_config_default() {
        let config = LlmConfig::default();
        assert!(!config.is_configured()); // 默认无 API Key
        assert_eq!(config.model, "gpt-4o-mini");
    }

    #[test]
    fn test_llm_config_from_env_empty() {
        // 清除环境变量以确保测试隔离
        std::env::remove_var("LLM_API_KEY");
        std::env::remove_var("OPENAI_API_KEY");
        let config = LlmConfig::from_env();
        assert!(!config.is_configured());
    }

    #[test]
    fn test_extract_json_from_response_plain() {
        let input = r#"{"name": "test", "nodes": []}"#;
        let result = extract_json_from_response(input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_extract_json_from_response_code_block() {
        let input = "```json\n{\"name\": \"test\"}\n```";
        let result = extract_json_from_response(input);
        assert_eq!(result, "{\"name\": \"test\"}");
    }

    #[test]
    fn test_extract_json_from_response_code_block_no_lang() {
        let input = "```\n{\"name\": \"test\"}\n```";
        let result = extract_json_from_response(input);
        assert_eq!(result, "{\"name\": \"test\"}");
    }

    #[test]
    fn test_keyword_recommend_tools() {
        let tools = vec![
            ToolInfo {
                server_id: "s1".to_string(),
                name: "http_request".to_string(),
                description: "Make HTTP requests".to_string(),
            },
            ToolInfo {
                server_id: "s2".to_string(),
                name: "file_writer".to_string(),
                description: "Write files to disk".to_string(),
            },
            ToolInfo {
                server_id: "s3".to_string(),
                name: "db_query".to_string(),
                description: "Query database".to_string(),
            },
        ];
        let result = keyword_recommend_tools("download a file from http and save it", &tools);
        assert!(!result.is_empty());
        assert!(result.iter().any(|r| r.contains("http_request")));
        assert!(result.iter().any(|r| r.contains("file_writer")));
    }

    #[test]
    fn test_conversation_session() {
        let mut session = ConversationSession::new();
        session.add_user_message("帮我创建一个HTTP请求工作流");
        session.add_assistant_message("好的，已创建。需要修改吗？");
        assert_eq!(session.turns.len(), 2);
        assert_eq!(session.to_history().len(), 2);
    }

    #[test]
    fn test_build_intent_system_prompt() {
        let tools = vec![ToolInfo {
            server_id: "mcp-http".to_string(),
            name: "http_request".to_string(),
            description: "发送HTTP请求".to_string(),
        }];
        let prompt = build_intent_system_prompt(&tools);
        assert!(prompt.contains("mcp-http.http_request"));
        assert!(prompt.contains("工作流编排专家"));
    }
}
