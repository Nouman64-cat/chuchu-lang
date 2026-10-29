import type { Metadata } from "next";
import type { CSSProperties } from "react";
import SiteFooter from "@/components/site-footer";
import SiteNav from "@/components/site-nav";
import { JsonLd } from "@/lib/json-ld";
import { site } from "@/lib/site";
import { downloadsStructuredData } from "@/lib/structured-data";

// Lets style objects carry CSS custom properties like --d.
type Vars = CSSProperties & Record<`--${string}`, string>;

const TITLE = "Download Cheeku, the Chuchu compiler";
const DESCRIPTION =
  "Get Cheeku, the compiler for the high-performance Chuchu language: native archives for macOS, Linux and Windows, what each one needs, and how to build from source.";

export const metadata: Metadata = {
  title: "Downloads",
  description: DESCRIPTION,
  alternates: { canonical: "/downloads" },
  openGraph: {
    type: "website",
    url: "/downloads",
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

const PLATFORMS = [
  {
    "label": "macOS \u00b7 Apple Silicon",
    "archive": "cheeku-macos-arm64",
    "triple": "aarch64-apple-darwin",
    "needs": "Matching LLVM 23.1 shared libraries and Apple's clang (via xcrun).",
    "color": "#10B23F"
  },
  {
    "label": "macOS \u00b7 Intel",
    "archive": "cheeku-macos-x64",
    "triple": "x86_64-apple-darwin",
    "needs": "Matching Intel LLVM 23.1 shared libraries and Apple's clang (via xcrun).",
    "color": "#FAD705"
  },
  {
    "label": "Linux \u00b7 x86_64",
    "archive": "cheeku-linux-x64",
    "triple": "x86_64-unknown-linux-gnu",
    "needs": "Matching LLVM 23.1 shared libraries, a compatible glibc and libstdc++, Clang, the GNU linker and libc/libm development files.",
    "color": "#FD8AB9"
  },
  {
    "label": "Windows \u00b7 x86_64",
    "archive": "cheeku-windows-x64",
    "triple": "x86_64-pc-windows-msvc",
    "needs": "LLVM's clang-cl, the x64 Visual Studio C++ Build Tools, the Windows SDK and a compatible MSVC runtime.",
    "color": "#10B23F"
  }
];

export default function DownloadsPage() {
  return (
    <div id="top" className="v2">
      <JsonLd data={downloadsStructuredData()} />

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

      <SiteNav current="downloads" />

      {/* HERO */}
      <section className="wrap about-hero" aria-labelledby="downloads-title">
        <p className="glass rise hero-badge" style={{ "--d": ".05s" } as Vars}>
          <span className="ping" aria-hidden="true" />
          Downloads &middot; Native compiler &middot; MIT licensed
        </p>
        <h1 id="downloads-title" className="rise about-title" style={{ "--d": ".12s" } as Vars}>
          <span>Get Cheeku, the</span>
          <span className="accent">Chuchu compiler.</span>
        </h1>
        <p className="rise about-lede" style={{ "--d": ".22s" } as Vars}>
          Cheeku turns .chuchu source files into fast native executables
          through LLVM. It ships as one native archive per platform, and it is
          not a standalone SDK: LLVM and a C toolchain come from their vendors.
        </p>
        <div className="rise about-actions" style={{ "--d": ".32s" } as Vars}>
          <a className="btn btn-primary" href={site.links.install}>
            Installation guide{" "}
            <span className="arr" aria-hidden="true">
              &rarr;
            </span>
          </a>
          <a className="btn btn-glass" href={site.links.releases}>
            GitHub Releases
          </a>
        </div>
      </section>

      {/* STATUS NOTICE */}
      <section className="wrap about-section" aria-labelledby="status-title">
        <div className="glass card about-status-col">
          <p className="feature-label">RELEASE STATUS</p>
          <h2 id="status-title" className="about-card-title">
            No binary release has been published yet
          </h2>
          <p className="feature-text">
            Release archives are built and tested natively on every platform,
            and are published only after all four builds pass. Until the first
            release appears on GitHub Releases, build Cheeku from source — it
            takes a few commands.
          </p>
        </div>
      </section>

      {/* ARCHIVES */}
      <section className="wrap about-section" aria-labelledby="archives-title">
        <p className="eyebrow">
          <b />
          01 &middot; RELEASE ARCHIVES
        </p>
        <h2 id="archives-title" className="h2 about-h2">
          One archive per platform
        </h2>
        <p className="lede">
          Each archive holds the compiler, the MIT licence and the examples. Pick
          the one that matches your operating system and CPU — an ARM binary is
          not an x64 one.
        </p>
        <div className="bento">
          {PLATFORMS.map((platform) => (
            <article key={platform.archive} className="glass card b6 about-card">
              <p className="feature-label">{platform.triple.toUpperCase()}</p>
              <h3 className="about-card-title">{platform.label}</h3>
              <p className="feature-text">
                <code>{platform.archive}</code>
              </p>
              <ul className="about-list">
                <li>
                  <span className="about-dot" style={{ background: platform.color }} />
                  {platform.needs}
                </li>
              </ul>
            </article>
          ))}
        </div>
      </section>

      {/* BUILD FROM SOURCE */}
      <section className="wrap about-section" aria-labelledby="source-title">
        <p className="eyebrow">
          <b />
          02 &middot; BUILD FROM SOURCE
        </p>
        <h2 id="source-title" className="h2 about-h2">
          Build Cheeku yourself
        </h2>
        <p className="lede">
          You need Rust 1.88 or newer and an LLVM 23.1 installation. Point{" "}
          <code>LLVM_SYS_231_PREFIX</code> at it, then build in release mode.
          The installation guide covers each operating system in detail.
        </p>
        <div className="glass about-terminal">
          <div className="about-terminal-bar">
            <span>BUILD AND RUN</span>
          </div>
          <pre className="about-terminal-body">
            <code>
              git clone https://github.com/chuchu-lang/chuchu-core.git{"\n"}
              cd chuchu-core{"\n"}
              export LLVM_SYS_231_PREFIX=/path/to/llvm-23.1{"\n"}
              cargo build --locked --release{"\n"}
              ./target/release/cheeku examples/main.chuchu -o hello{"\n"}
              ./hello
            </code>
          </pre>
        </div>
      </section>

      {/* NOTES */}
      <section className="wrap about-section" aria-labelledby="notes-title">
        <p className="eyebrow">
          <b />
          03 &middot; GOOD TO KNOW
        </p>
        <h2 id="notes-title" className="h2 about-h2">
          Before you install
        </h2>
        <div className="bento">
          <article className="glass card b6 about-card">
            <p className="feature-label">DEPENDENCIES</p>
            <h3 className="about-card-title">LLVM is not bundled</h3>
            <p className="feature-text">
              Install LLVM and your platform&apos;s linker toolchain from their
              vendors and keep their licences. Use installations that match your
              architecture.
            </p>
          </article>
          <article className="glass card b6 about-card">
            <p className="feature-label">TARGETS</p>
            <h3 className="about-card-title">Host-native output</h3>
            <p className="feature-text">
              Cheeku produces executables for the machine it runs on. There is
              no cross-OS target flag yet.
            </p>
          </article>
        </div>
        <div className="rise about-actions">
          <a className="btn btn-glass" href={site.links.docs}>
            Read the docs
          </a>
          <a className="btn btn-glass" href={site.links.newIssue}>
            Report a download problem
          </a>
        </div>
      </section>

      <SiteFooter />
    </div>
  );
}
