import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import fs from 'fs';
import path from 'path';


// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue(), wasmMiddleware()],
  esbuild: {
    supported: {
      'top-level-await': true
    },
  },
  optimizeDeps: {
    esbuildOptions: {
      supported: {
        "top-level-await": true
      },
    },
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));

const wasmMiddleware = () => {
  return {
    name: 'wasm-middleware',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (req.url.endsWith('.wasm')) {
          const wasmPath = path.join(__dirname, '/wasm/rust_wasm/pkg', path.basename(req.url));
          try {
            const wasmFile = fs.readFileSync(wasmPath);
            res.setHeader('Content-Type', 'application/wasm');
            res.end(wasmFile);
          } catch (error) {
            console.error('Failed to load WASM file:', error);
            next();
          }
          return;
        }
        next();
      });
    },
  };
};
