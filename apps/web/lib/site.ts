/**
 * Single source of truth for site-wide identity, URLs and copy.
 *
 * Everything SEO-facing (metadata, canonical URLs, sitemap, robots, JSON-LD
 * and social images) derives from these values so the marketing site never
 * disagrees with itself.
 */

const configuredSiteUrl = new URL(
  process.env.NEXT_PUBLIC_SITE_URL ?? "https://www.chuchu-lang.org",
);

// Production redirects the apex domain to www. Canonicals and sitemap entries
// must name that final destination, including with an older deployment override.
if (
  configuredSiteUrl.hostname === "chuchu-lang.org" ||
  configuredSiteUrl.hostname === "www.chuchu-lang.org"
) {
  configuredSiteUrl.protocol = "https:";
  configuredSiteUrl.hostname = "www.chuchu-lang.org";
  configuredSiteUrl.port = "";
}

/** Absolute origin without a trailing slash. */
export const SITE_URL = configuredSiteUrl.origin;

export const site = {
  name: "Chuchu",
  /** Used as the `title.template` suffix on every page except the home page. */
  titleTemplate: "%s · Chuchu",
  tagline: "Simple syntax. Native possibilities.",
  description:
    "Chuchu is a high-performance, statically typed programming language with automatic type inference. Its Rust compiler, Cheeku, turns .chuchu source files into native executables through LLVM.",
  url: SITE_URL,
  locale: "en_US",
  language: "en",
  /** The compiler CLI, as opposed to the language itself. */
  compilerName: "Cheeku",
  license: "MIT",
  author: {
    name: "Nouman Ejaz",
    url: "https://github.com/chuchu-lang",
  },
  links: {
    docs: "https://docs.chuchu-lang.org",
    install: "https://docs.chuchu-lang.org/docs/getting-started/installation/",
    source: "https://github.com/chuchu-lang/chuchu-core",
    issues: "https://github.com/chuchu-lang/chuchu-core/issues",
    newIssue: "https://github.com/chuchu-lang/chuchu-core/issues/new",
    pulls: "https://github.com/chuchu-lang/chuchu-core/pulls",
    releases: "https://github.com/chuchu-lang/chuchu-core/releases",
    docsSource: "https://github.com/chuchu-lang/chuchu-lang-docs",
    webSource: "https://github.com/chuchu-lang/chuchu-lang-web",
  },
} as const;

/** Absolute URL for a site-relative path. */
export function absoluteUrl(path = "/"): string {
  return `${SITE_URL}${path.startsWith("/") ? path : `/${path}`}`;
}

/** Keywords shared by every page, tuned for language/tooling search intent. */
export const siteKeywords = [
  "Chuchu",
  "Chuchu programming language",
  "Cheeku compiler",
  "statically typed language",
  "type inference",
  "LLVM",
  "Rust compiler",
  "native executables",
  "systems programming",
  "high performance programming language",
  "fast compiled language",
  "LLVM optimizations",
  "download Cheeku compiler",
  "Chuchu community",
  "contribute to Chuchu",
  "open source compiler",
] as const;
