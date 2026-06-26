<p align="center">
  <img src="https://raw.githubusercontent.com/mcp-fusion/mcp-fusion/main/docs/assets/logo.png" alt="MCP Fusion Logo" width="200" />
</p>

<h1 align="center">MCP Fusion</h1>

<p align="center">
  <strong>MCP 生态的可视化融合编排桌面应用</strong>
</p>

<p align="center">
  <a href="https://github.com/mcp-fusion/mcp-fusion/actions"><img src="https://github.com/mcp-fusion/mcp-fusion/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
  <a href="https://github.com/mcp-fusion/mcp-fusion/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-AGPL--3.0-blue.svg" alt="License" /></a>
  <a href="https://github.com/mcp-fusion/mcp-fusion/releases"><img src="https://img.shields.io/github/v/release/mcp-fusion/mcp-fusion" alt="Release" /></a>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-brightgreen" alt="Platform" />
  <img src="https://img.shields.io/badge/rust-1.77%2B-orange" alt="Rust" />
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="#快速开始">快速开始</a> ·
  <a href="https://mcp-fusion.app/docs">文档</a> ·
  <a href="#架构">架构</a>
</p>

---

## MCP Fusion 是什么？

**MCP Fusion** 是一个基于 [Model Context Protocol (MCP)](https://modelcontextprotocol.io) 的**桌面原生可视化编排平台**。它让你能够跨多个 MCP 服务器组合、执行和监控多工具工作流——全部在本地桌面应用中运行，零云依赖。

> 像 **n8n** 一样编排，像 **Dify** 一样智能，**100% 本地运行**。

---

## 为什么选择 MCP Fusion？

| 特性 | MCP Fusion | n8n | Dify | LangChain |
|------|-----------|-----|------|-----------|
| **原生 MCP 协议** | ✅ 全协议 stdio/SSE/HTTP | ❌ | ❌ | ❌（仅 SDK） |
| **桌面应用** | ✅ Tauri (Rust) | ❌ 纯 Web | ❌ 纯 Web | ❌ |
| **可视化画布** | ✅ React Flow | ✅ | ✅ | ❌ |
| **LLM 意图解析** | ✅ OpenAI/本地模型 | ❌ | ✅ | ❌ |
| **分布式追踪** | ✅ OpenTelemetry | ❌ | ❌ | ✅ |
| **离线优先** | ✅ 本地 SQLite | ❌ | ❌ | ❌ |
| **插件市场** | ✅ GitHub/GitLab | ✅ | ❌ | ❌ |
| **跨平台** | ✅ Win/Mac/Linux | ✅ Web | ✅ Web | ❌ |

---

## 核心功能

### 可视化工作流编排
- **拖拽式画布**，基于 React Flow 的直观工作流设计
- **代码模式**，为高级用户提供 JSON/YAML 编辑
- 执行过程中实时节点状态可视化

### MCP 协议融合
- 支持全部三种 MCP 传输协议：**stdio**、**SSE**、**Streamable HTTP**
- 连接任意 MCP 兼容服务器（Filesystem、GitHub、Postgres、Brave Search 等）
- 自动工具发现和 Schema 内省

### LLM 驱动的意图引擎
- **自然语言 → 工作流**转换，兼容 OpenAI API
- **多轮对话**细化，迭代构建工作流
- **自动工具推荐**，根据任务描述智能匹配
- 未配置 LLM 时自动回退到关键词匹配

### 生产级运行时
- **拓扑排序调度器**，支持并行执行
- **熔断器模式**，故障隔离
- **速率限制器**，防止 API 滥用
- **幂等键**，安全重试
- **审计追踪**，加密链验证

### 内置可观测性
- **Prometheus 指标**（工作流执行、工具调用、服务器状态）
- **OpenTelemetry 分布式追踪**（OTLP 导出到 Jaeger/Tempo）
- **结构化 JSON 日志**（tracing-subscriber）
- 实时执行监控面板

### 插件市场
- **远程模板仓库**（GitHub/GitLab 集成）
- **一键安装**，从市场到本地工作流
- **版本管理**，更新通知
- **8 个内置模板**，离线可用

### 安全与合规
- **RBAC**（管理员 / 开发者 / 观察者）
- **API Key 认证**，加密存储（AES-256-GCM）
- **数据库加密**，保护敏感服务器配置
- **审计日志**，防篡改链验证

---

## 快速开始

### 环境要求
- **Node.js** >= 20
- **Rust** >= 1.77
- **Windows** / **macOS** / **Linux**

### 从 Release 安装

从 [Releases](https://github.com/mcp-fusion/mcp-fusion/releases) 下载最新安装包：

| 平台 | 格式 |
|------|------|
| Windows | `.msi` / `.exe` (NSIS) |
| macOS | `.dmg` (Apple Silicon / Intel) |
| Linux | `.AppImage` / `.deb` / `.rpm` |

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/mcp-fusion/mcp-fusion.git
cd mcp-fusion

# 安装依赖
npm install

# 开发模式运行
npm run tauri:dev

# 构建生产版本
npm run build:win    # Windows
npm run build:mac    # macOS
npm run build:linux  # Linux
```

### 配置 LLM（可选）

```bash
# 设置环境变量以启用 LLM 意图解析
export LLM_API_KEY="sk-your-openai-api-key"
export LLM_MODEL="gpt-4o-mini"

# 或使用本地模型（Ollama）
export LLM_API_URL="http://localhost:11434/v1/chat/completions"
export LLM_MODEL="qwen2.5:7b"
```

---

## 架构

```
┌─────────────────────────────────────────────────────────┐
│                    MCP Fusion 桌面应用                    │
│  ┌───────────────────────────────────────────────────┐  │
│  │              前端 (React + TypeScript)             │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │ 意图解析  │ │ 画布编辑  │ │ 插件市场浏览      │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  └───────────────────┬───────────────────────────────┘  │
│                      │ IPC (Tauri 桥接)                  │
│  ┌───────────────────┴───────────────────────────────┐  │
│  │              后端 (Rust + Tauri)                   │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │ LLM      │ │ 调度器    │ │ 网关             │  │  │
│  │  │ 引擎     │ │ (拓扑)    │ │ (stdio/SSE/HTTP) │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │ 指标     │ │ 追踪     │ │ 插件市场          │  │  │
│  │  │ (Prom)   │ │ (OTel)   │ │ (GitHub/GitLab)  │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐  │  │
│  │  │ 认证     │ │ 熔断器   │ │ 速率限制器        │  │  │
│  │  │ (RBAC)   │ │          │ │                  │  │  │
│  │  └──────────┘ └──────────┘ └──────────────────┘  │  │
│  │  ┌──────────────────────────────────────────────┐ │  │
│  │  │           SQLite (本地存储)                   │ │  │
│  │  └──────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

---

## 技术栈

| 层级 | 技术 |
|------|------|
| **桌面框架** | [Tauri 2](https://tauri.app) (Rust) |
| **前端** | React 18, TypeScript, Tailwind CSS |
| **画布编辑器** | [React Flow](https://reactflow.dev) (xyflow) |
| **状态管理** | [Zustand](https://zustand-demo.pmnd.rs) |
| **动画** | [Framer Motion](https://www.framer.com/motion/) |
| **数据库** | SQLite (via [rusqlite](https://github.com/rusqlite/rusqlite)) |
| **指标** | [Prometheus](https://prometheus.io) (rust client) |
| **追踪** | [OpenTelemetry](https://opentelemetry.io) (OTLP) |
| **加密** | AES-256-GCM |
| **LLM** | OpenAI-compatible API |

---

## 路线图

- [x] MCP stdio/SSE/Streamable HTTP 传输协议
- [x] 可视化画布编辑器 (React Flow)
- [x] LLM 驱动的意图解析
- [x] 插件市场 (GitHub/GitLab)
- [x] OpenTelemetry 分布式追踪
- [x] Prometheus 指标
- [x] 熔断器与速率限制器
- [x] RBAC 与 API Key 认证
- [x] 跨平台打包 (Windows/macOS/Linux)
- [x] 自动更新 (Tauri updater)
- [ ] MCP Resource 与 Prompt 支持
- [ ] WebSocket 传输协议
- [ ] 团队协作（实时）
- [ ] 云同步（Pro 功能）
- [ ] 移动端配套应用

---

## 贡献

欢迎贡献！详见 [CONTRIBUTING.md](CONTRIBUTING.md)。

- **新手友好 Issue**：[标记为 `good first issue` 的 Issue](https://github.com/mcp-fusion/mcp-fusion/labels/good%20first%20issue)
- **Discord**：[加入社区](https://discord.gg/mcp-fusion)

---

## 许可证

MCP Fusion 使用 **GNU Affero General Public License v3.0 (AGPL-3.0)** 许可。

- **开源使用**：个人、学术及开源项目免费
- **商业使用**：需购买商业许可证。详情请[联系我们](mailto:license@mcp-fusion.app)

详见 [LICENSE](LICENSE) 全文。

---

## Star 历史

<p align="center">
  <a href="https://star-history.com/#mcp-fusion/mcp-fusion&Date">
    <img src="https://api.star-history.com/svg?repos=mcp-fusion/mcp-fusion&type=Date" alt="Star History Chart" width="80%" />
  </a>
</p>

---

<p align="center">
  Made with ❤️ by the MCP Fusion team
</p>