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

### 🎬 See It In Action

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif" alt="MCP Fusion Canvas Demo" width="840" />
  <br/>
  <em>Visual canvas editor — drag, connect, and execute AI tool workflows</em>
</p>

### 🎥 Quick Demos

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif" alt="Workflow Canvas Demo" width="840" />
  <br/>
  <em>1. Visual canvas — drag, connect, and execute AI tool workflows</em>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-intent.gif" alt="LLM Intent Parsing Demo" width="840" />
  <br/>
  <em>2. Natural language → auto-generated workflow with AI</em>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-execution.gif" alt="Multi-view Navigation Demo" width="840" />
  <br/>
  <em>3. Seamless multi-view navigation and real-time execution</em>
</p>

---

## ✨ Why MCP Fusion?

| Capability | MCP Fusion | n8n | Dify | LangChain | Flowise |
|-----------|-----------|-----|------|-----------|---------|
| **Native MCP Protocol** | ✅ Full (stdio/SSE/HTTP) | ❌ | ❌ | ❌ (via lib) | ❌ |
| **Desktop App** | ✅ Tauri (Rust) | ❌ Web only | ❌ Web only | ❌ | ❌ Web only |
| **Visual Canvas** | ✅ React Flow | ✅ | ✅ | ❌ | ✅ |
| **LLM Intent Parsing** | ✅ OpenAI / Local | ❌ | ✅ | ❌ | ❌ |
| **Distributed Tracing** | ✅ OpenTelemetry | ❌ | ❌ | ✅ | ❌ |
| **Offline-First** | ✅ Local SQLite | ❌ | ❌ | ❌ | ❌ |
| **Plugin Marketplace** | ✅ GitHub / GitLab | ✅ | ❌ | ❌ | ✅ |
| **Cross-Platform** | ✅ Win / Mac / Linux | ✅ Web | ✅ Web | ❌ | ✅ Web |
| **Multi-Transport** | ✅ 3 protocols | ❌ | ❌ | ❌ | ❌ |
| **Auto-Update** | ✅ Built-in | ❌ | ❌ | ❌ | ❌ |
| **RBAC & Audit** | ✅ Built-in | ❌ | ❌ | ❌ | ❌ |
| **Circuit Breaker** | ✅ Production-grade | ❌ | ❌ | ❌ | ❌ |

---

## 🚀 Features

### 🎨 Visual Workflow Orchestration

Design complex AI tool pipelines without writing a single line of code. The canvas powered by **React Flow** gives you full drag-and-drop control with real-time execution feedback.

- **Drag-and-drop canvas** — connect tools visually, no code required
- **Code mode** — JSON/YAML editing for power users who want full control
- **Real-time node state visualization** — live status indicators during execution (pending → running → success → failed)
- **Undo/Redo** — full history tracking with keyboard shortcuts
- **Auto-layout** — smart node positioning for clean workflow diagrams

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-canvas-real.png" alt="Visual Canvas Editor" width="442" />
</p>

### 🧠 LLM-Powered Intent Engine

Describe what you want in natural language, and MCP Fusion builds the workflow for you. Powered by OpenAI-compatible APIs with offline fallback.

- **Natural language → workflow** in seconds — just describe your goal
- **Multi-turn conversation refinement** — iterate on the design through chat
- **Smart tool recommendation** — automatically matches MCP tools to task descriptions
- **Offline keyword matching** — works without any LLM when you're disconnected
- **Supports**: OpenAI, Azure OpenAI, Ollama, LM Studio, vLLM, and any OpenAI-compatible endpoint

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-intent-real.png" alt="LLM Intent Parsing" width="442" />
  <br/>
  <em>Describe your workflow in natural language — the AI generates nodes and edges automatically</em>
</p>

### 🔗 MCP Protocol Fusion

MCP Fusion is the first desktop app to support **all three MCP transport protocols** natively. Mix and match servers regardless of transport.

- **`stdio`** — subprocess-based servers (local tools)
- **`SSE`** — Server-Sent Events for remote streaming
- **`Streamable HTTP`** — latest MCP spec with full HTTP transport
- **Auto-discovery** — automatically detects tools and schemas from any MCP-compatible server
- **Multi-server orchestration** — connect Filesystem, GitHub, Postgres, Brave Search, and more in a single workflow
- **Hot-reload** — update server configurations without restarting the app

### ⚡ Production-Grade Runtime

Built with battle-tested reliability patterns from distributed systems engineering.

- **Topological sort scheduler** — determines optimal execution order with maximum parallelism
- **Circuit breaker** — isolates failing tools to prevent cascading failures
- **Rate limiter** — protects upstream APIs from accidental abuse
- **Idempotency keys** — safe retry without duplicate side effects
- **Exponential backoff** — graceful retry with jitter for transient failures
- **Execution timeout** — per-node deadline enforcement

### 📊 Built-in Observability

Understand exactly what your workflows are doing with production-grade monitoring.

- **Prometheus metrics** — workflow duration, tool call counts, success rates, server health
- **OpenTelemetry tracing** — export to Jaeger, Tempo, Grafana, or any OTLP backend
- **Structured JSON logging** — powered by `tracing-subscriber` with configurable log levels
- **Live execution dashboard** — real-time metrics and span visualization

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-metrics.jpg" alt="Metrics & Tracing Dashboard" width="600" />
  <br/>
  <em>Real-time metrics, distributed tracing, and server health monitoring</em>
</p>

### 🧩 Plugin Marketplace

Discover, install, and share workflow templates — all from within the app.

