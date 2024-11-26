import { fileURLToPath, URL } from "url";
import react from "@vitejs/plugin-react";
import { defineConfig, loadEnv } from "vite";
import environment from "vite-plugin-environment";
import dotenv from "dotenv";
import tailwindcss from "tailwindcss";
import autoprefixer from "autoprefixer";

dotenv.config();

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");

  return {
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
      host: true,
      port: 3000,
      proxy: {
        "/api": {
          target: "http://localhost:4943",
          changeOrigin: true,
          secure: false,
          rewrite: (path) => path.replace(/^\/api/, ""),
        },
      },
      headers: {
        "Cross-Origin-Opener-Policy": "same-origin",
        "Cross-Origin-Embedder-Policy": "require-corp",
        "Access-Control-Allow-Origin": "*",
        "Access-Control-Allow-Methods":
          "GET, POST, PUT, DELETE, PATCH, OPTIONS",
        "Access-Control-Allow-Headers":
          "Content-Type, Authorization, X-Requested-With",
      },
    },
    plugins: [react(), environment("all", { prefix: "VITE_" })],
    resolve: {
      alias: [
        {
          find: "declarations",
          replacement: fileURLToPath(
            new URL("./src/declarations", import.meta.url)
          ),
        },
        {
          find: "@",
          replacement: fileURLToPath(new URL("./src", import.meta.url)),
        },
      ],
    },
    define: {
      global: "globalThis",
      "process.env.DFX_NETWORK": JSON.stringify(env.DFX_NETWORK || "local"),
      "process.env.CANISTER_ID_INTERNET_IDENTITY": JSON.stringify(
        env.CANISTER_ID_INTERNET_IDENTITY
      ),
      "process.env.CANISTER_ID_ICPSEEKER_FRONTEND": JSON.stringify(
        env.CANISTER_ID_ICPSEEKER_FRONTEND
      ),
      "process.env.CANISTER_ID_ICPSEEKER_BACKEND": JSON.stringify(
        env.CANISTER_ID_ICPSEEKER_BACKEND
      ),
      "process.env.HOST": JSON.stringify(env.HOST || "http://localhost:4943"),
      "process.env.VITE_DFX_NETWORK": JSON.stringify(
        env.DFX_NETWORK || "local"
      ),
    },
    envPrefix: ["VITE_", "CANISTER_", "DFX_"],
    build: {
      target: "esnext",
      minify: false,
      sourcemap: true,
      commonjsOptions: {
        include: [/node_modules/],
        transformMixedEsModules: true,
      },
      rollupOptions: {
        external: [],
        output: {
          manualChunks: undefined,
        },
      },
    },
  };
});
