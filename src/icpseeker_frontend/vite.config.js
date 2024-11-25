import { fileURLToPath, URL } from "url";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import environment from "vite-plugin-environment";
import dotenv from "dotenv";
import tailwindcss from "tailwindcss";
import autoprefixer from "autoprefixer";

dotenv.config({ path: "../../.env" });

export default defineConfig({
  build: {
    emptyOutDir: true,
    rollupOptions: {
      external: [],
    },
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
    },
    include: [
      "@dfinity/auth-client",
      "@dfinity/identity",
      "@dfinity/agent",
      "@dfinity/principal",
    ],
  },
  css: {
    postcss: {
      plugins: [tailwindcss(), autoprefixer()],
    },
  },
  server: {
    host: "localhost",
    port: 3000,
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
        secure: false,
      },
    },
    headers: {
      "Cross-Origin-Opener-Policy": "same-origin",
      "Cross-Origin-Embedder-Policy": "require-corp",
    },
  },
  plugins: [
    react(),
    environment("all", { prefix: "CANISTER_" }),
    environment("all", { prefix: "DFX_" }),
  ],
  resolve: {
    alias: [
      {
        find: "declarations",
        replacement: fileURLToPath(new URL("../declarations", import.meta.url)),
      },
      {
        find: "@",
        replacement: fileURLToPath(new URL("./src", import.meta.url)),
      },
    ],
  },
  define: {
    "process.env.NODE_ENV": JSON.stringify(process.env.NODE_ENV),
    "process.env.DFX_NETWORK": JSON.stringify(
      process.env.DFX_NETWORK || "local"
    ),
    "process.env.CANISTER_ID_INTERNET_IDENTITY": JSON.stringify(
      process.env.CANISTER_ID_INTERNET_IDENTITY
    ),
    global: {},
  },
});
