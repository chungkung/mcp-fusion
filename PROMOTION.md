# 🚀 MCP Fusion 推广方案

## 目标：1000 GitHub Stars

---

## 第一步：优化 GitHub 仓库（立即执行）

### 1.1 设置仓库 Topics（在 GitHub 仓库页面 Settings → Topics 添加）

```
mcp model-context-protocol ai-workflow workflow-orchestration tauri rust
react-flow desktop-app n8n-alternative dify-alternative openai
opentelemetry llm-tools visual-programming offline-first
ai-agent automation-tools mcp-server tool-calling
```

### 1.2 设置仓库 About（仓库首页右侧 Description）

```
🖥️ Visual AI workflow orchestration desktop app for MCP — drag, connect, execute. 100% offline. Supports stdio / SSE / Streamable HTTP. Built with Tauri + Rust + React.
```

### 1.3 设置 Website

在仓库 About 区域填写：
```
https://github.com/chungkung/mcp-fusion#readme
```

---

## 第二步：发布到各平台（按优先级排列）

### 🔴 P0 — 最高优先级

#### 1. Product Hunt
- **网址**: https://www.producthunt.com/
- **时间**: 周二/周三/周四 北京时间晚上 10:00（太平洋时间 7:00 AM）发布
- **标题**: MCP Fusion — Visual AI workflow builder for MCP, 100% offline desktop app
- **Tagline**: Build, run & monitor AI tool workflows visually — zero cloud, fully local
- **内容**:

```
Hey Product Hunt! 👋

I built MCP Fusion because I was frustrated with existing AI workflow tools — they all require cloud accounts, send data to third parties, and don't support MCP natively.

MCP Fusion is a desktop app that lets you:
- 🎨 Visually drag-and-drop AI tools to build workflows
- 🧠 Describe workflows in natural language → AI generates them automatically
- 🔗 Connect ALL MCP server types (stdio / SSE / Streamable HTTP)
- 🔒 100% offline — your data never leaves your machine
- 📊 Built-in OpenTelemetry tracing + Prometheus metrics
- 🛡️ RBAC, audit trails, AES-256-GCM encryption, circuit breakers

Tech stack: Tauri 2.x (Rust) + React + React Flow + SQLite

🎯 Think n8n for AI toolchains, but running entirely on your desktop.

💡 Open source (AGPL-3.0), cross-platform (Windows/macOS/Linux).

Would love your feedback! 🙏
```

#### 2. Hacker News (Show HN)
- **网址**: https://news.ycombinator.com/
- **标题**: Show HN: MCP Fusion — Visual AI workflow builder for MCP, 100% offline desktop app
- **内容**:

```
I built a desktop app for visually orchestrating AI tool workflows using the Model Context Protocol (MCP). It supports all three MCP transport protocols (stdio, SSE, Streamable HTTP) and runs entirely offline on your local machine.

Key features:
- Visual canvas editor (React Flow) — drag, connect, and execute workflows
- LLM-powered intent engine — describe what you want in natural language, it builds the workflow
- Multi-protocol MCP support — mix stdio, SSE, and Streamable HTTP servers in one workflow
- Production-grade runtime — circuit breaker, rate limiter, idempotency, exponential backoff
- Observability — OpenTelemetry tracing + Prometheus metrics built-in
- Security — RBAC, AES-256-GCM encryption, tamper-evident audit trail
- Cross-platform — Windows, macOS, Linux (Tauri 2.x + Rust + React)

Everything runs locally. No cloud. No telemetry. Your API keys and data stay on your machine.

GitHub: https://github.com/chungkung/mcp-fusion
Downloads: https://github.com/chungkung/mcp-fusion/releases

Feedback welcome! What would make this more useful for your workflow?
```

#### 3. Reddit — r/rust
- **网址**: https://www.reddit.com/r/rust/
- **标题**: [Project] MCP Fusion — A Tauri 2.x desktop app for visual AI workflow orchestration
- **内容**:

```
I built MCP Fusion using Tauri 2.x + Rust backend + React frontend. It's a desktop app for visually orchestrating AI tool workflows using the Model Context Protocol (MCP).

Rust-specific highlights:
- stdio MCP transport via tokio::process::Command
- SSE client with reqwest streaming
- SQLite storage via rusqlite with incremental migrations
- OpenTelemetry tracing with OTLP export
- AES-256-GCM encryption for secrets at rest
- Prometheus metrics with process-level stats
- Production patterns: circuit breaker, rate limiter, idempotency keys

The frontend uses React Flow for the canvas, Zustand for state, and Framer Motion for animations.

GitHub: https://github.com/chungkung/mcp-fusion
27 frontend tests + Rust unit tests for gateway and storage modules.

Would appreciate code review from the Rust community! The Tauri IPC layer and gateway module might be particularly interesting.
```

