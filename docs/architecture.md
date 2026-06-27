# MCP Fusion 架构设计文档

## 1. 概述

MCP Fusion 是一个基于 Tauri 2.x 构建的多协议融合编排平台，采用 **Rust 后端 + React 前端** 的桌面应用架构，支持通过 MCP（Model Context Protocol）协议对多种外部服务进行可视化工作流编排与执行。

### 技术栈

| 层级 | 技术选型 |
|------|----------|
| 桌面框架 | Tauri 2.x |
| 后端语言 | Rust (Edition 2021) |
| 前端框架 | React 18 + TypeScript 5.6 |
| 构建工具 | Vite 6 |
| 状态管理 | Zustand 4 |
| 可视化 | React Flow (@xyflow/react) |
| 样式方案 | Tailwind CSS 3 + Framer Motion |
| 数据库 | SQLite (rusqlite 0.31, bundled) |
| 异步运行时 | Tokio 1 |
| 可观测性 | tracing + OpenTelemetry + Prometheus |
| 加密 | AES-256-GCM + SHA-256 |

---

## 2. 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                     Tauri 桌面应用                           │
│  ┌──────────────────────┐  ┌──────────────────────────────┐ │
│  │   React 前端层        │  │       Rust 后端层            │ │
│  │  (src-frontend/)     │  │     (src-tauri/src/)         │ │
│  │                      │  │                              │ │
│  │  pages/              │  │  gateway/   ← MCP 协议网关   │ │
│  │  components/         │  │  orchestrator/ ← 工作流引擎  │ │
│  │  stores/             │  │  storage/    ← SQLite 持久化 │ │
│  │  services/ipc.ts     │  │  llm.rs      ← LLM 意图解析  │ │
│  │                      │  │  marketplace.rs ← 插件市场   │ │
│  │  ── invoke() ────────┼──│  metrics.rs  ← Prometheus    │ │
│  │  ── listen() ────────│──│  tracing_otel.rs ← OTel      │ │
│  │                      │  │  crypto.rs   ← 加密模块      │ │
│  └──────────────────────┘  └──────────────────────────────┘ │
│                                                             │
│  ┌──────────────────────────────────────────────────────────┐│
│  │              src-shared/  共享类型定义                     ││
│  │    types/common.ts  types/mcp.ts  types/workflow.ts       ││
│  │    types/ipc.ts     types/marketplace.ts                  ││
│  │    constants.ts     error-codes.txt                       ││
│  └──────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

### 2.1 前端层 (src-frontend/)

前端采用 **React 18 + React Router 6** 单页应用架构，使用路由懒加载和 Zustand 状态管理。

**路由结构：**

| 路径 | 页面 | 功能 |
|------|------|------|
| `/intent` | IntentPage | 意图模式：自然语言生成工作流 |
| `/canvas` | CanvasMode | 画布模式：可视化拖拽编排 |
| `/code` | CodeMode | 代码模式：直接编辑工作流 JSON |
| `/marketplace` | Marketplace | 插件市场：模板浏览与安装 |
| `/settings` | Settings | 系统设置：MCP 服务器管理 |
| `/audit-logs` | AuditLogs | 审计日志查看 |
| `/executions` | ExecutionHistory | 执行历史记录 |

**组件层次：**

```
App.tsx
├── MainLayout.tsx              # 主布局（侧边栏 + 内容区）
│   ├── RouteLayout             # 路由动画（AnimatePresence）
│   │   ├── IntentPage          # 意图模式
│   │   ├── CanvasMode/
│   │   │   ├── Toolbar.tsx     # 工具栏
│   │   │   ├── ToolPanel.tsx   # 工具面板
│   │   │   ├── PropertyPanel   # 属性面板
│   │   │   ├── CanvasNode.tsx  # 画布节点
│   │   │   └── useCanvasStore  # 画布状态
│   │   ├── CodeMode            # 代码模式
│   │   ├── Marketplace         # 插件市场
│   │   ├── Settings            # 系统设置
│   │   ├── AuditLogs           # 审计日志
│   │   └── ExecutionHistory    # 执行历史
│   └── animations/
│       ├── LoadingSpinner.tsx
│       ├── PageTransition.tsx
│       └── ParticleBackground.tsx
```

