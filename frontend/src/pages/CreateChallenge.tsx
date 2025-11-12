import { GradientButton } from '@/components/GradientButton';

export const CreateChallenge = () => {
  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-background-light dark:bg-background text-primary dark:text-white px-4 py-12">
      {/* Title */}
      <h1 className="text-4xl md:text-5xl font-bold text-center mb-10">
        Create Your <span className="text-accent">{`{Challenge}`}</span>.
      </h1>

      {/* Form Card */}
      <form
        onSubmit={(e) => e.preventDefault()}
        className="border border-border-light dark:border-border p-8 w-full max-w-md shadow-md space-y-5 bg-secondary-light dark:bg-secondary"
      >
        {/* Challenge Name */}
        <div>
          <label className="block text-sm font-medium mb-1">
            Challenge Name
          </label>
          <input
            type="text"
            placeholder="e.g., Blockchain Data Encryption"
            className="w-full bg-input-light dark:bg-background text-primary dark:text-gray-300 placeholder-gray-500 p-2 text-sm border border-border-light dark:border-border focus:ring-2 focus:ring-accent outline-none"
          />
        </div>

        {/* Description */}
        <div>
          <label className="block text-sm font-medium mb-1">
            Description
          </label>
          <textarea
            placeholder="Explain the problem, provide examples, and outline expected input/output..."
            className="w-full bg-background text-gray-300 placeholder-gray-500 p-2 text-sm h-24 border border-border focus:ring-2 focus:ring-accent outline-none resize-none"
          />
        </div>

        {/* Difficulty Level */}
        <div>
          <label className="block text-sm font-medium mb-1">
            Difficulty Level
          </label>
          <select className="w-full bg-background text-gray-300 p-2 text-sm border border-border focus:ring-2 focus:ring-accent outline-none">
            <option value="">Choose Difficulty</option>
            <option value="Easy">Easy</option>
            <option value="Medium">Medium</option>
            <option value="Hard">Hard</option>
          </select>
        </div>

        {/* Programming Languages */}
        <div>
          <label className="block text-sm font-medium mb-1">
            Programming Language(s)
          </label>
          <select className="w-full bg-background text-gray-300 p-2 text-sm border border-border focus:ring-2 focus:ring-accent outline-none">
            <option value="">Select...</option>
            <option value="Python">Python</option>
            <option value="JavaScript">JavaScript</option>
            <option value="C++">C++</option>
            <option value="Java">Java</option>
            <option value="Solidity">Solidity</option>
          </select>
        </div>

        {/* Starter Code */}
        <div>
          <label className="block text-sm font-medium mb-1">
            Starter Code (Optional)
          </label>
          <textarea
            placeholder="Provide boilerplate code if needed..."
            className="w-full bg-background text-gray-300 placeholder-gray-500 p-2 text-sm h-20 border border-border focus:ring-2 focus:ring-accent outline-none font-mono resize-none"
          />
        </div>

        {/* Test Cases */}
        <div>
          <label className="block text-sm font-medium mb-1">
            Test Cases & Expected Output
          </label>
          <textarea
            placeholder="Define correct input/output for automated validation..."
            className="w-full bg-background text-gray-300 placeholder-gray-500 p-2 text-sm h-20 border border-border focus:ring-2 focus:ring-accent outline-none resize-none"
          />
        </div>

        {/* Tags */}
        <div>
          <label className="block text-sm font-medium mb-1">
            Tags (Optional)
          </label>
          <input
            type="text"
            placeholder="e.g., Algorithms, Web3, Data Structures"
            className="w-full bg-input-light dark:bg-background text-primary dark:text-gray-300 placeholder-gray-500 p-2 text-sm border border-border-light dark:border-border focus:ring-2 focus:ring-accent outline-none"
          />
        </div>

        {/* Submit Button */}
        <div className="w-full [&>div]:w-full [&_button]:w-full">
          <GradientButton>
            Publish Challenge
          </GradientButton>
        </div>
      </form>
    </div>
  );
};
