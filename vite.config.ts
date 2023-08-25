import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
// https://github.com/vuetifyjs/vuetify-loader/tree/next/packages/vite-plugin
import vuetify from "vite-plugin-vuetify";
import eslintPlugin from "vite-plugin-eslint";
import path from "path";
import { fileURLToPath, URL } from "node:url";
// https://vitejs.dev/config/
export default defineConfig(async () => {
  return {
    assetsInclude: ["./src/asset/image/**/*"],
    plugins: [
      vue(),
      vuetify({ autoImport: true }),
      //   eslintPlugin({
      //   include:['src/**/*.ts','src/**/*.vue']
      // })
    ],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
      port: 8080,
      strictPort: true,
    },
    resolve: {
      alias: {
        "@": fileURLToPath(new URL("src", import.meta.url)),
      },
    },
    // 3. to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ["VITE_", "TAURI_"],
  };
});
