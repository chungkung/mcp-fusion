<p align="center">
  <img src="https://raw.githubusercontent.com/mcp-fusion/mcp-fusion/main/docs/assets/logo.png" alt="MCP Fusion Logo" width="200" />
</p>

<h1 align="center">MCP Fusion</h1>

<p align="center">
  <strong>Visual Orchestration Desktop App for the MCP Ecosystem</strong>
</p>

<p align="center">
  <a href="https://github.com/mcp-fusion/mcp-fusion/actions"><img src="https://github.com/mcp-fusion/mcp-fusion/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="https://github.com/mcp-fusion/mcp-fusion/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-AGPL--3.0-blue.svg" alt="License" /></a>
  <a href="https://github.com/mcp-fusion/mcp-fusion/releases"><img src="https://img.shields.io/github/v/release/mcp-fusion/mcp-fusion" alt="Release" /></a>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-brightgreen" alt="Platform" />
  <img src="https://img.shields.io/badge/rust-1.77%2B-orange" alt="Rust" />
</p>

<p align="center">
  <a href="README_zh.md">中文文档</a> ·
  <a href="#quick-start">Quick Start</a> ·
  <a href="https://mcp-fusion.app/docs">Documentation</a> ·
  <a href="#architecture">Architecture</a>
</p>

---

## What is MCP Fusion?

