import React from "react";
import Button from "../components/Button"; 
import Navbar from "../components/Navbar/Navbar"; 
import { Link } from "react-router-dom";

const JobPage = () => {
  return (
    <>
    <Navbar />
    <div className="min-h-screen">
      <div className="bg-purple-custom text-white py-12 px-4">
        <div className="max-w-6xl mx-auto">
          <h1 className="text-center text-3xl font-bold mb-6">Available Jobs</h1>
          <p className="text-center mb-6">
            Psst... by registering and uploading your CV, you will get access <br/>
            to personalized jobs catered specifically for your skills!
          </p>
          <div className="flex justify-center">
          <Link to="/register">
            <Button variant="secondary" size="large">Register</Button>
            </Link>
          </div>
        </div>
      </div>

      <div className="bg-purple-baselight text-gray-800 py-12 px-4 rounded-t-3xl">
        <div className="max-w-6xl mx-auto">
          <div className="flex justify-between items-center mb-6 gap-x-4">
            <input
              type="text"
              placeholder=""
              className="form-input mt-1 block w-full border-2 border-purple-900 shadow-sm focus:border-purple-500 focus:ring focus:ring-purple-custom focus:ring-opacity-50 rounded-md"
            />
            <Button variant="primary" size="medium">Filter</Button>
          </div>
          <div className="grid grid-cols-3 gap-4">
            {Array.from({ length: 9 }).map((_, idx) => (
              <div key={idx} className="p-4 bg-gray-100 rounded-lg shadow">
                <h2 className="font-semibold">UI/UX Designer</h2>
                <p className="text-sm">UI/UX Designer</p>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
    </>
  );
};

export default JobPage;
