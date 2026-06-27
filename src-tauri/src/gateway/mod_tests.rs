use super::*;
use crate::gateway::stdio::JsonRpcRequest;

// ============================================================
// JSON-RPC 消息类型测试
// ============================================================

#[test]
fn test_json_rpc_request_serialization() {
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: 1,
        method: "tools/list".to_string(),
        params: serde_json::json!({}),
    };

    let serialized = serde_json::to_string(&request).unwrap();
    assert!(serialized.contains("2.0"));
    assert!(serialized.contains("tools/list"));
    assert!(serialized.contains("\"id\":1"));
}

#[test]
fn test_json_rpc_response_deserialization() {
    let json = r#"{"jsonrpc":"2.0","id":1,"result":{"tools":[]}}"#;
    let response: crate::gateway::stdio::JsonRpcResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.id, Some(1));
    assert!(response.result.is_some());
    assert!(response.error.is_none());
}

#[test]
fn test_json_rpc_error_deserialization() {
    let json = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32601,"message":"Method not found"}}"#;
    let response: crate::gateway::stdio::JsonRpcResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.id, Some(1));
    assert!(response.error.is_some());
    assert_eq!(response.error.as_ref().unwrap().code, -32601);
}

#[test]
fn test_mcp_tool_info_deserialization() {
    let json = r#"{
        "tools": [{
            "name": "fetch",
            "description": "Fetch a URL",
            "inputSchema": {"type": "object", "properties": {}},
            "outputSchema": {"type": "object"}
        }]
    }"#;

    let result: serde_json::Value = serde_json::from_str(json).unwrap();
    let tools = result["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0]["name"], "fetch");
}

// ============================================================
// create_mcp_client 工厂函数测试
// ============================================================

#[tokio::test]
async fn test_create_mcp_client_unknown_protocol() {
    let result = create_mcp_client(
        "unknown",
        "",
        &[],
        &std::collections::HashMap::new(),
        "",
    )
    .await;
    assert!(result.is_err());
    match result {
        Err(e) => assert!(e.contains("不支持的协议")),
        _ => unreachable!(),
    }
}

#[tokio::test]
async fn test_create_mcp_client_sse_missing_endpoint() {
    let result = create_mcp_client(
        "sse",
        "",
        &[],
        &std::collections::HashMap::new(),
        "",
    )
    .await;
    assert!(result.is_err());
    match result {
        Err(e) => assert!(e.contains("endpoint")),
        _ => unreachable!(),
    }
}

#[tokio::test]
async fn test_create_mcp_client_streamable_http_missing_endpoint() {
    let result = create_mcp_client(
        "streamable-http",
        "",
        &[],
        &std::collections::HashMap::new(),
        "",
    )
    .await;
    assert!(result.is_err());
    match result {
        Err(e) => assert!(e.contains("endpoint")),
        _ => unreachable!(),
    }
}

// ============================================================
// McpClient 枚举匹配测试
// ============================================================

#[test]
fn test_mcp_client_error_handling() {
    // 验证 create_mcp_client 风格的错误处理
    let err: Result<(), String> = Err("不支持的协议: test".to_string());
    assert!(err.is_err());

    let msg = match err {
        Err(e) => e,
        Ok(_) => unreachable!(),
    };
    assert!(msg.contains("不支持的协议"));
}