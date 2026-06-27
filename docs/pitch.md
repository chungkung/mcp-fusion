# MCP Fusion — 一页纸简介

## 一句话
**MCP Fusion** 是一个完全离线的桌面应用，让你用拖拽的方式可视化构建和运行 AI 工具工作流。

## 三句话
1. 像搭积木一样拖拽 MCP 工具到画布，连接它们，一键执行 — 全程可视化。
2. 支持自然语言描述需求，AI 自动生成工作流，无需写一行代码。
3. 100% 本地运行，零云依赖，你的 API Key 和数据永远在你的机器上。

## 对标产品
MCP Fusion = n8n（工作流编排）+ Dify（AI 应用）+ Postman（API 工具）— 全部离线运行在桌面上

## 核心差异点
| | MCP Fusion | n8n | Dify |
|---|---|---|---|
| 桌面应用 | ✅ | ❌ | ❌ |
| MCP 原生支持 | ✅ 全协议 | ❌ | ❌ |
| 离线运行 | ✅ | ❌ | ❌ |
| 可视化编排 | ✅ | ✅ | ✅ |
| LLM 意图解析 | ✅ | ❌ | ✅ |
| 可观测性 | ✅ OTel+Prom | ❌ | ❌ |
| 安全合规 | ✅ RBAC+审计 | ❌ | ❌ |

## 目标用户
- AI 开发者：需要快速测试和组合 MCP 工具
- 安全敏感团队：数据不能离开本地环境
- 独立开发者：需要免费的离线 AI 工作流工具
- 企业用户：需要 RBAC、审计、加密等合规能力

## 技术亮点
- 首个同时支持 stdio / SSE / Streamable HTTP 三种 MCP 协议的桌面应用
- Tauri 2.x 架构，Rust 后端性能优异，内存占用 < 100MB
- React Flow 画布引擎，Zustand 状态管理，Framer Motion 动画
- SQLite + WAL 模式，支持增量迁移
- 生产级可靠性：熔断器、限流器、幂等键、指数退避

## 开源信息
- License: AGPL-3.0
- Stars 目标: 1000+
- 仓库: https://github.com/chungkung/mcp-fusion