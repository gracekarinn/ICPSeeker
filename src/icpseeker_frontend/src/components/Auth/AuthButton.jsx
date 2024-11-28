import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { AuthManager } from "../../auth/AuthManager";
import Button from "../Button"; // Asumsikan Button telah diimpor dengan benar

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

  return (
    <Button
      onClick={handleAuth}
      variant="secondary" // Menggunakan varian 'secondary'
      size="medium" // Menetapkan ukuran
      isLoading={isLoading} // Menangani loading state
    >
      {isAuthenticated ? "Logout" : "Login"}
    </Button>
  );
};

export default AuthButton;
