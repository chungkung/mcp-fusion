use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
use tokio::sync::{mpsc, oneshot, Mutex};

use super::stdio::{JsonRpcResponse, McpToolInfo, ToolCallResult};

// ============================================================
// MCP SSE 客户端
// ============================================================
// SSE 传输协议：
// 1. GET /sse → 服务器返回 SSE 流，首个事件包含 message endpoint URL
// 2. POST {endpoint} → 客户端发送 JSON-RPC 请求
// 3. SSE 流 → 服务器推送 JSON-RPC 响应

pub struct SseClient {
    base_url: String,
    message_endpoint: String,
    http_client: Client,
    next_id: u64,
    initialized: bool,
    /// 接收 SSE 事件的通道
    event_rx: mpsc::UnboundedReceiver<SseEvent>,
    /// 并发请求响应分发器：id → oneshot sender
    pending_requests: Arc<Mutex<HashMap<u64, oneshot::Sender<JsonRpcResponse>>>>,
}

#[derive(Debug, Clone)]
struct SseEvent {
    event_type: String,
    data: String,
}

impl SseClient {
    pub async fn connect(base_url: &str) -> Result<Self, String> {
        let http_client = Client::new();
        let sse_url = format!("{}/sse", base_url.trim_end_matches('/'));

        let response = http_client
            .get(&sse_url)
            .header("Accept", "text/event-stream")
            .send()
            .await
            .map_err(|e| format!("SSE 连接失败: {e}"))?;

        if !response.status().is_success() {
            return Err(format!("SSE 连接失败，HTTP 状态: {}", response.status()));
        }

        let (tx, rx) = mpsc::unbounded_channel();
        let pending_requests: Arc<Mutex<HashMap<u64, oneshot::Sender<JsonRpcResponse>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending = pending_requests.clone();

        // 后台任务读取 SSE 流
        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            let mut buffer = String::new();
            let mut current_event = String::new();
            let mut current_data = String::new();

            use futures::StreamExt;
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));
                        while let Some(line_end) = buffer.find('\n') {
                            let line = buffer[..line_end].trim().to_string();
                            buffer = buffer[line_end + 1..].to_string();

                            if line.is_empty() {
                                // 空行表示事件结束
                                if !current_data.is_empty() {
                                    let event = SseEvent {
                                        event_type: current_event.clone(),
                                        data: current_data.clone(),
                                    };
                                    // 尝试解析为 JSON-RPC 响应并按 ID 分发
                                    if let Ok(response) =
                                        serde_json::from_str::<JsonRpcResponse>(&event.data)
                                    {
                                        if let Some(id) = response.id {
                                            let mut pending = pending.lock().await;
                                            if let Some(tx) = pending.remove(&id) {
                                                let _ = tx.send(response);
                                                current_event.clear();
                                                current_data.clear();
                                                continue;
                                            }
                                        }
                                    }
                                    if tx.send(event).is_err() {
                                        return; // 接收端已关闭
                                    }
                                }
                                current_event.clear();
                                current_data.clear();
                            } else if let Some(field_data) = line.strip_prefix("event:") {
                                current_event = field_data.trim().to_string();
                            } else if let Some(field_data) = line.strip_prefix("data:") {
                                if !current_data.is_empty() {
                                    current_data.push('\n');
                                }
                                current_data.push_str(field_data.trim());
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("SSE 流读取错误: {}", crate::crypto::sanitize_log(&e.to_string()));
                        break;
                    }
                }
            }
        });

        let mut client = Self {
            base_url: base_url.to_string(),
            message_endpoint: String::new(),
            http_client,
            next_id: 1,
            initialized: false,
            event_rx: rx,
            pending_requests: pending_requests.clone(),
        };

        // 等待 endpoint 事件
        client.wait_for_endpoint().await?;
        client.initialize().await.map_err(|e| format!("SSE 初始化失败: {e}"))?;

        Ok(client)
    }

    /// 等待 SSE 流中的 endpoint 事件
    async fn wait_for_endpoint(&mut self) -> Result<(), String> {
        use tokio::time::{sleep, Duration};

        let start = std::time::Instant::now();
        let timeout = Duration::from_secs(10);

        loop {
            if start.elapsed() > timeout {
                return Err("等待 SSE endpoint 超时".to_string());
            }

            match self.event_rx.try_recv() {
                Ok(event) => {
                    if event.event_type == "endpoint" {
                        self.message_endpoint = event.data.trim().to_string();
                        return Ok(());
                    }
                }
                Err(mpsc::error::TryRecvError::Empty) => {
                    sleep(Duration::from_millis(100)).await;
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    return Err("SSE 连接已断开".to_string());
                }
            }
        }
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

        let response = self.send_request("initialize", params).await?;

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
    ) -> Result<JsonRpcResponse> {
        let id = self.next_id;
        self.next_id += 1;

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        let post_url = if self.message_endpoint.starts_with("http") {
            self.message_endpoint.clone()
        } else {
            format!(
                "{}{}",
                self.base_url.trim_end_matches('/'),
                self.message_endpoint
            )
        };

        // 注册 oneshot 通道，用于接收匹配 ID 的响应
        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        let response = self.http_client
            .post(&post_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                // 请求失败时清理 pending
                let pending = self.pending_requests.clone();
                tokio::spawn(async move {
                    pending.lock().await.remove(&id);
                });
                anyhow::anyhow!("发送请求失败: {e}")
            })?;

        if !response.status().is_success() {
            let pending = self.pending_requests.clone();
            tokio::spawn(async move {
                pending.lock().await.remove(&id);
            });
            return Err(anyhow::anyhow!(
                "HTTP 请求失败，状态码: {}",
                response.status()
            ));
        }

        // 等待 oneshot 通道返回匹配的响应
        match tokio::time::timeout(std::time::Duration::from_secs(30), rx).await {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err(anyhow::anyhow!("响应通道已关闭: {}", method)),
            Err(_) => {
                // 超时清理
                let pending = self.pending_requests.clone();
                tokio::spawn(async move {
                    pending.lock().await.remove(&id);
                });
                Err(anyhow::anyhow!("等待响应超时: {}", method))
            }
        }
    }

    async fn send_notification(&mut self, method: &str, params: Value) -> Result<()> {
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });

        let post_url = if self.message_endpoint.starts_with("http") {
            self.message_endpoint.clone()
        } else {
            format!(
                "{}{}",
                self.base_url.trim_end_matches('/'),
                self.message_endpoint
            )
        };

        self.http_client
            .post(&post_url)
            .json(&notification)
            .send()
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

impl Drop for SseClient {
    fn drop(&mut self) {
        // SSE 连接在后台任务中管理，Drop 时接收端关闭会自动结束
    }
}