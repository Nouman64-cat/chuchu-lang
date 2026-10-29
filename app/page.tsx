import type { CSSProperties } from "react";
import Image from "next/image";
import HowItWorks from "@/components/how-it-works";
import OptimizationLevels from "@/components/optimization-levels";

const docsUrl = "https://docs.chuchu-lang.org";
const installUrl = `${docsUrl}/docs/getting-started/installation/`;
const sourceUrl = "https://github.com/chuchu-lang/chuchu-core";

// Logo intrinsic size from public/chuchu-logo-3.svg (976 x 586).
const LOGO_RATIO = 586 / 976;

// Lets style objects carry CSS custom properties like --d and --r.
type Vars = CSSProperties & Record<`--${string}`, string>;

const HERO_ORBS: Vars[] = [
  { left: "58%", top: 60, width: 560, height: 560, background: "#10B23F", opacity: 0.34, animation: "drift 22s ease-in-out infinite" },
  { left: "-6%", top: 340, width: 440, height: 440, background: "#FAD705", opacity: 0.4, animation: "drift 26s ease-in-out infinite reverse" },
  { left: "42%", top: 540, width: 460, height: 460, background: "#FD8AB9", opacity: 0.34, animation: "drift 30s ease-in-out infinite" },
];

const PIXELS: Vars[] = [
  { left: "7%", top: 210, width: 12, height: 12, background: "#10B23F", "--t": "7s", "--d": "0s" },
  { left: "15%", top: 560, width: 8, height: 8, background: "#FD8AB9", "--t": "9s", "--d": "-2s" },
  { left: "44%", top: 150, width: 10, height: 10, border: "2px solid #14201A", opacity: 0.55, "--t": "8s", "--d": "-1s" },
  { left: "53%", top: 690, width: 12, height: 12, background: "#FAD705", "--t": "10s", "--d": "-4s" },
  { left: "91%", top: 250, width: 9, height: 9, background: "#FD8AB9", "--t": "7.5s", "--d": "-3s" },
  { left: "96%", top: 560, width: 12, height: 12, border: "2px solid #10B23F", "--t": "9s", "--d": "-5s" },
  { left: "3%", top: 760, width: 10, height: 10, background: "#FAD705", "--t": "11s", "--d": "-2s" },
];

const MARQUEE: { label: string; color: string }[] = [
  { label: "int", color: "#10B23F" },
  { label: "float", color: "#FAD705" },
  { label: "bool", color: "#FD8AB9" },
  { label: "arrays", color: "#10B23F" },
  { label: "functions", color: "#FAD705" },
  { label: "if / else", color: "#FD8AB9" },
  { label: "while loops", color: "#10B23F" },
  { label: "checked arithmetic", color: "#FAD705" },
  { label: "-O0 to -O3", color: "#FD8AB9" },
  { label: "LLVM backend", color: "#10B23F" },
  { label: "native executables", color: "#FAD705" },
];

const INFERRED: { name: string; value: string; type: string; chip: string }[] = [
  { name: "count", value: "5", type: "int", chip: "chip-int" },
  { name: "ratio", value: "3.14", type: "float", chip: "chip-float" },
  { name: "ready", value: "true", type: "bool", chip: "chip-bool" },
  { name: "scores", value: "[1, 2, 3]", type: "int array · 3", chip: "chip-int" },
];

