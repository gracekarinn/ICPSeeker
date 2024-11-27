import React, { useState } from "react";

export const Tabs = ({ tabs }) => {
  const [activeTab, setActiveTab] = useState(tabs[0]?.id || 0);

  return (
    <div className="flex space-x-4">
      {tabs.map((tab) => (
        <div
          key={tab.id}
          className={`px-6 py-2 rounded-lg transition cursor-pointer ${
            activeTab === tab.id
              ? "bg-purple-light text-purple-dark font-semibold shadow-md"
              : "bg-gray-100 text-gray-400"
          }`}
          onClick={() => setActiveTab(tab.id)}
        >
          {tab.label}
        </div>
      ))}
    </div>
  );
};
