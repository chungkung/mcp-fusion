# Hacker News (Show HN) Post

**Title:** Show HN: MCP Fusion — Visual Workflow Builder for the Model Context Protocol

**Body:**

MCP Fusion is a cross-platform desktop app that brings visual workflow orchestration to the MCP ecosystem. It's the first desktop app to support all three MCP transports (stdio, SSE, Streamable HTTP) natively.

Key differentiators:
- 100% offline — runs entirely on your machine with SQLite
- LLM intent parsing — describe your workflow in natural language, get a working pipeline
- Production reliability — circuit breaker, rate limiter, retry with exponential backoff
- Built with Rust + Tauri 2 for a lightweight native experience

I built this because I was frustrated with having to write boilerplate code every time I wanted to chain MCP tools together. Would love to hear your thoughts!

https://github.com/chungkung/mcp-fusion