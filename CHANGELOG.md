# Changelog

All notable changes to MCP Fusion will be documented in this file.

## [0.1.0] - Unreleased

### Added

#### Core Platform
- MCP stdio transport protocol support
- MCP SSE (Server-Sent Events) transport protocol support
- MCP Streamable HTTP transport protocol support
- MCP server CRUD management with pagination
- Automatic tool discovery and schema introspection
- Server connection health checking (ping)

#### Workflow Orchestration
- Visual canvas editor with drag-and-drop (React Flow)
- Code mode for JSON/YAML workflow editing
- Topological sort scheduler with parallel execution
- Node-level data passing with template reference resolution
- Workflow execution with real-time status events
- Idempotency keys for safe retries
- Workflow lock/release mechanism
- Retry failed workflows from checkpoint

#### LLM Integration
- OpenAI-compatible API integration
- Natural language to workflow generation
- Multi-turn conversation workflow refinement
- Automatic MCP tool recommendation
- Keyword matching fallback when LLM unavailable
- Support for local models via Ollama

#### Observability
- Prometheus metrics (Counter, Histogram, Gauge)
- Workflow execution metrics
- Tool call recording
- Server connection status tracking
- OpenTelemetry distributed tracing (OTLP)
- Node-level span instrumentation
- Trace ID propagation
- Structured JSON logging with tracing-subscriber

#### Plugin Marketplace
- Remote template registry (GitHub/GitLab)
- Category filtering and keyword search
- One-click template installation
- Version management with update notifications
- 8 built-in templates for offline fallback

#### Security
- RBAC (Admin, Developer, Viewer roles)
- API Key authentication with encrypted storage
- AES-256-GCM encryption for sensitive data
- Audit logging with cryptographic chain verification
- Rate limiter with configurable rules
- Circuit breaker pattern for fault isolation
- Input sanitization for sensitive data in logs

#### Desktop Application
- Tauri 2 desktop framework
- Cross-platform packaging (Windows NSIS, macOS DMG, Linux AppImage/DEB/RPM)
- Auto-update mechanism (Tauri updater)
- System tray integration
- Health check endpoint
- Database backup and restore

#### Frontend
- Responsive Settings page with 5 tabs (General, MCP, Permissions, System, About)
- Execution history with pagination and status filters
- Audit log viewer with search
- Marketplace browser with category filtering
- Dark/Light theme support
- Page transition animations