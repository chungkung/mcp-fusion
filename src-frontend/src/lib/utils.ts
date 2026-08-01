import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * 合并 className，结合 clsx 的条件合并和 tailwind-merge 的冲突去重
 */
export function cn(...inputs: ClassValue[]): string {
    return twMerge(clsx(inputs));
}