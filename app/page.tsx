import Image from "next/image";
import Link from "next/link";

const docsUrl = "https://docs.chuchu-lang.org";
const sourceUrl = "https://github.com/chuchu-lang/chuchu-core";

export default function Home() {
  return (
    <main className="site-shell">
      <header className="site-header page-width">
        <Link className="brand" href="/" aria-label="Chuchu home">
          <Image
            className="brand-logo"
            src="/chuchu-logo-3.svg"
            alt=""
            width={44}
            height={27}
            priority
          />
          <span className="brand-name">chuchu</span>
        </Link>
        <nav className="site-nav" aria-label="Main navigation">
          <a href={docsUrl}>Documentation</a>
          <a href={sourceUrl} target="_blank" rel="noreferrer">
            GitHub
          </a>
          <a className="nav-cta" href={`${docsUrl}/docs/getting-started/installation/`}>
            Get started <span aria-hidden="true">-&gt;</span>
          </a>
        </nav>
      </header>

      <section className="hero page-width" aria-labelledby="hero-title">
        <div className="hero-copy">
          <p className="eyebrow">
            <span className="status-dot" aria-hidden="true" />
            A statically typed language built in Rust
          </p>
          <h1 id="hero-title">
            Simple syntax.
            <br />
            <span>Native possibilities.</span>
          </h1>
          <p className="hero-description">
            Meet Chuchu: a small, evolving language with inferred local types
            and a compiler that turns your code into native executables.
          </p>
          <div className="hero-actions">
            <a className="button button-primary" href={docsUrl}>
              Explore the docs <span aria-hidden="true">-&gt;</span>
            </a>
            <a
              className="button button-secondary"
              href={sourceUrl}
              target="_blank"
              rel="noreferrer"
            >
              View source
            </a>
          </div>
          <p className="hero-note">
            Early development <span aria-hidden="true">&middot;</span> Validated
            on macOS with Apple Silicon
          </p>
        </div>

        <div className="code-card" aria-label="Chuchu code example">
          <div className="code-card-header">
            <div className="window-dots" aria-hidden="true">
              <span />
              <span />
              <span />
            </div>
            <span>hello.chuchu</span>
            <span className="code-language">CHUCHU</span>
          </div>
          <pre>
            <code>
              <span className="code-muted">01 </span>
              <span className="code-keyword">assign</span> x{" "}
              <span className="code-keyword">to</span> 5{"\n"}
              <span className="code-muted">02 </span>
              <span className="code-keyword">reassign</span> x{" "}
              <span className="code-keyword">to</span> 6{"\n"}
              <span className="code-muted">03 </span>
              <span className="code-keyword">show</span> x
            </code>
          </pre>
          <div className="code-output">
            <span>OUTPUT</span>
            <code>6</code>
          </div>
          <div className="code-card-caption">
            <span>Readable from the first line.</span>
            <span className="caption-dot" aria-hidden="true" />
          </div>
        </div>
      </section>

      <section className="features page-width" aria-label="Language highlights">
        <article className="feature">
          <span className="feature-number">01</span>
          <h2>Write naturally</h2>
          <p>
            Clear statements and inferred local types keep the focus on what
            your program does.
          </p>
        </article>
        <article className="feature">
          <span className="feature-number">02</span>
          <h2>Compile to native</h2>
          <p>
            Cheeku, the Chuchu compiler, uses LLVM to build host-native
            executables.
          </p>
        </article>
        <article className="feature">
          <span className="feature-number">03</span>
          <h2>Build with confidence</h2>
          <p>
            Static type checking and checked numerical operations help catch
            mistakes early.
          </p>
        </article>
      </section>

      <footer className="site-footer page-width">
        <Link className="footer-brand" href="/" aria-label="Chuchu home">
          <span className="footer-mark" aria-hidden="true" />
          Chuchu
        </Link>
        <span>Simple syntax. Native possibilities.</span>
        <a href={sourceUrl} target="_blank" rel="noreferrer">
          Open source on GitHub
        </a>
      </footer>
    </main>
  );
}
