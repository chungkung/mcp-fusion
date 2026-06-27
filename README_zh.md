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

### 🎬 一睹为快

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif" alt="MCP Fusion 画布演示" width="840" />
  <br/>
  <em>可视化画布编辑器 — 拖拽、连接、执行 AI 工具工作流</em>
</p>

### 🎥 快速演示

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-workflow.gif" alt="工作流画布演示" width="840" />
  <br/>
  <em>1. 可视化画布 — 拖拽、连接、执行 AI 工具工作流</em>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-intent.gif" alt="LLM 意图解析演示" width="840" />
  <br/>
  <em>2. 自然语言 → AI 自动生成工作流</em>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/demo-execution.gif" alt="多视图导航演示" width="840" />
  <br/>
  <em>3. 无缝多视图切换与实时执行</em>
</p>

---

## ✨ 为什么选择 MCP Fusion？

| 能力 | MCP Fusion | n8n | Dify | LangChain | Flowise |
|------|-----------|-----|------|-----------|---------|
| **原生 MCP 协议** | ✅ 全协议 (stdio/SSE/HTTP) | ❌ | ❌ | ❌ (仅 SDK) | ❌ |
| **桌面应用** | ✅ Tauri (Rust) | ❌ 纯 Web | ❌ 纯 Web | ❌ | ❌ 纯 Web |
| **可视化画布** | ✅ React Flow | ✅ | ✅ | ❌ | ✅ |
| **LLM 意图解析** | ✅ OpenAI / 本地模型 | ❌ | ✅ | ❌ | ❌ |
| **分布式追踪** | ✅ OpenTelemetry | ❌ | ❌ | ✅ | ❌ |
| **离线优先** | ✅ 本地 SQLite | ❌ | ❌ | ❌ | ❌ |
| **插件市场** | ✅ GitHub / GitLab | ✅ | ❌ | ❌ | ✅ |
| **跨平台** | ✅ Win / Mac / Linux | ✅ Web | ✅ Web | ❌ | ✅ Web |
| **多协议传输** | ✅ 3 种协议 | ❌ | ❌ | ❌ | ❌ |
| **自动更新** | ✅ 内置支持 | ❌ | ❌ | ❌ | ❌ |
| **RBAC 与审计** | ✅ 内置 | ❌ | ❌ | ❌ | ❌ |
| **熔断器** | ✅ 生产级 | ❌ | ❌ | ❌ | ❌ |

---

## 🚀 核心功能

### 🎨 可视化工作流编排

无需编写一行代码即可设计复杂的 AI 工具管线。基于 **React Flow** 的画布提供完整的拖拽式控制，实时反馈执行状态。

- **拖拽式画布** — 可视化连接工具，无需代码
- **代码模式** — 支持 JSON/YAML 编辑，满足高级用户需求
- **实时节点状态可视化** — 执行过程中展示实时状态指示器（等待 → 运行中 → 成功 → 失败）
- **撤销/重做** — 完整的历史记录追踪，支持键盘快捷键
- **自动布局** — 智能节点定位，生成整洁的工作流图

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-canvas.png" alt="可视化画布编辑器" width="840" />
</p>

### 🧠 LLM 驱动的意图引擎

用自然语言描述你想要什么，MCP Fusion 自动为你构建工作流。支持 OpenAI 兼容 API，提供离线回退。

- **自然语言 → 工作流** — 秒级转换，描述你的目标即可
- **多轮对话细化** — 通过对话迭代优化工作流设计
- **智能工具推荐** — 基于任务描述自动匹配 MCP 工具
- **离线关键词匹配** — 未配置 LLM 时也能正常工作
- **支持模型**：OpenAI、Azure OpenAI、Ollama、LM Studio、vLLM 及任意 OpenAI 兼容端点

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-intent.png" alt="LLM 意图解析" width="840" />
  <br/>
  <em>用自然语言描述工作流 — AI 自动生成节点和连线</em>
</p>

### 🔗 MCP 协议融合

MCP Fusion 是首个**原生支持全部三种 MCP 传输协议**的桌面应用，可混合使用不同协议的服务器。