**状态管理 (Zustand Stores)：**

- `useGlobalStore` -- 全局状态（主题、侧边栏、加载/错误状态）
- `useWorkflowStore` -- 工作流 CRUD 与执行
- `useMCPStore` -- MCP 服务器管理、工具列表、连接状态
- `useCanvasStore` -- 画布模式下的节点/边操作

### 2.2 后端层 (src-tauri/src/)

Rust 后端通过 Tauri 的 `#[tauri::command]` 宏暴露 IPC 接口，前端通过 `invoke()` 调用。

**模块结构：**

```
src-tauri/src/
├── main.rs            # 入口点，条件编译 tauri-runtime 特性
├── lib.rs             # 核心：Tauri 命令、认证、速率限制、熔断器
├── gateway/           # MCP 协议网关
│   ├── mod.rs         # 统一 McpClient 枚举
│   ├── stdio.rs       # Stdio 传输实现
│   ├── sse.rs         # SSE 传输实现
│   └── streamable_http.rs  # Streamable HTTP 传输实现
├── orchestrator/      # 工作流编排引擎
│   ├── mod.rs         # 执行结果类型定义
│   └── scheduler.rs   # 调度器核心（拓扑排序、执行模式）
├── storage/           # 持久化层
│   ├── mod.rs         # Storage trait 抽象接口
│   └── sqlite.rs      # SQLite 实现（CRUD、迁移、审计链）
├── llm.rs             # LLM 意图解析（OpenAI-compatible）
├── marketplace.rs     # 插件市场（模板仓库、安装管理）
├── metrics.rs         # Prometheus 指标
├── tracing_otel.rs    # OpenTelemetry 分布式追踪
└── crypto.rs          # 加密工具（AES-256-GCM、SHA-256、日志脱敏）
```

### 2.3 共享层 (src-shared/)

前后端共享的类型定义，使用 TypeScript 接口与 Rust 结构体通过 `serde` 序列化保持同步。

---

## 3. MCP 传输协议

MCP Fusion 通过统一的 `McpClient` 枚举封装三种 MCP 传输协议：

### 3.1 Stdio 协议

**适用场景：** 本地命令行工具（如 `npx`, `python`, `uvx`）

**实现方式：** `gateway/stdio.rs`

```
流程：
1. 启动子进程（Command::new），管道连接 stdin/stdout/stderr
2. 发送 JSON-RPC 2.0 初始化请求（initialize）
3. 发送 initialized 通知
4. 通过 stdin 发送请求，逐行读取 stdout 解析响应
5. 后台任务持续读取 stderr 并记录日志
6. Drop 时自动 kill 子进程
```

**JSON-RPC 消息格式：**

```json
// 请求
{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}

// 响应
{"jsonrpc":"2.0","id":1,"result":{"tools":[...]}}

// 通知（无 id 字段）
{"jsonrpc":"2.0","method":"initialized","params":{}}
```

### 3.2 SSE 协议

**适用场景：** 远程 HTTP 服务，支持服务器推送事件

**实现方式：** `gateway/sse.rs`

```
流程：
1. GET /sse → 获取 SSE 流，首个事件包含 message endpoint URL
2. POST {endpoint} → 发送 JSON-RPC 请求
3. SSE 流 → 接收服务器推送的 JSON-RPC 响应
4. 通过 oneshot 通道按请求 ID 分发响应
5. 后台 tokio 任务持续读取 SSE 流
```

### 3.3 Streamable HTTP 协议

**适用场景：** 新一代 MCP 传输协议，支持流式响应

**实现方式：** `gateway/streamable_http.rs`

```
流程：
1. POST /mcp → 发送 JSON-RPC 请求
2. 支持 Accept: text/event-stream 进行流式响应
3. 支持 Accept: application/json 进行普通请求-响应
4. 捕获 Mcp-Session-Id 响应头用于会话管理
```

### 3.4 统一客户端接口

