import React from "react"; 
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import Navbar from "./components/Navbar/Navbar";
import AuthButton from "./components/Auth/AuthButton";
import { AuthManager } from "./auth/AuthManager";  // Pastikan jalur impor ini benar

// Komponen untuk Landing Page
const LandingPage = () => {
  return (
    <div className="min-h-screen bg-gradient-to-r from-purple-normal to-blue-dark text-white flex flex-col justify-center items-center px-4">
      <h1 className="font-heading text-h1 text-center">Jobseeking. Decentralized.</h1>
      <p className="font-body text-p2 text-center mt-6">
        Say goodbye to middlemen and hello to opportunities. Connect directly with employers,
        showcase your talent, and take charge of your career on our decentralized platform.
      </p>
      <div className="mt-8 flex space-x-4">
        <button className="bg-purple-normal hover:bg-purple-dark text-white font-bold py-2 px-4 rounded">
          Start hiring
        </button>
        <button className="bg-blue-normal hover:bg-blue-dark text-white font-bold py-2 px-4 rounded">
          Find a job
        </button>
      </div>
    </div>
  );
};

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

// Komponen ProfileSetup
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

// Komponen Dashboard
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
      </Routes>
    </BrowserRouter>
  );
}

export default App;
