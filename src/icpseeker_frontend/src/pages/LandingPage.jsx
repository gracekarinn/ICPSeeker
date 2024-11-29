import React from "react";
import Button from "../components/Button";
import { Link } from "react-router-dom";

const LandingPage = () => {
  return (
    <div className="min-h-screen bg-gradient-to-r from-purple-normal to-blue-dark text-white flex flex-col justify-center items-center px-4">
      <h1 className="font-heading text-h1 text-center text-gradient">Jobseeking. <br/> Decentralized.</h1>
      <p className="font-body text-p2 text-center mt-6">
        Say goodbye to middlemen and hello to opportunities. <br/> Connect directly with employers,
        showcase your talent, <br/> and take charge of your career on our decentralized platform.
      </p>
      <div className="mt-8 flex space-x-4">
        <Button variant="gradient" size="medium">Start hiring</Button>
        <Link to="/jobs"><Button variant="secondary" size="medium">Find a job</Button></Link>
      </div>
    </div>
  );
};

export default LandingPage;
