import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent } from "@dfinity/agent";

const DFX_NETWORK = process.env.DFX_NETWORK || "local";
const II_CANISTER_ID =
  process.env.CANISTER_ID_INTERNET_IDENTITY || "be2us-64aaa-aaaaa-qaabq-cai";

const LOCAL_HOST =
  window.location.hostname === "localhost" ? "localhost" : "127.0.0.1";
const LOCAL_PORT = "4943";

const getHost = () => {
  return DFX_NETWORK === "ic"
    ? "https://identity.ic0.app"
    : `http://${LOCAL_HOST}:${LOCAL_PORT}`;
};

const II_URL =
  DFX_NETWORK === "ic"
    ? "https://identity.ic0.app/#authorize"
    : `http://${LOCAL_HOST}:${LOCAL_PORT}?canisterId=${II_CANISTER_ID}#authorize`;

export class AuthManager {
  static #authClient = null;
  static #agent = null;

  static async init() {
    if (!this.#authClient) {
      try {
        this.#authClient = await AuthClient.create({
          idleOptions: {
            disableIdle: true,
            disableDefaultIdleCallback: true,
          },
        });

        this.#agent = new HttpAgent({
          host: getHost(),
        });

        if (DFX_NETWORK !== "ic") {
          await this.#agent.fetchRootKey().catch(console.error);
        }

        await this.#handleAuthenticated();
      } catch (error) {
        console.error("Failed to initialize auth client:", error);
      }
    }
    return this.#authClient;
  }

  static async login() {
    try {
      const authClient = await this.init();

      return new Promise((resolve, reject) => {
        authClient.login({
          identityProvider: II_URL,
          maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000), // 7 days
          derivationOrigin: `http://${LOCAL_HOST}:${LOCAL_PORT}`,
          windowOpenerFeatures:
            `left=${window.screen.width / 2 - 525 / 2},` +
            `top=${window.screen.height / 2 - 705 / 2},` +
            `toolbar=0,location=0,menubar=0,width=525,height=705`,
          onSuccess: async () => {
            await this.#handleAuthenticated();
            resolve();
          },
          onError: (error) => {
            console.error("Login failed:", error);
            reject(error);
          },
        });
      });
    } catch (error) {
      console.error("Login process failed:", error);
      throw error;
    }
  }

  static async logout() {
    try {
      const authClient = await this.init();
      await authClient.logout();
      window.location.reload();
    } catch (error) {
      console.error("Logout failed:", error);
      throw error;
    }
  }

  static async isAuthenticated() {
    try {
      const authClient = await this.init();
      return await authClient.isAuthenticated();
    } catch (error) {
      console.error("Auth check failed:", error);
      return false;
    }
  }

  static async getIdentity() {
    try {
      const authClient = await this.init();
      return authClient.getIdentity();
    } catch (error) {
      console.error("Failed to get identity:", error);
      throw error;
    }
  }

  static async #handleAuthenticated() {
    try {
      const authClient = await this.init();
      if (await authClient.isAuthenticated()) {
        const identity = authClient.getIdentity();
        const principal = identity.getPrincipal().toString();
        console.log("Authenticated principal:", principal);

        if (this.#agent) {
          this.#agent.replaceIdentity(identity);
        }
      }
    } catch (error) {
      console.error("Handle authentication failed:", error);
    }
  }
}
