import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent } from "@dfinity/agent";
import { createActor } from "../../../declarations/icpseeker_backend";

export class AuthManager {
  static async create() {
    try {
      const authClient = await AuthClient.create();
      return new AuthManager(authClient);
    } catch (error) {
      console.error("Failed to create AuthClient:", error);
      throw new Error("Authentication initialization failed");
    }
  }

  constructor(authClient) {
    this.authClient = authClient;
    this.backendActor = null;
  }

  async initBackendActor() {
    try {
      const identity = await this.authClient.getIdentity();
      const agent = new HttpAgent({
        identity,
        host: this.getHost(),
      });

      if (import.meta.env.VITE_DFX_NETWORK !== "ic") {
        await agent.fetchRootKey();
      }

      return createActor(import.meta.env.VITE_CANISTER_ID_ICPSEEKER_BACKEND, {
        agent,
      });
    } catch (error) {
      console.error("Failed to initialize backend actor:", error);
      throw error;
    }
  }

  async login() {
    try {
      const identityProvider = this.getIdentityProviderUrl();
      console.log("Starting login with provider:", identityProvider);

      return new Promise((resolve, reject) => {
        this.authClient.login({
          identityProvider,
          maxTimeToLive: BigInt(7) * BigInt(24) * BigInt(3600000000000),
          onSuccess: async () => {
            try {
              console.log("Login successful, initializing backend...");
              this.backendActor = await this.initBackendActor();
              console.log("Backend actor initialized");

              console.log("Calling backend login...");
              const loginResult = await this.backendActor.login();
              console.log("Login result:", loginResult);

              console.log("Getting user info...");
              const userResponse = await this.backendActor.get_user();
              console.log("User response:", userResponse);

              if (
                "Error" in userResponse &&
                userResponse.Error.includes("not found")
              ) {
                console.log("User not found, redirecting to registration...");
                window.location.href = "/register";
              } else if ("Success" in userResponse) {
                const user = userResponse.Success;
                if (user.name && user.email) {
                  window.location.href = "/";
                } else {
                  window.location.href = "/register";
                }
              } else {
                window.location.href = "/register";
              }

              resolve(userResponse);
            } catch (error) {
              console.error("Backend initialization failed:", error);
              window.location.href = "/register";
              reject(error);
            }
          },
          onError: (error) => {
            console.error("Login error:", error);
            reject(error);
          },
        });
      });
    } catch (error) {
      console.error("Login failed:", error);
      throw error;
    }
  }

  getIdentityProviderUrl() {
    const network = import.meta.env.VITE_DFX_NETWORK;
    if (network === "ic") {
      return "https://identity.ic0.app/#authorize";
    }
    return `http://${
      import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY
    }.localhost:4943/#authorize`;
  }

  getHost() {
    return import.meta.env.VITE_DFX_NETWORK === "ic"
      ? "https://ic0.app"
      : "http://localhost:4943";
  }

  async isAuthenticated() {
    try {
      const isAuth = await this.authClient.isAuthenticated();
      if (isAuth && !this.backendActor) {
        this.backendActor = await this.initBackendActor();
      }
      return isAuth;
    } catch (error) {
      console.error("Auth check failed:", error);
      return false;
    }
  }

  async logout() {
    try {
      await this.authClient.logout();
      this.backendActor = null;
      window.location.href = "/";
    } catch (error) {
      console.error("Logout failed:", error);
      throw error;
    }
  }
}
