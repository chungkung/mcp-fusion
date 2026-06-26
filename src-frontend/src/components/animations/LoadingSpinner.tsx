import { cn } from "@/lib/utils";
import { type JSX } from "react";

type SpinnerSize = "sm" | "md" | "lg";

interface LoadingSpinnerProps {
    size?: SpinnerSize;
    className?: string;
    label?: string;
}

const sizeMap: Record<SpinnerSize, string> = {
    sm: "h-4 w-4 border-2",
    md: "h-8 w-8 border-[3px]",
    lg: "h-12 w-12 border-4",
};

const labelSizeMap: Record<SpinnerSize, string> = {
    sm: "text-xs",
    md: "text-sm",
    lg: "text-base",
};

function LoadingSpinner({
    size = "md",
    className = "",
    label,
}: LoadingSpinnerProps): JSX.Element {
    return (
        <div
            className={cn(
                "flex flex-col items-center justify-center gap-3",
                className,
            )}
            role="status"
            aria-label={label ?? "Loading"}
        >
            <div
                className={cn(
                    "animate-spin rounded-full",
                    "border-muted border-t-primary",
                    sizeMap[size],
                )}
            />
            {label && (
                <span
                    className={cn(
                        "text-muted-foreground",
                        labelSizeMap[size],
                    )}
                >
                    {label}
                </span>
            )}
        </div>
    );
}

export default LoadingSpinner;