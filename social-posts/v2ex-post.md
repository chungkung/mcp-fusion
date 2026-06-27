# V2EX 分享创造 Post

**标题:** 【分享创造】MCP Fusion — 用可视化方式编排 AI 工具工作流的桌面应用

**正文:**

最近一直在做一个开源项目，今天终于可以分享了！

MCP Fusion 是一个跨平台桌面应用，可以让你用拖拽的方式连接各种 MCP 工具，构建 AI 工作流。类似于 n8n 但专为 MCP 协议设计。

核心亮点：
- 🎨 可视化拖拽画布，零代码编排 AI 工具链
- 🧠 自然语言描述需求，自动生成工作流
- 🔗 原生支持 stdio / SSE / Streamable HTTP 三种 MCP 传输协议
- ⚡ 内置熔断器、限流器、幂等键等生产级特性
- 📊 集成 OpenTelemetry 分布式追踪 + Prometheus 指标
- 🖥️ 基于 Rust + Tauri 2，完全本地运行，零云依赖

GitHub: https://github.com/chungkung/mcp-fusion
文档: 中英文 README 都有

欢迎 Star 和体验反馈！