import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { AuthManager } from "../auth/AuthManager";
import { Loader2, Upload, FileText } from "lucide-react";

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
        console.log("Initializing auth...");
        const manager = await AuthManager.create();

        const isAuth = await manager.isAuthenticated();
        console.log("Is authenticated:", isAuth);

        if (!isAuth) {
          console.log("Not authenticated, redirecting to login...");
          navigate("/");
          return;
        }

        console.log("Initializing backend actor...");
        const actor = await manager.initBackendActor();
        manager.backendActor = actor;

        setAuthManager(manager);
        console.log("Auth initialization complete");
      } catch (error) {
        console.error("Auth initialization failed:", error);
        setError("Authentication initialization failed: " + error.message);
      }
    };

    initAuth();
  }, [navigate]);

  const [userProfile, setUserProfile] = useState({
    name: "",
    email: "",
    phone_number: "",
    city: "",
    country: "",
  });

  const [cv, setCV] = useState({
    title: "",
    content: "",
    file: null,
    analysis: null,
    uploadType: "text",
  });

  const [analyzing, setAnalyzing] = useState(false);

  const handleUserProfileSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError("");

    try {
      if (!authManager) {
        throw new Error("Authentication not initialized");
      }

      if (!authManager.backendActor) {
        console.log("Initializing backend actor...");
        const actor = await authManager.initBackendActor();
        authManager.backendActor = actor;
      }

      console.log("Creating login session...");
      await authManager.backendActor.login();

      console.log("Checking if user exists...");
      const userExists = await authManager.backendActor.get_user();
      console.log("User exists response:", userExists);

      const userPayload = {
        name: userProfile.name,
        email: userProfile.email,
        phone_number: userProfile.phone_number,
        city: userProfile.city,
        country: userProfile.country,
      };

      console.log("Sending create/update request with payload:", userPayload);
      const response = await authManager.backendActor.update_user({
        name: [userPayload.name],
        email: [userPayload.email],
        phone_number: [userPayload.phone_number],
        city: [userPayload.city],
        country: [userPayload.country],
      });

      console.log("Create/Update response:", response);

      if ("Success" in response) {
        setStep(1);
      } else {
        setError(response.Error || "Failed to create/update user");
      }
    } catch (error) {
      console.error("Profile update error:", error);
      setError(error.message || "Failed to update profile");
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

  const analyzeCVContent = (content) => {
    const words = content.split(/\s+/).filter((word) => word.length > 0);
    const sentences = content
      .split(/[.!?]+/)
      .filter((s) => s.trim().length > 0);

    const keywords = new Set();
    const commonKeywords = [
      "experience",
      "education",
      "skills",
      "projects",
      "work",
    ];
    words.forEach((word) => {
      if (commonKeywords.includes(word.toLowerCase())) {
        keywords.add(word.toLowerCase());
      }
    });

    const skillPatterns = [
      /javascript|python|java|react|node\.js|sql|html|css/gi,
      /management|leadership|communication|teamwork/gi,
      /analysis|design|development|testing/gi,
    ];

    const skills = new Set();
    skillPatterns.forEach((pattern) => {
      const matches = content.match(pattern);
      if (matches) {
        matches.forEach((match) => skills.add(match.toLowerCase()));
      }
    });

    return {
      wordCount: words.length,
      sentenceCount: sentences.length,
      characterCount: content.length,
      keywords: Array.from(keywords),
      detectedSkills: Array.from(skills),
      averageWordLength:
        words.reduce((sum, word) => sum + word.length, 0) / words.length,
    };
  };

  const handleFileUpload = async (e) => {
    const file = e.target.files[0];
    if (file) {
      setAnalyzing(true);
      try {
        const reader = new FileReader();

        reader.onload = (event) => {
          const text = event.target.result;
          const analysis = analyzeCVContent(text);

          setCV({
            ...cv,
            title: file.name.replace(/\.[^/.]+$/, ""),
            content: text,
            file: file,
            analysis: analysis,
            uploadType: "file",
          });
          setAnalyzing(false);
        };

        reader.onerror = (error) => {
          setError("Failed to read file: " + error.message);
          setAnalyzing(false);
        };

        reader.readAsText(file);
      } catch (error) {
        setError("Failed to process file: " + error.message);
        setAnalyzing(false);
      }
    }
  };

  const handleContentChange = (e) => {
    const content = e.target.value;
    const analysis = analyzeCVContent(content);

    setCV({
      ...cv,
      content: content,
      analysis: analysis,
      uploadType: "text",
    });
  };

  const handleCVSubmit = async (e) => {
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
        authManager.backendActor = await authManager.initBackendActor();
      }

      const cvPayload = {
        title: cv.title,
        content: cv.content,
        analysis: cv.analysis,
      };

      const response = await authManager.backendActor.upload_cv(cvPayload);
      if (response.cv) {
        navigate("/");
      } else {
        setError(response.message || "Failed to upload CV");
      }
    } catch (error) {
      setError("Failed to upload CV: " + error.message);
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const renderCVForm = () => (
    <form onSubmit={handleCVSubmit} className="space-y-6">
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

      <div className="space-y-4">
        <div className="flex space-x-4">
          <button
            type="button"
            onClick={() => setCV({ ...cv, uploadType: "text" })}
            className={`flex-1 py-2 px-4 rounded-md flex items-center justify-center space-x-2 ${
              cv.uploadType === "text"
                ? "bg-blue-600 text-white"
                : "bg-gray-200 text-gray-700"
            }`}
          >
            <FileText className="w-4 h-4" />
            <span>Enter Text</span>
          </button>
          <button
            type="button"
            onClick={() => setCV({ ...cv, uploadType: "file" })}
            className={`flex-1 py-2 px-4 rounded-md flex items-center justify-center space-x-2 ${
              cv.uploadType === "file"
                ? "bg-blue-600 text-white"
                : "bg-gray-200 text-gray-700"
            }`}
          >
            <Upload className="w-4 h-4" />
            <span>Upload File</span>
          </button>
        </div>

        {cv.uploadType === "text" ? (
          <div>
            <label className="block text-sm font-medium text-gray-700">
              CV Content
            </label>
            <textarea
              value={cv.content}
              onChange={handleContentChange}
              rows={10}
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500"
              required
              placeholder="Paste your CV content here..."
            />
          </div>
        ) : (
          <div>
            <label className="block text-sm font-medium text-gray-700">
              Upload CV File
            </label>
            <input
              type="file"
              accept=".txt,.pdf,.doc,.docx"
              onChange={handleFileUpload}
              className="mt-1 block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-md file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
            />
          </div>
        )}
      </div>

      {analyzing && (
        <div className="flex items-center justify-center space-x-2 text-blue-600">
          <Loader2 className="w-5 h-5 animate-spin" />
          <span>Analyzing CV content...</span>
        </div>
      )}

      {cv.analysis && (
        <div className="bg-gray-50 rounded-lg p-4 space-y-3">
          <h3 className="font-medium text-gray-900">CV Analysis</h3>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <p className="text-gray-500">Word Count</p>
              <p className="font-medium">{cv.analysis.wordCount}</p>
            </div>
            <div>
              <p className="text-gray-500">Sentences</p>
              <p className="font-medium">{cv.analysis.sentenceCount}</p>
            </div>
            <div>
              <p className="text-gray-500">Avg Word Length</p>
              <p className="font-medium">
                {cv.analysis.averageWordLength.toFixed(1)} chars
              </p>
            </div>
            <div>
              <p className="text-gray-500">Total Characters</p>
              <p className="font-medium">{cv.analysis.characterCount}</p>
            </div>
          </div>
          {cv.analysis.detectedSkills.length > 0 && (
            <div>
              <p className="text-gray-500 mb-2">Detected Skills</p>
              <div className="flex flex-wrap gap-2">
                {cv.analysis.detectedSkills.map((skill, index) => (
                  <span
                    key={index}
                    className="bg-blue-100 text-blue-700 px-2 py-1 rounded-full text-xs"
                  >
                    {skill}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
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
          disabled={loading || !cv.content}
          className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:opacity-50 flex items-center justify-center space-x-2"
        >
          {loading ? (
            <>
              <Loader2 className="w-4 h-4 animate-spin" />
              <span>Saving...</span>
            </>
          ) : (
            "Complete Registration"
          )}
        </button>
      </div>
    </form>
  );

  return (
    <div className="min-h-screen bg-gradient-to-r from-purple-500 to-blue-600 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md mx-auto bg-white p-8 rounded-lg shadow">
        {!authManager ? (
          <div className="text-center">
            <Loader2 className="w-8 h-8 animate-spin mx-auto mb-4 text-blue-600" />
            <p className="text-gray-600">Initializing authentication...</p>
          </div>
        ) : (
          <>
            <StepIndicator currentStep={step} totalSteps={2} />

            <h2 className="text-center text-2xl font-bold mb-8">
              {step === 0 ? "Personal Information" : "Upload CV"}
            </h2>

            {error && (
              <div className="mb-4 p-3 bg-red-100 text-red-700 rounded-md">
                {error}
              </div>
            )}

            {step === 0 && renderUserProfileForm()}
            {step === 1 && renderCVForm()}
          </>
        )}
      </div>
    </div>
  );
};

export default Register;
