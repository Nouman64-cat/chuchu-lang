"use client";

import { useEffect, useRef, useState } from "react";

type Stage = {
  name: string;
  description: string;
  why: string;
  input: string;
  output: string;
};

const STAGES: readonly Stage[] = [
  {
    name: "Lexer",
    description:
      "Reads your text one character at a time and groups it into tokens: keywords, names, numbers and symbols.",
    why: "Anything the compiler does not recognise, like a stray @, is reported here with its exact position.",
    input: "assign total 2 + 3",
    output:
      "Set          assign\nIdentifier   total\nInteger      2\nPlus         +\nInteger      3",
  },
  {
    name: "Parser",
    description:
      "Checks the grammar and arranges the tokens into a tree that shows how the pieces fit together.",
    why: "Operator precedence is settled here, so 2 + 3 * 4 multiplies before it adds.",
    input: "Set  Identifier(total)\nInteger(2)  Plus  Integer(3)",
    output: "Set total\n└─ Add\n   ├─ 2\n   └─ 3",
  },
  {
    name: "Type checker",
    description:
      "Walks the tree and checks that every value fits: no adding a number to true or false, and no using a name before it exists.",
    why: "Mistakes are caught before your program is ever built, with the location of the problem.",
    input: "Set total\n└─ Add\n   ├─ 2\n   └─ 3",
    output:
      "2        is int\n3        is int\n2 + 3    is int\ntotal    is int   (ok)",
  },
  {
    name: "LLVM codegen",
    description:
      "Translates the checked tree into LLVM instructions, adding safety checks such as overflow and divide-by-zero.",
    why: "LLVM already knows how to produce fast machine code for many processors, so Chuchu does not have to.",
    input: "total : int = 2 + 3",
    output:
      "; simplified illustration\n%total = alloca i64\n%sum = add i64 2, 3   ; with overflow check\nstore i64 %sum, ptr %total",
  },
  {
    name: "Optimizer",
    description:
      "LLVM tidies the instructions and removes work that can be done ahead of time. Choose -O0 to -O3 to control how hard it tries.",
    why: "Optimization changes how fast the program runs, never what it does.",
    input: "%sum = add i64 2, 3\nstore i64 %sum, ptr %total",
    output: "; at -O2, 2 + 3 is folded away\nstore i64 5, ptr %total",
  },
  {
    name: "Linker",
    description:
      "Writes a native object file, then asks the system clang to link it into a finished executable.",
    why: "The result is a normal program you run directly, with no interpreter needed.",
    input: "program.o",
    output: "$ clang program.o -o main\n$ ./main",
  },
];

const AUTOPLAY_MS = 5200;

export default function HowItWorks() {
  const [selected, setSelected] = useState(0);
  const stopped = useRef(false);

  useEffect(() => {
    const timer = setInterval(() => {
      if (stopped.current) return;
      setSelected((current) => (current + 1) % STAGES.length);
    }, AUTOPLAY_MS);
    return () => clearInterval(timer);
  }, []);

  const pick = (index: number) => {
    stopped.current = true;
    setSelected(index);
  };

  return (
    <>
      <div className="track" aria-hidden="true">
        <div
          className="fill"
          style={{ transform: `scaleX(${(selected + 1) / STAGES.length})` }}
        />
      </div>
      <div className="steps">
        {STAGES.map((s, i) => (
          <button
            key={s.name}
            type="button"
            className={i === selected ? "step step-on" : "step"}
            onClick={() => pick(i)}
            aria-pressed={i === selected}
          >
            <span className="step-n">0{i + 1}</span>
            <span className="step-name">{s.name}</span>
          </button>
        ))}
      </div>

      {/* Every stage shares one grid cell, so the area is always as tall as
          the tallest stage and nothing below it moves on autoplay. Only the
          selected one is visible; its key changes on selection so it
          remounts and replays its entrance. */}
      <div className="panels">
        {STAGES.map((s, i) => {
          const on = i === selected;
          return (
            <div
              key={on ? `${i}-on` : i}
              className={on ? "panel" : "panel panel-off"}
              aria-hidden={on ? undefined : true}
              inert={!on}
            >
              <div className="glass panel-info">
                <p className="panel-step">
                  STEP {i + 1} OF {STAGES.length}
                </p>
                <h3 className="panel-title">{s.name}</h3>
                <p className="panel-desc">{s.description}</p>
                <div className="panel-why">
                  <p className="panel-why-label">WHY IT MATTERS</p>
                  <p className="panel-why-text">{s.why}</p>
                </div>
              </div>
              <div className="panel-code">
                <div className="panel-io">
                  <p className="panel-io-label">INPUT</p>
                  <pre>{s.input}</pre>
                </div>
                <div className="panel-io">
                  <p className="panel-io-label">OUTPUT</p>
                  <pre className="reveal">{s.output}</pre>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </>
  );
}
