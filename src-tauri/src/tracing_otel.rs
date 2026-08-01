// ============================================================
// OpenTelemetry 分布式追踪模块
//
// 功能：
// - 初始化 OTLP HTTP/protobuf 导出器（兼容 Jaeger/Tempo）
// - 将 tracing Span 桥接到 OpenTelemetry Span
// - 工作流、节点、MCP 工具调用级别的 Span 插桩
// - trace_id 通过 tracing 上下文传播
// ============================================================

use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_otlp::Protocol;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{Config, TracerProvider};
use opentelemetry_sdk::Resource;
use std::sync::OnceLock;

/// 全局 TracerProvider 持有者，确保在程序生命周期内不被 drop
static TRACER_PROVIDER: OnceLock<TracerProvider> = OnceLock::new();

/// 初始化 OpenTelemetry 追踪系统
///
/// # Arguments
/// * `endpoint` - OTLP HTTP 端点，如 "http://localhost:4318/v1/traces"
/// * `service_name` - 服务名称，默认为 "mcp-fusion"
///
/// 如果 endpoint 为空字符串，则不初始化（追踪功能禁用）。
///
/// 返回 `tracing_opentelemetry::OpenTelemetryLayer`，需要注册到 tracing subscriber。
/// 返回 None 表示追踪功能未启用。
pub fn init_tracer(
    endpoint: &str,
    service_name: &str,
) -> Option<
    tracing_opentelemetry::OpenTelemetryLayer<
        tracing_subscriber::Registry,
        opentelemetry_sdk::trace::Tracer,
    >,
> {
    if endpoint.is_empty() {
        tracing::info!("OpenTelemetry 追踪未启用（未配置 OTLP endpoint）");
        return None;
    }

    let exporter = match opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(endpoint)
        .with_protocol(Protocol::HttpBinary)
        .build_span_exporter()
    {
        Ok(e) => e,
        Err(e) => {
            tracing::warn!("无法创建 OTLP exporter: {e}，追踪功能禁用");
            return None;
        }
    };

    let resource = Resource::new(vec![KeyValue::new(
        "service.name",
        service_name.to_string(),
    )]);

    let config = Config::default().with_resource(resource);

    let provider = TracerProvider::builder()
        .with_config(config)
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .build();

    let tracer = provider.tracer("mcp-fusion");
    let layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // 存储 provider 防止被 drop
    let _ = TRACER_PROVIDER.set(provider);

    tracing::info!(
        endpoint = %endpoint,
        service = %service_name,
        "OpenTelemetry 追踪已启用"
    );

    Some(layer)
}

/// 强制刷新待发送的追踪数据
///
/// 建议在程序退出前调用，确保所有 Span 数据已发送。
pub fn shutdown_tracer() {
    if let Some(provider) = TRACER_PROVIDER.get() {
        let _ = provider.force_flush();
    }
    opentelemetry::global::shutdown_tracer_provider();
}
