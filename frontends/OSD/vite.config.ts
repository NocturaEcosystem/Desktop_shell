import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

const host = process.env.TAURI_DEV_HOST;
const port = process.env.VITE_PORT ? parseInt(process.env.VITE_PORT) : 1421;

export default defineConfig(async () => ({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: port,
    strictPort: true, 
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: port,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
    fs: {
      allow: [".."]
    }
  },
}));