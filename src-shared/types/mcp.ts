import { MCPProtocol } from "./common";

// ============================================================
// 接口定义
// ============================================================

/** MCP 服务端连接状态 */
export type ConnectionStatus = "connected" | "disconnected" | "connecting" | "error";

/** MCP 服务端配置 */
export interface MCPServer {
    id: string;
    name: string;
    description: string;
    protocol: MCPProtocol;
    endpoint: string;
    command: string;
    args: string[];
    env: Record<string, string>;
    enabled: boolean;
    connectionStatus: ConnectionStatus;
    createdAt: number;
    updatedAt: number;
}

/** MCP 工具定义 */
export interface MCPTool {
    name: string;
    description: string;
    inputSchema: Record<string, unknown>;
    outputSchema: Record<string, unknown>;
    serverId: string;
}