- **`stdio`** — 基于子进程的服务器（本地工具）
- **`SSE`** — 服务端推送事件，支持远程流式传输
- **`Streamable HTTP`** — 最新 MCP 规范，完整 HTTP 传输
- **自动发现** — 自动检测任意 MCP 兼容服务器的工具和 Schema
- **跨服务器编排** — 在单个工作流中连接 Filesystem、GitHub、Postgres、Brave Search 等
- **热重载** — 无需重启即可更新服务器配置

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-mcp-config.png" alt="MCP 服务器配置" width="477" />
  <br/>
  <em>管理 MCP 服务器 — 支持 stdio / SSE / Streamable HTTP 三种传输协议，自动发现与健康检查</em>
</p>

### ⚡ 生产级运行时

内置分布式系统工程中久经考验的可靠性模式。

- **拓扑排序调度器** — 确定最优执行顺序，最大化并行度
- **熔断器** — 隔离故障工具，防止级联失败
- **速率限制器** — 保护上游 API 免遭意外滥用
- **幂等键** — 安全重试，无重复副作用
- **指数退避重试** — 带抖动的优雅重试，应对瞬时故障
- **执行超时** — 每个节点独立设置截止时间

### 📊 内置可观测性

使用生产级监控能力，精确掌握工作流运行状态。

- **Prometheus 指标** — 工作流耗时、工具调用次数、成功率、服务器健康状态
- **OpenTelemetry 追踪** — 导出到 Jaeger、Tempo、Grafana 或任意 OTLP 后端
- **结构化 JSON 日志** — 基于 `tracing-subscriber`，支持可配置日志级别
- **实时执行监控面板** — 实时指标和 Span 可视化

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-metrics.jpg" alt="指标与追踪面板" width="600" />
  <br/>
  <em>实时指标、分布式追踪和服务器健康监控</em>
</p>

### 🧩 插件市场

在应用内发现、安装和分享工作流模板。

- **一键安装** — 浏览并安装来自 GitHub / GitLab 的模板
- **版本管理** — 追踪已安装插件的更新和变更日志
- **8 个内置模板** — 开箱即用的常见任务工作流（数据聚合、网页抓取、通知管线）
- **社区驱动** — 分享你的模板，发现他人的创作

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-marketplace.png" alt="插件市场" width="477" />
  <br/>
  <em>浏览、安装和管理社区工作流模板</em>
</p>

### 🔒 安全与合规

- **RBAC** — 管理员、开发者、观察者角色，细粒度权限控制
- **AES-256-GCM 加密** — API Key 和密钥静态加密存储
- **数据库加密** — 敏感配置加密存储在 SQLite 中
- **防篡改审计追踪** — 加密链式操作日志
- **零遥测** — 除非你配置了 OTLP 导出，否则零数据离开你的机器

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-permissions.png" alt="RBAC 与 API Key 管理" width="477" />
  <br/>
  <em>RBAC 角色管理、API Key 生成与细粒度权限控制</em>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/chungkung/mcp-fusion/main/docs/assets/screenshot-audit.png" alt="审计追踪" width="477" />
  <br/>
  <em>防篡改审计日志 — 加密链式操作记录</em>
</p>

---

## 📦 快速开始

### 环境要求

| 依赖 | 版本 | 检查命令 |
|------|------|----------|
| **Node.js** | ≥ 20 | `node --version` |
| **Rust** | ≥ 1.77 | `rustc --version` |
| **操作系统** | Windows 10+ / macOS 11+ / Linux | |

### 下载安装

| 平台 | 安装包 | 架构 |
|------|--------|------|
| **Windows** | `.msi` / `.exe` (NSIS) | x86_64 |
| **macOS** | `.dmg` | Apple Silicon (aarch64) · Intel (x86_64) |
| **Linux** | `.AppImage` · `.deb` · `.rpm` | x86_64 |

