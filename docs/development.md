# MCP Fusion 开发指南

## 1. 环境要求

### 必需工具

| 工具 | 最低版本 | 说明 |
|------|----------|------|
| Node.js | 20.0.0+ | 前端运行时与构建 |
| pnpm | 最新稳定版 | 包管理器 |
| Rust | 1.75+ (Edition 2021) | 后端编译 |
| Cargo | 随 Rust 安装 | Rust 包管理器 |

### Windows 额外要求

- [Microsoft Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)（含 C++ 构建工具）
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)（Windows 10 version 1803+ 已预装）

### macOS 额外要求

- Xcode Command Line Tools: `xcode-select --install`

### Linux 额外要求

```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file \
  libappindicator-gtk3-devel librsvg2-devel
```

### 推荐 IDE 插件

- VS Code 扩展（见 `.vscode/extensions.json`）：
  - Rust Analyzer
  - Tauri
  - Tailwind CSS IntelliSense

---

## 2. 项目结构

```
mcp_fusion/
├── src-frontend/              # React 前端
│   ├── src/
│   │   ├── components/        # 通用组件
│   │   │   ├── animations/    # 动画组件（加载、过渡、粒子背景）
│   │   │   ├── canvas/        # 画布组件（节点、连线）
│   │   │   ├── layout/        # 布局组件（MainLayout）
│   │   │   └── ui/            # 基础 UI 组件（button, card, textarea）
│   │   ├── lib/               # 工具函数
│   │   ├── pages/             # 页面组件
│   │   │   ├── IntentPage.tsx
│   │   │   ├── CanvasMode/    # 画布模式（含 store）
│   │   │   ├── CodeMode/
│   │   │   ├── Marketplace/
│   │   │   ├── Settings/
│   │   │   ├── AuditLogs/
│   │   │   └── ExecutionHistory/
│   │   ├── services/          # IPC 服务层
│   │   │   └── ipc.ts         # 统一 IPC 调用封装
│   │   ├── stores/            # Zustand 状态管理
│   │   │   ├── useGlobalStore.ts
│   │   │   ├── useWorkflowStore.ts
│   │   │   └── useMCPStore.ts
│   │   ├── styles/            # 全局样式
│   │   ├── test/              # 测试文件
│   │   ├── App.tsx            # 应用入口
│   │   ├── main.tsx           # 渲染入口
│   │   └── routes.tsx         # 路由配置
│   ├── index.html             # HTML 入口
│   ├── vite.config.ts         # Vite 配置
│   ├── tailwind.config.js     # Tailwind CSS 配置
│   └── tsconfig.json          # TypeScript 配置
│
├── src-tauri/                 # Rust 后端
│   ├── src/
│   │   ├── main.rs            # 入口点
│   │   ├── lib.rs             # 核心模块（Tauri 命令、认证、限流）
│   │   ├── gateway/           # MCP 协议网关
│   │   │   ├── mod.rs         # 统一客户端接口
│   │   │   ├── stdio.rs       # Stdio 传输
│   │   │   ├── sse.rs         # SSE 传输
│   │   │   └── streamable_http.rs  # Streamable HTTP 传输
│   │   ├── orchestrator/      # 工作流编排引擎
│   │   │   ├── mod.rs         # 执行结果类型
│   │   │   └── scheduler.rs   # 调度器核心
│   │   ├── storage/           # 持久化层
│   │   │   ├── mod.rs         # Storage trait
│   │   │   └── sqlite.rs      # SQLite 实现
│   │   ├── llm.rs             # LLM 意图解析
│   │   ├── marketplace.rs     # 插件市场
│   │   ├── metrics.rs         # Prometheus 指标
│   │   ├── tracing_otel.rs    # OpenTelemetry 追踪
│   │   └── crypto.rs          # 加密与脱敏
│   ├── Cargo.toml             # Rust 依赖
│   ├── build.rs               # 构建脚本
│   ├── tauri.conf.json        # Tauri 配置
│   ├── capabilities/          # 权限声明
│   └── icons/                 # 应用图标
│
├── src-shared/                # 前后端共享
│   ├── types/                 # 共享类型定义
│   │   ├── common.ts          # 通用枚举、审计日志、执行记录
│   │   ├── mcp.ts             # MCP 服务器与工具
│   │   ├── workflow.ts        # 工作流节点与连线
│   │   ├── ipc.ts             # IPC 结果类型
│   │   ├── marketplace.ts     # 插件市场类型
│   │   └── index.ts           # 统一导出
│   ├── constants.ts           # 共享常量（IPC 通道、路由等）
│   └── error-codes.txt        # 错误码定义
│
├── docs/                      # 文档
│   ├── architecture.md        # 架构设计文档
│   ├── development.md         # 开发指南（本文档）
│   └── api.md                 # API 参考
│
├── examples/                  # 示例文件
│   ├── workflows/             # 示例工作流
│   └── mcp-servers.json       # 示例 MCP 服务器配置
│
├── package.json               # 项目配置与脚本
├── .env.example               # 环境变量模板
├── CHANGELOG.md               # 变更日志
├── CONTRIBUTING.md            # 贡献指南
└── README.md / README_zh.md   # 项目说明
```