export default function Home() {
  return (
    <div id="top" className="v2">
      {/* AMBIENT LAYER */}
      <div className="ambient" aria-hidden="true">
        <div className="gridpaper" />
        {HERO_ORBS.map((style, i) => (
          <div key={i} className="orb" style={style} />
        ))}
        {PIXELS.map((style, i) => (
          <i key={i} className="px" style={style} />
        ))}
        <svg className="grain" width="100%" height="100%" aria-hidden="true">
          <filter id="grain">
            <feTurbulence
              type="fractalNoise"
              baseFrequency="0.85"
              numOctaves={2}
              stitchTiles="stitch"
            />
            <feColorMatrix type="saturate" values="0" />
          </filter>
          <rect width="100%" height="100%" filter="url(#grain)" />
        </svg>
      </div>

      {/* NAV */}
      <div className="navwrap">
        <nav className="glass navpill" aria-label="Main navigation">
          <a className="brand" href="#top" aria-label="Chuchu home">
            <Image
              className="brand-logo"
              src="/chuchu-logo-3.svg"
              alt=""
              width={50}
              height={Math.round(50 * LOGO_RATIO)}
              loading="eager"
            />
            <span className="brand-name">chuchu</span>
          </a>
          <div className="navlinks">
            <a className="navlink" href="#how-it-works">
              How it works
            </a>
            <a className="navlink" href="#features">
              Features
            </a>
            <a className="navlink" href={docsUrl}>
              Documentation
            </a>
            <a className="navlink" href={sourceUrl}>
              GitHub
            </a>
          </div>
          <a className="btn btn-primary btn-nav" href={installUrl}>
            Get started{" "}
            <span className="arr" aria-hidden="true">
              &rarr;
            </span>
          </a>
        </nav>
      </div>

      {/* HERO */}
      <section className="wrap hero" aria-labelledby="hero-title">
        <div className="hero-copy">
          <p className="glass rise hero-badge" style={{ "--d": ".05s" } as Vars}>
            <span className="ping" aria-hidden="true" />
            Early development &middot; Open source &middot; Built in Rust
          </p>
          <h1
            id="hero-title"
            className="rise hero-title"
            style={{ "--d": ".14s" } as Vars}
          >
            <span>Simple syntax.</span>
            <span className="accent">
              <span className="uline">
                Native
                <svg
                  className="sq"
                  viewBox="0 0 240 14"
                  preserveAspectRatio="none"
                  aria-hidden="true"
                >
                  <path
                    d="M3 9 C 28 1, 52 15, 78 7 S 130 3, 156 9 S 210 13, 237 5"
                    pathLength={1}
                  />
                </svg>
              </span>{" "}
              possibilities.
            </span>
          </h1>
          <p className="rise hero-text" style={{ "--d": ".28s" } as Vars}>
            Chuchu is a small, statically typed language with inferred types.
            Its compiler, Cheeku, turns your code into fast native programs
            through LLVM.
          </p>
          <div className="rise hero-actions" style={{ "--d": ".4s" } as Vars}>
            <a className="btn btn-primary" href={docsUrl}>
              Read the docs{" "}
              <span className="arr" aria-hidden="true">
                &rarr;
              </span>
            </a>
            <a className="btn btn-glass" href={sourceUrl}>
              View on GitHub
            </a>
          </div>
          <p className="rise hero-note" style={{ "--d": ".5s" } as Vars}>
            Currently validated on macOS with Apple Silicon.
          </p>
        </div>

        <div className="rise hero-visual" style={{ "--d": ".3s" } as Vars}>
          <Image
            className="float hero-logo"
            style={{ "--r": "-4deg" } as Vars}
            src="/chuchu-logo-3.svg"
            alt=""
            width={196}
            height={Math.round(196 * LOGO_RATIO)}
            loading="eager"
          />
          <div
            className="glass float hero-chip hero-chip-types"
            style={{ "--d": "-2s", "--r": "-3deg" } as Vars}
          >
            <svg
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="#0F7A3B"
              strokeWidth="2.6"
              strokeLinecap="round"
              strokeLinejoin="round"
              aria-hidden="true"
            >
              <path d="M20 6L9 17l-5-5" />
            </svg>
            Types checked
          </div>
          <div
            className="glass float hero-chip hero-chip-opt"
            style={{ "--d": "-4s", "--r": "3deg" } as Vars}
          >
            <i aria-hidden="true" />
            -O2 &middot; native
          </div>

          <div className="glass glass-hi editor">
            <div className="editor-bar">
              <div className="editor-dots" aria-hidden="true">
                <span style={{ background: "#FD8AB9" }} />
                <span style={{ background: "#FAD705" }} />
                <span style={{ background: "#10B23F" }} />
              </div>
              <span className="editor-file">hello.chuchu</span>
              <span className="editor-lang">CHUCHU</span>
            </div>
            <div className="cycle">
              <div className="editor-code">
                <div className="editor-line">
                  <span className="editor-ln">1</span>
                  <span className="tp t1">
                    <span className="kw">assign</span> x{" "}
                    <span className="kw">to</span> <span className="num">5</span>
                  </span>
                </div>
                <div className="editor-line">
                  <span className="editor-ln">2</span>
                  <span className="tp t2">
                    <span className="kw">reassign</span> x{" "}
                    <span className="kw">to</span> <span className="num">6</span>
                  </span>
                </div>
                <div className="editor-line">
                  <span className="editor-ln">3</span>
                  <span className="tp t3">
                    <span className="kw">show</span> x
                  </span>
                </div>
              </div>
              <div className="terminal">
                <div className="terminal-label">TERMINAL</div>
                <div className="terminal-line">
                  <span className="terminal-prompt">$ </span>
                  <span className="tp t4">cheeku hello.chuchu</span>
                </div>
                <div className="terminal-line">
                  <span className="terminal-prompt">$ </span>
                  <span className="tp t5">./main</span>
                </div>
                <div className="out terminal-out">
                  6<span className="caret" aria-hidden="true" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* MARQUEE */}
      <div className="marquee marquee-row" aria-label="What Chuchu supports">
        <div className="mtrack">
          {[false, true].map((duplicate) => (
            <div
              key={String(duplicate)}
              className="mset"
              aria-hidden={duplicate || undefined}
            >
              {MARQUEE.map((item) => (
                <span key={item.label} className="glass mpill">
                  <i style={{ background: item.color }} />
                  {item.label}
                </span>
              ))}
            </div>
          ))}
        </div>
      </div>

      {/* HOW IT WORKS */}
      <section id="how-it-works" className="how" aria-labelledby="how-title">
        <div
          aria-hidden="true"
          className="orb"
          style={{ left: "-8%", top: 120, width: 420, height: 420, background: "#10B23F", opacity: 0.22 }}
        />
        <div
          aria-hidden="true"
          className="orb"
          style={{ right: "-6%", top: 420, width: 460, height: 460, background: "#FD8AB9", opacity: 0.26 }}
        />
        <div className="wrap">
          <p className="eyebrow">
            <b />
            01 &middot; HOW IT WORKS
          </p>
          <h2 id="how-title" className="h2 how-title">
            From your code to a native program, in six steps
          </h2>
          <p className="lede">
            Cheeku checks everything before it generates a single instruction.
            It plays through the stages on its own, or pick one to explore.
          </p>

          <HowItWorks />

          <p className="how-note">
            A simplified illustration of how Cheeku handles{" "}
            <span>assign total to 2 + 3</span>. The real compiler does more at
            every stage.
          </p>
        </div>
      </section>

      {/* FEATURES */}
      <section id="features" className="features" aria-labelledby="feat-title">
        <div
          aria-hidden="true"
          className="orb"
          style={{ right: "8%", top: 40, width: 420, height: 420, background: "#FAD705", opacity: 0.3 }}
        />
        <div
          aria-hidden="true"
          className="orb"
          style={{ left: "4%", top: 520, width: 480, height: 480, background: "#10B23F", opacity: 0.2 }}
        />
        <div className="wrap">
          <p className="eyebrow">
            <b />
            02 &middot; WHY CHUCHU
          </p>
          <h2 id="feat-title" className="h2 features-title">
            Small language. Serious foundations.
          </h2>

          <div className="bento">
            <article className="glass card b7 feature">
              <p className="feature-label">AT COMPILE TIME</p>
              <h3 className="feature-title">Caught before it runs</h3>
              <p className="feature-text feature-text-wide">
                Static type checking finds the mistake and points to exactly
                where it is, so a wrong program never becomes an executable.
              </p>
              <div className="err">
                <div className="err-code">
                  <div>
                    <span className="err-ln">{"1  "}</span>
                    <span className="err-kw">assign</span> x{" "}
                    <span className="err-kw">to</span> 5
                  </div>
                  <div>
                    <span className="err-ln">{"2  "}</span>
                    <span className="err-kw">reassign</span> x{" "}
                    <span className="err-kw">to</span>{" "}
                    <span className="err-wavy">true</span>
                  </div>
                </div>
                <div className="err-msg">
                  Type mismatch: expected Int, found Bool at 28..32
                </div>
              </div>
            </article>

            <article className="glass card b5 feature">
              <p className="feature-label">NATIVE SPEED</p>
              <h3 className="feature-title">You choose how hard it optimizes</h3>
              <p className="feature-text">
                Compile through LLVM at four levels. Optimization changes speed,
                never behavior.
              </p>
              <OptimizationLevels />
            </article>

            <article className="glass card b5 feature">
              <p className="feature-label">AT RUN TIME</p>
              <h3 className="feature-title">Stopped when it matters</h3>
              <p className="feature-text">
                Overflow, division by zero and out-of-range array indexes halt
                with a clear message instead of failing silently.
              </p>
              <div className="err">
                <div className="err-code">
                  <div>
                    <span className="err-ln">{"1  "}</span>
                    <span className="err-kw">assign</span> a{" "}
                    <span className="err-kw">to</span> [1, 2, 3]
                  </div>
                  <div>
                    <span className="err-ln">{"2  "}</span>
                    <span className="err-kw">show</span> a[5]
                  </div>
                </div>
                <div className="err-msg">
                  cheeku runtime: array index out of bounds at 27..31
                </div>
              </div>
            </article>

            <article className="glass card b7 feature">
              <p className="feature-label">TYPE INFERENCE</p>
              <h3 className="feature-title">Types without the typing</h3>
              <p className="feature-text feature-text-wide">
                Write the value and Chuchu works out the type. You only annotate
                function parameters and return types.
              </p>
              <div className="trows">
                {INFERRED.map((row) => (
                  <div key={row.name} className="trow">
                    <span className="trow-code">
                      <span className="kw">assign</span> {row.name}{" "}
                      <span className="kw">to</span> {row.value}
                    </span>
                    <span className={`chip ${row.chip}`}>{row.type}</span>
                  </div>
                ))}
              </div>
            </article>
          </div>
        </div>
      </section>

      {/* CTA */}
      <section className="wrap cta" aria-labelledby="cta-title">
        <div className="ring">
          <div className="ring-in cta-inner">
            <div
              aria-hidden="true"
              className="orb"
              style={{ right: -60, top: -80, width: 360, height: 360, background: "#10B23F", opacity: 0.4 }}
            />
            <div
              aria-hidden="true"
              className="orb"
              style={{ left: -60, bottom: -120, width: 340, height: 340, background: "#FD8AB9", opacity: 0.26 }}
            />
            <div
              aria-hidden="true"
              className="orb"
              style={{ left: "46%", top: -100, width: 240, height: 240, background: "#FAD705", opacity: 0.2 }}
            />
            <div className="cta-body">
              <div className="cta-copy">
                <h2 id="cta-title" className="cta-title">
                  Write your first Chuchu program.
                </h2>
                <p className="cta-text">
                  Build the compiler, compile an example, and run it as a native
                  program.
                </p>
              </div>
              <div className="cta-actions">
                <a className="btn btn-light" href={installUrl}>
                  Get started{" "}
                  <span className="arr" aria-hidden="true">
                    &rarr;
                  </span>
                </a>
                <a className="btn btn-dark-glass" href={sourceUrl}>
                  View on GitHub
                </a>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* FOOTER */}
      <div className="wrap">
        <footer className="footer">
          <a className="footer-brand" href="#top" aria-label="Chuchu home">
            <Image
              className="footer-logo"
              src="/chuchu-logo-3.svg"
              alt=""
              width={40}
              height={Math.round(40 * LOGO_RATIO)}
              loading="eager"
            />
            chuchu
          </a>
          <span>Simple syntax. Native possibilities.</span>
          <nav className="footer-nav" aria-label="Footer navigation">
            <a className="navlink" href={docsUrl}>
              Documentation
            </a>
            <a className="navlink" href={sourceUrl}>
              GitHub
            </a>
          </nav>
        </footer>
      </div>
    </div>
  );
}
