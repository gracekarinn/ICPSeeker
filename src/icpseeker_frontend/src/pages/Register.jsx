import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { AuthManager } from "../auth/AuthManager";

const StepIndicator = ({ currentStep, totalSteps }) => (
  <div className="flex justify-center space-x-2 mb-8">
    {[...Array(totalSteps)].map((_, index) => (
      <div
        key={index}
        className={`w-3 h-3 rounded-full ${
          index <= currentStep ? "bg-blue-600" : "bg-gray-300"
        }`}
      />
    ))}
  </div>
);

const Register = () => {
  const navigate = useNavigate();
  const [step, setStep] = useState(0);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [authManager, setAuthManager] = useState(null);

  useEffect(() => {
    const initAuth = async () => {
      try {
        const manager = await AuthManager.create();
        setAuthManager(manager);
      } catch (error) {
        console.error("Failed to initialize auth:", error);
        setError("Authentication initialization failed");
      }
    };

    initAuth();
  }, []);

  const [userProfile, setUserProfile] = useState({
    name: "",
    email: "",
    phone_number: "",
    city: "",
    country: "",
  });

  const [education, setEducation] = useState({
    high_school: {
      school_name: "",
      track: "",
      city: "",
      country: "",
      start_year: 2020,
      end_year: 2024,
      status: "InProgress",
    },
    university: [
      {
        university_name: "",
        level: "Bachelor",
        major: "",
        city: "",
        country: "",
        start_year: 2024,
        gpa: null,
        status: "InProgress",
      },
    ],
  });

  const [bankInfo, setBankInfo] = useState({
    account_holder_name: "",
    bank_name: "",
    swift_code: "",
    account_number: "",
    bank_country: "",
    bank_branch: "",
  });

  const [cv, setCV] = useState({
    title: "",
    content: "",
  });

  const handleUserProfileSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    if (!authManager) {
      setError("Authentication not initialized");
      setLoading(false);
      return;
    }

    try {
      if (!authManager.backendActor) {
        await authManager.initBackendActor();
      }

      await authManager.backendActor.login();

      const response = await authManager.backendActor.update_user({
        name: [userProfile.name],
        email: [userProfile.email],
        phone_number: [userProfile.phone_number],
        city: [userProfile.city],
        country: [userProfile.country],
      });

      console.log("Update response:", response);

      if ("Success" in response) {
        setStep(1);
      } else {
        setError(response.Error || "Update failed");
      }
    } catch (error) {
      console.error("Profile update error:", error);
      setError(error.message || "Failed to update profile");
    } finally {
      setLoading(false);
    }
  };

  const handleEducationSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    try {
      const authManager = await AuthManager.create();
      const actor = authManager.backendActor;

      const payload = {
        high_school: [education.high_school],
        university: [education.university],
      };

      const response = await actor.add_education(payload);
      if ("Success" in response) {
        setStep(2);
      } else {
        setError(response.Error);
      }
    } catch (error) {
      setError("Failed to save education information");
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const handleBankInfoSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    try {
      const authManager = await AuthManager.create();
      const actor = authManager.backendActor;

      const response = await actor.add_bank_info(bankInfo);
      if ("Success" in response) {
        setStep(3);
      } else {
        setError(response.Error);
      }
    } catch (error) {
      setError("Failed to save bank information");
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const handleCVSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    try {
      const authManager = await AuthManager.create();
      const actor = authManager.backendActor;

      const response = await actor.upload_cv(cv);
      if (response.cv) {
        navigate("/dashboard");
      } else {
        setError(response.message);
      }
    } catch (error) {
      setError("Failed to upload CV");
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const renderUserProfileForm = () => (
    <form onSubmit={handleUserProfileSubmit} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-700">Name</label>
        <input
          type="text"
          value={userProfile.name}
          onChange={(e) =>
            setUserProfile({ ...userProfile, name: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">Email</label>
        <input
          type="email"
          value={userProfile.email}
          onChange={(e) =>
            setUserProfile({ ...userProfile, email: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">
          Phone Number
        </label>
        <input
          type="tel"
          value={userProfile.phone_number}
          onChange={(e) =>
            setUserProfile({ ...userProfile, phone_number: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">City</label>
        <input
          type="text"
          value={userProfile.city}
          onChange={(e) =>
            setUserProfile({ ...userProfile, city: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">
          Country
        </label>
        <input
          type="text"
          value={userProfile.country}
          onChange={(e) =>
            setUserProfile({ ...userProfile, country: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <button
        type="submit"
        disabled={loading}
        className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:opacity-50"
      >
        {loading ? "Saving..." : "Next"}
      </button>
    </form>
  );

  const renderEducationForm = () => (
    <form onSubmit={handleEducationSubmit} className="space-y-6">
      <div className="space-y-4">
        <h3 className="text-lg font-medium">High School Education</h3>
        <div>
          <label className="block text-sm font-medium text-gray-700">
            School Name
          </label>
          <input
            type="text"
            value={education.high_school.school_name}
            onChange={(e) =>
              setEducation({
                ...education,
                high_school: {
                  ...education.high_school,
                  school_name: e.target.value,
                },
              })
            }
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            required
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Track
          </label>
          <input
            type="text"
            value={education.high_school.track}
            onChange={(e) =>
              setEducation({
                ...education,
                high_school: {
                  ...education.high_school,
                  track: e.target.value,
                },
              })
            }
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            required
          />
        </div>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700">
              Start Year
            </label>
            <input
              type="number"
              value={education.high_school.start_year}
              onChange={(e) =>
                setEducation({
                  ...education,
                  high_school: {
                    ...education.high_school,
                    start_year: parseInt(e.target.value),
                  },
                })
              }
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700">
              End Year
            </label>
            <input
              type="number"
              value={education.high_school.end_year}
              onChange={(e) =>
                setEducation({
                  ...education,
                  high_school: {
                    ...education.high_school,
                    end_year: parseInt(e.target.value),
                  },
                })
              }
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
            />
          </div>
        </div>
      </div>

      <div className="space-y-4">
        <h3 className="text-lg font-medium">University Education</h3>
        <div>
          <label className="block text-sm font-medium text-gray-700">
            University Name
          </label>
          <input
            type="text"
            value={education.university[0].university_name}
            onChange={(e) =>
              setEducation({
                ...education,
                university: [
                  {
                    ...education.university[0],
                    university_name: e.target.value,
                  },
                ],
              })
            }
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Level
          </label>
          <select
            value={education.university[0].level}
            onChange={(e) =>
              setEducation({
                ...education,
                university: [
                  { ...education.university[0], level: e.target.value },
                ],
              })
            }
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          >
            <option value="Bachelor">Bachelor</option>
            <option value="Master">Master</option>
            <option value="PhD">PhD</option>
            <option value="Other">Other</option>
          </select>
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700">
            Major
          </label>
          <input
            type="text"
            value={education.university[0].major}
            onChange={(e) =>
              setEducation({
                ...education,
                university: [
                  { ...education.university[0], major: e.target.value },
                ],
              })
            }
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          />
        </div>
        <div>
          <label className="block text-sm font-medium text-gray-700">GPA</label>
          <input
            type="number"
            step="0.01"
            min="0"
            max="4"
            value={education.university[0].gpa || ""}
            onChange={(e) =>
              setEducation({
                ...education,
                university: [
                  {
                    ...education.university[0],
                    gpa: parseFloat(e.target.value),
                  },
                ],
              })
            }
            className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          />
        </div>
      </div>

      <div className="flex space-x-4">
        <button
          type="button"
          onClick={() => setStep(0)}
          className="w-full bg-gray-200 text-gray-700 py-2 px-4 rounded-md hover:bg-gray-300"
        >
          Back
        </button>
        <button
          type="submit"
          disabled={loading}
          className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {loading ? "Saving..." : "Next"}
        </button>
      </div>
    </form>
  );

  const renderBankInfoForm = () => (
    <form onSubmit={handleBankInfoSubmit} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-700">
          Account Holder Name
        </label>
        <input
          type="text"
          value={bankInfo.account_holder_name}
          onChange={(e) =>
            setBankInfo({ ...bankInfo, account_holder_name: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">
          Bank Name
        </label>
        <input
          type="text"
          value={bankInfo.bank_name}
          onChange={(e) =>
            setBankInfo({ ...bankInfo, bank_name: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">
          SWIFT Code
        </label>
        <input
          type="text"
          value={bankInfo.swift_code}
          onChange={(e) =>
            setBankInfo({ ...bankInfo, swift_code: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">
          Account Number
        </label>
        <input
          type="text"
          value={bankInfo.account_number}
          onChange={(e) =>
            setBankInfo({ ...bankInfo, account_number: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">
          Bank Country
        </label>
        <input
          type="text"
          value={bankInfo.bank_country}
          onChange={(e) =>
            setBankInfo({ ...bankInfo, bank_country: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div>
        <label className="block text-sm font-medium text-gray-700">
          Bank Branch
        </label>
        <input
          type="text"
          value={bankInfo.bank_branch}
          onChange={(e) =>
            setBankInfo({ ...bankInfo, bank_branch: e.target.value })
          }
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
        />
      </div>
      <div className="flex space-x-4">
        <button
          type="button"
          onClick={() => setStep(1)}
          className="w-full bg-gray-200 text-gray-700 py-2 px-4 rounded-md hover:bg-gray-300"
        >
          Back
        </button>
        <button
          type="submit"
          disabled={loading}
          className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {loading ? "Saving..." : "Next"}
        </button>
      </div>
    </form>
  );

  const renderCVForm = () => (
    <form onSubmit={handleCVSubmit} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-700">
          CV Title
        </label>
        <input
          type="text"
          value={cv.title}
          onChange={(e) => setCV({ ...cv, title: e.target.value })}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
          placeholder="e.g., Software Engineer CV 2024"
        />
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700">
          CV Content
        </label>
        <textarea
          value={cv.content}
          onChange={(e) => setCV({ ...cv, content: e.target.value })}
          rows={10}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
          required
          placeholder="Paste your CV content here..."
        />
      </div>

      <div className="flex space-x-4">
        <button
          type="button"
          onClick={() => setStep(2)}
          className="w-full bg-gray-200 text-gray-700 py-2 px-4 rounded-md hover:bg-gray-300"
        >
          Back
        </button>
        <button
          type="submit"
          disabled={loading}
          className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {loading ? "Saving..." : "Complete Registration"}
        </button>
      </div>
    </form>
  );

  return (
    <div className="min-h-screen bg-gradient-to-r from-purple-normal to-blue-dark py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md mx-auto bg-white p-8 rounded-lg shadow">
        <StepIndicator currentStep={step} totalSteps={4} />

        <h2 className="text-center text-2xl font-bold mb-8">
          {step === 0
            ? "Personal Information"
            : step === 1
            ? "Education Details"
            : step === 2
            ? "Bank Information"
            : "Upload CV"}
        </h2>

        {error && (
          <div className="mb-4 p-3 bg-red-100 text-red-700 rounded-md">
            {error}
          </div>
        )}

        {step === 0 && renderUserProfileForm()}
        {step === 1 && renderEducationForm()}
        {step === 2 && renderBankInfoForm()}
        {step === 3 && renderCVForm()}
      </div>
    </div>
  );
};

export default Register;
