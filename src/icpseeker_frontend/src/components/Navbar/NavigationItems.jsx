import React from "react";

const navigationItems= [
    { label: "Home", path: "/" },
    { label: "For Businesses", path: "/businesses" },
    { label: "Find a Job", path: "/job" },
    { label: "Quick Projects", path: "/projects" },
    { label: "Verify Certificate", path: "/certificate" },
];

export const NavigationItems = () => {
    return navigationItems.map((item) => (
        <button
            key={item.path}
            className="self-stretch my-auto"
            tabIndex={0}
        >
            {item.label}
        </button>
    ));
};