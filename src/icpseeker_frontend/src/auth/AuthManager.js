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
    this.authStateListeners = new Set();
    this.backendActor = null;
  }

  async login() {
    const days = BigInt(7);
    const hours = BigInt(24);
    const nanoseconds = BigInt(3600000000000);

    try {
      console.log("Starting login process...");
      const identityProvider = this.getIdentityProviderUrl();
      console.log("Using Identity Provider:", identityProvider);

      if (import.meta.env.VITE_CANISTER_ID_ICPSEEKER_BACKEND) {
        const agent = new HttpAgent({
          host: this.getHost(),
        });

        if (import.meta.env.VITE_DFX_NETWORK !== "ic") {
          await agent.fetchRootKey();
        }

        this.backendActor = createActor(
          import.meta.env.VITE_CANISTER_ID_ICPSEEKER_BACKEND,
          {
            agent,
          }
        );
      }

      return await this.authClient.login({
        identityProvider,
        maxTimeToLive: days * hours * nanoseconds,
        windowOpenerFeatures:
          "width=525,height=705,left=calc(50% - 262.5px),top=calc(50% - 352.5px)",
        onSuccess: async () => {
          console.log("Login successful");
          try {
            if (this.backendActor) {
              const authedAgent = await this.getAgent();
              this.backendActor = createActor(
                import.meta.env.VITE_CANISTER_ID_ICPSEEKER_BACKEND,
                {
                  agent: authedAgent,
                }
              );
              await this.backendActor.login();
            }
            window.location.href = "/profile-setup";
          } catch (error) {
            console.error("Failed to initialize backend:", error);

            window.location.href = "/profile-setup";
          }
        },
        onError: (error) => {
          console.error("Login error:", error);
        },
      });
    } catch (error) {
      console.error("Login failed:", error);
      throw error;
    }
  }

  getIdentityProviderUrl() {
    if (import.meta.env.VITE_DFX_NETWORK === "ic") {
      return "https://identity.ic0.app";
    }

    const canisterId = import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY;
    const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);

    if (isSafari) {
      return `http://127.0.0.1:4943/?canisterId=${canisterId}`;
    }

    return `http://${canisterId}.localhost:4943`;
  }

  getHost() {
    const isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent);
    return isSafari ? "http://127.0.0.1:4943" : "http://localhost:4943";
  }

  async getAgent() {
    try {
      const identity = await this.authClient.getIdentity();
      const agent = new HttpAgent({
        identity,
        host: this.getHost(),
      });

      if (import.meta.env.VITE_DFX_NETWORK !== "ic") {
        await agent.fetchRootKey();
      }

      return agent;
    } catch (error) {
      console.error("Failed to create agent:", error);
      throw error;
    }
  }

  async logout() {
    try {
      if (this.backendActor) {
        try {
          this.backendActor = null;
        } catch (error) {
          console.error("Backend logout error:", error);
        }
      }
      await this.authClient.logout();
      window.location.href = "/";
    } catch (error) {
      console.error("Logout failed:", error);
      throw error;
    }
  }

  async isAuthenticated() {
    try {
      const authenticated = await this.authClient.isAuthenticated();
      if (
        authenticated &&
        !this.backendActor &&
        import.meta.env.VITE_CANISTER_ID_ICPSEEKER_BACKEND
      ) {
        const agent = await this.getAgent();
        this.backendActor = createActor(
          import.meta.env.VITE_CANISTER_ID_ICPSEEKER_BACKEND,
          {
            agent,
          }
        );
      }
      return authenticated;
    } catch (error) {
      console.error("Auth check failed:", error);
      return false;
    }
  }
}
