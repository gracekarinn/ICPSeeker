import { useState } from "react";
import React from "react";
import Button from "./components/button";
import { FaHome } from "react-icons/fa";
import AuthButton from "./components/Auth/AuthButton";
import Navbar from "./components/Navbar/Navbar";
import { Checkbox } from "./components/Checkbox";
import { RadioButton } from "./components/RadioButton";
import { Tabs } from "./components/Tabs";

function App() {

  const tabsData = [
    { id: 1, label: "Seminar" },
    { id: 2, label: "Workshop" },
    { id: 3, label: "Talkshow" },
  ];
  
  // State for managing selected Radio Button
  const [selectedRadio, setSelectedRadio] = useState(null);

  // Checkbox Items
  const checkboxItems = [
    { id: 1, labelText: "Checkbox 1", initialFilled: false, active: true },
    { id: 2, labelText: "Checkbox 2", initialFilled: true, active: true },
    { id: 3, labelText: "Checkbox 3", initialFilled: false, active: false },
    { id: 4, labelText: "Checkbox 4", initialFilled: true, active: false },
  ];

  // RadioButton Items
  const radioButtonItems = [
    { id: 1, labelText: "Radio 1", active: true },
    { id: 2, labelText: "Radio 2", active: true },
    { id: 3, labelText: "Radio 3", active: false },
  ];

  return (
    <div className="min-h-screen bg-gray-100 flex flex-col">
      {/* Navbar */}
      <Navbar />

      <div className="container mx-auto p-8 bg-gray-50">
        {/* Header Section */}
        <div className="bg-white rounded-lg shadow-md p-8 mb-8">
          <h1 className="text-2xl font-bold mb-4 text-center">ICPSeeker</h1>
          <AuthButton />
        </div>

                {/* Tabs Section */}
                <div className="mt-12 bg-white p-6 rounded-lg shadow">
          <h2 className="text-2xl font-heading mb-4">Tabs Component</h2>
          <div className="border-dashed border-2 border-purple-300 p-4">
            <Tabs tabs={tabsData} />
          </div>
        </div>

        {/* Checkbox Section */}
        <div className="mt-12 bg-white p-6 rounded-lg shadow">
          <h2 className="text-2xl font-heading mb-4">Checkbox Button</h2>
          <div className="flex flex-wrap gap-4 border-dashed border-2 border-purple-300 p-4">
            {checkboxItems.map((item) => (
              <Checkbox
                key={item.id}
                initialFilled={item.initialFilled}
                labelText={item.labelText}
                active={item.active}
              />
            ))}
          </div>
        </div>

        {/* Radio Button Section */}
        <div className="mt-12 bg-white p-6 rounded-lg shadow">
          <h2 className="text-2xl font-heading mb-4">Radio Button</h2>
          <div className="flex flex-wrap gap-4 border-dashed border-2 border-blue-300 p-4">
            {radioButtonItems.map((item) => (
              <RadioButton
                key={item.id}
                filled={selectedRadio === item.id}
                labelText={item.labelText}
                active={item.active}
                onSelect={() => setSelectedRadio(item.id)}
              />
            ))}
          </div>
        </div>

        {/* Colors and Gradients Section */}
        <div className="mt-12 bg-white p-6 rounded-lg shadow space-y-4">
          <h2 className="text-2xl font-heading mb-4">Colors and Gradients</h2>
          <div className="flex flex-wrap gap-4">
            <div className="bg-purple-light p-4">Light Background</div>
            <div className="bg-purple-normal text-purple-darker p-4">Normal Text</div>
            <div className="hover:bg-purple-dark-active p-4">Hover State</div>
            <div className="bg-orange-light p-4">Light Orange Background</div>
            <div className="text-orange-darker p-4">Darker Orange Text</div>
            <div className="hover:bg-orange-dark-active p-4">Hover State</div>
            <div className="bg-blue-light p-4">Light Blue Background</div>
            <div className="text-blue-darker p-4">Darker Blue Text</div>
            <div className="hover:bg-blue-dark-active p-4">Hover State</div>
            <div className="bg-gradient-to-l from-gradient-start to-gradient-end p-4">
              Gradient Background
            </div>
            <div className="bg-state-success text-white p-4">Success</div>
            <div className="bg-state-danger text-white p-4">Danger</div>
            <div className="bg-neutral-300 text-neutral-900 p-4">Neutral Colors</div>
          </div>
        </div>

        {/* Buttons Section */}
        <div className="mt-12 bg-white p-6 rounded-lg shadow space-y-4">
          <h1 className="text-3xl font-heading mb-6">Button Variants</h1>
          <div className="flex flex-wrap gap-4">
            <Button variant="primary" size="medium">
              Primary
            </Button>
            <Button variant="secondary" size="medium">
              Secondary
            </Button>
            <Button variant="neutral" size="medium">
              Neutral
            </Button>
            <Button variant="gradient" size="large">
              Gradient
            </Button>
            <Button variant="primary" size="medium" icon={<FaHome />}>
              With Icon
            </Button>
            <Button variant="primary" size="medium" isLoading={true}>
              Loading
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
