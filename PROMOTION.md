# 🚀 MCP Fusion 推广方案

## 目标：1000 GitHub Stars

---

## 可用素材清单

| 文件名 | 类型 | 用途 |
|--------|------|------|
| `demo-workflow.gif` | GIF 动图 | 展示拖拽连线构建工作流的完整过程 |
| `demo-intent.gif` | GIF 动图 | 展示自然语言描述 → 自动生成工作流 |
| `demo-execution.gif` | GIF 动图 | 展示工作流执行过程、实时状态变化 |
| `screenshot-canvas.png` | 截图 | 画布主界面，展示完整工作流布局 |
| `screenshot-intent.png` | 截图 | 意图解析面板，展示 LLM 交互界面 |
| `screenshot-mcp-config.png` | 截图 | MCP 服务器配置界面 |
| `screenshot-metrics.jpg` | 截图 | Prometheus + OpenTelemetry 可观测性面板 |
| `screenshot-marketplace.png` | 截图 | 工具市场，展示可用的 MCP 工具列表 |
| `screenshot-permissions.png` | 截图 | RBAC 权限管理界面 |
| `logo.jpg` | Logo | 项目 Logo，用于封面/头像 |

素材原始 URL 前缀：
```
https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/
```

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

---

### 🔴 P0 — 最高优先级

---

#### 1. Product Hunt

- **网址**: https://www.producthunt.com/
- **时间**: 周二/周三/周四 北京时间晚上 10:00（太平洋时间 7:00 AM）发布
- **标题**: MCP Fusion — Visual AI workflow builder for MCP, 100% offline desktop app
- **Tagline**: Build, run & monitor AI tool workflows visually — zero cloud, fully local

**内容**:

```

Hey Product Hunt! 👋

I built MCP Fusion because I was frustrated with existing AI workflow tools — they all require cloud accounts, send data to third parties, and don't support MCP natively.

![MCP Fusion Workflow Canvas](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif)

MCP Fusion is a desktop app that lets you:

🎨 **Visual Drag-and-Drop Canvas**
Build AI tool workflows by connecting nodes on a React Flow canvas. See real-time status updates as each step executes.

![Intent Engine](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-intent.png)

🧠 **Natural Language → Workflow**
Describe what you want in plain English — the LLM-powered intent engine generates the workflow automatically. Supports OpenAI, Azure, Ollama, LM Studio, and vLLM.

🔗 **All MCP Protocols**
Connect stdio, SSE, and Streamable HTTP servers — mix and match in a single workflow. No other desktop tool supports all three.

![MCP Config](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-mcp-config.png)

🔒 **100% Offline**
Your data never leaves your machine. SQLite storage, AES-256-GCM encryption, zero telemetry.

📊 **Built-in Observability**
OpenTelemetry tracing + Prometheus metrics out of the box.

🛡️ **Production-Grade**
RBAC, tamper-evident audit trail, circuit breakers, rate limiters, idempotency keys, and exponential backoff retry.

Tech stack: **Tauri 2.x (Rust) + React + React Flow + SQLite**

🎯 Think n8n for AI toolchains, but running entirely on your desktop.

💡 Open source (AGPL-3.0), cross-platform (Windows/macOS/Linux).

Would love your feedback! 🙏
```

**📸 配图建议**:

| 序号 | 文件 | 说明 |
|------|------|------|
| 1 | `demo-workflow.gif` | 作为帖子第一张图，展示核心拖拽连线体验 |
| 2 | `screenshot-intent.png` | 展示自然语言 → 工作流生成能力 |
| 3 | `screenshot-mcp-config.png` | 展示全协议 MCP 支持 |
| 4 | `screenshot-metrics.jpg` | 展示可观测性面板（Gallery 第五张） |
| 5 | `logo.jpg` | 作为 Product Hunt 图标/缩略图 |

> Product Hunt Gallery 支持上传 5 张图，第一张推荐用 GIF 动图吸引眼球。

---

#### 2. Product Hunt 详细攻略

##### 2.1 发布前准备清单

