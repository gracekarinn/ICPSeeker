import { Buffer } from "buffer";

if (typeof window !== "undefined") {
  window.global = window;
  window.Buffer = Buffer;
  window.process = {
    env: {
      NODE_ENV: process.env.NODE_ENV || "development",
      DFX_NETWORK: process.env.DFX_NETWORK || "local",
    },
    version: "",
  };
}
