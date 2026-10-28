# Chuchu documentation

Docusaurus 3 + TypeScript documentation for the [Chuchu language](https://github.com/chuchu-lang/chuchu-core). Includes a custom responsive homepage, 18 documentation pages, Chuchu syntax highlighting, and local full-text search.

## Development

Requires Node.js 22+ (Node 24 selected by `.nvmrc`).

```sh
nvm use
npm ci
npm start
```

```sh
npm run check  # TypeScript + production build with strict link/anchor checking
npm run serve  # Preview the production build, including local search
```

Search is indexed at build time. The development server does not provide the full production search index. No API key, hosted search account, database, or runtime server is required.

## Content

- `docs/`: implementation-backed guides and language reference.
- `sidebars.ts`: documentation navigation.
- `src/pages/`: custom homepage.
- `src/css/`: site-wide design tokens and documentation styling.
- `src/theme/prism-include-languages.ts`: custom Chuchu syntax grammar.
- `static/`: brand assets, social preview, and robots.txt.

Use descriptive frontmatter titles and descriptions. Link to Markdown files within docs so the build validates routes. Keep runnable Chuchu examples aligned with compiler behavior. Current compiler support is documented explicitly; no invented playground or unsupported release downloads are linked.

## Deployment

`npm run build` produces static files in `build/`. Deploy that directory to a static host. No Node server is required in production.

### Production domain

Default canonical origin: `https://docs.chuchu-lang.org`  
Default base path: `/`

These defaults configure navigation, asset paths, canonical links, social metadata, sitemap entries, and search for the domain root. Local development is available at `http://localhost:3000/`.

### Vercel

Import the `chuchu-lang-docs` repository and use:

- Framework preset: Docusaurus
- Build command: `npm run build`
- Output directory: `build`
- Install command: `npm ci`
- Node version: 24

Add `docs.chuchu-lang.org` in the project's domain settings and apply the DNS record Vercel provides at your DNS provider. Deploying this repository does not configure domain ownership or DNS automatically.

No environment overrides are required for production. If the project already has `SITE_URL` or `BASE_URL` configured, remove stale values or set them to `https://docs.chuchu-lang.org` and `/` respectively.

### Other hosts and URL overrides

The output also works with Cloudflare, Netlify, or another static host. To override the defaults at build time:

```sh
SITE_URL=https://docs.chuchu-lang.org BASE_URL=/ npm run build
```

`SITE_URL` is the origin without a path; `BASE_URL` must begin and end with `/`. Use the host's real 404 handling and directory index support; do not rewrite every missing path to the homepage.

The included GitHub Pages workflow validates pull requests and deploys pushes to `main` when Pages is enabled. Its URL defaults also use the custom domain. If using GitHub Pages instead of Vercel, configure that same custom domain in repository Pages settings and follow GitHub's DNS instructions. Repository variables `SITE_URL` and `BASE_URL` override the workflow defaults. Use a single production host for this domain.

## SEO

Docusaurus prerenders page content as HTML and generates canonical links, per-page titles/descriptions, social metadata, and `sitemap.xml`. English is explicitly configured. The supplied Chuchu logo is used for the navbar, favicon, and social preview. `robots.txt` permits crawling; submit the production sitemap to your search console after deployment. Search and 404 utility pages should not compete with documentation pages.

Set the real origin/base path before publishing. Do not use production indexing settings for publicly accessible previews: add host-level `X-Robots-Tag: noindex` headers or access controls for preview deployments. Useful content and external discovery still matter; framework configuration does not guarantee rankings.

## Versions

One development documentation line is maintained. When stable releases need separate references, Docusaurus can snapshot docs with `npm run docusaurus docs:version <version>`. Avoid versioning every development change.