---

## 3. 快速开始

### 3.1 安装依赖

```bash
# 克隆项目
git clone https://github.com/your-org/mcp-fusion.git
cd mcp-fusion

# 安装前端依赖
pnpm install
```

### 3.2 配置环境变量

```bash
# 复制环境变量模板
cp .env.example .env

# 编辑 .env 文件，配置 LLM（可选）
# LLM_API_KEY=sk-your-key-here
# LLM_API_URL=https://api.openai.com/v1/chat/completions
# LLM_MODEL=gpt-4o-mini

# 配置加密密钥（生产环境必须）
# MCP_FUSION_ENCRYPTION_KEY=your-secure-random-key

# 配置 OpenTelemetry（可选）
# OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4318/v1/traces
# OTEL_SERVICE_NAME=mcp-fusion
```

### 3.3 启动开发模式

```bash
# 同时启动前端 dev server 和 Tauri 桌面窗口
npm run tauri:dev
```

> 前端 dev server 运行在 `http://localhost:1420`，Tauri 会自动加载该地址。

### 3.4 纯前端开发（浏览器模式）

```bash
# 仅启动前端开发服务器（无需 Rust 编译）
npm run dev
```

浏览器模式下，所有 IPC 调用自动降级返回 Mock 数据，适合 UI 开发调试。

---

## 4. 构建与发布

### 4.1 开发构建

```bash
# 前端构建 + Tauri 发布构建
npm run build:all

# 仅前端构建
npm run build:frontend

# 仅 Tauri 构建（会先构建前端）
npm run tauri:build

# Tauri Debug 构建（更快，但体积更大）
npm run tauri:build:debug
```

### 4.2 平台特定构建

```bash
# Windows (x64)
npm run build:win

# macOS (Apple Silicon)
npm run build:mac

# macOS (Intel)
npm run build:mac:intel

# Linux (x64)
npm run build:linux

# Linux (ARM64)
npm run build:linux:arm64
```

### 4.3 构建产物

构建产物位于 `src-tauri/target/release/bundle/`：

| 平台 | 产物格式 |
|------|----------|
| Windows | `.msi` (WiX) / `.exe` (NSIS installer) |
| macOS | `.dmg` / `.app` |
| Linux | `.deb` / `.rpm` / `.AppImage` |

### 4.4 macOS 代码签名与公证

```bash
# 签名
npm run sign:mac

# 公证
npm run notarize:mac
```

> 签名前需修改 `package.json` 中的 `YOUR_TEAM`、`YOUR_APPLE_ID` 等占位符。

---

## 5. 测试

### 5.1 前端测试

```bash
# 运行所有测试
npm test

# 监听模式
npm run test:watch

# 测试覆盖率
npm run test:coverage
```