- [ ] **Maker 账户**：至少提前 1 周注册 Product Hunt 账号，完善个人资料
- [ ] **邀请 Maker**：如果有合作者，提前邀请他们作为 Maker 加入项目
- [ ] **Hunter 选择**：
  - 推荐联系活跃的 Hunter（如 @chrismessina、@kevin、@bramk）帮忙 Hunt
  - 如果没有 Hunter，可以自己发布（Maker Launch），但 Hunter 的曝光量通常更大
  - 在 Twitter/X 上提前 1-2 周与目标 Hunter 互动
- [ ] **Gallery 图片**：准备 5 张高质量截图/GIF（见上方配图建议）
- [ ] **Logo**：上传 240x240 的项目 Logo
- [ ] **Tagline**：控制在 60 字符以内，简洁有力
- [ ] **First Comment**：产品发布后第一时间在评论区发一条详细介绍，包含：
  - 项目背景和动机
  - 与竞品（n8n、Dify、LangFlow）的差异化
  - 技术栈亮点
  - 开源协议和 GitHub 链接
- [ ] **社交媒体预备**：提前写好 Twitter/X、Reddit、LinkedIn 的推广文案，发布当天同步推送
- [ ] **团队就位**：发布当天确保团队在线，及时回复评论

##### 2.2 发布时间策略

| 事项 | 建议 |
|------|------|
| 发布日 | 周二 / 周三 / 周四（避开周末和周一） |
| 发布时间 | 北京时间 22:00（太平洋时间 07:00 AM） |
| 投票窗口 | 发布后 24 小时内集中获取 Upvote |
| 预热 | 发布前 2 天在 Twitter/X 上预告 |

##### 2.3 发布日执行清单

- [ ] 00:01（太平洋时间）— 产品上线，立即发布 First Comment
- [ ] 00:15 — 在 Twitter/X 发布推广帖，附 Product Hunt 链接
- [ ] 00:30 — 在 Reddit 相关子版块发帖（r/programming、r/rust）
- [ ] 01:00 — 在 Discord 社区（MCP、Tauri）分享
- [ ] 02:00-08:00 — 持续回复评论，保持活跃
- [ ] 08:00 — 在 LinkedIn 发布文章
- [ ] 12:00 — 第二轮社交媒体推送（覆盖不同时区）
- [ ] 24:00 — 统计当日数据，记录经验

##### 2.4 评论区话术准备

**当有人问"和 n8n/Dify 有什么区别？"**

> Great question! The key differences:
> 1. **Offline-first**: MCP Fusion runs 100% on your desktop — no cloud, no accounts. n8n and Dify both require server deployment.
> 2. **Native MCP**: We support all 3 MCP transport protocols (stdio/SSE/Streamable HTTP) natively. n8n doesn't have MCP support at all.
> 3. **Desktop app**: Built with Tauri + Rust, so it's lightweight (~15MB) and fast. No Docker, no Kubernetes.
> 4. **Open source**: AGPL-3.0, fully transparent.

**当有人问"只有桌面端吗？有没有 Web 版？"**

> Desktop-first for now, but we're exploring a headless mode for server deployment. The architecture supports it — the Rust backend is already decoupled from the frontend. PRs welcome!

**当有人问"支持哪些 LLM？"**

> The intent engine supports OpenAI, Azure OpenAI, Ollama, LM Studio, vLLM, and any OpenAI-compatible endpoint. You can also use the offline keyword-matching fallback if you don't want to use any LLM at all.

---

#### 3. Hacker News (Show HN)

- **网址**: https://news.ycombinator.com/
- **标题**: Show HN: MCP Fusion — Visual AI workflow builder for MCP, 100% offline desktop app
- **内容**:

```
I built a desktop app for visually orchestrating AI tool workflows using the Model Context Protocol (MCP). It supports all three MCP transport protocols (stdio, SSE, Streamable HTTP) and runs entirely offline on your local machine.

Key features:

🎨 Visual canvas editor (React Flow) — drag, connect, and execute workflows
🧠 LLM-powered intent engine — describe what you want in natural language, it builds the workflow
🔗 Multi-protocol MCP support — mix stdio, SSE, and Streamable HTTP servers in one workflow
⚡ Production-grade runtime — circuit breaker, rate limiter, idempotency, exponential backoff
📊 Observability — OpenTelemetry tracing + Prometheus metrics built-in
🔒 Security — RBAC, AES-256-GCM encryption, tamper-evident audit trail
🖥️ Cross-platform — Windows, macOS, Linux (Tauri 2.x + Rust + React)

Everything runs locally. No cloud. No telemetry. Your API keys and data stay on your machine.

Demo: https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif
GitHub: https://github.com/chungkung/mcp-fusion
Downloads: https://github.com/chungkung/mcp-fusion/releases

Feedback welcome! What would make this more useful for your workflow?
```

