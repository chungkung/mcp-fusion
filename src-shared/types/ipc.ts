/** IPC 调用结果 */
export interface IpcResult<T = unknown> {
    success: boolean;
    data?: T;
    error?: string;
}