前端使用 **Vitest** + **Testing Library** + **jsdom** 进行单元测试。

### 5.2 Rust 测试

```bash
# 运行所有 Rust 测试
cargo test

# 运行特定模块测试
cargo test --lib gateway
cargo test --lib orchestrator
cargo test --lib storage

# 运行测试并显示输出
cargo test -- --nocapture
```

### 5.3 类型检查

```bash
# TypeScript 类型检查（不生成文件）
npm run lint
```

---

## 6. 代码格式化

```bash
# 前端格式化（Prettier）
npm run format

# Rust 格式化
cargo fmt
```

---

## 7. 项目配置说明

### 7.1 Tauri 配置 (`tauri.conf.json`)

| 配置项 | 说明 |
|--------|------|
| `build.devUrl` | 开发模式前端地址：`http://localhost:1420` |
| `build.frontendDist` | 构建产物目录：`../src-frontend/dist` |
| `app.windows` | 窗口配置：默认 1400x900，最小 900x600 |
| `app.security.csp` | 内容安全策略 |
| `bundle.targets` | 打包目标：`all` |
| `plugins.updater` | 自动更新公钥和更新端点 |

### 7.2 前端路径别名 (`vite.config.ts`)

```typescript
// 路径别名配置
"@": "./src",           // 前端源码
"@shared": "../src-shared"  // 共享类型
```

### 7.3 前后端类型同步

共享类型定义在 `src-shared/types/`，前端通过 `@shared/types` 导入，Rust 端在 `storage/sqlite.rs` 中定义了对应的结构体。两者通过 `serde` 的 `#[serde(rename = "camelCase")]` 保持字段名一致。

---

## 8. 数据库操作

### 8.1 数据库位置

- Windows: `%APPDATA%/mcp-fusion/mcp_fusion.db`
- 其他: `$HOME/.local/share/mcp-fusion/mcp_fusion.db`

### 8.2 备份与恢复

```bash
# 通过 IPC 命令备份（在应用内调用）
backup_database --backup-path "my_backup.db"

# 通过 IPC 命令恢复
restore_database --backup-path "my_backup.db"
```

---

## 9. 常用调试技巧

### 9.1 查看 Rust 日志

```bash
# 设置日志级别
$env:RUST_LOG="debug"  # PowerShell
export RUST_LOG=debug   # bash/zsh

# 日志文件位置
# Windows: %APPDATA%/mcp-fusion/logs/mcp-fusion.log
```

### 9.2 查看 Prometheus 指标

在应用内调用 `metrics` IPC 命令，返回 Prometheus 文本格式的指标数据。

### 9.3 验证审计链

在应用内调用 `verify_audit_chain` IPC 命令，检查审计日志哈希链的完整性。

### 9.4 浏览器 DevTools

在 Tauri 开发模式下，右键点击窗口可打开 Chrome DevTools 调试前端。

---

## 10. CI/CD

### 10.1 GitHub Actions

项目包含以下 CI 工作流（`.github/workflows/`）：

| 工作流 | 触发条件 | 说明 |
|--------|----------|------|
| `ci.yml` | Push / PR | 代码检查、测试、构建 |
| `release.yml` | Tag 推送 | 自动构建并发布各平台产物 |
| `stale.yml` | 定时 | 自动关闭过期 Issue/PR |

### 10.2 自动更新

应用通过 `tauri-plugin-updater` 支持自动更新：

```bash
# 生成更新签名密钥对
npm run gen:update-key
```

更新端点配置在 `tauri.conf.json` 的 `plugins.updater.endpoints` 中。

---

## 11. 贡献指南

1. Fork 项目并创建功能分支
2. 遵循现有代码风格（`cargo fmt` + `npm run format`）
3. 添加测试覆盖新功能
4. 确保 `npm test` 和 `cargo test` 全部通过
5. 提交 PR 到 `main` 分支

详见 `CONTRIBUTING.md`。