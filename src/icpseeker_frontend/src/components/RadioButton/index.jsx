import React from "react";

export const RadioButton = ({ filled, labelText, active, onSelect }) => {
  const baseStyle =
    "w-6 h-6 flex items-center justify-center border-2 rounded-full transition";
  const activeStyle = active
    ? filled
      ? "bg-blue-light border-blue-dark text-blue-dark cursor-pointer"
      : "bg-transparent border-blue-dark cursor-pointer"
    : "bg-neutral-300 border-neutral-500 text-neutral-700 cursor-not-allowed";

  const labelStyle = labelText
    ? `ml-2 ${
        active
          ? "text-blue-dark"
          : "text-neutral-500 cursor-not-allowed opacity-50"
      }`
    : "";

  return (
    <div
      className={`flex items-center ${active ? "cursor-pointer" : "cursor-not-allowed"}`}
      onClick={() => active && onSelect()}
    >
      <div
        className={`${baseStyle} ${activeStyle}`}
        role="radio"
        aria-checked={filled}
      >
        {filled && <span className="w-3 h-3 bg-blue-dark rounded-full"></span>}
      </div>
      {labelText && <span className={labelStyle}>{labelText}</span>}
    </div>
  );
};