```rust
pub enum McpClient {
    Stdio(stdio::StdioClient),
    Sse(sse::SseClient),
    StreamableHttp(streamable_http::StreamableHttpClient),
}

impl McpClient {
    pub async fn list_tools(&mut self) -> Result<Vec<McpToolInfo>>;
    pub async fn call_tool(&mut self, tool_name: &str, arguments: Value) -> Result<ToolCallResult>;
}
```

---

## 4. 三种工作流模式

### 4.1 Intent 模式（意图模式）

通过自然语言描述自动生成工作流：

1. 收集所有已配置 MCP 服务器的工具信息
2. 优先使用 LLM（OpenAI-compatible API）解析意图
3. LLM 不可用时回退到关键词匹配算法
4. 支持多轮对话细化工作流（`refine_workflow`）
5. 支持工具推荐（`recommend_tools`）

**LLM 配置：** 通过环境变量 `LLM_API_KEY`、`LLM_API_URL`、`LLM_MODEL` 等配置。

### 4.2 Canvas 模式（画布模式）

基于 React Flow 的可视化拖拽编排：

- 节点类型：`mcpTool`（MCP 工具调用节点）
- 连线类型：`smoothstep`（贝塞尔曲线），支持动画
- 画布操作：拖拽节点、连线、属性编辑
- 自动布局：节点按位置 (x, y) 排列
- 实时状态推送：通过 Tauri 事件 `node-state-change` 推送节点执行状态

### 4.3 Code 模式（代码模式）

直接编辑工作流的 JSON 定义，适合高级用户和批量操作。

---

## 5. SQLite 存储层

### 5.1 数据库表结构