#### 4. Reddit — r/programming
- **网址**: https://www.reddit.com/r/programming/
- **标题**: MCP Fusion: An open-source desktop app to visually build AI tool workflows
- **内容**:

```
I built an open-source desktop app that lets you visually orchestrate AI tool workflows using the Model Context Protocol (MCP).

Think n8n, but for AI toolchains. Think Dify, but running entirely offline on your desktop.

It supports all 3 MCP transport protocols (stdio, SSE, Streamable HTTP), has a visual canvas with drag-and-drop, LLM-powered natural language workflow generation, and production-grade reliability features (circuit breaker, rate limiter, idempotency).

Built with Tauri 2.x (Rust) + React + React Flow + SQLite.

GitHub: https://github.com/chungkung/mcp-fusion
```

#### 5. Reddit — r/LocalLLaMA
- **网址**: https://www.reddit.com/r/LocalLLaMA/
- **标题**: MCP Fusion — AI workflow builder that runs 100% offline with local LLM support
- **内容**:

```
Sharing a project that might interest the local AI community.

MCP Fusion is a desktop app for building AI tool workflows. It runs entirely offline and supports local LLMs via Ollama, LM Studio, vLLM, or any OpenAI-compatible endpoint.

Key things for the local-first crowd:
- 100% offline — SQLite storage, no cloud, no telemetry
- Supports Ollama / LM Studio / vLLM for intent parsing
- AES-256-GCM encrypted API keys and secrets at rest
- All MCP tools run locally (stdio protocol)
- Cross-platform (Windows/macOS/Linux)

GitHub: https://github.com/chungkung/mcp-fusion
```

#### 6. Reddit — r/selfhosted
- **网址**: https://www.reddit.com/r/selfhosted/
- **标题**: MCP Fusion — Self-hosted AI workflow orchestration (desktop app, fully offline)
- **内容**:

```
For the self-hosted community — a desktop app that lets you build AI tool workflows with zero cloud dependency.

- No cloud, no telemetry, no accounts
- All data stored in local SQLite
- Supports local LLMs (Ollama, LM Studio, vLLM)
- MIT-licensed MCP tools from the community
- Cross-platform desktop app

Think of it as n8n for AI toolchains, but running entirely on your machine.

GitHub: https://github.com/chungkung/mcp-fusion
```

---

### 🟡 P1 — 高优先级

#### 7. 掘金 (juejin.cn)
- **网址**: https://juejin.cn/
- **标题**: 我开源了一个 MCP 可视化工作流编排桌面应用 — 支持全协议、零云依赖
- **标签**: 前端, Rust, 开源, AI
- **内容**:

```
## 背景

最近 MCP（Model Context Protocol）生态越来越火，但我发现市面上缺少一个能**可视化编排** MCP 工具的工作流工具。n8n、Dify 都需要云端部署，而且不支持 MCP 原生协议。于是我花了几个月时间，用 Tauri 2.x + Rust + React 做了一个完全离线的桌面应用。

## 项目亮点

### 🎨 可视化画布
基于 React Flow 实现的拖拽式画布，可以像搭积木一样连接 MCP 工具：

- 拖拽工具到画布 → 连接节点 → 一键执行
- 实时状态反馈（pending → running → success → failed）
- 撤销/重做、自动布局、导出/导入

### 🧠 LLM 意图解析
用自然语言描述需求，AI 自动生成工作流：

- 支持 OpenAI / Azure / Ollama / LM Studio / vLLM
- 离线关键词匹配降级方案
- 多轮对话优化

### 🔗 全协议 MCP 支持
首个同时支持三种 MCP 传输协议的桌面应用：

- stdio — 子进程通信（本地工具）
- SSE — 服务端推送
- Streamable HTTP — 最新 MCP 规范

### ⚡ 生产级运行时
- 拓扑排序调度器
- 熔断器 + 速率限制器
- 幂等键 + 指数退避重试
- 工作流执行锁 + 断点续传

### 🔒 安全合规
- RBAC 三角色权限控制
- AES-256-GCM 密钥加密
- 防篡改审计哈希链
- 零遥测

## 技术栈

| 层 | 技术 |
|---|------|
| 桌面框架 | Tauri 2.x |
| 后端 | Rust (tokio, rusqlite, reqwest) |
| 前端 | React 18 + TypeScript |
| 画布 | @xyflow/react (React Flow) |
| 状态管理 | Zustand |
| 动画 | Framer Motion |
| 样式 | Tailwind CSS |
| 数据库 | SQLite (WAL mode) |
| 可观测性 | OpenTelemetry + Prometheus |
| 测试 | Vitest + React Testing Library |

## 仓库地址

https://github.com/chungkung/mcp-fusion

欢迎 Star ⭐ 和 PR！
```

