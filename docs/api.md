# MCP Fusion API 参考

## 1. 概述

MCP Fusion 通过 Tauri IPC 机制暴露 Rust 后端能力给前端。前端通过 `invoke(channel, args)` 调用命令，后端通过 `#[tauri::command]` 宏注册命令处理器。

所有命令返回 `Result<T, String>`，前端封装为 `IpcResult<T>`：

```typescript
// src-shared/types/ipc.ts
interface IpcResult<T = unknown> {
    success: boolean;
    data?: T;
    error?: string;
}
```

---

## 2. Tauri IPC 命令

### 2.1 MCP 服务器管理

#### `list_servers` -- 列出所有服务器

```
返回值: Vec<McpServer>
权限: server.list
```

#### `list_servers_paginated` -- 分页查询服务器

```
参数:
  offset: usize   -- 偏移量
  limit:  usize   -- 每页数量
返回值: PaginatedResult<McpServer>
权限: server.list
```

#### `add_server` -- 添加服务器

```
参数:
  server: McpServer   -- 服务器配置
返回值: McpServer
权限: server.add

校验规则:
  - server.id 不能为空
  - server.name 不能为空
  - protocol 必须是: stdio | sse | streamable-http
  - stdio 协议必须提供 command
  - sse/streamable-http 协议必须提供 endpoint
```

#### `get_server` -- 获取单个服务器

```
参数:
  id: String   -- 服务器 ID
返回值: Option<McpServer>
权限: server.get
```

#### `update_server` -- 更新服务器

```
参数:
  server: McpServer
返回值: McpServer
权限: server.update
```

#### `remove_server` -- 删除服务器

```
参数:
  id: String
返回值: ()
权限: server.remove
```

#### `ping_server` -- 检测服务器连通性

```
参数:
  server_id: String
返回值: { status: "connected"|"error", latency_ms: u64, tool_count?: usize, error?: String }
权限: server.ping
```

#### `list_tools` -- 列出服务器的 MCP 工具

```
参数:
  server_id: String
返回值: Vec<{ name: String, description: String, inputSchema: Value }>
权限: server.list_tools
速率限制: 20 次/分钟
熔断保护: 连续失败 3 次后熔断 30 秒
```

#### `execute_tool` -- 直接执行 MCP 工具

```
参数:
  server_id: String
  tool_name: String
  inputs:   Value (JSON 对象)
返回值: { content: Vec<ToolContent>, isError: bool }
权限: server.execute_tool
速率限制: 30 次/分钟
输入验证: inputs 必须是对象，大小不超过 1MB
```

---

### 2.2 工作流 CRUD

#### `list_workflows` -- 列出所有工作流

```
返回值: Vec<Workflow>
权限: workflow.list
```

#### `list_workflows_paginated` -- 分页查询工作流

```
参数:
  offset: usize
  limit:  usize
返回值: PaginatedResult<Workflow>
权限: workflow.list
```

#### `save_workflow` -- 保存工作流（创建或更新）

```
参数:
  workflow: Workflow
返回值: Workflow
权限: workflow.save

行为:
  - 原子化 upsert（ON CONFLICT DO UPDATE）
  - 自动维护 created_at / updated_at 时间戳
```

#### `get_workflow` -- 获取单个工作流

```
参数:
  id: String
返回值: Option<Workflow>
权限: workflow.get
```

#### `remove_workflow` -- 删除工作流

```
参数:
  id: String
返回值: ()
权限: workflow.remove
```

---

### 2.3 工作流执行

#### `execute_workflow` -- 执行工作流

```
参数:
  id:                 String           -- 工作流 ID
  idempotency_key:    Option<String>   -- 幂等键（可选）
  resume_from:        Option<String>   -- 断点续传：上次执行记录 ID（可选）
返回值: OrchestrationResult
权限: workflow.execute
速率限制: 5 次/分钟

执行流程:
  1. 权限检查
  2. 速率限制检查
  3. 输入验证（节点数不超过 200）
  4. 幂等检查（相同 idempotency_key 返回已有结果）
  5. 获取执行锁
  6. 解析断点续传的已完成节点
  7. 创建执行记录
  8. 拓扑排序 → 逐层执行
  9. 更新执行记录
  10. 释放执行锁
```

#### `retry_workflow` -- 断点续传重试

```
参数:
  id:                      String   -- 工作流 ID
  resume_from_execution_id: String  -- 上次失败的执行记录 ID
返回值: OrchestrationResult
权限: workflow.retry
```

#### `runtime_status` -- 查询运行时状态

