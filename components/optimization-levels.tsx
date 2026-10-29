"use client";

import { useState } from "react";

const LEVELS = [
  {
    key: "-O0",
    text: "No optimization. The unoptimized baseline: quickest to compile and easiest to debug.",
  },
  {
    key: "-O1",
    text: "Light scalar optimizations that clean up the generated code.",
  },
  {
    key: "-O2",
    text: "The default. LLVM’s standard pipeline, with loop vectorization switched on.",
  },
  {
    key: "-O3",
    text: "LLVM’s more aggressive standard pipeline, for when raw speed matters most.",
  },
] as const;

export default function OptimizationLevels() {
  const [level, setLevel] = useState(2);

  return (
    <>
      <div className="seg" role="group" aria-label="Optimization level">
        <div
          className="seg-thumb"
          aria-hidden="true"
          style={{ transform: `translateX(${level * 100}%)` }}
        />
        {LEVELS.map((l, i) => (
          <button
            key={l.key}
            type="button"
            className={i === level ? "seg-btn seg-on" : "seg-btn"}
            onClick={() => setLevel(i)}
            aria-pressed={i === level}
          >
            {l.key}
          </button>
        ))}
      </div>
      {/* All texts share one grid cell so the card keeps the height of the
          longest one and does not jump when the level changes. */}
      <div className="opt-texts">
        {LEVELS.map((l, i) => (
          <p
            key={l.key}
            className={i === level ? "opt-text" : "opt-text opt-text-off"}
            aria-hidden={i === level ? undefined : true}
          >
            {l.text}
          </p>
        ))}
      </div>
    </>
  );
}