- **One-click install** — browse and install templates from GitHub / GitLab
- **Version management** — track updates and changelogs for installed plugins
- **8 built-in templates** — ready-to-use workflows for common tasks (data aggregation, web scraping, notification pipelines)
- **Community-driven** — share your templates and discover others' creations

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-marketplace-real.png" alt="Plugin Marketplace" width="442" />
  <br/>
  <em>Browse, install, and manage workflow templates from the community</em>
</p>

### 🔒 Security & Compliance

- **RBAC** — Admin, Developer, Viewer roles with granular permissions
- **AES-256-GCM encryption** — API keys and secrets encrypted at rest
- **Database encryption** — sensitive configurations stored encrypted in SQLite
- **Tamper-evident audit trail** — cryptographically chained operation logs
- **No telemetry** — zero data leaves your machine unless you configure OTLP export

---

## 📦 Quick Start

### Prerequisites

| Requirement | Version | Check |
|------------|---------|-------|
| **Node.js** | ≥ 20 | `node --version` |
| **Rust** | ≥ 1.77 | `rustc --version` |
| **OS** | Windows 10+ / macOS 11+ / Linux | |

### Download & Install

| Platform | Package | Architecture |
|----------|---------|-------------|
| **Windows** | `.msi` / `.exe` (NSIS) | x86_64 |
| **macOS** | `.dmg` | Apple Silicon (aarch64) · Intel (x86_64) |
| **Linux** | `.AppImage` · `.deb` · `.rpm` | x86_64 |

👉 [**Download Latest Release**](https://github.com/chungkung/mcp-fusion/releases)

### Build from Source

```bash
git clone https://github.com/chungkung/mcp-fusion.git
cd mcp-fusion
npm install

# Development mode (hot-reload)
npm run tauri:dev

# Production build
npm run build:win    # Windows
npm run build:mac    # macOS
npm run build:linux  # Linux
```

### Configure LLM (Optional)

MCP Fusion works without an LLM for manual workflow building. Enable AI-powered generation:

```bash
# OpenAI / compatible API
export LLM_API_KEY="sk-your-api-key"
export LLM_MODEL="gpt-4o-mini"

# Local model via Ollama
export LLM_API_URL="http://localhost:11434/v1/chat/completions"
export LLM_MODEL="qwen2.5:7b"

# Azure OpenAI
export LLM_API_URL="https://your-resource.openai.azure.com"
export LLM_API_KEY="your-azure-key"
export LLM_MODEL="gpt-4o"
```

### First Workflow in 30 Seconds

1. **Launch** MCP Fusion
2. **Add an MCP server** — click "+" in the server panel, paste your server config
3. **Type your goal** in the Intent panel — e.g. *"Fetch the top 5 HN stories and save them to a file"*
4. **Review** the auto-generated workflow on the canvas
5. **Click Run** — watch nodes execute in real-time

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
| **Desktop Shell** | [Tauri 2](https://tauri.app) | Lightweight (~10MB), Rust-powered, cross-platform |
| **UI Framework** | React 18 + TypeScript | Type-safe, mature ecosystem, Vite bundler |
| **Styling** | Tailwind CSS | Utility-first, rapid iteration, dark mode |
| **Canvas** | [React Flow](https://reactflow.dev) | Battle-tested node editor, 40k+ GitHub stars |
| **State** | [Zustand](https://zustand-demo.pmnd.rs) | Minimal boilerplate, hook-based |
| **Animation** | [Framer Motion](https://www.framer.com/motion/) | Declarative, performant transitions |
| **Database** | SQLite via [rusqlite](https://github.com/rusqlite/rusqlite) | Zero-config, embedded, ACID |
| **Metrics** | [Prometheus](https://prometheus.io) (rust) | Industry standard, pull-based |
| **Tracing** | [OpenTelemetry](https://opentelemetry.io) | OTLP export, vendor-neutral |
| **Encryption** | AES-256-GCM | Military-grade, hardware-accelerated |
| **LLM** | OpenAI-compatible API | Broad model support, local and cloud |
| **IPC** | Tauri Commands | Rust-native, type-safe bridge |

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

### Completed ✅
- [x] MCP stdio / SSE / Streamable HTTP transport
- [x] Visual canvas editor (React Flow)
- [x] LLM-powered intent parsing (OpenAI-compatible)
- [x] Plugin marketplace (GitHub / GitLab)
- [x] OpenTelemetry distributed tracing
- [x] Prometheus metrics
- [x] Circuit breaker & rate limiter
- [x] RBAC & API key authentication
- [x] Cross-platform packaging (Windows / macOS / Linux)
- [x] Auto-update (Tauri updater)

### In Progress 🚧
- [ ] MCP Resource & Prompt support
- [ ] WebSocket transport
- [ ] Conditional branching (if/else nodes)
- [ ] Loop / iteration nodes

### Planned 🎯
- [ ] Real-time team collaboration
- [ ] Cloud sync (Pro)
- [ ] Mobile companion app
- [ ] gRPC transport
- [ ] Workflow version history & diff

---

## 🤝 Contributing

We welcome contributions of all kinds — code, docs, templates, bug reports, and feature ideas!

- 📖 **[CONTRIBUTING.md](CONTRIBUTING.md)** — guidelines and development workflow
- 🐛 **[Good First Issues](https://github.com/chungkung/mcp-fusion/labels/good%20first%20issue)** — beginner-friendly tasks
- 💬 **[Discord](https://discord.gg/mcp-fusion)** — join our community

### Development Setup

```bash
git clone https://github.com/chungkung/mcp-fusion.git
cd mcp-fusion
npm install
npm run tauri:dev
```

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
    <img src="https://api.star-history.com/svg?repos=chungkung/mcp-fusion&type=Date" alt="Star History" width="600" />
  </a>
</p>

---

<p align="center">
  <sub>Built with Rust, TypeScript, and love by the MCP Fusion team</sub>
</p>