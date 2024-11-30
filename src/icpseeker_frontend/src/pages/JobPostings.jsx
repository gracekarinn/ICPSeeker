import React from 'react';
import Sidebar from '../components/Sidebar';

const jobData = [
  { id: 1, name: "Susi Puji Astuti", role: "Business Analyst", description: "I am a passionate job seeker eager to join a dynamic team in the Business Case Competition arena. With a strong background in strategic thinking and problem-solving." },
  { id: 2, name: "Taylor Smith", role: "Web Developer", description: "As a committed web developer, I am enthusiastic about leveraging my knowledge to support a dynamic team in the technology field. With a solid foundation in web technology." },
  { id: 3, name: "Mackenzie Holland", role: "Software Engineer", description: "A dedicated software engineer, I am excited to bring my skills to a forward-thinking team in the tech industry. With a solid foundation in coding and software development." },
];

const JobPostings = () => {
  return (
    <div className="flex h-screen">
      <Sidebar />
      <div className="flex-grow p-10">
        <div className="grid grid-cols-3 gap-4">
          {jobData.map(job => (
            <div key={job.id} className="bg-white p-4 rounded-lg shadow">
              <h3 className="text-lg font-semibold">{job.name}</h3>
              <p className="text-purple-800">{job.role}</p>
              <p className="text-sm text-gray-600 mt-2">{job.description}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default JobPostings;
