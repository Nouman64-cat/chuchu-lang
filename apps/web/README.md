# chuchu-lang-web

The homepage for [Chuchu](https://chuchu-lang.org), a high-performance, statically typed
programming language with inferred types. Its compiler, Cheeku, turns Chuchu
code into native programs through LLVM.

This repository is only the marketing site. The documentation lives at
[docs.chuchu-lang.org](https://docs.chuchu-lang.org) and the compiler source at
[chuchu-lang/chuchu-core](https://github.com/chuchu-lang/chuchu-core).

## Stack

- Next.js (App Router) with `cacheComponents` enabled
- Plain CSS in `app/globals.css`
- Fonts via `next/font/google`: Bricolage Grotesque, Geist and Geist Mono

## SEO

Site-wide metadata lives in `app/layout.tsx` and is driven by the shared
config in `lib/site.ts`, so the canonical URL, social tags and structured
data can never drift apart.

- `lib/site.ts` — origin, name, links and keywords (single source of truth)
- `lib/structured-data.ts` — schema.org entities (`WebSite`, `Organization`,
  `ComputerLanguage`, `SoftwareSourceCode`, `SoftwareApplication`, `AboutPage`)
- `lib/json-ld.tsx` — renders JSON-LD as a plain `<script>` tag, with the
  payload escaped so it cannot break out of the tag
- `app/manifest.ts`, `app/robots.ts`, `app/sitemap.ts` — metadata routes
- `app/opengraph-image.tsx`, `app/twitter-image.tsx`, `app/about/opengraph-image.tsx`
  — social cards generated with `next/og`, using the vendored brand fonts in
  `assets/fonts`
- `app/apple-icon.tsx` — generated apple touch icon

Set `NEXT_PUBLIC_SITE_URL` to override the production origin (defaults to
`https://www.chuchu-lang.org`). Production apex-domain overrides are normalized
to `https://www.chuchu-lang.org` to match the hosting redirect. Sitemap entries,
robots.txt, canonical links and structured data all use this same origin.

## Running locally

```bash
npm install
npm run dev
```

Open http://localhost:3000.

Optional: copy `.env.example` to `.env.local` and set `NEXT_PUBLIC_GA_ID` to
enable Google Analytics. Without it, no analytics script is loaded.

## Scripts

| Command         | What it does                         |
| --------------- | ------------------------------------ |
| `npm run dev`   | Start the development server         |
| `npm run build` | Create a production build            |
| `npm run start` | Serve the production build           |
| `npm run lint`  | Run ESLint                           |

## Layout

- `app/page.tsx`: the homepage (server component)
- `app/about/page.tsx`: the About page (server component)
- `components/how-it-works.tsx`: the six-stage compiler stepper (client)
- `components/optimization-levels.tsx`: the -O0 to -O3 selector (client)
- `components/site-nav.tsx`, `components/site-footer.tsx`: shared chrome
- `app/sitemap.ts`, `app/robots.ts`: SEO metadata routes