```
返回值: { status: "running"|"idle", message: String }
权限: runtime.status
```

#### `runtime_stop` -- 停止运行中的工作流

```
参数:
  workflow_id: Option<String>  -- 指定工作流 ID（None 则停止全部）
返回值: ()
权限: runtime.stop
```

#### `force_release_lock` -- 强制释放执行锁

```
参数:
  workflow_id: String
返回值: ()
权限: workflow.force_unlock（仅 Admin）

用于解锁因异常导致死锁的工作流。
```

---

### 2.4 意图解析

#### `intent_parse` -- 解析自然语言意图

```
参数:
  text: String   -- 用户输入的自然语言描述
返回值: Workflow
权限: intent_parse

行为:
  1. 优先使用 LLM 解析（需配置 LLM_API_KEY）
  2. LLM 失败或未配置时回退到关键词匹配
```

#### `intent_parse_llm` -- 强制使用 LLM 解析

```
参数:
  text: String
返回值: Workflow
权限: intent_parse

与 intent_parse 的区别：不降级，LLM 失败直接返回错误。
```

#### `refine_workflow` -- 多轮对话细化工作流

```
参数:
  current_workflow_json: String       -- 当前工作流 JSON
  conversation_history:  Vec<Vec<String>>  -- 对话历史 [["user", "assistant"], ...]
  refinement_text:       String       -- 用户的细化描述
返回值: Workflow
权限: intent_parse
```

#### `recommend_tools` -- 推荐 MCP 工具

```
参数:
  text: String   -- 用户意图描述
返回值: Vec<String>   -- 推荐的工具 ID 列表（格式: "server_id.tool_name"）
权限: intent_parse
```

---

### 2.5 插件市场

#### `list_marketplace_templates` -- 列出市场模板

```
参数:
  category: Option<String>   -- 分类过滤
  search:   Option<String>   -- 搜索关键词
返回值: Vec<MarketplaceTemplate>
权限: workflow.list

远程仓库不可用时自动回退到内置模板。
```

#### `get_marketplace_template` -- 获取模板详情

```
参数:
  template_id: String
  version:     Option<String>   -- 版本号，默认 "1.0.0"
返回值: TemplateDetail（含完整工作流定义）
权限: workflow.list
```

#### `install_template` -- 安装模板

```
参数:
  template_id: String
  version:     Option<String>
返回值: Workflow（新创建的工作流）
权限: workflow.save

行为:
  1. 获取模板详情（远程或内置）
  2. 创建新工作流并保存
  3. 记录安装历史
  4. 审计日志
```

#### `check_template_updates` -- 检查模板更新

```
返回值: Vec<TemplateUpdate>
权限: workflow.list
```

---

### 2.6 审计日志

#### `list_audit_logs` -- 分页查询审计日志

```
参数:
  offset: usize
  limit:  usize
返回值: PaginatedResult<AuditLog>
权限: execution.list
```

#### `search_audit_logs` -- 搜索审计日志

```
参数:
  action:     Option<String>   -- 操作类型
  resource:   Option<String>   -- 资源
  start_time: Option<i64>      -- 开始时间戳（毫秒）
  end_time:   Option<i64>      -- 截止时间戳（毫秒）
  offset:     usize
  limit:      usize
返回值: PaginatedResult<AuditLog>
权限: execution.list
```

#### `verify_audit_chain` -- 验证审计链完整性

```
返回值: { total: usize, valid: usize, invalid: usize, intact: bool, details: Vec<String> }
权限: execution.list
```

---

### 2.7 执行记录

#### `list_executions` -- 查询工作流执行记录

```
参数:
  workflow_id: String
  offset:      usize
  limit:       usize
返回值: PaginatedResult<WorkflowExecution>
权限: execution.list
```

---

### 2.8 认证与授权

#### `auth_init` -- 初始化认证

```
返回值: { configured: bool }
权限: 无限制

从数据库加载持久化的 API Key。
```

#### `auth_generate_key` -- 生成 API Key

```
返回值: String（API Key 明文，仅返回一次）
权限: 无限制

行为: 生成 Key → SHA-256 哈希 → 持久化到数据库 → 审计日志
```

#### `auth_set_key` -- 手动设置 API Key

```
参数:
  key: String
返回值: ()
权限: 无限制
```

#### `auth_verify_key` -- 验证 API Key

```
参数:
  key: String
返回值: bool
权限: 无限制
速率限制: 5 次/分钟
```

#### `auth_status` -- 获取认证状态

```
返回值: { configured: bool, role: "admin"|"developer"|"viewer" }
权限: 无限制
```

