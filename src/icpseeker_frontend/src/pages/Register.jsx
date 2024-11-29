import React from "react";
import Button from "../components/Button";
import { Link } from "react-router-dom";
import Navbar from "../components/Navbar/Navbar";

const Register = () => {
  return (
    <div className="min-h-screen bg-gradient-to-r from-purple-normal to-blue-dark text-white flex flex-col justify-center items-center px-4">
      <div className="w-full max-w-md bg-transparent p-8 rounded-lg shadow-lg">
        <h2 className="font-heading text-2xl text-center mb-6">Masuk ke akun</h2>
        <p className="text-center mb-8">Masukkan kredensial yang valid untuk mengakses seluruh fitur KaryaKita</p>
        <form className="space-y-6">
          <div>
            <label htmlFor="email" className="block text-sm font-medium text-gray-300">Email</label>
            <input
              type="email"
              id="email"
              name="email"
              placeholder="Masukkan Email"
              className="mt-1 block w-full px-3 py-2 bg-white text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <div>
            <label htmlFor="password" className="block text-sm font-medium text-gray-300">Password</label>
            <input
              type="password"
              id="password"
              name="password"
              placeholder="Masukkan Password"
              className="mt-1 block w-full px-3 py-2 bg-white text-gray-900 rounded-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
            />
          </div>
          <Button variant="primary" size="large">Masuk</Button>
        </form>
        <div className="mt-4 text-center">
          <Link to="/register" className="text-indigo-200 hover:text-indigo-400">Tidak memiliki akun? Buat Akun</Link>
        </div>
        <div className="mt-4 flex justify-center">
          <Link to="/jobs">
            <Button variant="secondary" size="large">Start hiring</Button>
          </Link>
        </div>
      </div>
    </div>
  );
};

export default Register;
