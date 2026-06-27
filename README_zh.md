<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/logo.jpg">
    <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/logo.jpg" alt="MCP Fusion Logo" width="180" />
  </picture>
</p>

<h1 align="center">MCP Fusion</h1>

<p align="center">
  <strong>可视化构建、运行和监控 AI 工具工作流 — 本地运行，零云依赖</strong>
</p>

<p align="center">
  <a href="https://github.com/chungkung/mcp-fusion/actions/workflows/ci.yml"><img src="https://github.com/chungkung/mcp-fusion/actions/workflows/ci.yml/badge.svg" alt="CI 状态" /></a>
  <a href="https://github.com/chungkung/mcp-fusion/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-AGPL--3.0-blue.svg" alt="许可证: AGPL-3.0" /></a>
  <a href="https://github.com/chungkung/mcp-fusion/releases"><img src="https://img.shields.io/github/v/release/chungkung/mcp-fusion?color=teal" alt="最新版本" /></a>
  <a href="https://github.com/chungkung/mcp-fusion/releases"><img src="https://img.shields.io/github/downloads/chungkung/mcp-fusion/total?color=blue" alt="下载量" /></a>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-brightgreen" alt="平台: Windows | macOS | Linux" />
  <img src="https://img.shields.io/badge/rust-1.77%2B-orange" alt="Rust: 1.77+" />
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="#-快速开始">快速开始</a> ·
  <a href="#-核心功能">核心功能</a> ·
  <a href="#-架构">架构</a> ·
  <a href="https://github.com/chungkung/mcp-fusion/releases">下载</a> ·
  <a href="#-贡献">贡献</a>
</p>

---

## MCP Fusion 是什么？

