import type { Metadata } from "next";
import type { CSSProperties } from "react";
import Link from "next/link";
import SiteFooter from "@/components/site-footer";
import SiteNav from "@/components/site-nav";
import { JsonLd } from "@/lib/json-ld";
import { site } from "@/lib/site";
import { aboutStructuredData } from "@/lib/structured-data";

// Lets style objects carry CSS custom properties like --d.
type Vars = CSSProperties & Record<`--${string}`, string>;

const TITLE = "About Chuchu";
const DESCRIPTION =
  "What Chuchu is, how the Cheeku compiler turns code into native programs, the principles behind the language, and where the project stands today.";

export const metadata: Metadata = {
  title: "About",
  description: DESCRIPTION,
  alternates: {
    canonical: "/about",
  },
  openGraph: {
    type: "article",
    url: "/about",
    siteName: site.name,
    title: `${TITLE} · ${site.name}`,
    description: DESCRIPTION,
  },
  twitter: {
    card: "summary_large_image",
    title: `${TITLE} · ${site.name}`,
    description: DESCRIPTION,
  },
};

const PRINCIPLES = [
  {
    label: "READABILITY",
    title: "Statements that read like sentences",
    text: "declare is assign, update is reassign, print is show. A program says what it does, so the reader never has to translate it first.",
  },
  {
    label: "INFERENCE",
    title: "Types inferred, not spelled out",
    text: "Local variables take their type from the value you assign. Only function parameters and return types are annotated, where the contract actually matters.",
  },
  {
    label: "SAFETY",
    title: "Wrong programs never become executables",
    text: "Static type checking rejects mismatches before a single instruction is generated, and runtime guards stop overflow, division by zero and out-of-range indexes.",
  },
  {
    label: "NATIVE OUTPUT",
    title: "Real optimisations, real binaries",
    text: "Cheeku lowers to LLVM and links a host-native executable. Pick O0 through O3 and change the speed, never the behaviour.",
  },
];

const PIPELINE = [
  {
    n: "01",
    title: "Lex",
    text: "The source becomes tokens — keywords, literals, symbols, newlines — each carrying the byte span it came from.",
  },
  {
    n: "02",
    title: "Parse",
    text: "Tokens become an abstract syntax tree with precedence-based expressions, blocks, calls and indexing.",
  },
  {
    n: "03",
    title: "Resolve",
    text: "A symbol table tracks lexical name lookup and scope, so shadowing and forward references resolve the way they read.",
  },
  {
    n: "04",
    title: "Check",
    text: "The type checker infers local types, validates signatures, conditions, arrays, scope and returns, and collects function signatures before any body is checked.",
  },
  {
    n: "05",
    title: "Lower",
    text: "Validated statements are lowered into an LLVM module: i64 integers, f64 floats, i1 booleans and fixed-size numeric arrays.",
  },
  {
    n: "06",
    title: "Optimise",
    text: "The target triple and data layout are configured, then LLVM's O1 to O3 pass pipelines run. O0 skips the optional passes.",
  },
  {
    n: "07",
    title: "Link",
    text: "A native object is emitted and handed to the system Clang linker. The executable is staged beside the destination and renamed only after linking succeeds.",
  },
];

const FACTS = [
  { k: "Language", v: "Chuchu" },
  { k: "Compiler", v: "Cheeku" },
  { k: "Implementation", v: "Rust, Edition 2024" },
  { k: "Backend", v: "LLVM 23.1.x via Inkwell" },
  { k: "Linker", v: "System Clang" },
  { k: "Types", v: "i64 · f64 · i1 · numeric arrays" },
  { k: "Optimisation", v: "O0 through O3" },
  { k: "Output", v: "Host-native executable" },
  { k: "Licence", v: "MIT" },
  { k: "Validated platform", v: "macOS on Apple Silicon" },
];

const IMPLEMENTED = [
  "Inferred local variables with typed reassignment",
  "Signed integers, floats, booleans and homogeneous fixed-size numeric arrays",
  "Arithmetic, comparisons, grouping and unary negation",
  "if / else, while, lexical scope and block shadowing",
  "Top-level typed functions, forward calls, recursion and mutual recursion",
  "Numerical built-ins: abs, sqrt, min, max, sum, len",
  "Scalar output through show",
  "Native object emission and executable linking",
  "O0–O3 optimisation, CPU selection, timing JSON, IR and assembly output",
  "Runtime checks for integer arithmetic, division and array bounds",
];

