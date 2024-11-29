import React from "react";
import { Link } from "react-router-dom";

const navigationItems = [
  { label: "Home", path: "/" },
  { label: "For Businesses", path: "/businesses" },
  { label: "Find a Job", path: "/jobs" },
  { label: "Ask AI", path: "/ask" },
  { label: "Verify Certificate", path: "/verifycertificate" },
];

export const NavigationItems = () => {
  return (
    <div className="flex justify-between flex-grow max-w-2xl mx-auto">
      {navigationItems.map((item) => (
        <Link
          key={item.path}
          to={item.path}
          className="text-white text-lg hover:underline focus:outline-none"
          tabIndex={0}
        >
          {item.label}
        </Link>
      ))}
    </div>
  );
};
