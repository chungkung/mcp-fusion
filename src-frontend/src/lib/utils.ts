import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * 合并 className，结合 clsx 的条件合并和 tailwind-merge 的冲突去重
 */
export function cn(...inputs: ClassValue[]): string {
    return twMerge(clsx(inputs));
}

// ============================================================
// 通用动画配置
// ============================================================

import type { Variants } from "framer-motion";

/** 页面淡入 + 上滑进入 */
export const pageVariants: Variants = {
    initial: {
        opacity: 0,
        y: 12,
    },
    animate: {
        opacity: 1,
        y: 0,
        transition: {
            duration: 0.35,
            ease: [0.25, 0.46, 0.45, 0.94],
        },
    },
    exit: {
        opacity: 0,
        y: -12,
        transition: {
            duration: 0.25,
            ease: [0.25, 0.46, 0.45, 0.94],
        },
    },
};

/** 元素淡入 */
export const fadeInVariants: Variants = {
    initial: {
        opacity: 0,
    },
    animate: {
        opacity: 1,
        transition: {
            duration: 0.3,
        },
    },
    exit: {
        opacity: 0,
        transition: {
            duration: 0.2,
        },
    },
};

/** 元素缩放弹出 */
export const scaleVariants: Variants = {
    initial: {
        opacity: 0,
        scale: 0.92,
    },
    animate: {
        opacity: 1,
        scale: 1,
        transition: {
            duration: 0.25,
            ease: [0.34, 1.56, 0.64, 1],
        },
    },
    exit: {
        opacity: 0,
        scale: 0.92,
        transition: {
            duration: 0.15,
        },
    },
};

/** 列表项交错入场 */
export function staggerVariants(delayPerItem: number = 0.06): Variants {
    return {
        initial: {
            opacity: 0,
            y: 8,
        },
        animate: {
            opacity: 1,
            y: 0,
            transition: {
                staggerChildren: delayPerItem,
                delayChildren: 0.1,
            },
        },
    };
}

/** 默认动画过渡配置 */
export const defaultTransition = {
    type: "spring" as const,
    stiffness: 360,
    damping: 30,
};