**📸 配图建议**:

HN 不支持直接嵌入图片，但在正文中放置了 GIF 链接。可以额外在评论区第一条回复中附上截图链接：

| 序号 | 文件 | 链接 |
|------|------|------|
| 1 | `demo-workflow.gif` | 已在正文中 |
| 2 | `screenshot-canvas.png` | 评论区补充 |
| 3 | `screenshot-intent.png` | 评论区补充 |

---

#### 4. Reddit — r/rust

- **网址**: https://www.reddit.com/r/rust/
- **标题**: [Project] MCP Fusion — A Tauri 2.x desktop app for visual AI workflow orchestration
- **内容**:

```
I built MCP Fusion using Tauri 2.x + Rust backend + React frontend. It's a desktop app for visually orchestrating AI tool workflows using the Model Context Protocol (MCP).

![MCP Fusion Canvas](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-canvas.png)

**Rust-specific highlights:**

- 🦀 stdio MCP transport via `tokio::process::Command`
- 📡 SSE client with `reqwest` streaming
- 🗄️ SQLite storage via `rusqlite` with incremental migrations
- 🔍 OpenTelemetry tracing with OTLP export
- 🔐 AES-256-GCM encryption for secrets at rest
- 📊 Prometheus metrics with process-level stats
- ⚡ Production patterns: circuit breaker, rate limiter, idempotency keys

**Architecture highlights:**

![MCP Config](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-mcp-config.png)

- Tauri IPC layer bridges frontend ↔ Rust backend
- Gateway module handles all MCP transport negotiation
- Topological sort scheduler with parallel execution support
- Tamper-evident audit trail using SHA-256 hash chaining

The frontend uses React Flow for the canvas, Zustand for state, and Framer Motion for animations.

GitHub: https://github.com/chungkung/mcp-fusion
27 frontend tests + Rust unit tests for gateway and storage modules.

Would appreciate code review from the Rust community! The Tauri IPC layer and gateway module might be particularly interesting.
```

**📸 配图建议**:

| 序号 | 文件 | 说明 |
|------|------|------|
| 1 | `screenshot-canvas.png` | 主画布界面，展示完整工作流 |
| 2 | `screenshot-mcp-config.png` | MCP 配置界面，展示 Rust 后端能力 |

---

#### 5. Reddit — r/programming

- **网址**: https://www.reddit.com/r/programming/
- **标题**: MCP Fusion: An open-source desktop app to visually build AI tool workflows
- **内容**:

```
I built an open-source desktop app that lets you visually orchestrate AI tool workflows using the Model Context Protocol (MCP).

![Demo Workflow](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif)

Think n8n, but for AI toolchains. Think Dify, but running entirely offline on your desktop.

**What it does:**

- 🎨 Drag-and-drop visual canvas (React Flow) with real-time execution status
- 🧠 Natural language → workflow generation via LLM intent engine
- 🔗 Supports all 3 MCP transport protocols (stdio, SSE, Streamable HTTP)
- 📊 Built-in OpenTelemetry tracing + Prometheus metrics
- 🛡️ Production-grade: circuit breaker, rate limiter, idempotency, RBAC, audit trail

**Why I built it:**

Existing AI workflow tools (n8n, Dify, LangFlow) all require server deployment and cloud accounts. I wanted something that runs locally, respects privacy, and supports MCP natively.

![Intent Engine](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-intent.png)

Built with **Tauri 2.x (Rust) + React + React Flow + SQLite**.

Open source (AGPL-3.0), cross-platform (Windows/macOS/Linux).

GitHub: https://github.com/chungkung/mcp-fusion
```

**📸 配图建议**:

| 序号 | 文件 | 说明 |
|------|------|------|
| 1 | `demo-workflow.gif` | 最吸引眼球的动图，展示核心交互 |
| 2 | `screenshot-intent.png` | 展示自然语言生成工作流能力 |

---

#### 6. Reddit — r/LocalLLaMA