#### 8. 知乎
- **网址**: https://www.zhihu.com/
- **问题选题**:
  - "如何评价 MCP Fusion 这个开源项目？"
  - "有没有好用的 MCP 可视化编排工具推荐？"
  - "如何看待 AI 工作流编排工具的未来？"
- **回答内容**: 使用上面掘金的内容，调整为问答形式

#### 9. V2EX
- **网址**: https://www.v2ex.com/
- **节点**: `/go/create` (分享创造)
- **标题**: [分享创造] MCP Fusion — 一个完全离线的 MCP 可视化工作流编排桌面应用
- **内容**:

```
最近搞了个开源项目，用 Tauri 2.x + Rust + React 做了一个 MCP 可视化工作流编排桌面应用。

核心特性：
- 🎨 拖拽式画布编辑器（React Flow）
- 🧠 自然语言描述需求 → AI 自动生成工作流
- 🔗 支持 stdio / SSE / Streamable HTTP 三种 MCP 协议
- 🔒 100% 离线运行，数据不进云端
- 📊 内置 OpenTelemetry 追踪 + Prometheus 指标
- 🛡️ RBAC + AES-256-GCM 加密 + 审计哈希链

跨平台：Windows / macOS / Linux

GitHub: https://github.com/chungkung/mcp-fusion

欢迎体验和反馈 🙏
```

#### 10. 即刻 / 小红书
- **标题**: 开源了一个 MCP 可视化工作流工具，完全离线运行
- **内容**: 简短版 + 截图/GIF + GitHub 链接

---

### 🟢 P2 — 中优先级

#### 11. Twitter/X
```
🚀 I built MCP Fusion — a visual AI workflow builder for MCP that runs 100% offline on your desktop.

🎨 Drag-and-drop canvas
🧠 Natural language → workflow
🔗 stdio / SSE / Streamable HTTP
🔒 Zero cloud, zero telemetry

OSS (AGPL-3.0) | Tauri + Rust + React

github.com/chungkung/mcp-fusion
```

#### 12. Dev.to
- 使用掘金内容，翻译为英文

#### 13. HackerNoon / Medium
- 撰写一篇技术深度文章："Building a Desktop AI Workflow Orchestrator with Tauri and Rust"

#### 14. LinkedIn
- 发布到 LinkedIn Articles，标题同英文版

#### 15. Discord / Slack 社区
- MCP 官方 Discord: https://discord.gg/mcp
- Tauri Discord: https://discord.gg/tauri
- Rust 中文社区
- 各种 AI 工具群

---

## 第三步：持续运营

### 3.1 每周更新
- 在 GitHub 发布 Release Notes
- 在 Reddit/V2EX/掘金 发布更新日志

### 3.2 内容营销
- 撰写技术博客（如何用 Tauri 构建桌面应用）
- 录制 YouTube 教程
- 制作对比视频（vs n8n, vs Dify）

### 3.3 社区互动
- 在 GitHub Issues 快速响应
- 在 Reddit/Discord 回答相关问题
- 创建 GitHub Discussions 活跃社区

---

## 期望时间线

| 周次 | 目标 | 行动 |
|------|------|------|
| 第1周 | 100 Stars | Product Hunt + HN + Reddit 集中发布 |
| 第2周 | 300 Stars | 掘金 + 知乎 + V2EX + 技术博客 |
| 第3-4周 | 500 Stars | 持续社区互动 + 功能更新 + 案例分享 |
| 第2个月 | 800 Stars | 视频教程 + 英文技术博客 + 案例合集 |
| 第3个月 | 1000 Stars | 口碑传播 + 持续迭代 |