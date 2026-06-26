import { resolve, dirname } from "path";
import { fileURLToPath } from "url";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
    root: __dirname,

    plugins: [react()],

    resolve: {
        alias: {
            "@": resolve(__dirname, "src"),
            "@shared": resolve(__dirname, "../src-shared"),
        },
    },

    // 1. prevent Vite from obscuring rust errors
    clearScreen: false,

    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                  protocol: "ws",
                  host,
                  port: 1421,
              }
            : undefined,
        watch: {
            // 3. tell Vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },

    // Tauri expects a fixed port for the dev server
    build: {
        // 4. Tauri uses Chromium on Windows and WebKit on macOS/Linux
        target:
            process.env.TAURI_ENV_PLATFORM === "windows"
                ? "chrome105"
                : "safari13",
        // 5. don't minify for debug builds
        minify: !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
        // 6. produce sourcemaps for debug builds
        sourcemap: !!process.env.TAURI_ENV_DEBUG,
    },
}));