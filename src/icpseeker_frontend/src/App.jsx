import React from "react";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import AuthButton from "./components/Auth/AuthButton";

const ProtectedRoute = ({ children }) => {
  const checkAuth = async () => {
    try {
      const authClient = await AuthManager.create();
      return await authClient.isAuthenticated();
    } catch (error) {
      console.error("Auth check failed:", error);
      return false;
    }
  };

  if (!checkAuth()) {
    return <Navigate to="/" />;
  }

  return children;
};

const ProfileSetup = () => (
  <div className="p-8">
    <div className="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Profile Setup</h1>
        <AuthButton />
      </div>
      <pre>Profile form will go here</pre>
    </div>
  </div>
);

const Dashboard = () => (
  <div className="p-8">
    <div className="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Dashboard</h1>
        <AuthButton />
      </div>
      <pre>Dashboard content will go here</pre>
    </div>
  </div>
);

function App() {
  return (
    <BrowserRouter>
      <div className="min-h-screen bg-gray-100">
        <Routes>
          <Route
            path="/"
            element={
              <div className="flex flex-col items-center justify-center min-h-screen">
                <h1 className="text-4xl font-bold mb-8">Welcome</h1>
                <AuthButton />
              </div>
            }
          />
          <Route
            path="/profile-setup"
            element={
              <ProtectedRoute>
                <ProfileSetup />
              </ProtectedRoute>
            }
          />
          <Route
            path="/dashboard"
            element={
              <ProtectedRoute>
                <Dashboard />
              </ProtectedRoute>
            }
          />
        </Routes>
      </div>
    </BrowserRouter>
  );
}

export default App;