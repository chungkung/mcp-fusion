<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/logo.jpg">
    <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/logo.jpg" alt="MCP Fusion Logo" width="180" />
  </picture>
</p>

<h1 align="center">MCP Fusion</h1>

<p align="center">
  <strong>Build, Run &amp; Monitor AI Tool Workflows — Visually. Locally. Effortlessly.</strong>
</p>

<p align="center">
  <a href="https://github.com/chungkung/mcp-fusion/actions/workflows/ci.yml"><img src="https://github.com/chungkung/mcp-fusion/actions/workflows/ci.yml/badge.svg" alt="CI Status" /></a>
  <a href="https://github.com/chungkung/mcp-fusion/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-AGPL--3.0-blue.svg" alt="License: AGPL-3.0" /></a>
  <a href="https://github.com/chungkung/mcp-fusion/releases"><img src="https://img.shields.io/github/v/release/chungkung/mcp-fusion?color=teal" alt="Latest Release" /></a>
  <a href="https://github.com/chungkung/mcp-fusion/releases"><img src="https://img.shields.io/github/downloads/chungkung/mcp-fusion/total?color=blue" alt="Downloads" /></a>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-brightgreen" alt="Platform: Windows | macOS | Linux" />
  <img src="https://img.shields.io/badge/rust-1.77%2B-orange" alt="Rust: 1.77+" />
</p>

<p align="center">
  <a href="README_zh.md">中文文档</a> ·
  <a href="#-quick-start">Quick Start</a> ·
  <a href="#-features">Features</a> ·
  <a href="#-architecture">Architecture</a> ·
  <a href="https://github.com/chungkung/mcp-fusion/releases">Download</a> ·
  <a href="#-contributing">Contributing</a>
</p>

---

## What is MCP Fusion?

