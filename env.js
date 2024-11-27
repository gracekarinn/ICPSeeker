const fs = require("fs");
const path = require("path");
const dfxJson = require("./dfx.json");

// Read the DFX environment variables
function initCanisterIds() {
  try {
    const envPath = path.resolve(__dirname, ".env");
    if (!fs.existsSync(envPath)) {
      throw new Error(".env file not found");
    }

    const env = fs
      .readFileSync(envPath, "utf8")
      .split("\n")
      .filter((line) => line && !line.startsWith("#"))
      .reduce((acc, line) => {
        const [key, value] = line.split("=");
        acc[key.trim()] = value.trim().replace(/['"]/g, "");
        return acc;
      }, {});

    return env;
  } catch (error) {
    console.warn("Could not initialize canister IDs:", error.message);
    return {};
  }
}

module.exports = initCanisterIds();
