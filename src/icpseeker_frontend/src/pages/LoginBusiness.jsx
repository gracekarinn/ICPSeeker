import React from "react";
import Button from "../components/Button";
import { Link } from "react-router-dom";
//import Navbar from "../components/Navbar/Navbar";
import Logo from "../assets/logo.svg";

const LoginBusiness = () => {
  return (
    <>
      <div className="min-h-screen relative bg-gradient-to-l from-[#5F0EE7] to-[#350881] text-white flex flex-col justify-center items-center px-4">
        <img
          src="/landing/kanan-atas.png"
          alt="Decorative element"
          className="absolute top-0 right-0 w-48 h-96"
        />
        <img
          src="/landing/kanan-bawah.png"
          alt="Decorative element"
          className="absolute top-1/3 -translate-y-1/4 right-0 w-48 h-96"
        />
        <img
          src="/landing/kiri-atas.png"
          alt="Decorative element"
          className="absolute top-1/3 left-0 w-48 h-96"
        />
        <img
          src="/landing/kiri-bawah.png"
          alt="Decorative element"
          className="absolute top-1/3 translate-y-1/3 left-0 w-48 h-96"
        />
        {/* Login Form */}
      <div className="w-full max-w-md p-8 z-10">
        <img
          src={Logo}
          alt="ICP Seeker Logo"
          className="mx-auto h-20 w-auto mb-8 mt-16" 
        />
        <h2 className="text-2xl font-bold mb-6 text-left">Log In</h2>
        <p className="mb-6 text-left">Enter valid credentials to access all features of ICPSeeker</p>
        <form className ="w-full space-y-6">
          <div className="text-left"> 
            <label htmlFor="email" className="text-left block text-sm font-medium">Email</label>
            <input
              id="email"
              type="email"
              placeholder="Insert your email"
              className="w-full mr-36 p-2 rounded-md bg-white text-gray-900"
            />
          </div>
          <div>
            <label htmlFor="password" className="block text-sm font-medium text-left">Password</label>
            <input
              id="password"
              type="password"
              placeholder="Insert your password"
              className="w-full mr-36 p-2 rounded-md bg-white text-gray-900"
            />
          </div>
          <Button className="w-96" variant="gradient" size="large">Log in</Button>
        </form>
        <div className="mt-4 text-center">
          <Link to="/registerbusiness" className="text-h6 text-indigo-200 hover:text-indigo-400">Don’t have an account? Create Account</Link>
        </div>
      </div>
      </div>
    </>
  );
};

export default LoginBusiness;
