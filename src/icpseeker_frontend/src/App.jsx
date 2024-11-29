import React from "react"; 
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import Navbar from "./components/Navbar/Navbar";
import AuthButton from "./components/Auth/AuthButton";
import { AuthManager } from "./auth/AuthManager";
import Button from "./components/Button";
import LandingPage from "./pages/LandingPage";
import JobPage from "./pages/JobPage";



// Komponen ProtectedRoute untuk route yang memerlukan autentikasi
const ProtectedRoute = ({ children }) => {
  const checkAuth = async () => {
    try {
      const authClient = await AuthManager.create();
      const isAuthenticated = await authClient.isAuthenticated();
      return isAuthenticated;
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
      <Navbar />
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/profile-setup" element={
          <ProtectedRoute>
            <ProfileSetup />
          </ProtectedRoute>
        } />
        <Route path="/dashboard" element={
          <ProtectedRoute>
            <Dashboard />
          </ProtectedRoute>
        } />
        <Route path="/jobs" element={<JobPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