**MCP Fusion** is a **desktop-native visual orchestration platform** for the [Model Context Protocol (MCP)](https://modelcontextprotocol.io). It lets you compose, execute, and monitor multi-tool workflows across MCP servers — all from a local desktop app with zero cloud dependencies.

> Think **n8n-level orchestration**, **Dify-level intelligence**, running **100% locally** on your machine.

<p align="center">
  <img src="https://raw.githubusercontent.com/mcp-fusion/mcp-fusion/main/docs/assets/screenshot-canvas.png" alt="Canvas Mode" width="80%" />
</p>

---

## Why MCP Fusion?

| Feature | MCP Fusion | n8n | Dify | LangChain |
|---------|-----------|-----|------|-----------|
| **Native MCP Protocol** | ✅ Full stdio/SSE/HTTP | ❌ | ❌ | ❌ (via lib) |
| **Desktop App** | ✅ Tauri (Rust) | ❌ Web only | ❌ Web only | ❌ |
| **Visual Canvas** | ✅ React Flow | ✅ | ✅ | ❌ |
| **LLM Intent Parsing** | ✅ OpenAI/Local | ❌ | ✅ | ❌ |
| **Distributed Tracing** | ✅ OpenTelemetry | ❌ | ❌ | ✅ |
| **Offline-First** | ✅ Local SQLite | ❌ | ❌ | ❌ |
| **Plugin Marketplace** | ✅ GitHub/GitLab | ✅ | ❌ | ❌ |
| **Cross-Platform** | ✅ Win/Mac/Linux | ✅ Web | ✅ Web | ❌ |

---

## Core Features

### Visual Workflow Orchestration
- **Drag-and-drop canvas** with React Flow for intuitive workflow design
- **Code mode** for power users who prefer JSON/YAML editing
- Real-time node state visualization during execution

### MCP Protocol Fusion
- Supports all three MCP transport protocols: **stdio**, **SSE**, and **Streamable HTTP**
- Connect to any MCP-compatible server (Filesystem, GitHub, Postgres, Brave Search, etc.)
- Automatic tool discovery and schema introspection

### LLM-Powered Intent Engine
- **Natural language → Workflow** conversion via OpenAI-compatible APIs
- **Multi-turn conversation** refinement for iterative workflow building
- **Automatic tool recommendation** based on task description
- Offline fallback to keyword matching when no LLM is configured

### Production-Grade Runtime
- **Topological sort scheduler** with parallel execution support
- **Circuit breaker** pattern for fault isolation
- **Rate limiter** to prevent API abuse
- **Idempotency keys** for safe retries
- **Audit trail** with cryptographic chain verification

### Observability Built-In
- **Prometheus metrics** (workflow execution, tool calls, server status)
- **OpenTelemetry distributed tracing** (OTLP export to Jaeger/Tempo)
- **Structured JSON logging** with tracing-subscriber
- Real-time execution monitoring dashboard

### Plugin Marketplace
- **Remote template registry** (GitHub/GitLab integration)
- **One-click install** from marketplace to local workflow
- **Version management** with update notifications
- **8 built-in templates** for offline use

### Security & Compliance
- **RBAC** (Admin / Developer / Viewer roles)
- **API Key authentication** with encrypted storage (AES-256-GCM)
- **Database encryption** for sensitive server configurations
- **Audit log** with tamper-evident chain verification

---

## Quick Start

### Prerequisites
- **Node.js** >= 20
- **Rust** >= 1.77
- **Windows** / **macOS** / **Linux**

### Install from Release

Download the latest installer from [Releases](https://github.com/mcp-fusion/mcp-fusion/releases):

| Platform | Format |
|----------|--------|
| Windows | `.msi` / `.exe` (NSIS) |
| macOS | `.dmg` (Apple Silicon / Intel) |
| Linux | `.AppImage` / `.deb` / `.rpm` |

### Build from Source

```bash
# Clone the repository
git clone https://github.com/mcp-fusion/mcp-fusion.git
cd mcp-fusion

# Install dependencies
npm install

# Build and run in development mode
npm run tauri:dev

# Build for production
npm run build:win    # Windows
npm run build:mac    # macOS
npm run build:linux  # Linux
```

### Configure LLM (Optional)

```bash
# Set environment variables for LLM-powered intent parsing
export LLM_API_KEY="sk-your-openai-api-key"
export LLM_MODEL="gpt-4o-mini"

# Or use a local model via Ollama
export LLM_API_URL="http://localhost:11434/v1/chat/completions"
export LLM_MODEL="qwen2.5:7b"
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    MCP Fusion Desktop                    │
│  ┌───────────────────────────────────────────────────┐  │
│  │              Frontend (React + TypeScript)         │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │ Intent   │ │ Canvas   │ │ Marketplace      │  │  │
│  │  │ Parser   │ │ Editor   │ │ Browser          │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  └───────────────────┬───────────────────────────────┘  │
│                      │ IPC (Tauri Bridge)                │
│  ┌───────────────────┴───────────────────────────────┐  │
│  │              Backend (Rust + Tauri)                │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │ LLM      │ │ Scheduler│ │ Gateway          │  │  │
│  │  │ Engine   │ │ (Topo)   │ │ (stdio/SSE/HTTP) │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │ Metrics  │ │ Tracing  │ │ Marketplace      │  │  │
│  │  │ (Prom)   │ │ (OTel)   │ │ (GitHub/GitLab)  │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │ Auth     │ │ Circuit  │ │ Rate             │  │  │
│  │  │ (RBAC)   │ │ Breaker  │ │ Limiter          │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  │  ┌──────────────────────────────────────────────┐ │  │
│  │  │           SQLite (Local Storage)             │ │  │
│  │  └──────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Desktop Framework** | [Tauri 2](https://tauri.app) (Rust) |
| **Frontend** | React 18, TypeScript, Tailwind CSS |
| **Canvas Editor** | [React Flow](https://reactflow.dev) (xyflow) |
| **State Management** | [Zustand](https://zustand-demo.pmnd.rs) |
| **Animations** | [Framer Motion](https://www.framer.com/motion/) |
| **Database** | SQLite (via [rusqlite](https://github.com/rusqlite/rusqlite)) |
| **Metrics** | [Prometheus](https://prometheus.io) (rust client) |
| **Tracing** | [OpenTelemetry](https://opentelemetry.io) (OTLP) |
| **Encryption** | AES-256-GCM |
| **LLM** | OpenAI-compatible API |

---

## Example: 3-Node API Aggregation Workflow

```json
{
  "name": "API Data Aggregation",
  "nodes": [
    {
      "tool_name": "fetch",
      "server_id": "fetch-server",
      "label": "Fetch Weather API",
      "position_x": 100, "position_y": 100
    },
    {
      "tool_name": "fetch",
      "server_id": "fetch-server",
      "label": "Fetch Stock API",
      "position_x": 400, "position_y": 100
    },
    {
      "tool_name": "aggregate",
      "server_id": "data-tools",
      "label": "Merge Results",
      "position_x": 250, "position_y": 300
    }
  ],
  "edges": [
    { "source_index": 0, "target_index": 2 },
    { "source_index": 1, "target_index": 2 }
  ]
}
```

Or describe it in natural language and let the LLM generate it:

> *"Fetch weather data from one API and stock data from another, then merge both results into a single report."*

---

## Roadmap

- [x] MCP stdio/SSE/Streamable HTTP transport
- [x] Visual canvas editor (React Flow)
- [x] LLM-powered intent parsing
- [x] Plugin marketplace (GitHub/GitLab)
- [x] OpenTelemetry distributed tracing
- [x] Prometheus metrics
- [x] Circuit breaker & rate limiter
- [x] RBAC & API key authentication
- [x] Cross-platform packaging (Windows/macOS/Linux)
- [x] Auto-update (Tauri updater)
- [ ] MCP Resource & Prompt support
- [ ] WebSocket transport
- [ ] Team collaboration (real-time)
- [ ] Cloud sync (Pro feature)
- [ ] Mobile companion app

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

- **Good First Issues**: [Issues labeled `good first issue`](https://github.com/mcp-fusion/mcp-fusion/labels/good%20first%20issue)
- **Discord**: [Join our community](https://discord.gg/mcp-fusion)

---

## License

MCP Fusion is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)**.

- **Open source use**: Free for personal, academic, and open source projects
- **Commercial use**: Requires a commercial license. [Contact us](mailto:license@mcp-fusion.app) for details.

See [LICENSE](LICENSE) for the full text.

---

## Star History

<p align="center">
  <a href="https://star-history.com/#mcp-fusion/mcp-fusion&Date">
    <img src="https://api.star-history.com/svg?repos=mcp-fusion/mcp-fusion&type=Date" alt="Star History Chart" width="80%" />
  </a>
</p>

---

<p align="center">
  Made with ❤️ by the MCP Fusion team
</p>