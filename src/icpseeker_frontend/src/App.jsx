import { useState } from "react";
import { icpseeker_backend } from "declarations/icpseeker_backend";
import React from "react";
import Button from "./components/button";
import { FaHome } from "react-icons/fa";
import AuthButton from "./components/Auth/AuthButton";

function App() {
  return (
    <div className="min-h-screen bg-gray-100 flex flex-col items-center justify-center">
      <div className="p-8 bg-white rounded-lg shadow-md">
        <h1 className="text-2xl font-bold mb-4 text-center">ICPSeeker</h1>
        <AuthButton />
      </div>
    </div>
  );
}

export default App;
