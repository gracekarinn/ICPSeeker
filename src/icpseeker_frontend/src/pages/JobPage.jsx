import React from "react";
import Button from "../components/Button"; 

const JobPage = () => {
  return (
    <div className="bg-purple-custom text-gray-800 min-h-screen">
      <div className="max-w-6xl mx-auto py-12 px-4">
        <h1 className="text-center text-white text-3xl font-bold mb-6">Available Jobs</h1>
        <p className="mb-6 text-white text-center">
          Psst... by registering and uploading your CV, you will get access
          to personalized jobs catered specifically for your skills!
        </p>
        <Button variant="secondary" size="medium">Register</Button>
        <div className="mt-6 flex justify-between items-center">
          <input
            type="text"
            placeholder="Filter"
            className="form-input mt-1 block w-full border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50 rounded-md"
          />
          <Button variant="secondary" size="medium">Filter</Button>
        </div>
        <div className="mt-6 grid grid-cols-3 gap-4">
          {Array.from({ length: 9 }).map((_, idx) => ( // Updated to fill the grid with 9 cards
            <div key={idx} className="p-4 bg-gray-100 rounded-lg shadow">
              <h2 className="font-semibold">UI/UX Designer</h2>
              <p className="text-sm">UI/UX Designer</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default JobPage;
