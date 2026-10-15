import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath, URL } from "node:url"; // MUST IMPORT THIS

const host = process.env.TAURI_DEV_HOST;
const port = process.env.VITE_PORT ? parseInt(process.env.VITE_PORT) : 1421;

export default defineConfig(async () => ({
  // THIS LINE IS CRITICAL - It forces Vite to serve the exact folder it is sitting in!
  root: fileURLToPath(new URL(".", import.meta.url)), 
  
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