#### `auth_clear_key` -- 清除 API Key

```
返回值: ()
权限: 无限制
```

#### `auth_set_role` -- 切换角色

```
参数:
  role: String   -- "admin" | "developer" | "viewer"
返回值: ()
权限: 仅 Admin 可操作
```

#### `auth_get_role` -- 获取当前角色及权限详情

```
返回值: {
    role: String,
    permissions: {
        servers:           bool,
        servers_manage:    bool,
        workflows:         bool,
        workflows_manage:  bool,
        workflows_execute: bool,
        executions:        bool,
        backup:            bool,
        restore:           bool,
    }
}
权限: 无限制
```

---

### 2.9 系统管理

#### `health_check` -- 健康检查

```
返回值: {
    status:    "ok",
    database:  bool,
    runtime:   "running"|"idle",
    version:   String,
    timestamp: i64,
}
权限: health
速率限制: 10 次/分钟
```

#### `backup_database` -- 备份数据库

```
参数:
  backup_path: Option<String>   -- 备份路径（可选，自动生成）
返回值: String（备份文件路径）
权限: backup（仅 Admin）

安全校验: 路径必须在备份目录内，禁止路径遍历。
```

#### `restore_database` -- 恢复数据库

```
参数:
  backup_path: String
返回值: ()
权限: restore（仅 Admin）

安全校验: 路径必须在备份目录内，禁止路径遍历。
```

#### `metrics` -- 导出 Prometheus 指标

```
返回值: String（Prometheus 文本格式）
权限: metrics.read（仅 Admin）
```

---

## 3. 共享类型定义

### 3.1 枚举

```typescript
// 运行状态
enum RunStatus {
    Idle = "idle",
    Running = "running",
    Success = "success",
    Failed = "failed",
    Timeout = "timeout",
}

// MCP 协议类型
enum MCPProtocol {
    Stdio = "stdio",
    Sse = "sse",
    StreamableHttp = "streamable-http",
    Http = "http",
    WebSocket = "websocket",
}

// 工作流模式
enum WorkflowMode {
    Intent = "intent",
    Canvas = "canvas",
    Code = "code",
}
```

### 3.2 MCP 服务器

```typescript
interface MCPServer {
    id: string;
    name: string;
    description: string;
    protocol: MCPProtocol;
    endpoint: string;          // SSE/HTTP 端点
    command: string;           // Stdio 命令
    args: string[];            // Stdio 参数
    env: Record<string, string>; // 环境变量（加密存储）
    enabled: boolean;
    connectionStatus: ConnectionStatus;  // connected|disconnected|connecting|error
    createdAt: number;
    updatedAt: number;
}

interface MCPTool {
    name: string;
    description: string;
    inputSchema: Record<string, unknown>;
    outputSchema: Record<string, unknown>;
    serverId: string;
}
```

### 3.3 工作流

```typescript
interface Workflow {
    id: string;
    name: string;
    description: string;
    mode: WorkflowMode;
    status: RunStatus;
    nodes: WorkflowNode[];
    edges: WorkflowEdge[];
    createdAt: number;
    updatedAt: number;
}

interface WorkflowNode {
    id: string;
    type: string;          // 节点类型，如 "mcpTool"
    position: WorkflowNodePosition;
    data: WorkflowNodeData;
}

interface WorkflowNodePosition {
    x: number;
    y: number;
}

interface WorkflowNodeData {
    label: string;
    tool?: MCPTool;        // 关联的 MCP 工具
    inputs: Record<string, unknown>;
    outputs: Record<string, unknown>;
    config: Record<string, unknown>;  // 如 { toolName, serverId, timeoutMs }
}

interface WorkflowEdge {
    id: string;
    source: string;
    target: string;
    sourceHandle?: string;
    targetHandle?: string;
    type?: string;         // 如 "smoothstep"
    animated?: boolean;
}
```

### 3.4 执行结果

```typescript
interface OrchestrationResult {
    workflow_id: string;
    execution_id: string;
    status: string;       // "success" | "failed"
    node_results: NodeResult[];
    error?: string;
}

interface NodeResult {
    node_id: string;
    status: string;       // "success" | "failed" | "skipped"
    output: unknown;
    error: string | null;
}

interface WorkflowExecution {
    id: string;
    workflowId: string;
    status: string;       // "running" | "success" | "failed" | "timeout" | "aborted"
    idempotencyKey?: string;
    completedNodes: unknown;  // JSON 数组，已完成节点 ID 列表
    startedAt: number;
    finishedAt: number | null;
    nodeResults: Record<string, unknown>;
    error: string | null;
}
```

