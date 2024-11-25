import { useState } from "react";
import { icpseeker_backend } from "declarations/icpseeker_backend";
import React from "react";
import Button from "./components/button";
import { FaHome } from "react-icons/fa";
import AuthButton from "./components/Auth/AuthButton";
import Navbar from "./components/Navbar/Navbar";

function App() {
  return (
    <div className="min-h-screen bg-gray-100 flex flex-col">
      <Navbar />
      <div className="container mx-auto p-8 bg-gray-50">
        {/* Header Section */}
        <div className="bg-white rounded-lg shadow-md p-8 mb-8">
          <h1 className="text-2xl font-bold mb-4 text-center">ICPSeeker</h1>
          <AuthButton />
        </div>

        {/* Additional Typography Section */}
        <div className="space-y-12">
          {/* Headings */}
          <div className="bg-white p-6 rounded-lg shadow">
            <h2 className="text-2xl font-heading mb-4">Headings (Plus Jakarta Sans)</h2>
            <div className="flex flex-wrap gap-4">
              <h1 className="text-h1 font-heading">H1</h1>
              <h2 className="text-h2 font-heading">H2</h2>
              <h3 className="text-h3 font-heading">H3</h3>
              <h4 className="text-h4 font-heading">H4</h4>
              <h5 className="text-h5 font-heading">H5</h5>
              <h6 className="text-h6 font-heading">H6</h6>
            </div>
          </div>

          {/* Body Typography (Regular) */}
          <div className="bg-white p-6 rounded-lg shadow">
            <h2 className="text-2xl font-heading mb-4">Body Typography (Regular - Syne)</h2>
            <div className="flex flex-wrap gap-4">
              <p className="text-p1 font-body">P1</p>
              <p className="text-p2 font-body">P2</p>
              <p className="text-p3 font-body">P3</p>
              <p className="text-p4 font-body">P4</p>
              <p className="text-p5 font-body">P5</p>
              <p className="text-p6 font-body">P6</p>
            </div>
          </div>

          {/* Body Typography (Semi-Bold) */}
          <div className="bg-white p-6 rounded-lg shadow">
            <h2 className="text-2xl font-heading mb-4">Body Typography (Semi-Bold - Syne)</h2>
            <div className="flex flex-wrap gap-4">
              <p className="text-p1 font-body font-semibold">B1</p>
              <p className="text-p2 font-body font-semibold">B2</p>
              <p className="text-p3 font-body font-semibold">B3</p>
              <p className="text-p4 font-body font-semibold">B4</p>
              <p className="text-p5 font-body font-semibold">B5</p>
              <p className="text-p6 font-body font-semibold">B6</p>
            </div>
          </div>
        </div>

        {/* Colors and Gradients Section */}
        <div className="mt-12 bg-white p-6 rounded-lg shadow space-y-4">
          <h2 className="text-2xl font-heading mb-4">Colors and Gradients</h2>
          <div className="flex flex-wrap gap-4">
            <div className="bg-purple-light p-4">Light Background</div>
            <div className="bg-purple-normal text-purple-darker p-4">Normal Text</div>
            <div className="hover:bg-purple-dark-active p-4">Hover State</div>
            <div className="bg-orange-light p-4">Light Orange Background</div>
            <div className="text-orange-darker p-4">Darker Orange Text</div>
            <div className="hover:bg-orange-dark-active p-4">Hover State</div>
            <div className="bg-blue-light p-4">Light Blue Background</div>
            <div className="text-blue-darker p-4">Darker Blue Text</div>
            <div className="hover:bg-blue-dark-active p-4">Hover State</div>
            <div className="bg-gradient-to-l from-gradient-start to-gradient-end p-4">Gradient Background</div>
            <div className="bg-state-success text-white p-4">Success</div>
            <div className="bg-state-danger text-white p-4">Danger</div>
            <div className="bg-neutral-300 text-neutral-900 p-4">Neutral Colors</div>
          </div>
        </div>

        {/* Buttons Section */}
        <div className="mt-12 bg-white p-6 rounded-lg shadow space-y-4">
          <h1 className="text-3xl font-heading mb-6">Button Variants</h1>
          <div className="flex flex-wrap gap-4">
            {/* Primary Button */}
            <Button variant="primary" size="medium">
              Primary
            </Button>

            {/* Secondary Button */}
            <Button variant="secondary" size="medium">
              Secondary
            </Button>

            {/* Neutral Button */}
            <Button variant="neutral" size="medium">
              Neutral
            </Button>

            {/* Gradient Button */}
            <Button variant="gradient" size="large">
              Gradient
            </Button>

            {/* Button with Icon */}
            <Button variant="primary" size="medium" icon={<FaHome />}>
              With Icon
            </Button>

            {/* Loading Button */}
            <Button variant="primary" size="medium" isLoading={true}>
              Loading
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
