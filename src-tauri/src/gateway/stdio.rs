use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

// ============================================================
// JSON-RPC 消息类型
// ============================================================

#[derive(Debug, Serialize)]
pub(crate) struct JsonRpcRequest {
    pub(crate) jsonrpc: String,
    pub(crate) id: u64,
    pub(crate) method: String,
    pub(crate) params: Value,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse {
    #[allow(dead_code)]
    pub jsonrpc: String,
    pub id: Option<u64>,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    #[allow(dead_code)]
    pub(crate) code: i64,
    pub message: String,
}

// ============================================================
// MCP 工具结果
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolInfo {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResult {
    pub content: Vec<ToolContent>,
    #[serde(rename = "isError")]
    pub is_error: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: Option<String>,
}

// ============================================================
// MCP Stdio 客户端
// ============================================================

pub struct StdioClient {
    child: Child,
    stdin: ChildStdin,
    reader: BufReader<ChildStdout>,
    next_id: u64,
    initialized: bool,
}

impl StdioClient {
    pub async fn connect(
        command: &str,
        args: &[String],
        env: &[(String, String)],
    ) -> Result<Self, String> {
        let mut cmd = Command::new(command);
        cmd.args(args);
        for (key, value) in env {
            cmd.env(key, value);
        }
        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("start process failed: {e}"))?;

        let stdin = child.stdin.take().ok_or("cannot get child stdin")?;
        let stdout = child.stdout.take().ok_or("cannot get child stdout")?;
        let reader = BufReader::new(stdout);

        // 启动后台任务持续读取 stderr，防止子进程阻塞
        if let Some(stderr) = child.stderr.take() {
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr);
                let mut line = String::new();
                loop {
                    line.clear();
                    match reader.read_line(&mut line).await {
                        Ok(0) => break, // EOF
                        Ok(_) => tracing::warn!(
                            "[MCP stderr] {}",
                            crate::crypto::sanitize_log(line.trim())
                        ),
                        Err(_) => break,
                    }
                }
            });
        }

        let mut client = Self {
            child,
            stdin,
            reader,
            next_id: 1,
            initialized: false,
        };

        client
            .initialize()
            .await
            .map_err(|e| format!("stdio 初始化失败: {e}"))?;

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

        let response = self.send_request("initialize", params).await?;

        if let Some(result) = response.result {
            let _server_info: Value = result;
        } else if let Some(err) = response.error {
            return Err(anyhow!("初始化失败: {}", err.message));
        }

        self.send_notification("initialized", serde_json::json!({}))
            .await?;

        self.initialized = true;
        Ok(())
    }

    async fn send_request(&mut self, method: &str, params: Value) -> Result<JsonRpcResponse> {
        let id = self.next_id;
        self.next_id += 1;

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.to_string(),
            params,
        };

        let mut request_str =
            serde_json::to_string(&request).map_err(|e| anyhow!("序列化请求失败: {e}"))?;
        request_str.push('\n');

        self.stdin
            .write_all(request_str.as_bytes())
            .await
            .map_err(|e| anyhow!("写入请求失败: {e}"))?;
        self.stdin
            .flush()
            .await
            .map_err(|e| anyhow!("刷新 stdin 失败: {e}"))?;

        self.read_response(id).await
    }

    async fn send_notification(&mut self, method: &str, params: Value) -> Result<()> {
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });

        let mut notif_str =
            serde_json::to_string(&notification).map_err(|e| anyhow!("序列化通知失败: {e}"))?;
        notif_str.push('\n');

        self.stdin
            .write_all(notif_str.as_bytes())
            .await
            .map_err(|e| anyhow!("写入通知失败: {e}"))?;
        self.stdin
            .flush()
            .await
            .map_err(|e| anyhow!("刷新 stdin 失败: {e}"))?;

        Ok(())
    }

    /// 使用持久化的 BufReader，避免数据丢失
    async fn read_response(&mut self, expected_id: u64) -> Result<JsonRpcResponse> {
        loop {
            let mut line = String::new();

            let n = self
                .reader
                .read_line(&mut line)
                .await
                .map_err(|e| anyhow!("读取响应失败: {e}"))?;

            if n == 0 {
                return Err(anyhow!("EOF: 子进程意外终止"));
            }

            if line.trim().is_empty() {
                continue;
            }

            let response: JsonRpcResponse =
                serde_json::from_str(&line).map_err(|e| anyhow!("解析响应 JSON 失败: {e}"))?;

            // 跳过无 id 的通知消息（如 progress 通知），继续读取
            if response.id.is_none() {
                continue;
            }

            if response.id != Some(expected_id) {
                return Err(anyhow!(
                    "响应 ID 不匹配: 期望 {}, 收到 {:?}",
                    expected_id,
                    response.id
                ));
            }

            return Ok(response);
        }
    }

    pub async fn list_tools(&mut self) -> Result<Vec<McpToolInfo>> {
        let response = self
            .send_request("tools/list", serde_json::json!({}))
            .await?;

        match response.result {
            Some(result) => {
                let tools: Vec<McpToolInfo> = serde_json::from_value(
                    result
                        .get("tools")
                        .cloned()
                        .unwrap_or(serde_json::json!([])),
                )
                .map_err(|e| anyhow!("解析工具列表失败: {e}"))?;
                Ok(tools)
            }
            None => {
                if let Some(err) = response.error {
                    Err(anyhow!("列出工具失败: {}", err.message))
                } else {
                    Err(anyhow!("列出工具: 未知错误"))
                }
            }
        }
    }

    pub async fn call_tool(&mut self, tool_name: &str, arguments: Value) -> Result<ToolCallResult> {
        let params = serde_json::json!({
            "name": tool_name,
            "arguments": arguments,
        });

        let response = self.send_request("tools/call", params).await?;

        match response.result {
            Some(result) => {
                let tool_result: ToolCallResult = serde_json::from_value(result)
                    .map_err(|e| anyhow!("解析工具调用结果失败: {e}"))?;
                Ok(tool_result)
            }
            None => {
                if let Some(err) = response.error {
                    Err(anyhow!("工具调用失败: {}", err.message))
                } else {
                    Err(anyhow!("工具调用: 未知错误"))
                }
            }
        }
    }
}

impl Drop for StdioClient {
    fn drop(&mut self) {
        // 先尝试优雅关闭
        let _ = self.child.start_kill();
        // 尝试非阻塞回收子进程，避免阻塞异步运行时
        // 如果进程尚未退出，内核会在进程结束时自动回收
        let _ = self.child.try_wait();
    }
}