**MCP Fusion** is a cross-platform desktop application that brings **visual workflow orchestration** to the [Model Context Protocol (MCP)](https://modelcontextprotocol.io) ecosystem. Drag, connect, and execute AI tool workflows — all running 100% on your machine, with zero cloud dependencies.

> Think **n8n** for AI toolchains. Think **Dify** for MCP servers. Running **entirely offline** on your desktop.

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-canvas.jpg" alt="MCP Fusion Canvas Mode" width="90%" />
</p>

---

## ✨ Why MCP Fusion?

| Capability | MCP Fusion | n8n | Dify | LangChain |
|-----------|-----------|-----|------|-----------|
| **Native MCP Protocol** | ✅ Full (stdio/SSE/HTTP) | ❌ | ❌ | ❌ (via lib) |
| **Desktop App** | ✅ Tauri (Rust) | ❌ Web only | ❌ Web only | ❌ |
| **Visual Canvas** | ✅ React Flow | ✅ | ✅ | ❌ |
| **LLM Intent Parsing** | ✅ OpenAI / Local | ❌ | ✅ | ❌ |
| **Distributed Tracing** | ✅ OpenTelemetry | ❌ | ❌ | ✅ |
| **Offline-First** | ✅ Local SQLite | ❌ | ❌ | ❌ |
| **Plugin Marketplace** | ✅ GitHub / GitLab | ✅ | ❌ | ❌ |
| **Cross-Platform** | ✅ Win / Mac / Linux | ✅ Web | ✅ Web | ❌ |
| **Multi-Transport** | ✅ 3 protocols | ❌ | ❌ | ❌ |
| **Auto-Update** | ✅ Built-in | ❌ | ❌ | ❌ |

---

## 🚀 Features

### 🎨 Visual Workflow Orchestration
- **Drag-and-drop canvas** powered by React Flow — no code required
- **Code mode** with JSON/YAML editing for power users
- **Real-time node state visualization** during execution with live status indicators
- **Undo/Redo** with full history tracking

### 🔗 MCP Protocol Fusion
- **All three transports** supported natively: `stdio`, `SSE`, `Streamable HTTP`
- **Auto-discovery** of tools and schemas from any MCP-compatible server
- **Multi-server orchestration** across Filesystem, GitHub, Postgres, Brave Search, and more
- **Hot-reload** server configurations without restarting

### 🧠 LLM-Powered Intent Engine
- **Natural language → workflow** in seconds via OpenAI-compatible APIs
- **Multi-turn conversation refinement** for iterative workflow design
- **Smart tool recommendation** based on task descriptions
- **Offline fallback** to keyword matching when no LLM is configured

### ⚡ Production-Grade Runtime
- **Topological sort scheduler** with parallel execution
- **Circuit breaker** pattern for fault isolation and graceful degradation
- **Rate limiter** to prevent upstream API abuse
- **Idempotency keys** for safe, repeatable execution
- **Retry with exponential backoff** for transient failures

### 📊 Built-in Observability
- **Prometheus metrics** — workflow duration, tool calls, server health
- **OpenTelemetry tracing** — export to Jaeger, Tempo, or any OTLP backend
- **Structured JSON logging** with tracing-subscriber
- **Live execution dashboard** with real-time metrics

### 🧩 Plugin Marketplace
- **One-click install** templates from GitHub / GitLab
- **Version management** with update notifications
- **8 built-in templates** for offline bootstrap
- **Community-driven** template sharing

### 🔒 Security & Compliance
- **RBAC** — Admin, Developer, Viewer roles
- **API Key encryption** with AES-256-GCM
- **Database encryption** for sensitive configurations
- **Tamper-evident audit trail** with cryptographic chain verification

---

## 📦 Quick Start

### Prerequisites

| Requirement | Version |
|------------|---------|
| **Node.js** | ≥ 20 |
| **Rust** | ≥ 1.77 |
| **OS** | Windows 10+ / macOS 11+ / Linux |

### Download

| Platform | Package |
|----------|---------|
| **Windows** | `.msi` / `.exe` (NSIS installer) |
| **macOS** | `.dmg` (Apple Silicon · Intel) |
| **Linux** | `.AppImage` · `.deb` · `.rpm` |

👉 [**Download Latest Release**](https://github.com/chungkung/mcp-fusion/releases)

### Build from Source

```bash
git clone https://github.com/chungkung/mcp-fusion.git
cd mcp-fusion
npm install

# Development
npm run tauri:dev

# Production build
npm run build:win    # Windows
npm run build:mac    # macOS
npm run build:linux  # Linux
```

### Configure LLM (Optional)

```bash
# OpenAI / compatible API
export LLM_API_KEY="sk-your-api-key"
export LLM_MODEL="gpt-4o-mini"

# Local model via Ollama
export LLM_API_URL="http://localhost:11434/v1/chat/completions"
export LLM_MODEL="qwen2.5:7b"
```

---

## 🏗 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      MCP Fusion Desktop                         │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   Frontend (React + TypeScript)            │  │
│  │  ┌──────────┐  ┌──────────┐  ┌───────────────────────┐   │  │
│  │  │ Intent   │  │ Canvas   │  │ Marketplace           │   │  │
│  │  │ Parser   │  │ Editor   │  │ Browser               │   │  │
│  │  └──────────┘  └──────────┘  └───────────────────────┘   │  │
│  └───────────────────────┬───────────────────────────────────┘  │
│                          │  IPC (Tauri Bridge)                  │
│  ┌───────────────────────┴───────────────────────────────────┐  │
│  │                    Backend (Rust + Tauri)                  │  │
│  │  ┌──────────┐  ┌──────────┐  ┌───────────────────────┐   │  │
│  │  │ LLM      │  │ Scheduler│  │ Gateway               │   │  │
│  │  │ Engine   │  │ (Topo)   │  │ (stdio / SSE / HTTP)  │   │  │
│  │  └──────────┘  └──────────┘  └───────────────────────┘   │  │
│  │  ┌──────────┐  ┌──────────┐  ┌───────────────────────┐   │  │
│  │  │ Metrics  │  │ Tracing  │  │ Marketplace           │   │  │
│  │  │ (Prom)   │  │ (OTel)   │  │ (GitHub / GitLab)     │   │  │
│  │  └──────────┘  └──────────┘  └───────────────────────┘   │  │
│  │  ┌──────────┐  ┌──────────┐  ┌───────────────────────┐   │  │
│  │  │ Auth     │  │ Circuit  │  │ Rate                  │   │  │
│  │  │ (RBAC)   │  │ Breaker  │  │ Limiter               │   │  │
│  │  └──────────┘  └──────────┘  └───────────────────────┘   │  │
│  │  ┌───────────────────────────────────────────────────┐   │  │
│  │  │              SQLite (Local Storage)                │   │  │
│  │  └───────────────────────────────────────────────────┘   │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🛠 Tech Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| **Desktop Shell** | [Tauri 2](https://tauri.app) | Lightweight, Rust-powered, cross-platform |
| **UI Framework** | React 18 + TypeScript | Type-safe, mature ecosystem |
| **Styling** | Tailwind CSS | Utility-first, rapid iteration |
| **Canvas** | [React Flow](https://reactflow.dev) | Battle-tested node editor |
| **State** | [Zustand](https://zustand-demo.pmnd.rs) | Minimal boilerplate |
| **Animation** | [Framer Motion](https://www.framer.com/motion/) | Declarative, performant |
| **Database** | SQLite via [rusqlite](https://github.com/rusqlite/rusqlite) | Zero-config, embedded |
| **Metrics** | [Prometheus](https://prometheus.io) (rust) | Industry standard |
| **Tracing** | [OpenTelemetry](https://opentelemetry.io) | OTLP export |
| **Encryption** | AES-256-GCM | Military-grade |
| **LLM** | OpenAI-compatible API | Broad model support |

---

## 📝 Example: 3-Node Aggregation Workflow

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

Or just describe it in natural language:

> *"Fetch weather data from one API and stock data from another, then merge both results into a single report."*

---

## 🗺 Roadmap

- [x] MCP stdio / SSE / Streamable HTTP transport
- [x] Visual canvas editor (React Flow)
- [x] LLM-powered intent parsing
- [x] Plugin marketplace (GitHub / GitLab)
- [x] OpenTelemetry distributed tracing
- [x] Prometheus metrics
- [x] Circuit breaker & rate limiter
- [x] RBAC & API key authentication
- [x] Cross-platform packaging (Windows / macOS / Linux)
- [x] Auto-update (Tauri updater)
- [ ] MCP Resource & Prompt support
- [ ] WebSocket transport
- [ ] Real-time team collaboration
- [ ] Cloud sync (Pro)
- [ ] Mobile companion app

---

## 🤝 Contributing

We welcome contributions of all kinds — code, docs, templates, bug reports, and feature ideas!

- 📖 **[CONTRIBUTING.md](CONTRIBUTING.md)** — guidelines and workflow
- 🐛 **[Good First Issues](https://github.com/chungkung/mcp-fusion/labels/good%20first%20issue)** — beginner-friendly tasks
- 💬 **[Discord](https://discord.gg/mcp-fusion)** — join our community

---

## 📄 License

MCP Fusion is licensed under **GNU Affero General Public License v3.0 (AGPL-3.0)**.

| Use Case | License |
|----------|---------|
| Personal, academic, open source | ✅ Free (AGPL-3.0) |
| Commercial / proprietary | 🔑 [Contact us](mailto:license@mcp-fusion.app) |

See [LICENSE](LICENSE) for full terms.

---

## ⭐ Star History

<p align="center">
  <a href="https://star-history.com/#chungkung/mcp-fusion&Date">
    <img src="https://api.star-history.com/svg?repos=chungkung/mcp-fusion&type=Date" alt="Star History" width="80%" />
  </a>
</p>

---

<p align="center">
  <sub>Built with Rust, TypeScript, and love by the MCP Fusion team</sub>
</p>