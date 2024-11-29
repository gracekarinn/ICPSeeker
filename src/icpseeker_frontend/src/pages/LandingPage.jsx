import React from "react";
import Button from "../components/Button";
import { Link } from "react-router-dom";
import Navbar from "../components/Navbar/Navbar";

const LandingPage = () => {
  return (
    <>
      <Navbar />
      <div className="min-h-screen relative bg-gradient-to-r from-indigo-600 to-indigo-900 text-white flex flex-col justify-center items-center px-4">
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
          className="absolute top-1/3 translate-y-1/2 left-0 w-48 h-96"
        />
        <h1 className="text-5xl font-bold text-center">
          Jobseeking. <br /> Decentralized.
        </h1>
        <p className="text-lg text-center mt-6 max-w-2xl">
          Say goodbye to middlemen and hello to opportunities. <br /> Connect
          directly with employers, showcase your talent, <br /> and take charge
          of your career on our decentralized platform.
        </p>
        <div className="mt-8 flex gap-4">
          <Button className="bg-gradient-to-r from-indigo-500 to-indigo-700 hover:from-indigo-600 hover:to-indigo-800">
            Start hiring
          </Button>
          <Link to="/jobs">
            <Button
              variant="outline"
              className="text-white border-white hover:bg-white/10"
            >
              Find a job
            </Button>
          </Link>
        </div>
      </div>
    </>
  );
};

export default LandingPage;
