import "./App.css";

import { useState } from "react";
import { div } from "calculator_web";

const DEFAULT_VALUE = 10;

function App() {
  const [count, setCount] = useState(DEFAULT_VALUE);
  const [error, setError] = useState("");

  return (
    <>
      <h1>{count}</h1>
      <div className="card">
        <button
          onClick={() => {
            setCount(div(DEFAULT_VALUE, 2));
            setError("");
          }}
        >
          Div by 2
        </button>
        <button
          onClick={() => {
            try {
              setCount(div(DEFAULT_VALUE, 0));
            } catch (err) {
              setError((err as Error).message);
            }
          }}
        >
          Div by 0
        </button>
        <button
          onClick={() => {
            setCount(DEFAULT_VALUE);
            setError("");
          }}
        >
          Reset
        </button>
        {error && <p className="error">Error: {error}</p>}
      </div>
    </>
  );
}

export default App;
