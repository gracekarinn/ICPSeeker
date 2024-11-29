import React from "react";
import Button from "../components/Button"; 
import Navbar from "../components/Navbar/Navbar"; 

const VerifyCertificate = () => {
  return (
    <>
    <Navbar />
    <div className="min-h-screen">
      <div className="bg-purple-custom text-white py-12 px-4">
        <div className="max-w-6xl mx-auto">
          <h1 className="text-center text-3xl font-bold mb-6">Verify Certificate</h1>
          <p className="text-center mb-6">
            Not sure if a candidate is handing in a genuine internship certificate? <br/>
            Verify its validity by entering the certificate number and their name below!
          </p>
        </div>
      </div>

      <div className="bg-purple-baselight text-gray-800 py-12 px-4 rounded-t-3xl">
        <div className="max-w-lg mx-auto">
          <div className="flex justify-between items-center mb-3 gap-x-4">
            <input
              type="text"
              placeholder="Enter certificate number"
              className="form-input mt-1 block w-full px-3 py-2 border-2 border-purple-900 shadow-sm focus:border-purple-500 focus:ring focus:ring-purple-custom focus:ring-opacity-50 rounded-md"
            />
          </div>
          <div className="flex justify-between items-center mb-6 gap-x-4">
            <input
              type="text"
              placeholder="Enter candidate name"
              className="form-input mt-1 block w-full px-3 py-2 border-2 border-purple-900 shadow-sm focus:border-purple-500 focus:ring focus:ring-purple-custom focus:ring-opacity-50 rounded-md"
            />
          </div>
          <div className="flex justify-center">
            <Button variant="primary" size="large">Check Validity</Button>
          </div>
        </div>
      </div>
    </div>
    </>
  );
};

export default VerifyCertificate;
