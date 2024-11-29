import React from "react";

const Button = ({
  children,
  variant = "primary", // primary, secondary, neutral, gradient
  icon = null, // Icon on the left side of the button
  size = "medium", // small, medium, large
  isLoading = false, // Adds a loading spinner
  className = "",
  ...props
}) => {
  const baseStyles =
    "flex items-center justify-center rounded-lg font-medium transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-offset-2";

  const sizeStyles = {
    small: "text-sm py-1 px-3",
    medium: "text-lg  py-2 px-4",
    large: "text-xl py-3 px-6",
  };

  const variantStyles = {
    primary:
      "bg-purple-normal text-white hover:bg-purple-dark focus:ring-purple-normal",
    secondary:
      "bg-purple-light text-purple-dark hover:bg-purple-light-hover focus:ring-purple-light",
    neutral:
      "bg-neutral-700 text-white hover:bg-neutral-900 focus:ring-neutral-700",
    gradient:
      "bg-gradient-to-r from-purple-normal to-blue-normal text-white hover:from-purple-dark hover:to-blue-dark focus:ring-purple-normal",
  };

  return (
    <button
      className={`${baseStyles} ${sizeStyles[size]} ${variantStyles[variant]} ${className}`}
      {...props}
    >
      {isLoading ? (
        <span className="animate-spin w-5 h-5 border-2 border-white border-t-transparent rounded-full"></span>
      ) : (
        <>
          {icon && <span className="mr-2">{icon}</span>}
          {children}
        </>
      )}
    </button>
  );
};

export default Button;
