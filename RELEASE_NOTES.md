# MCP Fusion v0.1.4 — Visual Polish & GitHub Ready

## 🎨 What's New

### Visual Overhaul
- **Animated demo GIFs** — 3 new 840px-wide GIFs showing full app window (canvas, intent, execution)
- **English README** — completely redesigned with hero CTA buttons, feature cards, project stats, and testimonials section
- **Chinese README (README_zh.md)** — fully synced with English version, all sections translated
- **Comparison table** — MCP Fusion vs n8n, Dify, LangChain, Flowise across 12 dimensions

### Testing & Quality
- **27 frontend tests** — Vitest + React Testing Library, all passing ✅
- **Rust backend tests** — gateway module (JSON-RPC + factory) + storage module (SQLite CRUD, migrations, audit chain)
- **CI/CD pipeline** — Prettier format check, Vitest, npm audit, cargo audit, cargo clippy

### Documentation  
- **Architecture Guide** (`docs/architecture.md`) — full system design with transport protocols, storage, IPC, orchestration engine, security, observability
- **Development Guide** (`docs/development.md`) — setup, build, test, debug instructions
- **API Reference** (`docs/api.md`) — 40+ IPC commands, JSON-RPC format, RBAC matrix, event types
- **Pull Request Template** — Rust + Frontend checklist
- **Dependabot config** — auto-updates for npm, Cargo, GitHub Actions

### GitHub Community
- **SUPPORT.md** — documentation links, community channels, issue reporting guide
- **PROMOTION.md** — 15-platform launch strategy with ready-to-paste posts (Product Hunt, HN, Reddit, 掘金, V2EX, 知乎, etc.)
- **One-page pitch deck** (`docs/pitch.md`)
- **Repository setup script** (`scripts/github-setup.ps1`)

### Bug Fixes
- Fix ExecutionHistory duplicate `totalPages` declaration causing 404 error
- Fix `JsonRpcRequest` visibility for test access (`pub(crate)`)
- Remove broken audit trail screenshot with Tauri env error

### Chores
- GitHub repository fully configured: 20 topics, description, discussions enabled, branch protection on `main`
- Updated README badges (tests, CI, license, platform)

---

## 📦 Assets

| Platform | File |
|----------|------|
| **Windows x64** | `mcp-fusion_0.1.4_x64_en-US.msi` |
| **Windows x64 (NSIS)** | `mcp-fusion_0.1.4_x64-setup.exe` |
| **macOS Apple Silicon** | `mcp-fusion_0.1.4_aarch64.dmg` |
| **macOS Intel** | `mcp-fusion_0.1.4_x64.dmg` |
| **Linux AppImage** | `mcp-fusion_0.1.4_amd64.AppImage` |
| **Linux deb** | `mcp-fusion_0.1.4_amd64.deb` |
| **Linux rpm** | `mcp-fusion_0.1.4_x86_64.rpm` |

---

**Full Changelog**: [v0.1.3...v0.1.4](https://github.com/chungkung/mcp-fusion/compare/v0.1.3...v0.1.4)