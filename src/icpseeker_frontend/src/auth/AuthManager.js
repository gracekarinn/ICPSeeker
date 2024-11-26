import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent } from "@dfinity/agent";
import { icpseeker_backend } from "../../../declarations/icpseeker_backend";

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

  async initBackendActor() {
    try {
      const agent = await this.getAgent();
      this.backendActor = icpseeker_backend.createActor(agent);
      await this.backendActor.login();
      return true;
    } catch (error) {
      console.error("Failed to initialize backend:", error);
      return false;
    }
  }

  addAuthStateListener(listener) {
    this.authStateListeners.add(listener);
  }

  removeAuthStateListener(listener) {
    this.authStateListeners.delete(listener);
  }

  notifyAuthStateChange(isAuthenticated) {
    this.authStateListeners.forEach((listener) => listener(isAuthenticated));
  }

  async login() {
    const days = BigInt(7);
    const hours = BigInt(24);
    const nanoseconds = BigInt(3600000000000);

    try {
      console.log("Starting login process...");
      console.log("Identity Provider:", this.getIdentityProviderUrl());
      console.log("Delegation Origin:", this.getDelegationOrigin());

      return await this.authClient.login({
        identityProvider: this.getIdentityProviderUrl(),
        windowOpenerFeatures: this.getWindowFeatures(),
        maxTimeToLive: days * hours * nanoseconds,
        derivationOrigin: this.getDelegationOrigin(),
        onSuccess: () => {
          console.log("Login successful");
          this.notifyAuthStateChange(true);
          window.location.reload();
        },
        onError: (error) => {
          console.error("Login failed:", error);
          this.notifyAuthStateChange(false);
        },
      });
    } catch (error) {
      console.error("Login process failed:", error);
      throw error;
    }
  }

  getIdentityProviderUrl() {
    if (import.meta.env.VITE_DFX_NETWORK === "ic") {
      return "https://identity.ic0.app";
    }

    const canisterId = import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY;

    if (window.chrome) {
      return `http://${canisterId}.localhost:4943`;
    }

    return `http://127.0.0.1:4943/?canisterId=${canisterId}`;
  }

  getDelegationOrigin() {
    // Always use the origin of your Vite app
    const host =
      window.location.hostname === "localhost" ? "localhost" : "127.0.0.1";
    return `http://${host}:3000`;
  }

  getWindowFeatures() {
    const width = 525;
    const height = 705;
    const left = window.screen.width / 2 - width / 2;
    const top = window.screen.height / 2 - height / 2;
    return `left=${left},top=${top},toolbar=0,location=0,menubar=0,width=${width},height=${height}`;
  }

  async getAgent() {
    try {
      const identity = await this.authClient.getIdentity();
      const agent = new HttpAgent({
        identity,
        host:
          import.meta.env.VITE_DFX_NETWORK === "ic"
            ? "https://ic0.app"
            : import.meta.env.VITE_HOST_URL,
      });

      if (import.meta.env.VITE_DFX_NETWORK === "local") {
        await agent.fetchRootKey();
      }

      return agent;
    } catch (error) {
      console.error("Failed to create agent:", error);
      throw error;
    }
  }

  async getPrincipal() {
    const identity = await this.authClient.getIdentity();
    return identity.getPrincipal();
  }

  async logout() {
    try {
      await this.authClient.logout();
      this.notifyAuthStateChange(false);
      window.location.reload();
    } catch (error) {
      console.error("Logout failed:", error);
      throw error;
    }
  }

  async isAuthenticated() {
    try {
      return await this.authClient.isAuthenticated();
    } catch (error) {
      console.error("Auth check failed:", error);
      return false;
    }
  }

  hasCachedCredentials() {
    return (
      this.authClient?.getIdentity()?.getPrincipal()?.isAnonymous() === false
    );
  }
}