- **网址**: https://www.reddit.com/r/LocalLLaMA/
- **标题**: MCP Fusion — AI workflow builder that runs 100% offline with local LLM support
- **内容**:

```
Sharing a project that might interest the local AI community.

MCP Fusion is a desktop app for building AI tool workflows. It runs entirely offline and supports local LLMs via Ollama, LM Studio, vLLM, or any OpenAI-compatible endpoint.

![Intent Engine with Local LLM](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-intent.png)

**Key things for the local-first crowd:**

- 🔒 100% offline — SQLite storage, no cloud, no telemetry
- 🦙 Supports Ollama / LM Studio / vLLM for intent parsing
- 🔐 AES-256-GCM encrypted API keys and secrets at rest
- 🔗 All MCP tools run locally (stdio protocol)
- 📊 Built-in Prometheus metrics + OpenTelemetry tracing
- 🖥️ Cross-platform (Windows/macOS/Linux)

![MCP Config](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-mcp-config.png)

**Offline fallback:** If you don't want to use any LLM at all, the intent engine has a keyword-matching fallback that works completely offline.

GitHub: https://github.com/chungkung/mcp-fusion
```

**📸 配图建议**:

| 序号 | 文件 | 说明 |
|------|------|------|
| 1 | `screenshot-intent.png` | 展示 LLM 意图解析面板 |
| 2 | `screenshot-mcp-config.png` | 展示本地 MCP 工具配置 |

---

#### 7. Reddit — r/selfhosted

- **网址**: https://www.reddit.com/r/selfhosted/
- **标题**: MCP Fusion — Self-hosted AI workflow orchestration (desktop app, fully offline)
- **内容**:

```
For the self-hosted community — a desktop app that lets you build AI tool workflows with zero cloud dependency.

![MCP Fusion Canvas](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-canvas.png)

**Zero cloud. Zero telemetry. Zero accounts.**

- 🏠 All data stored in local SQLite (WAL mode)
- 🦙 Supports local LLMs (Ollama, LM Studio, vLLM)
- 🔧 MIT-licensed MCP tools from the community
- 📊 Built-in monitoring — Prometheus metrics + OpenTelemetry
- 🖥️ Cross-platform desktop app (Windows/macOS/Linux)
- 🔐 RBAC, AES-256-GCM, audit trail

![Metrics Dashboard](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-metrics.jpg)

Think of it as n8n for AI toolchains, but running entirely on your machine.

GitHub: https://github.com/chungkung/mcp-fusion
```

**📸 配图建议**:

| 序号 | 文件 | 说明 |
|------|------|------|
| 1 | `screenshot-canvas.png` | 主画布界面 |
| 2 | `screenshot-metrics.jpg` | 可观测性面板，突出自托管监控能力 |

---

### 🟡 P1 — 高优先级

---

#### 8. GitHub Trending — 上榜策略

GitHub Trending 是自然流量最大的来源之一。上榜后日增 Stars 可达 50-200+。

##### 8.1 上榜条件

| 条件 | 说明 |
|------|------|
| 仓库语言 | 至少包含一种主要语言（Rust + TypeScript 已满足） |
| 仓库年龄 | 无硬性限制，但太新（< 1 周）可能权重较低 |
| Star 增速 | 这是核心指标。需要在短时间内（1-2 天）获得集中增长 |
| README 质量 | 非硬性条件，但高质量 README 能提高转化率，间接促进增速 |

##### 8.2 上榜策略

**时间策略：**

- **最佳发布时间**：**周一上午**（北京时间）。GitHub Trending 按周计算，周一发布可以积累整周的 Star 增量
- **备选时间**：周日晚上发布，让 Star 在周一凌晨开始积累

**星标增速策略：**

- 周一当天集中所有渠道推送（Product Hunt、Reddit、HN、掘金、V2EX 等同步发力）
- 目标：周一当天获得 100+ Stars
- 持续 3 天保持增速，基本可以稳定在 Trending 榜单

**README 质量优化：**

- [ ] 顶部放置 `demo-workflow.gif` 作为 Hero 图
- [ ] 清晰的 Badge 行（Stars、License、Downloads、Build Status）
- [ ] 3 句话以内说清楚项目是什么
- [ ] 特性列表使用 emoji 图标 + 简短描述
- [ ] 包含快速开始指南（Quick Start）
- [ ] 包含架构图或截图
- [ ] 包含 Roadmap 和 Contributing 指南

