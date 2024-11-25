import { useState, useEffect } from "react";
import { AuthManager } from "../../auth/AuthManager";

export default function AuthButton() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    checkAuth();
  }, []);

  const checkAuth = async () => {
    try {
      const authenticated = await AuthManager.isAuthenticated();
      setIsAuthenticated(authenticated);
    } catch (error) {
      console.error("Auth check failed:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleLogin = async () => {
    try {
      setIsLoading(true);
      await AuthManager.login();
      setIsAuthenticated(true);
    } catch (error) {
      console.error("Login failed:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleLogout = async () => {
    try {
      setIsLoading(true);
      await AuthManager.logout();
      setIsAuthenticated(false);
    } catch (error) {
      console.error("Logout failed:", error);
    } finally {
      setIsLoading(false);
    }
  };

  if (isLoading) {
    return (
      <button className="px-4 py-2 bg-gray-200 rounded-md" disabled>
        Loading...
      </button>
    );
  }

  return isAuthenticated ? (
    <button
      onClick={handleLogout}
      className="px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors"
    >
      Logout
    </button>
  ) : (
    <button
      onClick={handleLogin}
      className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors"
    >
      Login with Internet Identity
    </button>
  );
}
