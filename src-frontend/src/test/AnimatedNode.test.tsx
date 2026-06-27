import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { ReactFlowProvider } from "@xyflow/react";
import AnimatedNode, { type AnimatedNodeData } from "@/components/canvas/AnimatedNode";
import { RunStatus } from "@shared/types";

// 包裹 ReactFlowProvider（@xyflow/react 的 Handle 组件需要在 Provider 中）
function renderNode(data: AnimatedNodeData, selected = false) {
    return render(
        <ReactFlowProvider>
            <AnimatedNode
                id="test-node"
                type="custom"
                data={data}
                selected={selected}
                isConnectable={true}
                xPos={0}
                yPos={0}
                zIndex={0}
                dragging={false}
                // eslint-disable-next-line @typescript-eslint/no-explicit-any
                {...({} as any)}
            />
        </ReactFlowProvider>,
    );
}

describe("AnimatedNode", () => {
    it("应渲染节点标签", () => {
        renderNode({ label: "测试节点" });
        expect(screen.getByText("测试节点")).toBeInTheDocument();
    });

    it("应渲染工具名称（当 label 为空时回退到 toolName）", () => {
        renderNode({ label: "", toolName: "http_request" });
        expect(screen.getByText("http_request")).toBeInTheDocument();
    });

    it("应渲染服务器名称", () => {
        renderNode({ label: "测试", serverName: "Demo Server" });
        expect(screen.getByText("Demo Server")).toBeInTheDocument();
    });

    it("应渲染状态标签", () => {
        renderNode({ label: "测试", status: RunStatus.Idle });
        expect(screen.getByText("空闲")).toBeInTheDocument();
    });

    it("Running 状态应显示运行中标签", () => {
        renderNode({ label: "测试", status: RunStatus.Running });
        expect(screen.getByText("运行中")).toBeInTheDocument();
    });

    it("Success 状态应显示成功标签", () => {
        renderNode({ label: "测试", status: RunStatus.Success });
        expect(screen.getByText("成功")).toBeInTheDocument();
    });

    it("Failed 状态应显示失败标签", () => {
        renderNode({ label: "测试", status: RunStatus.Failed });
        expect(screen.getByText("失败")).toBeInTheDocument();
    });

    it("Timeout 状态应显示超时标签", () => {
        renderNode({ label: "测试", status: RunStatus.Timeout });
        expect(screen.getByText("超时")).toBeInTheDocument();
    });

    it("默认状态应为 Idle（空闲）", () => {
        renderNode({ label: "测试" });
        expect(screen.getByText("空闲")).toBeInTheDocument();
    });

    it("选中状态应添加 ring 样式", () => {
        const { container } = renderNode({ label: "测试" }, true);
        const node = container.querySelector(".ring-2");
        expect(node).toBeTruthy();
    });
});