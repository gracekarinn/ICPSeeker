import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { AuthManager } from "../../auth/AuthManager";

const AuthButton = () => {
  const [authClient, setAuthClient] = useState(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const navigate = useNavigate();

  useEffect(() => {
    const initAuth = async () => {
      try {
        const client = await AuthManager.create();
        setAuthClient(client);
        const authenticated = await client.isAuthenticated();
        setIsAuthenticated(authenticated);

        if (authenticated) {
          navigate("/profile-setup");
        }
      } catch (error) {
        console.error("Auth initialization failed:", error);
      }
      setIsLoading(false);
    };

    initAuth();
  }, [navigate]);

  const handleAuth = async () => {
    try {
      if (isAuthenticated) {
        await authClient.logout();
        setIsAuthenticated(false);
        navigate("/");
      } else {
        await authClient.login();
      }
    } catch (error) {
      console.error("Auth action failed:", error);
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