👉 [**下载最新版本**](https://github.com/chungkung/mcp-fusion/releases)

### 从源码构建

```bash
git clone https://github.com/chungkung/mcp-fusion.git
cd mcp-fusion
npm install

# 开发模式（热重载）
npm run tauri:dev

# 生产构建
npm run build:win    # Windows
npm run build:mac    # macOS
npm run build:linux  # Linux
```

### 配置 LLM（可选）

MCP Fusion 无需 LLM 即可手动构建工作流。启用 AI 生成功能：

```bash
# OpenAI / 兼容 API
export LLM_API_KEY="sk-your-api-key"
export LLM_MODEL="gpt-4o-mini"

# 本地模型 (Ollama)
export LLM_API_URL="http://localhost:11434/v1/chat/completions"
export LLM_MODEL="qwen2.5:7b"

# Azure OpenAI
export LLM_API_URL="https://your-resource.openai.azure.com"
export LLM_API_KEY="your-azure-key"
export LLM_MODEL="gpt-4o"
```

### 30 秒创建第一个工作流

1. **启动** MCP Fusion
2. **添加 MCP 服务器** — 在服务器面板点击 "+"，粘贴你的服务器配置
3. **输入你的目标** — 在意图面板中描述，例如 *"获取 Hacker News 前 5 条新闻并保存到文件"*
4. **审查** 自动生成的工作流
5. **点击运行** — 实时观察节点执行

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
| **桌面框架** | [Tauri 2](https://tauri.app) | 轻量（约 10MB）、Rust 驱动、跨平台 |
| **UI 框架** | React 18 + TypeScript | 类型安全、生态成熟、Vite 打包 |
| **样式** | Tailwind CSS | 原子化 CSS，快速迭代，暗色模式 |
| **画布** | [React Flow](https://reactflow.dev) | 久经考验的节点编辑器，40k+ GitHub Stars |
| **状态管理** | [Zustand](https://zustand-demo.pmnd.rs) | 极简样板代码，基于 Hook |
| **动画** | [Framer Motion](https://www.framer.com/motion/) | 声明式、高性能过渡动画 |
| **数据库** | SQLite via [rusqlite](https://github.com/rusqlite/rusqlite) | 零配置、嵌入式、ACID 事务 |
| **指标** | [Prometheus](https://prometheus.io) (rust) | 行业标准，基于拉取 |
| **追踪** | [OpenTelemetry](https://opentelemetry.io) | OTLP 导出，厂商中立 |
| **加密** | AES-256-GCM | 军事级加密，硬件加速 |
| **LLM** | OpenAI 兼容 API | 广泛模型支持，本地和云端 |
| **IPC** | Tauri Commands | Rust 原生，类型安全桥接 |

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

### 已完成 ✅
- [x] MCP stdio / SSE / Streamable HTTP 传输协议
- [x] 可视化画布编辑器 (React Flow)
- [x] LLM 驱动的意图解析 (OpenAI 兼容)
- [x] 插件市场 (GitHub / GitLab)
- [x] OpenTelemetry 分布式追踪
- [x] Prometheus 指标
- [x] 熔断器与速率限制器
- [x] RBAC 与 API Key 认证
- [x] 跨平台打包 (Windows / macOS / Linux)
- [x] 自动更新 (Tauri updater)

### 进行中 🚧
- [ ] MCP Resource 与 Prompt 支持
- [ ] WebSocket 传输协议
- [ ] 条件分支（if/else 节点）
- [ ] 循环/迭代节点

### 计划中 🎯
- [ ] 实时团队协作
- [ ] 云同步 (Pro)
- [ ] 移动端配套应用
- [ ] gRPC 传输协议
- [ ] 工作流版本历史与差异对比

---

## 🤝 贡献

欢迎各种形式的贡献 — 代码、文档、模板、Bug 报告和功能建议！

- 📖 **[CONTRIBUTING.md](CONTRIBUTING.md)** — 贡献指南与开发工作流
- 🐛 **[新手友好 Issue](https://github.com/chungkung/mcp-fusion/labels/good%20first%20issue)** — 入门级任务
- 💬 **[Discord](https://discord.gg/mcp-fusion)** — 加入社区

### 开发环境搭建

```bash
git clone https://github.com/chungkung/mcp-fusion.git
cd mcp-fusion
npm install
npm run tauri:dev
```

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
    <img src="https://api.star-history.com/svg?repos=chungkung/mcp-fusion&type=Date" alt="Star 历史" width="600" />
  </a>
</p>

---

<p align="center">
  <sub>由 MCP Fusion 团队用 Rust、TypeScript 和 ❤️ 构建</sub>
</p>