##### 8.3 执行清单

- [ ] 周一上午 8:00（北京时间）— 仓库 README 最终优化完成
- [ ] 周一上午 9:00 — 在 Reddit（r/rust, r/programming, r/LocalLLaMA, r/selfhosted）同步发帖
- [ ] 周一上午 10:00 — 掘金文章发布
- [ ] 周一中午 12:00 — V2EX 发布
- [ ] 周一下午 — 知乎回答发布
- [ ] 周一晚上 — Twitter/X 线程发布
- [ ] 周二 — 检查 Trending 状态，如果未上榜则补充 HN 和 Dev.to

##### 8.4 持续上榜

- 每次大版本发布时，重复上述策略
- 确保每次发布都有 Release Notes 和 Changelog
- 每次发布后 24 小时内集中推广

**📸 配图建议（README 优化）**:

| 序号 | 文件 | 放置位置 |
|------|------|----------|
| 1 | `demo-workflow.gif` | README 顶部 Hero 区域 |
| 2 | `screenshot-canvas.png` | 特性介绍区域 |
| 3 | `screenshot-intent.png` | 意图引擎区域 |
| 4 | `screenshot-metrics.jpg` | 可观测性区域 |
| 5 | `screenshot-mcp-config.png` | 配置指南区域 |

---

#### 9. 掘金 (juejin.cn)

- **网址**: https://juejin.cn/
- **标题**: 我开源了一个 MCP 可视化工作流编排桌面应用 — 支持全协议、零云依赖
- **标签**: 前端, Rust, 开源, AI
- **内容**:

```

## 背景

最近 MCP（Model Context Protocol）生态越来越火，但我发现市面上缺少一个能**可视化编排** MCP 工具的工作流工具。n8n、Dify 都需要云端部署，而且不支持 MCP 原生协议。于是我花了几个月时间，用 Tauri 2.x + Rust + React 做了一个完全离线的桌面应用。

![MCP Fusion 工作流画布](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif)

## 项目亮点

### 🎨 可视化画布

基于 React Flow 实现的拖拽式画布，可以像搭积木一样连接 MCP 工具：

- 拖拽工具到画布 → 连接节点 → 一键执行
- 实时状态反馈（pending → running → success → failed）
- 撤销/重做、自动布局、导出/导入

![画布主界面](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-canvas.png)

### 🧠 LLM 意图解析

用自然语言描述需求，AI 自动生成工作流：

- 支持 OpenAI / Azure / Ollama / LM Studio / vLLM
- 离线关键词匹配降级方案
- 多轮对话优化

![意图解析面板](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-intent.png)

### 🔗 全协议 MCP 支持

首个同时支持三种 MCP 传输协议的桌面应用：

- **stdio** — 子进程通信（本地工具）
- **SSE** — 服务端推送
- **Streamable HTTP** — 最新 MCP 规范

![MCP 配置界面](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-mcp-config.png)

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

![权限管理](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-permissions.png)

### 📊 可观测性

- OpenTelemetry 分布式追踪
- Prometheus 指标采集
- 内置 Metrics Dashboard

![可观测性面板](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-metrics.jpg)

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

**📸 配图建议**:

| 序号 | 文件 | 放置位置 |
|------|------|----------|
| 1 | `demo-workflow.gif` | 封面图 / 文章顶部 |
| 2 | `screenshot-canvas.png` | 可视化画布章节 |
| 3 | `screenshot-intent.png` | LLM 意图解析章节 |
| 4 | `screenshot-mcp-config.png` | 全协议 MCP 支持章节 |
| 5 | `screenshot-permissions.png` | 安全合规章节 |
| 6 | `screenshot-metrics.jpg` | 可观测性章节 |

---

#### 10. 知乎

- **网址**: https://www.zhihu.com/
- **问题选题**:
  - "如何评价 MCP Fusion 这个开源项目？"
  - "有没有好用的 MCP 可视化编排工具推荐？"
  - "如何看待 AI 工作流编排工具的未来？"
- **回答内容**:

```

最近发现了一个很有意思的开源项目——**MCP Fusion**，一个完全离线的 MCP 可视化工作流编排桌面应用。

