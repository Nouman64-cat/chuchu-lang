import type { Metadata } from "next";
import type { CSSProperties } from "react";
import SiteFooter from "@/components/site-footer";
import SiteNav from "@/components/site-nav";
import { JsonLd } from "@/lib/json-ld";
import { site } from "@/lib/site";
import { communityStructuredData } from "@/lib/structured-data";

// Lets style objects carry CSS custom properties like --d.
type Vars = CSSProperties & Record<`--${string}`, string>;

const TITLE = "Chuchu community";
const DESCRIPTION =
  "Join the open-source Chuchu community: report issues, propose changes and contribute to the high-performance Chuchu language, the Cheeku compiler, the docs and this site.";

export const metadata: Metadata = {
  title: "Community",
  description: DESCRIPTION,
  alternates: { canonical: "/community" },
  openGraph: {
    type: "website",
    url: "/community",
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

const CHANNELS = [
  {
    label: "BUGS & IDEAS",
    title: "Issues",
    text: "Report a compiler bug, a confusing error message or a missing feature. A minimal .chuchu file, your Cheeku output and your operating system make a report easy to act on.",
    cta: "Open an issue",
    href: site.links.newIssue,
    color: "#10B23F",
  },
  {
    label: "CODE",
    title: "Pull requests",
    text: "Propose a change to the language, the compiler, the docs or the website. Reviewed, tested work is merged into main.",
    cta: "Browse pull requests",
    href: site.links.pulls,
    color: "#FAD705",
  },
  {
    label: "SOURCE",
    title: "GitHub",
    text: "All development happens in the open on GitHub under the MIT licence. Star the repository to follow progress.",
    cta: "Visit chuchu-core",
    href: site.links.source,
    color: "#FD8AB9",
  },
];

const REPOSITORIES = [
  {
    name: "chuchu-core",
    text: "The Chuchu language and the Cheeku compiler, written in Rust on top of LLVM.",
    href: site.links.source,
  },
  {
    name: "chuchu-lang-docs",
    text: "The documentation site, covering installation, the language reference and guides.",
    href: site.links.docsSource,
  },
  {
    name: "chuchu-lang-web",
    text: "This website, built with Next.js.",
    href: site.links.webSource,
  },
];

const STEPS = [
  {
    n: "01",
    title: "Start with an issue",
    text: "Describe the bug or the idea first, so the approach can be agreed before you invest time in code.",
  },
  {
    n: "02",
    title: "Branch from main",
    text: "Keep feature branches short-lived and focused. There are no permanent per-OS forks; everyone works from the same repository.",
  },
  {
    n: "03",
    title: "Run the checks",
    text: "Formatting, tests and warnings-denied Clippy must pass before you open a pull request.",
  },
  {
    n: "04",
    title: "Open a pull request",
    text: "CI builds and tests the compiler natively on macOS (Apple Silicon and Intel), Linux and Windows. A failing platform is a real failure, not something to bypass.",
  },
];

const REPORT_TIPS = [
  "The Cheeku version, your operating system and CPU architecture",
  "The smallest .chuchu program that reproduces the problem",
  "The exact command you ran and the full output",
  "What you expected to happen instead",
];

export default function CommunityPage() {
  return (
    <div id="top" className="v2">
      <JsonLd data={communityStructuredData()} />

      {/* AMBIENT LAYER */}
      <div className="ambient" aria-hidden="true">
        <div className="gridpaper" />
        <div
          className="orb"
          style={{ left: "58%", top: 40, width: 520, height: 520, background: "#10B23F", opacity: 0.3 }}
        />
        <div
          className="orb"
          style={{ left: "-6%", top: 300, width: 420, height: 420, background: "#FAD705", opacity: 0.36 }}
        />
        <div
          className="orb"
          style={{ left: "38%", top: 520, width: 440, height: 440, background: "#FD8AB9", opacity: 0.3 }}
        />
      </div>

      <SiteNav current="community" />

      {/* HERO */}
      <section className="wrap about-hero" aria-labelledby="community-title">
        <p className="glass rise hero-badge" style={{ "--d": ".05s" } as Vars}>
          <span className="ping" aria-hidden="true" />
          Community &middot; Open source &middot; MIT licensed
        </p>
        <h1 id="community-title" className="rise about-title" style={{ "--d": ".12s" } as Vars}>
          <span>Help build a high-performance</span>
          <span className="accent">language, in the open.</span>
        </h1>
        <p className="rise about-lede" style={{ "--d": ".22s" } as Vars}>
          Chuchu and its Cheeku compiler are young, open-source projects, and
          early contributors shape them the most. Report a bug, sharpen an
          error message, improve the docs or write code — GitHub is where it
          all happens.
        </p>
        <div className="rise about-actions" style={{ "--d": ".32s" } as Vars}>
          <a className="btn btn-primary" href={site.links.newIssue}>
            Open an issue{" "}
            <span className="arr" aria-hidden="true">
              &rarr;
            </span>
          </a>
          <a className="btn btn-glass" href={site.links.source}>
            View on GitHub
          </a>
        </div>
      </section>

      {/* CHANNELS */}
      <section className="wrap about-section" aria-labelledby="channels-title">
        <p className="eyebrow">
          <b />
          01 &middot; WHERE TO TAKE PART
        </p>
        <h2 id="channels-title" className="h2 about-h2">
          Three ways to get involved
        </h2>
        <p className="lede">
          There is no separate forum to join. The project lives on GitHub, so
          every conversation sits right next to the code it is about.
        </p>
        <div className="bento">
          {CHANNELS.map((channel) => (
            <article key={channel.title} className="glass card b4 about-card">
              <p className="feature-label">{channel.label}</p>
              <h3 className="about-card-title">{channel.title}</h3>
              <p className="feature-text">{channel.text}</p>
              <p>
                <a className="btn btn-glass" href={channel.href}>
                  {channel.cta}
                </a>
              </p>
            </article>
          ))}
        </div>
      </section>

      {/* REPOSITORIES */}
      <section className="wrap about-section" aria-labelledby="repos-title">
        <p className="eyebrow">
          <b />
          02 &middot; THE REPOSITORIES
        </p>
        <h2 id="repos-title" className="h2 about-h2">
          Pick the part you care about
        </h2>
        <div className="bento">
          {REPOSITORIES.map((repo) => (
            <article key={repo.name} className="glass card b4 about-card">
              <h3 className="about-card-title">{repo.name}</h3>
              <p className="feature-text">{repo.text}</p>
              <p>
                <a className="btn btn-glass" href={repo.href}>
                  View repository
                </a>
              </p>
            </article>
          ))}
        </div>
      </section>

      {/* WORKFLOW */}
      <section className="wrap about-section" aria-labelledby="workflow-title">
        <p className="eyebrow">
          <b />
          03 &middot; CONTRIBUTING
        </p>
        <h2 id="workflow-title" className="h2 about-h2">
          From idea to merged change
        </h2>
        <ol className="about-pipeline">
          {STEPS.map((step) => (
            <li key={step.n} className="glass about-stage">
              <span className="about-stage-n">{step.n}</span>
              <div className="about-stage-body">
                <h3 className="about-stage-title">{step.title}</h3>
                <p className="about-stage-text">{step.text}</p>
              </div>
            </li>
          ))}
        </ol>

        <div className="glass about-terminal">
          <div className="about-terminal-bar">
            <span>BEFORE YOU OPEN A PULL REQUEST</span>
          </div>
          <pre className="about-terminal-body">
            <code>
              cargo fmt --all -- --check{"\n"}
              cargo test --locked{"\n"}
              cargo clippy --locked --all-targets -- -D warnings
            </code>
          </pre>
        </div>
      </section>

      {/* REPORTING */}
      <section className="wrap about-section" aria-labelledby="report-title">
        <p className="eyebrow">
          <b />
          04 &middot; WRITING A GOOD REPORT
        </p>
        <h2 id="report-title" className="h2 about-h2">
          What to include in an issue
        </h2>
        <div className="glass card about-status-col">
          <ul className="about-list">
            {REPORT_TIPS.map((tip) => (
              <li key={tip}>
                <span className="about-dot" style={{ background: "#10B23F" }} />
                {tip}
              </li>
            ))}
          </ul>
        </div>
      </section>

      <SiteFooter />
    </div>
  );
}
