import React, { useState, useEffect } from "react";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import AuthButton from "./components/Auth/AuthButton";
import Navbar from "./components/Navbar/Navbar";

const ProfileSetup = () => (
  <div className="p-8 text-center">
    <h1 className="text-2xl mb-4">Profile Setup</h1>
    <pre>Profile form will go here</pre>
  </div>
);

const Dashboard = () => (
  <div className="p-8 text-center">
    <h1 className="text-2xl mb-4">Dashboard</h1>
    <pre>Dashboard content will go here</pre>
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
          <Route path="/profile-setup" element={<ProfileSetup />} />
          <Route path="/dashboard" element={<Dashboard />} />
        </Routes>
      </div>
    </BrowserRouter>
  );
}

export default App;