![MCP Fusion 工作流演示](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif)

简单来说，它可以让你**像搭积木一样**，用拖拽的方式把各种 MCP 工具（搜索、文件操作、API 调用等）串联成自动化工作流。

## 为什么我觉得它值得关注

### 1. 真正的离线运行

现在市面上的 AI 工作流工具（n8n、Dify、Coze）基本都需要云端部署。MCP Fusion 是一个**桌面应用**，所有数据存在本地 SQLite 里，API Key 用 AES-256-GCM 加密存储。零遥测、零云端依赖。

### 2. 全协议 MCP 支持

这是目前我见过唯一一个同时支持 **stdio、SSE、Streamable HTTP** 三种 MCP 传输协议的桌面应用。这意味着你可以混合使用本地 MCP 工具和远程 MCP 服务。

![MCP 配置](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-mcp-config.png)

### 3. 自然语言生成工作流

最打动我的一点：你只需要用自然语言描述需求，比如"帮我查一下今天的天气，然后把结果发到 Slack"，它就能自动生成对应的工作流。

![意图解析](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-intent.png)

### 4. 生产级的工程质量

- 熔断器、速率限制器、幂等键、指数退避重试
- RBAC 权限控制
- 防篡改审计哈希链
- OpenTelemetry 追踪 + Prometheus 指标

![可观测性](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-metrics.jpg)

### 技术栈

Tauri 2.x (Rust) + React 18 + React Flow + SQLite

### 开源地址

https://github.com/chungkung/mcp-fusion

总的来说，如果你对 MCP 生态感兴趣，或者想找一个能本地运行的 AI 工作流工具，这个项目值得关注和 Star。
```

**📸 配图建议**:

| 序号 | 文件 | 放置位置 |
|------|------|----------|
| 1 | `demo-workflow.gif` | 回答顶部，吸引点击 |
| 2 | `screenshot-mcp-config.png` | MCP 协议支持段落 |
| 3 | `screenshot-intent.png` | 自然语言生成段落 |
| 4 | `screenshot-metrics.jpg` | 可观测性段落 |

---

#### 11. V2EX

- **网址**: https://www.v2ex.com/
- **节点**: `/go/create` (分享创造)
- **标题**: [分享创造] MCP Fusion — 一个完全离线的 MCP 可视化工作流编排桌面应用
- **内容**:

```
最近搞了个开源项目，用 Tauri 2.x + Rust + React 做了一个 MCP 可视化工作流编排桌面应用。

![工作流画布](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif)

核心特性：

- 🎨 拖拽式画布编辑器（React Flow），支持实时状态反馈
- 🧠 自然语言描述需求 → AI 自动生成工作流
- 🔗 支持 stdio / SSE / Streamable HTTP 三种 MCP 协议
- 🔒 100% 离线运行，数据不进云端，零遥测
- 📊 内置 OpenTelemetry 追踪 + Prometheus 指标
- 🛡️ RBAC + AES-256-GCM 加密 + 审计哈希链

![意图解析](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-intent.png)

跨平台：Windows / macOS / Linux

技术栈：Tauri 2.x (Rust) + React 18 + React Flow + SQLite

![MCP 配置](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-mcp-config.png)

GitHub: https://github.com/chungkung/mcp-fusion

欢迎体验和反馈 🙏
```

**📸 配图建议**:

| 序号 | 文件 | 说明 |
|------|------|------|
| 1 | `demo-workflow.gif` | 帖子顶部，展示核心交互 |
| 2 | `screenshot-intent.png` | 特性介绍后，展示 AI 能力 |
| 3 | `screenshot-mcp-config.png` | 技术栈前，展示配置能力 |

---

#### 12. 即刻 / 小红书

##### 12.1 即刻

**文案（短版）**:

```
开源了一个 MCP 可视化工作流编排桌面应用 🚀

🎨 拖拽连线搭工作流，像搭积木一样
🧠 用自然语言描述需求，AI 自动生成工作流
🔒 100% 离线运行，数据不进云端
🔗 支持 stdio / SSE / Streamable HTTP 全协议

Tauri 2.x + Rust + React 构建
GitHub: https://github.com/chungkung/mcp-fusion

