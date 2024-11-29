import React, { useState } from "react";
import Navbar from "../components/Navbar/Navbar";

const DUMMY_CERTIFICATES = [
  {
    id: "XY-29012938",
    name: "Dandi Sumardi",
    company: "TechCorp",
    date: "2024-01-15",
  },
  {
    id: "XY-29012939",
    name: "Sarah Chen",
    company: "DevInc",
    date: "2024-02-20",
  },
  {
    id: "XY-29012940",
    name: "John Smith",
    company: "WebSoft",
    date: "2024-03-10",
  },
];

const VerifyCertificate = () => {
  const [certNumber, setCertNumber] = useState("");
  const [ownerName, setOwnerName] = useState("");
  const [showModal, setShowModal] = useState(false);
  const [result, setResult] = useState(null);

  const handleVerify = () => {
    const certificate = DUMMY_CERTIFICATES.find(
      (cert) =>
        cert.id.toLowerCase() === certNumber.toLowerCase() &&
        cert.name.toLowerCase() === ownerName.toLowerCase()
    );

    setResult(certificate);
    setShowModal(true);
  };

  return (
    <>
      <Navbar />
      <div className="bg-gradient-to-l min-h-screen from-[#5F0EE7] to-[#350881] text-white py-12 px-4">
        <div className="flex translate-y-1/4 gap-x-20 justify-center items-center max-w-5xl mx-auto">
          <div className="max-w-2xl">
            <h1 className="text-6xl font-bold mb-6">Verify Certificate</h1>
            <p className="text-xl">
              Not sure if a candidate is handing in a genuine internship
              certificate? Verify its validity by entering the certificate
              number and their name below!
            </p>
          </div>
          <div className="w-full max-w-xl">
            <div className="bg-white rounded-3xl p-8 shadow-lg">
              <div className="space-y-6">
                <div>
                  <label className="block text-[#5F0EE7] text-lg mb-2">
                    Certificate Number
                  </label>
                  <input
                    type="text"
                    value={certNumber}
                    onChange={(e) => setCertNumber(e.target.value)}
                    placeholder="XY-29012938..."
                    className="w-full p-3 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-[#5F0EE7] placeholder-gray-400"
                  />
                </div>
                <div>
                  <label className="block text-[#5F0EE7] text-lg mb-2">
                    Certificate owner name
                  </label>
                  <input
                    type="text"
                    value={ownerName}
                    onChange={(e) => setOwnerName(e.target.value)}
                    placeholder="As written on the certificate, ex: Dandi Sumardi"
                    className="w-full p-3 rounded-lg border border-gray-300 focus:outline-none focus:ring-2 focus:ring-[#5F0EE7] placeholder-gray-400"
                  />
                </div>
                <button
                  onClick={handleVerify}
                  className="w-full bg-[#5F0EE7] text-white py-3 rounded-lg text-lg font-medium hover:bg-opacity-90"
                >
                  Check Validity
                </button>
              </div>
            </div>
          </div>
        </div>

        {showModal && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-2xl p-8 max-w-md w-full text-gray-800">
              <div className="text-center">
                {result ? (
                  <>
                    <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                      <svg
                        className="w-8 h-8 text-green-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth="2"
                          d="M5 13l4 4L19 7"
                        />
                      </svg>
                    </div>
                    <h3 className="text-2xl font-bold text-green-600 mb-4">
                      Valid Certificate
                    </h3>
                    <p className="mb-2">Certificate Number: {result.id}</p>
                    <p className="mb-2">Name: {result.name}</p>
                    <p className="mb-2">Company: {result.company}</p>
                    <p className="mb-4">Issue Date: {result.date}</p>
                  </>
                ) : (
                  <>
                    <div className="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4">
                      <svg
                        className="w-8 h-8 text-red-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth="2"
                          d="M6 18L18 6M6 6l12 12"
                        />
                      </svg>
                    </div>
                    <h3 className="text-2xl font-bold text-red-600 mb-4">
                      Invalid Certificate
                    </h3>
                    <p>The certificate number or name provided is not valid.</p>
                  </>
                )}
                <button
                  onClick={() => setShowModal(false)}
                  className="mt-6 bg-gray-100 text-gray-800 px-6 py-2 rounded-lg hover:bg-gray-200"
                >
                  Close
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </>
  );
};

export default VerifyCertificate;
