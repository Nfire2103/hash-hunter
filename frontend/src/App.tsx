import { useState } from 'react';
import './App.css';
import { Button } from './components/ui/button';

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <div className="flex flex-col items-center justify-center min-h-screen bg-background text-foreground space-y-4">
        <h1 className="text-3xl font-bold underline">Hello Vite + React!</h1>
        <Button
          className="font-bold py-2 px-4 rounded"
          onClick={() => setCount((count) => count + 1)}
        >
          count is {count}
        </Button>
        <p className="text-lg">
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
        <p className="text-lg">
          <a href="https://vitejs.dev/guide/features.html" target="_blank">
            Vite Docs
          </a>
        </p>
        <p className="text-lg">
          <a href="https://reactjs.org" target="_blank">
            React Docs
          </a>
        </p>
        <p className="text-lg">
          <a href="https://tailwindcss.com/docs" target="_blank">
            Tailwind CSS Docs
          </a>
        </p>
        <p className="text-lg">
          <a
            href="https://radix-ui.com/docs/primitives/overview"
            target="_blank"
          >
            Radix UI Docs
          </a>
        </p>
      </div>
      <div className="flex flex-col items-center justify-center min-h-svh">
        <Button>Click me</Button>
      </div>
    </>
  );
}

export default App;
