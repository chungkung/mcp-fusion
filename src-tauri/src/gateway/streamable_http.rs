use anyhow::Result;
use reqwest::Client;
use serde_json::Value;

use super::stdio::{McpToolInfo, ToolCallResult};

// ============================================================
// MCP Streamable HTTP 客户端
// ============================================================
// Streamable HTTP 传输协议：
// 1. POST /mcp → 发送 JSON-RPC 请求
// 2. 支持 Accept: text/event-stream 进行流式响应
// 3. 也支持 Accept: application/json 进行普通请求-响应

pub struct StreamableHttpClient {
    endpoint: String,
    http_client: Client,
    next_id: u64,
    initialized: bool,
    session_id: Option<String>,
}

impl StreamableHttpClient {
    pub async fn connect(endpoint: &str) -> Result<Self, String> {
        let http_client = Client::new();

        let mut client = Self {
            endpoint: endpoint.trim_end_matches('/').to_string(),
            http_client,
            next_id: 1,
            initialized: false,
            session_id: None,
        };

        client.initialize().await.map_err(|e| format!("streamable-http 初始化失败: {e}"))?;

        Ok(client)
    }

    async fn initialize(&mut self) -> Result<()> {
        let params = serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {
                "name": "mcp-fusion",
                "version": "0.1.0"
            }
        });

        let id = self.next_id;
        self.next_id += 1;

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": "initialize",
            "params": params,
        });

        let http_response = self
            .http_client
            .post(&self.endpoint)
            .header("Accept", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("HTTP 请求失败: {e}"))?;

        // 捕获 Mcp-Session-Id 响应头
        if let Some(sid) = http_response.headers().get("mcp-session-id") {
            self.session_id = Some(sid.to_str().unwrap_or("").to_string());
        }

        if !http_response.status().is_success() {
            return Err(anyhow::anyhow!(
                "初始化失败，HTTP 状态码: {}",
                http_response.status()
            ));
        }

        let response: super::stdio::JsonRpcResponse = http_response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("解析初始化响应失败: {e}"))?;

        if let Some(err) = response.error {
            return Err(anyhow::anyhow!("初始化失败: {}", err.message));
        }

        self.send_notification("initialized", serde_json::json!({}))
            .await?;

        self.initialized = true;
        Ok(())
    }

    async fn send_request(
        &mut self,
        method: &str,
        params: Value,
    ) -> Result<super::stdio::JsonRpcResponse> {
        let id = self.next_id;
        self.next_id += 1;

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        let mut req = self
            .http_client
            .post(&self.endpoint)
            .header("Accept", "application/json")
            .json(&request);

        if let Some(ref sid) = self.session_id {
            req = req.header("Mcp-Session-Id", sid);
        }

        let response = req
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("HTTP 请求失败: {e}"))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "HTTP 请求失败，状态码: {}",
                response.status()
            ));
        }

        let response_body: super::stdio::JsonRpcResponse = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("解析响应 JSON 失败: {e}"))?;

        if response_body.id != Some(id) {
            return Err(anyhow::anyhow!(
                "响应 ID 不匹配: 期望 {}, 收到 {:?}",
                id,
                response_body.id
            ));
        }

        Ok(response_body)
    }

    async fn send_notification(&mut self, method: &str, params: Value) -> Result<()> {
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });

        let mut req = self.http_client
            .post(&self.endpoint)
            .json(&notification);
        if let Some(ref sid) = self.session_id {
            req = req.header("Mcp-Session-Id", sid);
        }

        req.send()
            .await
            .map_err(|e| anyhow::anyhow!("发送通知失败: {e}"))?;

        Ok(())
    }

    pub async fn list_tools(&mut self) -> Result<Vec<McpToolInfo>> {
        let response = self
            .send_request("tools/list", serde_json::json!({}))
            .await?;

        match response.result {
            Some(result) => {
                let tools: Vec<McpToolInfo> = serde_json::from_value(
                    result.get("tools").cloned().unwrap_or(serde_json::json!([])),
                )
                .map_err(|e| anyhow::anyhow!("解析工具列表失败: {e}"))?;
                Ok(tools)
            }
            None => {
                if let Some(err) = response.error {
                    Err(anyhow::anyhow!("列出工具失败: {}", err.message))
                } else {
                    Err(anyhow::anyhow!("列出工具: 未知错误"))
                }
            }
        }
    }

    pub async fn call_tool(
        &mut self,
        tool_name: &str,
        arguments: Value,
    ) -> Result<ToolCallResult> {
        let params = serde_json::json!({
            "name": tool_name,
            "arguments": arguments,
        });

        let response = self.send_request("tools/call", params).await?;

        match response.result {
            Some(result) => {
                let tool_result: ToolCallResult = serde_json::from_value(result)
                    .map_err(|e| anyhow::anyhow!("解析工具调用结果失败: {e}"))?;
                Ok(tool_result)
            }
            None => {
                if let Some(err) = response.error {
                    Err(anyhow::anyhow!("工具调用失败: {}", err.message))
                } else {
                    Err(anyhow::anyhow!("工具调用: 未知错误"))
                }
            }
        }
    }
}