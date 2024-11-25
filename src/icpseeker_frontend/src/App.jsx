import { useState } from "react";
import { icpseeker_backend } from "declarations/icpseeker_backend";
import React from "react";
import Button from "./components/button";
import { FaHome } from "react-icons/fa"; 

function App() {
  const [greeting, setGreeting] = useState("");

  function handleSubmit(event) {
    event.preventDefault();
    const name = event.target.elements.name.value;
    icpseeker_backend.greet(name).then((greeting) => {
      setGreeting(greeting);
    });
    return false;
  }

  return (
    <main className="min-h-screen flex flex-col items-center justify-center bg-gray-100 p-8 font-body">
      {/* Logo */}
      <img
        src="/logo2.svg"
        alt="DFINITY logo"
        className="w-40 mb-4 animate-bounce"
      />
      {/* Form */}
      <form
        action="#"
        onSubmit={handleSubmit}
        className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4"
      >
        <label
          htmlFor="name"
          className="block text-gray-700 text-sm font-bold mb-2 font-heading"
        >
          Enter your name (Plus Jakarta Sans):
        </label>
        <input
          id="name"
          alt="Name"
          type="text"
          className="font-body shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          placeholder="Input field (Syne)"
        />
        <button
          type="submit"
          className="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline font-heading"
        >
          Submit (Plus Jakarta Sans)
        </button>
      </form>
      <section
        id="greeting"
        className={`mt-4 text-xl font-semibold ${
          greeting ? "text-green-500" : "hidden"
        }`}
      >
        {greeting} (Syne)
      </section>

      {/* Additional Typography Section */}
      <div className="mt-8 w-full space-y-8">
        {/* Headings */}
        <div>
          <h2 className="text-2xl font-heading mb-4">Headings (Plus Jakarta Sans)</h2>
          <div className="flex flex-row flex-wrap gap-4">
            <h1 className="text-h1 font-heading">H1</h1>
            <h2 className="text-h2 font-heading">H2</h2>
            <h3 className="text-h3 font-heading">H3</h3>
            <h4 className="text-h4 font-heading">H4</h4>
            <h5 className="text-h5 font-heading">H5</h5>
            <h6 className="text-h6 font-heading">H6</h6>
          </div>
        </div>

        {/* Body Typography (Regular) */}
        <div>
          <h2 className="text-2xl font-heading mb-4">Body Typography (Regular - Syne)</h2>
          <div className="flex flex-row flex-wrap gap-4">
            <p className="text-p1 font-body">P1</p>
            <p className="text-p2 font-body">P2</p>
            <p className="text-p3 font-body">P3</p>
            <p className="text-p4 font-body">P4</p>
            <p className="text-p5 font-body">P5</p>
            <p className="text-p6 font-body">P6</p>
          </div>
        </div>

        {/* Body Typography (Semi-Bold) */}
        <div>
          <h2 className="text-2xl font-heading mb-4">Body Typography (Semi-Bold - Syne)</h2>
          <div className="flex flex-row flex-wrap gap-4">
            <p className="text-p1 font-body font-semibold">B1</p>
            <p className="text-p2 font-body font-semibold">B2</p>
            <p className="text-p3 font-body font-semibold">B3</p>
            <p className="text-p4 font-body font-semibold">B4</p>
            <p className="text-p5 font-body font-semibold">B5</p>
            <p className="text-p6 font-body font-semibold">B6</p>
          </div>
        </div>
      </div>

        <div className="flex flex-row flex-wrap gap-4 p-4">
        <div className="bg-purple-light p-4">Light Background</div>
        <div className="bg-purple-normal text-purple-darker p-4">Normal Text</div>
        <div className="hover:bg-purple-dark-active p-4">Hover State</div>
        <div className="bg-orange-light p-4">Light Orange Background</div>
        <div className="text-orange-darker p-4">Darker Orange Text</div>
        <div className="hover:bg-orange-dark-active p-4">Hover State</div>
        <div className="bg-blue-light p-4">Light Blue Background</div>
        <div className="text-blue-darker p-4">Darker Blue Text</div>
        <div className="hover:bg-blue-dark-active p-4">Hover State</div>
        <div className="bg-gradient-to-l from-gradient-start to-gradient-end">Gradient Background</div>
        <div className="bg-state-success text-white">Success</div>
        <div className="bg-state-danger text-white">Danger</div>
        <div className="bg-neutral-300 text-neutral-900">Neutral Colors</div>
    </div>
    <div className="min-h-screen flex flex-col items-center justify-center space-y-4 bg-gray-50 p-8">
      <h1 className="text-3xl font-heading mb-6">Button Variants</h1>
      <div className="flex flex-wrap gap-4">
        {/* Primary Button */}
        <Button variant="primary" size="medium">
          Primary
        </Button>

        {/* Secondary Button */}
        <Button variant="secondary" size="medium">
          Secondary
        </Button>

        {/* Neutral Button */}
        <Button variant="neutral" size="medium">
          Neutral
        </Button>

        {/* Gradient Button */}
        <Button variant="gradient" size="large">
          Gradient
        </Button>

        {/* Button with Icon */}
        <Button variant="primary" size="medium" icon={<FaHome />}>
          With Icon
        </Button>

        {/* Loading Button */}
        <Button variant="primary" size="medium" isLoading={true}>
          Loading
        </Button>
      </div>
    </div>

    </main>
  );
}

export default App;
