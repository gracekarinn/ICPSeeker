import React, { useState } from "react";

export const Checkbox = ({ initialFilled, labelText, active }) => {
  const [filled, setFilled] = useState(initialFilled);

  const baseStyle =
    "w-6 h-6 flex items-center justify-center border-2 rounded-md transition";
  const activeStyle = active
    ? filled
      ? "bg-purple-light border-purple-dark text-purple-darker cursor-pointer"
      : "bg-transparent border-purple-dark cursor-pointer"
    : "bg-neutral-300 border-neutral-500 text-neutral-700 cursor-not-allowed";

  const labelStyle = labelText
    ? `ml-2 ${
        active
          ? "text-purple-dark"
          : "text-neutral-500 cursor-not-allowed opacity-50"
      }`
    : "";

  return (
    <div
      className={`flex items-center ${active ? "cursor-pointer" : "cursor-not-allowed"}`}
      onClick={() => active && setFilled(!filled)}
    >
      <div
        className={`${baseStyle} ${activeStyle}`}
        role="checkbox"
        aria-checked={filled}
      >
        {filled && <span className="w-3 h-3 bg-purple-dark rounded"></span>}
      </div>
      {labelText && <span className={labelStyle}>{labelText}</span>}
    </div>
  );
};
