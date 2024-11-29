import React from "react";
import Navbar from "../components/Navbar/Navbar";

const Businesses = () => {
  return (
    <>
      <Navbar />
      <div className="min-h-screen bg-gradient-to-l mx-auto from-[#5F0EE7] to-[#350881] text-white flex flex-col justify-center items-center px-4">
        <div className="flex flex-col md:flex-row w-full mx-auto justify-center items-center max-w-6xl gap-8">
          <div className="max-w-md md:w-1/2 space-y-6">
            <h1 className="text-5xl md:text-6xl font-semibold leading-tight">
              Looking for Quality Talent?
            </h1>
            <p className="text-lg md:text-xl text-gray-200">
              Streamline your hiring process, access top-tier talent, and pay
              seamlessly—anywhere in the world.
            </p>
            <button className="bg-white text-[#5F0EE7] px-8 py-3 rounded-lg text-lg font-medium hover:bg-opacity-90 transition-colors">
              Start hiring
            </button>
          </div>
          <div>
            <div className="w-xl">
              <img src="/bisnis.png" alt="Hiring" />
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default Businesses;
