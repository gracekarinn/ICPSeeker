import React from "react";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import AuthButton from "./components/Auth/AuthButton";
import { AuthManager } from "./auth/AuthManager";
import Button from "./components/Button";
import LandingPage from "./pages/LandingPage";
import JobPage from "./pages/JobPage";
import VerifyCertificate from "./pages/VerifyCertificate";
import Register from "./pages/Register";
import Login from "./pages/Login";

// const ProtectedRoute = ({ children }) => {
//   const checkAuth = async () => {
//     try {
//       const authClient = await AuthManager.create();
//       const isAuthenticated = await authClient.isAuthenticated();
//       return isAuthenticated;
//     } catch (error) {
//       console.error("Auth check failed:", error);
//       return false;
//     }
//   };

//   if (!checkAuth()) {
//     return <Navigate to="/" />;
//   }

//   return children;
// };

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/jobs" element={<JobPage />} />
        <Route path="/verifycertificate" element={<VerifyCertificate />} />
        <Route path="/register" element={<Register />} />
        <Route path="/login" element={<Login />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