### 3.5 审计日志

```typescript
interface AuditLog {
    id: string;
    action: string;       // 如 "workflow.execute", "server.add"
    resource: string;     // 如 "workflow:xxx", "server:xxx"
    detail: string;       // JSON 格式额外信息
    prevHash: string;     // 上一条日志的哈希
    chainHash: string;    // 当前日志的哈希（防篡改链）
    createdAt: number;
}
```

### 3.6 插件市场

```typescript
interface MarketplaceTemplate {
    id: string;
    name: string;
    description: string;
    category: string;
    version: string;
    author: string;
    stars: number;
    downloads: number;
    icon: string;
    node_count: number;
    edge_count: number;
    tags: string[];
    source_url: string;
    file_url: string;
    updated_at: string;
}

interface TemplateDetail {
    template: MarketplaceTemplate;
    workflow: WorkflowTemplate;
}

interface WorkflowTemplate {
    name: string;
    description: string;
    nodes: WorkflowNode[];
    edges: WorkflowEdge[];
}

interface TemplateUpdate {
    template_id: string;
    workflow_id: string;
    current_version: string;
    latest_version: string;
    has_update: boolean;
}
```

### 3.7 分页

```typescript
interface PaginatedResult<T> {
    items: T[];
    total: number;
    offset: number;
    limit: number;
}
```

---

## 4. MCP JSON-RPC 消息格式

### 4.1 请求格式

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "tools/list",
    "params": {}
}
```

### 4.2 响应格式

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "tools": [
            {
                "name": "http_request",
                "description": "发送 HTTP 请求",
                "inputSchema": { "type": "object", "properties": { "url": { "type": "string" } } }
            }
        ]
    }
}
```

### 4.3 错误响应

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "error": {
        "code": -32600,
        "message": "Invalid Request"
    }
}
```

### 4.4 通知（无 id）

```json
{
    "jsonrpc": "2.0",
    "method": "initialized",
    "params": {}
}
```

### 4.5 MCP 初始化握手

```
Client → Server:  initialize (protocolVersion, capabilities, clientInfo)
Server → Client:  initialize result (protocolVersion, capabilities, serverInfo)
Client → Server:  initialized (notification)
```

### 4.6 工具调用

```json
// 请求
{
    "jsonrpc": "2.0",
    "id": 2,
    "method": "tools/call",
    "params": {
        "name": "http_request",
        "arguments": { "url": "https://api.example.com/data" }
    }
}

// 响应
{
    "jsonrpc": "2.0",
    "id": 2,
    "result": {
        "content": [
            { "type": "text", "text": "{\"status\": 200, \"body\": \"...\"}" }
        ],
        "isError": false
    }
}
```

---

## 5. 认证令牌

### 5.1 API Key 管理

API Key 使用 SHA-256 哈希存储，原始 Key 仅在生成时返回一次。

```typescript
// 生成 Key
const { data: key } = await authService.generateKey();
// key = "550e8400-e29b-41d4-a716-446655440000"

// 存储哈希: hex(SHA-256(key))
```

### 5.2 RBAC 权限矩阵

| 操作 | Admin | Developer | Viewer |
|------|-------|-----------|--------|
| `server.list` | Y | Y | Y |
| `server.get` | Y | Y | Y |
| `server.add` | Y | Y | - |
| `server.update` | Y | Y | - |
| `server.remove` | Y | Y | - |
| `server.ping` | Y | Y | Y |
| `server.list_tools` | Y | Y | Y |
| `server.execute_tool` | Y | Y | - |
| `workflow.list` | Y | Y | Y |
| `workflow.get` | Y | Y | Y |
| `workflow.save` | Y | Y | - |
| `workflow.remove` | Y | Y | - |
| `workflow.execute` | Y | Y | - |
| `workflow.retry` | Y | Y | - |
| `workflow.force_unlock` | Y | Y | - |
| `execution.list` | Y | Y | Y |
| `runtime.status` | Y | Y | Y |
| `runtime.stop` | Y | Y | - |
| `health` | Y | Y | Y |
| `backup` | Y | Y | - |
| `restore` | Y | Y | - |
| `intent_parse` | Y | Y | - |

---

## 6. 事件

### 6.1 后端推送事件

后端通过 Tauri 事件系统向前端推送实时状态：

| 事件名 | 载荷类型 | 说明 |
|--------|----------|------|
| `node-state-change` | `NodeStateEvent` | 节点执行状态变更 |

```typescript
// NodeStateEvent 结构
interface NodeStateEvent {
    workflow_id: string;
    node_id: string;
    state: "idle" | "running" | "success" | "failed" | "skipped" | "timeout";
    output?: unknown;
    error?: string;
    trace_id: string;   // OpenTelemetry trace_id
}
```

### 6.2 前端监听示例

```typescript
import { listenIPC } from "@/services/ipc";

