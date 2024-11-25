import { useState } from "react";
import { icpseeker_backend } from "declarations/icpseeker_backend";

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
        <div className="space-y-4">
          <h2 className="text-2xl font-heading mb-2">Headings (Plus Jakarta Sans)</h2>
          <h1 className="text-h1 font-heading">H1: The art of Typography</h1>
          <h2 className="text-h2 font-heading">H2: Craft Beautiful Text</h2>
          <h3 className="text-h3 font-heading">H3: Typography Rules</h3>
          <h4 className="text-h4 font-heading">H4: Subheading Example</h4>
          <h5 className="text-h5 font-heading">H5: Smaller Heading</h5>
          <h6 className="text-h6 font-heading">H6: Smallest Heading</h6>
        </div>

        {/* Body Typography (Regular) */}
        <div className="space-y-4">
          <h2 className="text-2xl font-heading mb-2">Body Typography (Regular - Syne)</h2>
          <p className="text-p1 font-body">P1: Regular - 24px, 150% Line Height</p>
          <p className="text-p2 font-body">P2: Regular - 20px, 150% Line Height</p>
          <p className="text-p3 font-body">P3: Regular - 16px, 150% Line Height</p>
          <p className="text-p4 font-body">P4: Regular - 14px, 150% Line Height</p>
          <p className="text-p5 font-body">P5: Regular - 12px, 150% Line Height</p>
          <p className="text-p6 font-body">P6: Regular - 10px, 150% Line Height</p>
        </div>

        {/* Body Typography (Semi-Bold) */}
        <div className="space-y-4">
          <h2 className="text-2xl font-heading mb-2">Body Typography (Semi-Bold - Syne)</h2>
          <p className="text-p1 font-body font-semibold">B1: Semi-Bold - 24px, 150% Line Height</p>
          <p className="text-p2 font-body font-semibold">B2: Semi-Bold - 20px, 150% Line Height</p>
          <p className="text-p3 font-body font-semibold">B3: Semi-Bold - 16px, 150% Line Height</p>
          <p className="text-p4 font-body font-semibold">B4: Semi-Bold - 14px, 150% Line Height</p>
          <p className="text-p5 font-body font-semibold">B5: Semi-Bold - 12px, 150% Line Height</p>
          <p className="text-p6 font-body font-semibold">B6: Semi-Bold - 10px, 150% Line Height</p>
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
    </div>
    </main>
  );
}

export default App;