#开源 #MCP #AI工具 #独立开发
```

**📸 配图建议**:

| 序号 | 文件 | 说明 |
|------|------|------|
| 1 | `demo-workflow.gif` | 首图，动图最吸引眼球 |
| 2 | `screenshot-intent.png` | 第二张，展示 AI 能力 |
| 3 | `screenshot-canvas.png` | 第三张，展示完整画布 |

##### 12.2 小红书

**文案（小红书风格）**:

```
标题：独立开发｜我做了一个 AI 工作流可视化工具，完全离线 🎨

正文：

花了好几个月，终于把我的开源项目做出来了！

MCP Fusion —— 一个可视化的 AI 工作流编排桌面应用 ✨

简单来说就是：
👉 拖拖拽拽就能搭建 AI 工作流
👉 用大白话描述需求，AI 自动帮你生成
👉 支持各种 MCP 工具（搜索、文件、API 等）
👉 最重要的：完全离线！数据就不出你的电脑

技术栈：Tauri 2.x + Rust + React + React Flow

GitHub 搜 "mcp-fusion" 就能找到～
欢迎 Star ⭐ 和体验！

#独立开发者 #开源项目 #AI工具 #MCP #工作流自动化 #桌面应用 #Rust
```

**📸 配图建议**（小红书支持 9 图）:

| 序号 | 文件 | 说明 |
|------|------|------|
| 1 | `logo.jpg` | 封面图，Logo + 项目名称 |
| 2 | `demo-workflow.gif` | 动图展示核心交互 |
| 3 | `screenshot-canvas.png` | 完整画布截图 |
| 4 | `screenshot-intent.png` | 意图解析面板 |
| 5 | `screenshot-mcp-config.png` | MCP 配置界面 |
| 6 | `screenshot-marketplace.png` | 工具市场 |
| 7 | `screenshot-metrics.jpg` | 可观测性面板 |
| 8 | `screenshot-permissions.png` | 权限管理 |
| 9 | `demo-execution.gif` | 工作流执行过程 |

> 小红书图片建议 3:4 竖版比例，可以在截图上方加文字标题。封面图用 Canva 制作，加上 "MCP Fusion" 标题 + "可视化 AI 工作流" 副标题。

---

### 🟢 P2 — 中优先级

---

#### 13. Twitter/X 线程（5 条推文故事线）

**第 1 条 — 问题引入 + 项目展示**

```
I was frustrated with AI workflow tools.

They all need cloud accounts. They send your data to third parties. None of them support MCP natively.

So I built my own: MCP Fusion 🚀

A visual AI workflow builder that runs 100% offline on your desktop.

🧵👇
```

**📸 配图**: `demo-workflow.gif` — 展示核心拖拽连线体验

---

**第 2 条 — 核心功能展示**

```
Here's what makes it different:

🎨 Drag-and-drop visual canvas (React Flow)
🧠 Describe what you want in natural language → AI builds the workflow
🔗 All 3 MCP protocols: stdio / SSE / Streamable HTTP
🔒 100% offline — your data never leaves your machine

No cloud. No accounts. No telemetry.
```

**📸 配图**: `screenshot-intent.png` — 展示自然语言 → 工作流的能力

---

**第 3 条 — 技术深度**

```
Under the hood:

🦀 Tauri 2.x + Rust backend (tokio, rusqlite, reqwest)
⚛️ React 18 + React Flow + Zustand
🗄️ SQLite (WAL mode) for local storage
📊 OpenTelemetry tracing + Prometheus metrics
🔐 AES-256-GCM encryption + RBAC + audit trail

Production-grade reliability baked in.
```

**📸 配图**: `screenshot-mcp-config.png` — 展示 MCP 配置和技术架构

---

**第 4 条 — 竞品对比 + 差异化**

```
Think n8n for AI toolchains. Think Dify without the cloud.

Key differences:
✅ Desktop app — no Docker, no Kubernetes
✅ Native MCP support — all 3 transport protocols
✅ Offline-first — local LLMs via Ollama/LM Studio/vLLM
✅ Built-in observability — no extra setup

Open source (AGPL-3.0).
```

**📸 配图**: `screenshot-metrics.jpg` — 展示可观测性面板

---

**第 5 条 — Call to Action**

```
MCP Fusion is live on GitHub.

