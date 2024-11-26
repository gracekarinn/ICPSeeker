import React, { useState, useEffect } from "react";
import { AuthManager } from "../../auth/AuthManager";
import { icpseeker_backend } from "../../../../declarations/icpseeker_backend";

const AuthButton = () => {
  const [authClient, setAuthClient] = useState(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const initAuth = async () => {
      try {
        const client = await AuthManager.create();
        setAuthClient(client);
        const authenticated = await client.isAuthenticated();
        setIsAuthenticated(authenticated);

        if (authenticated) {
          await checkUserProfile(client);
        }
      } catch (error) {
        console.error("Auth initialization failed:", error);
      } finally {
        setIsLoading(false);
      }
    };
    initAuth();
  }, []);

  const checkUserProfile = async (client) => {
    try {
      const agent = await client.getAgent();
      const actor = icpseeker_backend.createActor(agent);
      const response = await actor.get_user();

      if ("Success" in response) {
        window.location.href = "/dashboard";
      } else {
        window.location.href = "/profile-setup";
      }
    } catch (error) {
      console.error("Error checking profile:", error);
    }
  };

  const handleAuth = async () => {
    if (isAuthenticated) {
      try {
        await authClient.logout();
        setIsAuthenticated(false);
        window.location.reload();
      } catch (error) {
        console.error("Logout failed:", error);
      }
    } else {
      try {
        await authClient.login();
      } catch (error) {
        console.error("Login failed:", error);
      }
    }
  };

  if (isLoading) {
    return (
      <button
        className="bg-gray-400 text-white font-bold py-2 px-4 rounded opacity-50 cursor-not-allowed"
        disabled
      >
        Loading...
      </button>
    );
  }

  return (
    <button
      onClick={handleAuth}
      className={`font-bold py-2 px-4 rounded transition-colors ${
        isAuthenticated
          ? "bg-red-500 hover:bg-red-700 text-white"
          : "bg-blue-500 hover:bg-blue-700 text-white"
      }`}
    >
      {isAuthenticated ? "Logout" : "Login with Internet Identity"}
    </button>
  );
};

export default AuthButton;
