import React, { useState } from "react";
import { MessageCircle } from "lucide-react";
import Button from "../components/Button";
import Navbar from "../components/Navbar/Navbar";

const JobRecommendations = () => {
  const [showAIConsult, setShowAIConsult] = useState(false);

  // const buat sekarang
  const recommendedJobs = [
    {
      title: "Frontend Developer",
      company: "WebSoft",
      location: "New York",
      type: "Full-time",
      matchPercentage: 95,
    },
    {
      title: "UI/UX Designer",
      company: "TechCo",
      location: "Remote",
      type: "Full-time",
      matchPercentage: 88,
    },
    {
      title: "Full Stack Developer",
      company: "StackSolutions",
      location: "Amsterdam",
      type: "Full-time",
      matchPercentage: 82,
    },
  ];

  return (
    <>
      <Navbar />
      <div className="min-h-screen bg-gray-50 p-8">
        <div className="max-w-4xl mx-auto">
          <div className="text-center mb-8">
            <h1 className="text-4xl font-bold text-gray-900 mb-4">
              Based on our CV analysis, here are jobs that might suit you
            </h1>
            <Button
              variant="gradient"
              size="large"
              icon={<MessageCircle className="w-5 h-5" />}
              onClick={() => setShowAIConsult(!showAIConsult)}
            >
              Consult with AI
            </Button>
          </div>

          {showAIConsult && (
            <div className="mb-8 bg-purple-50 border border-purple-200 rounded-lg p-4">
              <h3 className="text-purple-800 text-lg font-semibold mb-2">
                AI Career Assistant
              </h3>
              <p className="text-purple-600">
                Based on your CV, I notice you have strong skills in frontend
                development and UI design. Would you like specific advice about
                career progression in these areas?
              </p>
            </div>
          )}

          <div className="space-y-6">
            {recommendedJobs.map((job, index) => (
              <div
                key={index}
                className="bg-white rounded-xl shadow-sm hover:shadow-md transition-shadow duration-200 p-6"
              >
                <div className="flex justify-between items-center mb-4">
                  <h2 className="text-xl font-bold text-gray-900">
                    {job.title}
                  </h2>
                  <span className="text-purple-600 font-semibold">
                    {job.matchPercentage}% Match
                  </span>
                </div>

                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <span className="text-gray-600">{job.company}</span>
                    <span className="bg-purple-100 text-purple-800 px-3 py-1 rounded-full text-sm">
                      {job.type}
                    </span>
                  </div>
                  <p className="text-gray-500">{job.location}</p>
                  <div className="pt-4">
                    <Button variant="primary" size="medium" className="w-full">
                      View Details
                    </Button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </>
  );
};

export default JobRecommendations;
