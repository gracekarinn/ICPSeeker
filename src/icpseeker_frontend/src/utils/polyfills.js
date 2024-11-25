window.global = window;
window.process = {
  env: {
    NODE_ENV: process.env.NODE_ENV,
    DFX_NETWORK: process.env.DFX_NETWORK || "local",
  },
};