const LIMITS = [
  "No strings, characters or comments yet",
  "No for, break, continue or logical operators",
  "No imports, user-defined types or file I/O",
  "Arrays are 1–256 elements and cannot be assigned element by element",
  "Function annotations cover scalars only",
  "No language-level input API, JIT or automatic parallelism",
  "Host-native only — there is no cross-OS target flag",
];

const TEAM = [
  { name: "Nouman Ejaz", initials: "NE", color: "#10B23F" },
  { name: "Abdul Sammad", initials: "AS", color: "#FAD705" },
  { name: "Arslan Amjad", initials: "AA", color: "#FD8AB9" },
  { name: "Ammar Baig", initials: "AB", color: "#A7B8FF" },
];

export default function AboutPage() {
  return (
    <div id="top" className="v2">
      <JsonLd data={aboutStructuredData()} />

      {/* AMBIENT LAYER */}
      <div className="ambient" aria-hidden="true">
        <div className="gridpaper" />
        <div
          className="orb"
          style={{
            left: "58%",
            top: 40,
            width: 520,
            height: 520,
            background: "#10B23F",
            opacity: 0.3,
          }}
        />
        <div
          className="orb"
          style={{
            left: "-6%",
            top: 300,
            width: 420,
            height: 420,
            background: "#FAD705",
            opacity: 0.36,
          }}
        />
        <div
          className="orb"
          style={{
            left: "38%",
            top: 520,
            width: 440,
            height: 440,
            background: "#FD8AB9",
            opacity: 0.3,
          }}
        />
      </div>

      <SiteNav current="about" />

      {/* HERO */}
      <section className="wrap about-hero" aria-labelledby="about-title">
        <p className="glass rise hero-badge" style={{ "--d": ".05s" } as Vars}>
          <span className="ping" aria-hidden="true" />
          About the project &middot; Open source &middot; MIT licensed
        </p>
        <h1 id="about-title" className="rise about-title" style={{ "--d": ".12s" } as Vars}>
          <span>A high-performance language,</span>
          <span className="accent">built carefully.</span>
        </h1>
        <p className="rise about-lede" style={{ "--d": ".22s" } as Vars}>
          Chuchu is a statically typed language with automatic type inference.
          Cheeku is the Rust compiler that turns it into native programs through
          LLVM. Both are young, both are open source, and this page is honest
          about what exists today — and what does not.
        </p>
        <div className="rise about-actions" style={{ "--d": ".32s" } as Vars}>
          <a className="btn btn-primary" href={site.links.docs}>
            Read the docs{" "}
            <span className="arr" aria-hidden="true">
              &rarr;
            </span>
          </a>
          <a className="btn btn-glass" href={site.links.source}>
            View on GitHub
          </a>
        </div>
      </section>

      {/* TWO THINGS */}
      <section className="wrap about-section" aria-labelledby="two-things-title">
        <p className="eyebrow">
          <b />
          01 &middot; TWO THINGS, TWO NAMES
        </p>
        <h2 id="two-things-title" className="h2 about-h2">
          Chuchu is the language. Cheeku is the compiler.
        </h2>
        <p className="lede">
          The names are easy to mix up, so it is worth being precise: one is the
          thing you write, the other is the thing that turns it into a binary.
        </p>

        <div className="bento">
          <article className="glass card b6 about-card">
            <p className="feature-label">THE LANGUAGE</p>
            <h3 className="about-card-title">Chuchu</h3>
            <p className="feature-text">
              A high-performance, statically typed language with inferred local types,
              explicit function signatures and checked numerical operations. It
              reads like plain sentences and compiles to native code.
            </p>
            <div className="about-code">
              <div className="about-code-head">
                <span className="about-code-file">hello.chuchu</span>
                <span className="about-code-lang">CHUCHU</span>
              </div>
              <pre className="about-code-body">
                <code>
                  <span className="kw">assign</span> greeting to 42
                  {"\n"}
                  <span className="kw">show</span> greeting
                </code>
              </pre>
              <div className="about-code-out">
                <span className="about-code-prompt">$ </span>
                <span>cheeku hello.chuchu -o hello</span>
              </div>
            </div>
          </article>

          <article className="glass card b6 about-card">
            <p className="feature-label">THE COMPILER</p>
            <h3 className="about-card-title">Cheeku</h3>
            <p className="feature-text">
              The command-line compiler, distributed as the Rust crate{" "}
              <code>chuchu</code>. It type-checks first, then lowers through
              LLVM, optimises, and links a native executable with the system
              Clang.
            </p>
            <ul className="about-list">
              <li>
                <span className="about-dot" style={{ background: "#10B23F" }} />
                Compiles and links; it does not run your program for you.
              </li>
              <li>
                <span className="about-dot" style={{ background: "#FAD705" }} />
                Reports errors with the source byte span that caused them.
              </li>
              <li>
                <span className="about-dot" style={{ background: "#FD8AB9" }} />
                Emits IR and assembly, or timings as JSON, when you ask.
              </li>
            </ul>
          </article>
        </div>
      </section>

      {/* PRINCIPLES */}
      <section className="wrap about-section" aria-labelledby="principles-title">
        <p className="eyebrow">
          <b />
          02 &middot; DESIGN PRINCIPLES
        </p>
        <h2 id="principles-title" className="h2 about-h2">
          Four decisions that shape the whole language
        </h2>
        <p className="lede">
          Everything else in Chuchu follows from these. When a feature is
          ambiguous, they are the tie-breaker.
        </p>

        <div className="bento">
          {PRINCIPLES.map((principle) => (
            <article key={principle.label} className="glass card b6 about-card">
              <p className="feature-label">{principle.label}</p>
              <h3 className="about-card-title">{principle.title}</h3>
              <p className="feature-text">{principle.text}</p>
            </article>
          ))}
        </div>
      </section>

      {/* PIPELINE */}
      <section className="wrap about-section" aria-labelledby="pipeline-title">
        <p className="eyebrow">
          <b />
          03 &middot; HOW CHEEKU WORKS
        </p>
        <h2 id="pipeline-title" className="h2 about-h2">
          From a .chuchu file to a native executable
        </h2>
        <p className="lede">
          One compiler, one pipeline. Each stage hands a checked result to the
          next, and nothing reaches the LLVM backend that has not already
          passed the type checker.
        </p>

        <ol className="about-pipeline">
          {PIPELINE.map((stage) => (
            <li key={stage.n} className="glass about-stage">
              <span className="about-stage-n">{stage.n}</span>
              <div className="about-stage-body">
                <h3 className="about-stage-title">{stage.title}</h3>
                <p className="about-stage-text">{stage.text}</p>
              </div>
            </li>
          ))}
        </ol>

        <div className="glass about-terminal">
          <div className="about-terminal-bar">
            <span>THE WHOLE PIPELINE</span>
          </div>
          <pre className="about-terminal-body">
            <code>
              .chuchu source{"\n"}
              {"    ↓ "}lexer{"\n"}
              tokens with byte spans{"\n"}
              {"    ↓ "}parser{"\n"}
              abstract syntax tree{"\n"}
              {"    ↓ "}type checker + symbol table{"\n"}
              validated program{"\n"}
              {"    ↓ "}typed code generation{"\n"}
              LLVM module{"\n"}
              {"    ↓ "}target configuration + optimisation{"\n"}
              host-native object{"\n"}
              {"    ↓ "}Clang linker{"\n"}
              executable
            </code>
          </pre>
        </div>
      </section>

      {/* FACTS */}
      <section className="wrap about-section" aria-labelledby="facts-title">
        <p className="eyebrow">
          <b />
          04 &middot; UNDER THE HOOD
        </p>
        <h2 id="facts-title" className="h2 about-h2">
          The details worth knowing
        </h2>

        <dl className="about-facts">
          {FACTS.map((fact) => (
            <div key={fact.k} className="glass about-fact">
              <dt className="about-fact-k">{fact.k}</dt>
              <dd className="about-fact-v">{fact.v}</dd>
            </div>
          ))}
        </dl>
      </section>

      {/* STATUS */}
      <section className="wrap about-section" aria-labelledby="status-title">
        <p className="eyebrow">
          <b />
          05 &middot; WHERE IT STANDS
        </p>
        <h2 id="status-title" className="h2 about-h2">
          What works today, and what does not
        </h2>
        <p className="lede">
          Chuchu is in early development. These are the current boundaries of
          the implementation, not a roadmap — the documentation is the source
          of truth.
        </p>

        <div className="about-status">
          <div className="glass card about-status-col">
            <p className="feature-label">IMPLEMENTED</p>
            <ul className="about-list">
              {IMPLEMENTED.map((item) => (
                <li key={item}>
                  <span className="about-dot" style={{ background: "#10B23F" }} />
                  {item}
                </li>
              ))}
            </ul>
          </div>
          <div className="glass card about-status-col">
            <p className="feature-label">NOT YET</p>
            <ul className="about-list">
              {LIMITS.map((item) => (
                <li key={item}>
                  <span className="about-dot" style={{ background: "#FD8AB9" }} />
                  {item}
                </li>
              ))}
            </ul>
          </div>
        </div>

        <p className="how-note">
          The validated native environment is macOS on Apple Silicon. Linux and
          Windows builds exist, but the public docs do not yet claim native
          support for them.
        </p>
      </section>

      {/* TEAM */}
      <section className="wrap about-section" aria-labelledby="team-title">
        <p className="eyebrow">
          <b />
          06 &middot; THE TEAM
        </p>
        <h2 id="team-title" className="h2 about-h2">
          The minds behind Chuchu
        </h2>
        <p className="lede">
          Meet the people building the Chuchu programming language.
        </p>

        <ul className="about-team" role="list">
          {TEAM.map((member) => (
            <li key={member.name} className="glass card about-team-card">
              <span
                className="about-team-initials"
                style={{ backgroundColor: member.color }}
                aria-hidden="true"
              >
                {member.initials}
              </span>
              <h3 className="about-team-name">{member.name}</h3>
            </li>
          ))}
        </ul>
      </section>

      {/* OPEN SOURCE */}
      <section className="wrap about-section" aria-labelledby="open-title">
        <p className="eyebrow">
          <b />
          07 &middot; OPEN SOURCE
        </p>
        <h2 id="open-title" className="h2 about-h2">
          Built in the open
        </h2>
        <p className="lede">
          Chuchu is MIT licensed. The compiler, the documentation and this
          website all live in public repositories, and contributions are
          welcome — compiler behaviour, diagnostics, examples or docs.
        </p>

        <div className="bento">
          <a
            className="glass card b4 about-repo"
            href={site.links.source}
            rel="noreferrer"
          >
            <p className="feature-label">COMPILER</p>
            <h3 className="about-card-title">chuchu-core</h3>
            <p className="feature-text">
              The Rust crate and the Cheeku binary. Lexer, parser, type checker,
              LLVM backend, optimisation and native linking.
            </p>
            <span className="about-repo-link">
              View repository <span aria-hidden="true">&rarr;</span>
            </span>
          </a>
          <a
            className="glass card b4 about-repo"
            href={site.links.docsSource}
            rel="noreferrer"
          >
            <p className="feature-label">DOCUMENTATION</p>
            <h3 className="about-card-title">chuchu-lang-docs</h3>
            <p className="feature-text">
              The reference for the language, the tooling and the compiler
              architecture, published at docs.chuchu-lang.org.
            </p>
            <span className="about-repo-link">
              View repository <span aria-hidden="true">&rarr;</span>
            </span>
          </a>
          <a
            className="glass card b4 about-repo"
            href={site.links.webSource}
            rel="noreferrer"
          >
            <p className="feature-label">WEBSITE</p>
            <h3 className="about-card-title">chuchu-lang-web</h3>
            <p className="feature-text">
              This site. A Next.js App Router project with plain CSS and no
              runtime dependencies beyond React.
            </p>
            <span className="about-repo-link">
              View repository <span aria-hidden="true">&rarr;</span>
            </span>
          </a>
        </div>
      </section>

      {/* CTA */}
      <section className="wrap cta" aria-labelledby="about-cta-title">
        <div className="ring">
          <div className="ring-in cta-inner">
            <div
              aria-hidden="true"
              className="orb"
              style={{
                right: -60,
                top: -80,
                width: 360,
                height: 360,
                background: "#10B23F",
                opacity: 0.4,
              }}
            />
            <div
              aria-hidden="true"
              className="orb"
              style={{
                left: -60,
                bottom: -120,
                width: 340,
                height: 340,
                background: "#FD8AB9",
                opacity: 0.26,
              }}
            />
            <div className="cta-body">
              <div className="cta-copy">
                <h2 id="about-cta-title" className="cta-title">
                  See it for yourself.
                </h2>
                <p className="cta-text">
                  Install Cheeku, compile the bundled example, and run a native
                  program you built yourself.
                </p>
              </div>
              <div className="cta-actions">
                <a className="btn btn-light" href={site.links.install}>
                  Get started{" "}
                  <span className="arr" aria-hidden="true">
                    &rarr;
                  </span>
                </a>
                <Link className="btn btn-dark-glass" href="/">
                  Back to the homepage
                </Link>
              </div>
            </div>
          </div>
        </div>
      </section>

      <SiteFooter />
    </div>
  );
}
