import React, {useState} from "react";
import Button from "../components/Button";
import { Link, useNavigate } from "react-router-dom";
import { RadioButton } from '../components/RadioButton';
import Logo from "../assets/logo.svg";

const RegisterBusiness = () => {
    const [role, setRole] = useState('');
    const navigate = useNavigate();
    const handleSelectRole = (selectedRole) => {
      setRole(selectedRole);
    };
    const handleFormSubmit = (event) => {
        event.preventDefault();  
        if (role === 'jobseeker') {
            navigate('/');  
        } else if (role === 'business') {
            navigate('/onboardingregister');  
        }
    };
    
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

      <div className="w-full max-w-md p-8 z-10">
        <img
          src={Logo}
          alt="ICP Seeker Logo"
          className="mx-auto h-20 w-auto mb-8 mt-16" 
        />
        <h2 className="text-2xl font-bold mb-6 text-left">Register</h2>
        <form className ="w-full space-y-6" onSubmit={handleFormSubmit}>
          <div className="text-left"> 
            <label htmlFor="fullName" className="text-left block text-sm font-medium">Full Name</label>
            <input
              id="fullName"
              type="text"
              placeholder="Insert your full name"
              className="w-full mr-36 p-2 rounded-md bg-white text-gray-900"
            />
          </div>
          <div>
            <label htmlFor="email" className="block text-sm font-medium text-left">Email</label>
            <input
              id="email"
              type="email"
              placeholder="Insert your email address"
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
          <div>
            <label htmlFor="confirmPassword" className="block text-sm font-medium text-left">confirm Password</label>
            <input
              id="confirmPassword"
              type="password"
              placeholder="Confirm your password"
              className="w-full mr-36 p-2 rounded-md bg-white text-gray-900"
            />
          </div>
          <div className="mb-6 mr-24">
          <label className="block text-sm font-bold mb-2">
            Your Role
          </label>
          <div className="flex gap-4 ">
          <RadioButton
            filled={role === 'jobseeker'}
            labelText="Jobseeker"
            active={true}
            onSelect={() => handleSelectRole('jobseeker')}
          />
          <RadioButton
            filled={role === 'business'}
            labelText="Business"
            active={true}
            onSelect={() => handleSelectRole('business')}
          />
          </div>
        </div>
          <Button className="w-96" variant="gradient" size="large">Create Account</Button>
        </form>
      </div>
      </div>
    </>
  );
};

export default RegisterBusiness;
