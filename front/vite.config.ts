import { reactRouter } from "@react-router/dev/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [tailwindcss(), reactRouter(), wasm()],
  resolve: {
    tsconfigPaths: true,
  },
});
