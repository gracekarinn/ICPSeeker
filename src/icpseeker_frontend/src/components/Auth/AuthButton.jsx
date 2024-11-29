import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { AuthManager } from "../../auth/AuthManager";
import Button from "../Button";

const AuthButton = () => {
  const [authClient, setAuthClient] = useState(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const navigate = useNavigate();

  useEffect(() => {
    const initAuth = async () => {
      try {
        setIsLoading(true);
        const client = await AuthManager.create();
        setAuthClient(client);

        const authenticated = await client.isAuthenticated();
        setIsAuthenticated(authenticated);

        if (authenticated) {
          try {
            const actor = client.backendActor;
            const userResponse = await actor.get_user();

            if ("Success" in userResponse) {
              const user = userResponse.Success;
              if (user.name && user.email) {
                navigate("/dashboard");
              } else {
                navigate("/register");
              }
            } else {
              navigate("/register");
            }
          } catch (error) {
            console.error("Failed to check user profile:", error);
          }
        }
      } catch (error) {
        console.error("Auth initialization failed:", error);
      } finally {
        setIsLoading(false);
      }
    };

    initAuth();
  }, [navigate]);

  const handleAuth = async () => {
    try {
      setIsLoading(true);
      if (isAuthenticated) {
        await authClient.logout();
        setIsAuthenticated(false);
      } else {
        await authClient.login();
        setIsAuthenticated(true);
      }
    } catch (error) {
      console.error("Auth action failed:", error);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Button
      onClick={handleAuth}
      variant="secondary"
      size="medium"
      isLoading={isLoading}
    >
      {isAuthenticated ? "Logout" : "Login"}
    </Button>
  );
};

export default AuthButton;
