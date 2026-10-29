# chuchu-lang-web

The homepage for [Chuchu](https://chuchu-lang.org), a small, statically typed
programming language with inferred types. Its compiler, Cheeku, turns Chuchu
code into native programs through LLVM.

This repository is only the marketing site. The documentation lives at
[docs.chuchu-lang.org](https://docs.chuchu-lang.org) and the compiler source at
[chuchu-lang/chuchu-core](https://github.com/chuchu-lang/chuchu-core).

## Stack

- Next.js (App Router) with `cacheComponents` enabled
- Plain CSS in `app/globals.css`
- Fonts via `next/font/google`: Bricolage Grotesque, Geist and Geist Mono

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
- `components/how-it-works.tsx`: the six-stage compiler stepper (client)
- `components/optimization-levels.tsx`: the -O0 to -O3 selector (client)
- `app/sitemap.ts`, `app/robots.ts`: SEO metadata routes