⭐ Star the repo: github.com/chungkung/mcp-fusion
⬇️ Download: github.com/chungkung/mcp-fusion/releases

Cross-platform: Windows, macOS, Linux.

If you're building with MCP or AI toolchains, I'd love your feedback!

RT to support open source 🙏
```

**📸 配图**: `screenshot-canvas.png` — 以完整画布截图收尾，给人留下印象

**📸 线程总配图建议**:

| 推文 | 文件 | 说明 |
|------|------|------|
| 1 | `demo-workflow.gif` | 开场吸引眼球 |
| 2 | `screenshot-intent.png` | 展示 AI 能力 |
| 3 | `screenshot-mcp-config.png` | 展示技术深度 |
| 4 | `screenshot-metrics.jpg` | 展示可观测性 |
| 5 | `screenshot-canvas.png` | 结尾 Call to Action |

---

#### 14. Dev.to

- **网址**: https://dev.to/
- **内容**: 使用掘金内容，翻译为英文
- **标签**: `#rust` `#tauri` `#ai` `#opensource` `#mcp`

**📸 配图建议**: 与掘金一致，使用相同的 6 张截图/GIF。

---

#### 15. HackerNoon / Medium

- **内容**: 撰写一篇技术深度文章："Building a Desktop AI Workflow Orchestrator with Tauri and Rust"
- **大纲**:
  1. Why desktop? The case for offline-first AI tools
  2. Architecture overview (Tauri IPC, Rust backend, React frontend)
  3. MCP protocol handling (stdio, SSE, Streamable HTTP)
  4. Production reliability patterns (circuit breaker, rate limiter, idempotency)
  5. Observability with OpenTelemetry and Prometheus
  6. Lessons learned and future roadmap

**📸 配图建议**:

| 序号 | 文件 | 放置位置 |
|------|------|----------|
| 1 | `screenshot-canvas.png` | 开头，展示成品 |
| 2 | `screenshot-mcp-config.png` | MCP 协议章节 |
| 3 | `screenshot-metrics.jpg` | 可观测性章节 |
| 4 | `demo-workflow.gif` | 结尾，动态展示 |

---

#### 16. LinkedIn

- **内容**: 发布到 LinkedIn Articles，使用英文版内容
- **标题**: MCP Fusion: Building a Visual AI Workflow Orchestrator with Tauri and Rust
- **风格**: 专业、技术向，突出工程能力和开源贡献

**📸 配图建议**:

| 序号 | 文件 | 说明 |
|------|------|------|
| 1 | `logo.jpg` | 文章封面 |
| 2 | `demo-workflow.gif` | 正文顶部 |
| 3 | `screenshot-canvas.png` | 特性介绍 |

---

#### 17. Discord / Slack 社区

- MCP 官方 Discord: https://discord.gg/mcp
- Tauri Discord: https://discord.gg/tauri
- Rust 中文社区
- 各种 AI 工具群

**简短文案**:

```
Hey everyone! I built MCP Fusion — a visual AI workflow builder for MCP that runs 100% offline as a desktop app.

🎨 Drag-and-drop canvas
🧠 Natural language → workflow
🔗 stdio / SSE / Streamable HTTP
🔒 Zero cloud, zero telemetry

GitHub: https://github.com/chungkung/mcp-fusion
Demo: https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif

Would love feedback!
```

**📸 配图建议**: 在 Discord 消息中直接粘贴 `demo-workflow.gif` 链接（Discord 会自动展开预览）。

---

## 第三步：持续运营

### 3.1 每周更新
- 在 GitHub 发布 Release Notes
- 在 Reddit / V2EX / 掘金 发布更新日志
- 每次更新配一张新功能截图

### 3.2 内容营销
- 撰写技术博客（如何用 Tauri 构建桌面应用）
- 录制 YouTube 教程
- 制作对比视频（vs n8n, vs Dify）

### 3.3 社区互动
- 在 GitHub Issues 快速响应
- 在 Reddit / Discord 回答相关问题
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

---

## 附录：素材快速引用

```
# Logo
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/logo.jpg)

# 动图
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif)
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-intent.gif)
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-execution.gif)

# 截图
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-canvas.png)
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-intent.png)
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-mcp-config.png)
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-metrics.jpg)
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-marketplace.png)
![](https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-permissions.png)
```