pub mod sse;
pub mod stdio;
pub mod streamable_http;

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;

use anyhow::Result;
use serde_json::Value;

use stdio::{McpToolInfo, ToolCallResult};

/// 统一的 MCP 客户端，封装三种传输协议
pub enum McpClient {
    Stdio(stdio::StdioClient),
    Sse(sse::SseClient),
    StreamableHttp(streamable_http::StreamableHttpClient),
}

impl McpClient {
    pub async fn list_tools(&mut self) -> Result<Vec<McpToolInfo>> {
        match self {
            McpClient::Stdio(client) => client.list_tools().await,
            McpClient::Sse(client) => client.list_tools().await,
            McpClient::StreamableHttp(client) => client.list_tools().await,
        }
    }

    pub async fn call_tool(&mut self, tool_name: &str, arguments: Value) -> Result<ToolCallResult> {
        match self {
            McpClient::Stdio(client) => client.call_tool(tool_name, arguments).await,
            McpClient::Sse(client) => client.call_tool(tool_name, arguments).await,
            McpClient::StreamableHttp(client) => client.call_tool(tool_name, arguments).await,
        }
    }
}

/// 根据服务器协议创建对应的 MCP 客户端
pub async fn create_mcp_client(
    protocol: &str,
    command: &str,
    args: &[String],
    env: &std::collections::HashMap<String, String>,
    endpoint: &str,
) -> Result<McpClient, String> {
    match protocol {
        "stdio" => {
            let env_vec: Vec<(String, String)> =
                env.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            let client = stdio::StdioClient::connect(command, args, &env_vec)
                .await
                .map_err(|e| format!("连接 stdio MCP 服务器失败: {e}"))?;
            Ok(McpClient::Stdio(client))
        }
        "sse" => {
            if endpoint.is_empty() {
                return Err("SSE 协议需要提供 endpoint".to_string());
            }
            let client = sse::SseClient::connect(endpoint)
                .await
                .map_err(|e| format!("连接 SSE MCP 服务器失败: {e}"))?;
            Ok(McpClient::Sse(client))
        }
        "streamable-http" | "streamable_http" => {
            if endpoint.is_empty() {
                return Err("streamable-http 协议需要提供 endpoint".to_string());
            }
            let client = streamable_http::StreamableHttpClient::connect(endpoint)
                .await
                .map_err(|e| format!("连接 streamable-http MCP 服务器失败: {e}"))?;
            Ok(McpClient::StreamableHttp(client))
        }
        other => Err(format!("不支持的协议: {}", other)),
    }
}