const unlisten = await listenIPC<NodeStateEvent>("node-state-change", (event) => {
    console.log(`节点 ${event.node_id} 状态: ${event.state}`);
});

// 取消监听
unlisten();
```

---

## 7. IPC 通道常量

```typescript
// src-shared/constants.ts
const IPC_CHANNELS = {
    // 工作流
    FLOW_LOAD: "list_workflows",
    FLOW_SAVE: "save_workflow",
    FLOW_DELETE: "remove_workflow",
    FLOW_EXECUTE: "execute_workflow",
    FLOW_GET: "get_workflow",

    // MCP 服务器
    MCP_LIST_SERVERS: "list_servers",
    MCP_ADD_SERVER: "add_server",
    MCP_REMOVE_SERVER: "remove_server",
    MCP_GET_SERVER: "get_server",
    MCP_PING_SERVER: "ping_server",

    // MCP 工具
    MCP_LIST_TOOLS: "list_tools",
    MCP_EXECUTE_TOOL: "execute_tool",

    // 意图解析
    INTENT_PARSE: "intent_parse",
    INTENT_PARSE_LLM: "intent_parse_llm",
    REFINE_WORKFLOW: "refine_workflow",
    RECOMMEND_TOOLS: "recommend_tools",

    // 运行时
    RUNTIME_STATUS: "runtime_status",
    RUNTIME_STOP: "runtime_stop",

    // 审计日志
    AUDIT_LIST: "list_audit_logs",
    AUDIT_SEARCH: "search_audit_logs",

    // 执行记录
    EXECUTIONS_LIST: "list_executions",

    // 插件市场
    MARKETPLACE_LIST: "list_marketplace_templates",
    MARKETPLACE_GET_TEMPLATE: "get_marketplace_template",
    MARKETPLACE_INSTALL: "install_template",
    MARKETPLACE_CHECK_UPDATES: "check_template_updates",

    // 认证与权限
    AUTH_INIT: "auth_init",
    AUTH_GENERATE_KEY: "auth_generate_key",
    AUTH_SET_KEY: "auth_set_key",
    AUTH_VERIFY_KEY: "auth_verify_key",
    AUTH_STATUS: "auth_status",
    AUTH_CLEAR_KEY: "auth_clear_key",
    AUTH_SET_ROLE: "auth_set_role",
    AUTH_GET_ROLE: "auth_get_role",

    // 系统管理
    METRICS: "metrics",
    HEALTH_CHECK: "health_check",
    BACKUP_DATABASE: "backup_database",
    RESTORE_DATABASE: "restore_database",
    FORCE_RELEASE_LOCK: "force_release_lock",
    RETRY_WORKFLOW: "retry_workflow",
    VERIFY_AUDIT_CHAIN: "verify_audit_chain",
};
```

---

## 8. 默认配置常量

```typescript
const DEFAULT_CONFIG = {
    pageSize: 20,           // 默认分页大小
    maxPageSize: 100,       // 最大分页大小
    requestTimeout: 30_000, // 请求超时（毫秒）
    flowMaxNodes: 200,      // 工作流最大节点数
    canvasGridSize: 20,     // 画布网格大小
    autosaveInterval: 30_000, // 自动保存间隔（毫秒）
};
```

---

## 9. 错误处理

所有 IPC 命令返回 `Result<T, String>`，错误信息通过 `IpcResult.error` 字段传递。常见错误类型：

| 错误类型 | 示例 |
|----------|------|
| 权限不足 | `权限不足：角色 'viewer' 不允许执行操作 'workflow.execute'` |
| 速率限制 | `操作 'execute_workflow' 请求过于频繁（5 次/60 秒），请稍后重试` |
| 熔断保护 | `服务 xxx 已被熔断保护，请稍后重试` |
| 输入验证 | `server name cannot be empty` |
| 资源不存在 | `Workflow not found: xxx` |
| 执行中 | `工作流 xxx 正在执行中，请等待当前执行完成后再试` |
| 循环依赖 | `工作流存在循环依赖，无法执行` |
| 无效边 | `工作流包含 N 条无效边，存在源或目标节点不存在的连接` |