**MCP Fusion** 是一个跨平台桌面应用，为 [Model Context Protocol (MCP)](https://modelcontextprotocol.io) 生态提供**可视化工作流编排**。拖拽、连接、执行 AI 工具工作流 — 全部在你的机器上运行，零云依赖。

> 像 **n8n** 一样编排 AI 工具链。像 **Dify** 一样驱动 MCP 服务。**完全离线**运行在你的桌面上。

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-canvas.jpg" alt="MCP Fusion 画布模式" width="90%" />
</p>

---

## ✨ 为什么选择 MCP Fusion？

| 能力 | MCP Fusion | n8n | Dify | LangChain |
|------|-----------|-----|------|-----------|
| **原生 MCP 协议** | ✅ 全协议 (stdio/SSE/HTTP) | ❌ | ❌ | ❌ (仅 SDK) |
| **桌面应用** | ✅ Tauri (Rust) | ❌ 纯 Web | ❌ 纯 Web | ❌ |
| **可视化画布** | ✅ React Flow | ✅ | ✅ | ❌ |
| **LLM 意图解析** | ✅ OpenAI / 本地模型 | ❌ | ✅ | ❌ |
| **分布式追踪** | ✅ OpenTelemetry | ❌ | ❌ | ✅ |
| **离线优先** | ✅ 本地 SQLite | ❌ | ❌ | ❌ |
| **插件市场** | ✅ GitHub / GitLab | ✅ | ❌ | ❌ |
| **跨平台** | ✅ Win / Mac / Linux | ✅ Web | ✅ Web | ❌ |
| **多协议传输** | ✅ 3 种协议 | ❌ | ❌ | ❌ |
| **自动更新** | ✅ 内置支持 | ❌ | ❌ | ❌ |

---

## 🚀 核心功能

### 🎨 可视化工作流编排
- **拖拽式画布**，基于 React Flow，无需编写代码
- **代码模式**，支持 JSON/YAML 编辑，满足高级用户需求
- **实时节点状态可视化**，执行过程中展示实时状态指示器
- **撤销/重做**，完整的历史记录追踪

### 🔗 MCP 协议融合
- **原生支持全部三种传输协议**：`stdio`、`SSE`、`Streamable HTTP`
- **自动发现**任意 MCP 兼容服务器的工具和 Schema
- **跨服务器编排**，连接 Filesystem、GitHub、Postgres、Brave Search 等
- **热重载**服务器配置，无需重启

### 🧠 LLM 驱动的意图引擎
- **自然语言 → 工作流**，秒级转换，兼容 OpenAI API
- **多轮对话细化**，迭代式工作流设计
- **智能工具推荐**，基于任务描述自动匹配
- **离线回退**，未配置 LLM 时自动切换到关键词匹配

### ⚡ 生产级运行时
- **拓扑排序调度器**，支持并行执行
- **熔断器模式**，故障隔离与优雅降级
- **速率限制器**，防止上游 API 滥用
- **幂等键**，确保安全可重复执行
- **指数退避重试**，应对瞬时故障

### 📊 内置可观测性
- **Prometheus 指标** — 工作流耗时、工具调用、服务器健康状态
- **OpenTelemetry 追踪** — 导出到 Jaeger、Tempo 或任意 OTLP 后端
- **结构化 JSON 日志**，基于 tracing-subscriber
- **实时执行监控面板**，展示实时指标

### 🧩 插件市场
- **一键安装**来自 GitHub / GitLab 的模板
- **版本管理**，更新通知
- **8 个内置模板**，离线即可使用
- **社区驱动**的模板分享

### 🔒 安全与合规
- **RBAC** — 管理员、开发者、观察者角色
- **API Key 加密**，AES-256-GCM 加密存储
- **数据库加密**，保护敏感配置
- **防篡改审计追踪**，加密链验证

---

## 📦 快速开始

### 环境要求

| 依赖 | 版本 |
|------|------|
| **Node.js** | ≥ 20 |
| **Rust** | ≥ 1.77 |
| **操作系统** | Windows 10+ / macOS 11+ / Linux |

### 下载安装

| 平台 | 安装包 |
|------|--------|
| **Windows** | `.msi` / `.exe` (NSIS 安装器) |
| **macOS** | `.dmg` (Apple Silicon · Intel) |
| **Linux** | `.AppImage` · `.deb` · `.rpm` |

👉 [**下载最新版本**](https://github.com/chungkung/mcp-fusion/releases)

### 从源码构建

```bash
git clone https://github.com/chungkung/mcp-fusion.git
cd mcp-fusion
npm install

# 开发模式
npm run tauri:dev

# 生产构建
npm run build:win    # Windows
npm run build:mac    # macOS
npm run build:linux  # Linux
```

### 配置 LLM（可选）

```bash
# OpenAI / 兼容 API
export LLM_API_KEY="sk-your-api-key"
export LLM_MODEL="gpt-4o-mini"

# 本地模型 (Ollama)
export LLM_API_URL="http://localhost:11434/v1/chat/completions"
export LLM_MODEL="qwen2.5:7b"
```

---

## 🏗 架构

```
┌─────────────────────────────────────────────────────────────────┐
│                      MCP Fusion 桌面应用                         │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   前端 (React + TypeScript)                │  │
│  │  ┌──────────┐  ┌──────────┐  ┌───────────────────────┐   │  │
│  │  │ 意图解析  │  │ 画布编辑  │  │ 插件市场浏览           │   │  │
│  │  └──────────┘  └──────────┘  └───────────────────────┘   │  │
│  └───────────────────────┬───────────────────────────────────┘  │
│                          │  IPC (Tauri 桥接)                    │
│  ┌───────────────────────┴───────────────────────────────────┐  │
│  │                    后端 (Rust + Tauri)                     │  │
│  │  ┌──────────┐  ┌──────────┐  ┌───────────────────────┐   │  │
│  │  │ LLM      │  │ 调度器    │  │ 网关                   │   │  │
│  │  │ 引擎     │  │ (拓扑)    │  │ (stdio / SSE / HTTP)  │   │  │
│  │  └──────────┘  └──────────┘  └───────────────────────┘   │  │
│  │  ┌──────────┐  ┌──────────┐  ┌───────────────────────┐   │  │
│  │  │ 指标     │  │ 追踪     │  │ 插件市场               │   │  │
│  │  │ (Prom)   │  │ (OTel)   │  │ (GitHub / GitLab)     │   │  │
│  │  └──────────┘  └──────────┘  └───────────────────────┘   │  │
│  │  ┌──────────┐  ┌──────────┐  ┌───────────────────────┐   │  │
│  │  │ 认证     │  │ 熔断器   │  │ 速率限制器             │   │  │
│  │  │ (RBAC)   │  │          │  │                       │   │  │
│  │  └──────────┘  └──────────┘  └───────────────────────┘   │  │
│  │  ┌───────────────────────────────────────────────────┐   │  │
│  │  │              SQLite (本地存储)                     │   │  │
│  │  └───────────────────────────────────────────────────┘   │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🛠 技术栈

| 层级 | 技术 | 选型理由 |
|------|------|----------|
| **桌面框架** | [Tauri 2](https://tauri.app) | 轻量、Rust 驱动、跨平台 |
| **UI 框架** | React 18 + TypeScript | 类型安全、生态成熟 |
| **样式** | Tailwind CSS | 原子化 CSS，快速迭代 |
| **画布** | [React Flow](https://reactflow.dev) | 久经考验的节点编辑器 |
| **状态管理** | [Zustand](https://zustand-demo.pmnd.rs) | 极简样板代码 |
| **动画** | [Framer Motion](https://www.framer.com/motion/) | 声明式、高性能 |
| **数据库** | SQLite via [rusqlite](https://github.com/rusqlite/rusqlite) | 零配置、嵌入式 |
| **指标** | [Prometheus](https://prometheus.io) (rust) | 行业标准 |
| **追踪** | [OpenTelemetry](https://opentelemetry.io) | OTLP 导出 |
| **加密** | AES-256-GCM | 军事级加密 |
| **LLM** | OpenAI 兼容 API | 广泛模型支持 |

---

## 📝 示例：3 节点聚合工作流

```json
{
  "name": "API 数据聚合",
  "nodes": [
    {
      "tool_name": "fetch",
      "server_id": "fetch-server",
      "label": "获取天气 API",
      "position_x": 100, "position_y": 100
    },
    {
      "tool_name": "fetch",
      "server_id": "fetch-server",
      "label": "获取股票 API",
      "position_x": 400, "position_y": 100
    },
    {
      "tool_name": "aggregate",
      "server_id": "data-tools",
      "label": "合并结果",
      "position_x": 250, "position_y": 300
    }
  ],
  "edges": [
    { "source_index": 0, "target_index": 2 },
    { "source_index": 1, "target_index": 2 }
  ]
}
```

或者直接用自然语言描述：

> *"从天气 API 获取天气数据，从股票 API 获取股票数据，然后将两个结果合并为一份报告。"*

---

## 🗺 路线图

- [x] MCP stdio / SSE / Streamable HTTP 传输协议
- [x] 可视化画布编辑器 (React Flow)
- [x] LLM 驱动的意图解析
- [x] 插件市场 (GitHub / GitLab)
- [x] OpenTelemetry 分布式追踪
- [x] Prometheus 指标
- [x] 熔断器与速率限制器
- [x] RBAC 与 API Key 认证
- [x] 跨平台打包 (Windows / macOS / Linux)
- [x] 自动更新 (Tauri updater)
- [ ] MCP Resource 与 Prompt 支持
- [ ] WebSocket 传输协议
- [ ] 实时团队协作
- [ ] 云同步 (Pro)
- [ ] 移动端配套应用

---

## 🤝 贡献

欢迎各种形式的贡献 — 代码、文档、模板、Bug 报告和功能建议！

- 📖 **[CONTRIBUTING.md](CONTRIBUTING.md)** — 贡献指南与工作流
- 🐛 **[新手友好 Issue](https://github.com/chungkung/mcp-fusion/labels/good%20first%20issue)** — 入门级任务
- 💬 **[Discord](https://discord.gg/mcp-fusion)** — 加入社区

---

## 📄 许可证

MCP Fusion 使用 **GNU Affero General Public License v3.0 (AGPL-3.0)** 许可。

| 使用场景 | 许可证 |
|----------|--------|
| 个人、学术、开源项目 | ✅ 免费 (AGPL-3.0) |
| 商业 / 闭源使用 | 🔑 [联系我们](mailto:license@mcp-fusion.app) |

详见 [LICENSE](LICENSE) 全文。

---

## ⭐ Star 历史

<p align="center">
  <a href="https://star-history.com/#chungkung/mcp-fusion&Date">
    <img src="https://api.star-history.com/svg?repos=chungkung/mcp-fusion&type=Date" alt="Star 历史" width="80%" />
  </a>
</p>

---

<p align="center">
  <sub>由 MCP Fusion 团队用 Rust、TypeScript 和 ❤️ 构建</sub>
</p>