```sql
-- MCP 服务器配置
CREATE TABLE mcp_servers (
    id TEXT PRIMARY KEY, name TEXT NOT NULL, description TEXT,
    protocol TEXT NOT NULL DEFAULT 'stdio',
    endpoint TEXT, command TEXT, args TEXT, env TEXT,
    enabled INTEGER DEFAULT 1, created_at INTEGER, updated_at INTEGER
);

-- 工作流定义
CREATE TABLE workflows (
    id TEXT PRIMARY KEY, name TEXT NOT NULL, description TEXT,
    mode TEXT DEFAULT 'canvas', status TEXT DEFAULT 'idle',
    nodes TEXT DEFAULT '[]', edges TEXT DEFAULT '[]',
    locked INTEGER DEFAULT 0, locked_at INTEGER,
    created_at INTEGER, updated_at INTEGER
);

-- 审计日志（含哈希链防篡改）
CREATE TABLE audit_logs (
    id TEXT PRIMARY KEY, action TEXT, resource TEXT, detail TEXT,
    prev_hash TEXT, chain_hash TEXT, created_at INTEGER
);

-- 工作流执行记录
CREATE TABLE workflow_executions (
    id TEXT PRIMARY KEY, workflow_id TEXT NOT NULL,
    status TEXT DEFAULT 'running',
    idempotency_key TEXT, completed_nodes TEXT DEFAULT '[]',
    started_at INTEGER, finished_at INTEGER,
    node_results TEXT DEFAULT '{}', error TEXT,
    FOREIGN KEY (workflow_id) REFERENCES workflows(id) ON DELETE CASCADE
);

-- 认证令牌
CREATE TABLE auth_tokens (
    id TEXT PRIMARY KEY, key_hash TEXT, label TEXT DEFAULT 'default',
    role TEXT DEFAULT 'admin', created_at INTEGER
);

-- 已安装模板
CREATE TABLE installed_templates (
    template_id TEXT, workflow_id TEXT, version TEXT, installed_at INTEGER,
    PRIMARY KEY (template_id, workflow_id)
);

-- Schema 版本管理
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### 5.2 数据库迁移

采用增量迁移策略，通过 `schema_version` 表追踪当前版本：

| 版本 | 内容 |
|------|------|
| v1 | 初始表结构 |
| v2 | 执行锁（locked/locked_at）、幂等键（idempotency_key）、断点续传（completed_nodes） |
| v3 | RBAC 角色字段（auth_tokens.role） |
| v4 | 审计日志哈希链字段（prev_hash/chain_hash） |

### 5.3 审计日志哈希链

每条审计日志通过 SHA-256 哈希链保证防篡改：

```
chain_hash = SHA-256(id + action + resource + detail + prev_hash + created_at)
```

提供 `verify_audit_chain()` 方法验证整条链的完整性，用于合规审计。

### 5.4 执行锁机制

防止同一工作流并发执行：

- `acquire_workflow_lock()` -- 原子获取锁，返回是否成功
- `release_workflow_lock()` -- 正常释放
- `force_release_workflow_lock()` -- 管理员强制释放（死锁恢复）

### 5.5 幂等执行

通过 `workflow_id + idempotency_key` 唯一组合保证同一请求只执行一次，重复请求返回已有结果。

### 5.6 断点续传

失败的工作流可以从上次失败的节点恢复执行，跳过已完成的节点，注入其输出供下游使用。

### 5.7 Storage 抽象接口

```rust
pub trait Storage: Send + Sync {
    // Server CRUD, Workflow CRUD, Execution Lock, Audit Log,
    // Execution, Auth
}
```

当前仅实现 SQLite，未来可扩展 PostgreSQL 等。

---

## 6. IPC 通信机制

### 6.1 前端 → 后端

前端通过 `invokeIPC()` 封装函数调用 Tauri 命令：

```typescript
// src-frontend/src/services/ipc.ts
export async function invokeIPC<T = unknown>(
    channel: string,
    args?: Record<string, unknown>,
): Promise<IpcResult<T>> {
    if (!isTauri()) {
        // 浏览器模式：返回 Mock 数据
        return getMock(channel, args);
    }
    const { invoke } = await import("@tauri-apps/api/core");
    const data = await invoke<T>(channel, args);
    return { success: true, data };
}
```

**IPC 通道常量：** 定义在 `src-shared/constants.ts` 的 `IPC_CHANNELS` 对象中，涵盖工作流、MCP 服务器、意图解析、审计日志、认证、系统管理等。

### 6.2 后端 → 前端

后端通过 Tauri 事件系统推送实时状态：

```rust
// 节点状态变更事件
window.emit("node-state-change", NodeStateEvent {
    workflow_id, node_id, state, output, error, trace_id,
});
```

前端通过 `listenIPC()` 注册事件监听：

```typescript
const unlisten = await listenIPC<PayloadType>("event-name", handler);
```

### 6.3 浏览器降级

在纯浏览器环境中（非 Tauri 应用），IPC 调用自动降级返回 Mock 数据，支持前端独立开发调试。

---

## 7. 工作流编排引擎

### 7.1 拓扑排序

使用 Kahn 算法将节点按依赖层级分组，同层节点可并行执行。

```
输入: nodes=[A,B,C,D], edges=[A->C, B->C, C->D]
输出: layers=[[A,B], [C], [D]]
```

### 7.2 执行模式

| 模式 | 说明 |
|------|------|
| `Serial` | 逐节点串行执行 |
| `Parallel` | 同层节点并发执行（通过 Semaphore 控制并发数） |
| `Conditional` | 根据节点输出中的 `__allowed_next` 选择下游路径 |
| `Loop` | 重复执行指定次数，支持 `__loop_break` 提前终止 |

### 7.3 数据传递

支持模板引用语法 `${node_id.field.sub_field}`，自动解析上游节点的输出值：

```json
{
    "message": "${node1.result}",
    "user": "${node2.data.name}"
}
```

### 7.4 调度器配置

```rust
pub struct SchedulerConfig {
    pub timeout: Duration,          // 单节点超时（默认 30s）
    pub max_concurrency: usize,     // 最大并发数（默认 4）
    pub mode: ExecutionMode,        // 执行模式
    pub abort_on_error: bool,       // 失败时中断
    pub retry_count: u32,           // 重试次数（默认 3）
    pub retry_backoff_ms: u64,      // 退避时间（默认 500ms）
}
```

### 7.5 连接池

执行期间共享 MCP 客户端连接池（`Arc<Mutex<HashMap<String, Arc<Mutex<McpClient>>>>>`），避免重复建立连接。

---

## 8. 认证与授权

### 8.1 认证机制

- 基于 SHA-256 哈希的 API Key 认证
- 支持 Key 生成、设置、验证、清除
- API Key 持久化到 SQLite `auth_tokens` 表
- 支持从环境变量加载持久化密钥

### 8.2 RBAC 角色模型

| 角色 | 权限范围 |
|------|----------|
| **Admin** | 全部权限，可切换角色、管理备份 |
| **Developer** | 服务器管理、工作流 CRUD、执行、意图解析 |
| **Viewer** | 只读：查看服务器、工作流、执行记录、健康检查 |

### 8.3 速率限制

针对关键操作实施速率限制：

| 操作 | 限制 |
|------|------|
| 执行工作流 | 5 次/分钟 |
| 执行工具 | 30 次/分钟 |
| 列出工具 | 20 次/分钟 |
| 认证验证 | 5 次/分钟 |
| 健康检查 | 10 次/分钟 |

### 8.4 熔断器

当某 MCP 服务器连续失败 3 次后自动熔断 30 秒，保护系统稳定性。

---

## 9. 可观测性

### 9.1 结构化日志

- 控制台输出：带线程 ID 和行号
- 文件输出：JSON 格式，按天轮转（`mcp-fusion.log`）
- 日志级别：通过 `RUST_LOG` 环境变量控制
- 敏感数据脱敏：API Key、Token、密码等自动替换为 `[REDACTED]`

### 9.2 Prometheus 指标

| 指标 | 类型 | 说明 |
|------|------|------|
| `mcp_fusion_workflow_executions_total` | CounterVec | 工作流执行次数（按状态） |
| `mcp_fusion_workflow_execution_duration_seconds` | HistogramVec | 工作流执行耗时 |
| `mcp_fusion_node_executions_total` | CounterVec | 节点执行次数（按状态） |
| `mcp_fusion_node_execution_duration_seconds` | HistogramVec | 节点执行耗时 |
| `mcp_fusion_mcp_tool_calls_total` | CounterVec | 工具调用次数（按状态） |
| `mcp_fusion_mcp_tool_call_duration_seconds` | HistogramVec | 工具调用耗时 |
| `mcp_fusion_mcp_server_connection_status` | Gauge | 服务器连接状态 |
| `mcp_fusion_active_workflows` | IntGauge | 当前活跃工作流数 |

通过 `metrics` IPC 命令导出 Prometheus 文本格式。

### 9.3 OpenTelemetry 分布式追踪

- 通过 OTLP HTTP/protobuf 导出到 Jaeger/Tempo 等后端
- 配置环境变量：`OTEL_EXPORTER_OTLP_ENDPOINT`、`OTEL_SERVICE_NAME`
- 三级 Span 插桩：`workflow_execute` → `node_execute` → `mcp_tool_call`
- 每个工作流执行分配独立的 `trace_id`（UUID v4）
- 程序退出时自动刷新待发送数据

---

## 10. 加密

### 10.1 敏感数据加密

- 算法：AES-256-GCM（认证加密）
- 密钥派生：SHA-256(密码短语 + 固定盐值)
- 随机 Nonce（12 字节），Base64 编码存储
- 加密范围：MCP 服务器的 `env` 字段（API Key 等敏感配置）

### 10.2 加密密钥管理

通过环境变量 `MCP_FUSION_ENCRYPTION_KEY` 配置，生产环境必须设置。

---

## 11. 插件市场

### 11.1 模板仓库

- 远程仓库：GitHub Raw 文件（`index.json` + `{id}/{version}/template.json`）
- 内置模板：8 个离线可用模板（API 聚合、文件处理、Git 流水线等）
- 远程不可用时自动回退到内置模板

### 11.2 模板安装

- 从模板创建工作流并保存到数据库
- 记录安装历史（`installed_templates` 表）
- 支持版本更新检查
- 审计日志记录安装操作

---

## 12. 应用生命周期

```
main() → 初始化日志 → 打开数据库 → 运行迁移 → 创建 Tauri 应用
    → 注册所有命令 → 绑定 AppState → 应用运行
    → 退出时：中止运行的工作流 → 刷新 OTel 数据 → 清理子进程
```

数据库路径：
- Windows: `%APPDATA%/mcp-fusion/mcp_fusion.db`
- macOS/Linux: `$HOME/.local/share/mcp-fusion/mcp_fusion.db`