import React, { useState, useEffect } from "react";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { AuthManager } from "./auth/AuthManager";
import LandingPage from "./pages/LandingPage";
import JobPage from "./pages/JobPage";
import VerifyCertificate from "./pages/VerifyCertificate";
import Register from "./pages/Register";
import Login from "./pages/Login";
import Businesses from "./pages/Businesses";
import AskAI from "./pages/AskAI";
import CVAnalyzer from "./pages/CVAnalyzer";
import LoginBusiness from "./pages/LoginBusiness";
import JobPostings from "./pages/JobPostings";
import RegisterBusiness from "./pages/RegisterBusiness";
import OnBoardingRegister from "./pages/OnBoardingRegister";

const ProtectedRoute = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const checkAuth = async () => {
      try {
        const authClient = await AuthManager.create();
        const isAuth = await authClient.isAuthenticated();
        setIsAuthenticated(isAuth);
      } catch (error) {
        console.error("Auth check failed:", error);
        setIsAuthenticated(false);
      } finally {
        setLoading(false);
      }
    };

    checkAuth();
  }, []);

  if (loading) {
    return <div>Loading...</div>;
  }

  return isAuthenticated ? children : <Navigate to="/" />;
};

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/jobs" element={<JobPage />} />
        <Route path="/verifycertificate" element={<VerifyCertificate />} />
        <Route path="/businesses" element={<Businesses />} />
        <Route path="/ask" element={<AskAI />} />
        <Route
          path="/register"
          element={
            <ProtectedRoute>
              <Register />
            </ProtectedRoute>
          }
        />
        <Route
          path="/cvanalyzer"
          element={
            <ProtectedRoute>
              <CVAnalyzer />
            </ProtectedRoute>
          }
        />
        <Route path="/login" element={<Login />} />
        <Route path="/loginbusiness" element={<LoginBusiness />} />
        <Route path="/jobpostings" element={<JobPostings />} />
        <Route path="/registerbusiness" element={<RegisterBusiness />} />
        <Route path="/onboardingregister" element={<OnBoardingRegister />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
