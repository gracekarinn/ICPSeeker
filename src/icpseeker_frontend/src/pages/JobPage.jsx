import React, { useState } from "react";
import Navbar from "../components/Navbar/Navbar";

const DUMMY_JOBS = [
  {
    title: "UI/UX Designer",
    company: "TechCo",
    location: "Remote",
    type: "Full-time",
  },
  {
    title: "Frontend Developer",
    company: "WebSoft",
    location: "New York",
    type: "Full-time",
  },
  {
    title: "Backend Engineer",
    company: "DataTech",
    location: "San Francisco",
    type: "Contract",
  },
  {
    title: "Product Manager",
    company: "ProductHouse",
    location: "London",
    type: "Full-time",
  },
  {
    title: "DevOps Engineer",
    company: "CloudTech",
    location: "Singapore",
    type: "Full-time",
  },
  {
    title: "Data Scientist",
    company: "AILabs",
    location: "Remote",
    type: "Contract",
  },
  {
    title: "Mobile Developer",
    company: "AppWorks",
    location: "Berlin",
    type: "Full-time",
  },
  {
    title: "Software Architect",
    company: "SystemsInc",
    location: "Toronto",
    type: "Full-time",
  },
  {
    title: "QA Engineer",
    company: "QualityTech",
    location: "Remote",
    type: "Contract",
  },
  {
    title: "Full Stack Developer",
    company: "StackSolutions",
    location: "Amsterdam",
    type: "Full-time",
  },
  {
    title: "Systems Analyst",
    company: "AnalyticsCo",
    location: "Sydney",
    type: "Full-time",
  },
  {
    title: "Cloud Architect",
    company: "CloudWorks",
    location: "Remote",
    type: "Contract",
  },
  {
    title: "UX Researcher",
    company: "UserFirst",
    location: "Paris",
    type: "Full-time",
  },
  {
    title: "Security Engineer",
    company: "SecureTech",
    location: "Tokyo",
    type: "Full-time",
  },
  {
    title: "ML Engineer",
    company: "MLSolutions",
    location: "Remote",
    type: "Contract",
  },
  {
    title: "Technical Lead",
    company: "LeadTech",
    location: "Dubai",
    type: "Full-time",
  },
  {
    title: "Database Admin",
    company: "DataCorp",
    location: "Chicago",
    type: "Full-time",
  },
  {
    title: "Network Engineer",
    company: "NetWorks",
    location: "Remote",
    type: "Contract",
  },
  {
    title: "Blockchain Developer",
    company: "ChainTech",
    location: "Miami",
    type: "Full-time",
  },
  {
    title: "AI Researcher",
    company: "AIResearch",
    location: "Boston",
    type: "Full-time",
  },
];

const JobPage = () => {
  const [searchTerm, setSearchTerm] = useState("");
  const [filteredJobs, setFilteredJobs] = useState(DUMMY_JOBS);

  const handleSearch = (e) => {
    const term = e.target.value.toLowerCase();
    setSearchTerm(term);
    const filtered = DUMMY_JOBS.filter(
      (job) =>
        job.title.toLowerCase().includes(term) ||
        job.company.toLowerCase().includes(term) ||
        job.location.toLowerCase().includes(term) ||
        job.type.toLowerCase().includes(term)
    );
    setFilteredJobs(filtered);
  };

  return (
    <>
      <Navbar />
      <div className="min-h-screen">
        <div className="bg-gradient-to-l from-[#5F0EE7] to-[#350881] text-white py-12 px-4">
          <div className="max-w-6xl mx-auto text-center">
            <h1 className="text-5xl font-bold mb-6">Available Jobs</h1>
            <p className="text-xl mb-8">
              Psst... by registering and uploading your CV, you will get access
              to
              <br />
              personalized jobs catered specifically for your skills!
            </p>
            <button className="bg-white text-[#5F0EE7] px-8 py-3 rounded-lg text-lg font-medium">
              Register now!
            </button>
          </div>
        </div>

        <div className="bg-white -mt-8 rounded-t-[2.5rem] px-4 py-8">
          <div className="max-w-6xl mx-auto">
            <div className="flex gap-4 mb-8">
              <input
                type="text"
                value={searchTerm}
                onChange={handleSearch}
                placeholder="Search opportunities by job title..."
                className="flex-1 p-4 rounded-lg border border-gray-200 focus:outline-none focus:ring-2 focus:ring-purple-500"
              />
              <button className="bg-[#5F0EE7] text-white px-6 py-2 rounded-lg flex items-center gap-2">
                <span>Filter</span>
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"
                  />
                </svg>
              </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {filteredJobs.map((job, i) => (
                <div key={i} className="bg-gray-50 p-6 rounded-xl shadow-sm">
                  <h2 className="text-[#5F0EE7] text-xl font-semibold mb-3">
                    {job.title}
                  </h2>
                  <div className="space-y-2 text-gray-600">
                    <p>{job.company}</p>
                    <p>{job.location}</p>
                    <p>{job.type}</p>
                  </div>
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
