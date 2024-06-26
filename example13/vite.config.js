import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue({
      template: {
        compilerOptions: {
          isCustomElement: tag => tag.startsWith('ion-')
        }
      },
      define: {
        __VUE_OPTIONS_API__: false, // Enable if you're using Vue's Options API
        __VUE_PROD_DEVTOOLS__: false, // Enable to use Vue devtools in production
        __VUE_PROD_HYDRATION_MISMATCH_DETAILS__: true // Enable for detailed hydration mismatch warnings
      }
    })],

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
