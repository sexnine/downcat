import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import svgLoader from "vite-svg-loader";

// https://vitejs.dev/config/
/**
 * @type {import('vite').UserConfig}
 */
export default defineConfig({
  plugins: [
    vue({
      reactivityTransform: true,
    }),
    svgLoader(),
  ],
  build: {
    cssCodeSplit: false,
  },
  server: {
    port: 3000,
  },
});
