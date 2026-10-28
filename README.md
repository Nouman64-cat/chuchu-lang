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

### GitHub Pages (configured default)

Default canonical origin: `https://chuchu-lang.github.io`
Default base path: `/chuchu-lang-docs/`

The included workflow validates pull requests and deploys pushes to `main`. In repository **Settings → Pages → Build and deployment**, select **GitHub Actions**. Push the repository to trigger the workflow. This setup does not publish anything until pushed and Pages is enabled.

### Custom domain / Cloudflare / Netlify / Vercel

Set both variables at build time:

```sh
SITE_URL=https://your-real-domain.example BASE_URL=/ npm run build
```

Replace the example domain with your actual production origin, without a path. `BASE_URL` must begin and end with `/`. These variables control canonical URLs, sitemap entries, navigation, asset paths, and the local search route. For GitHub Actions, set repository variables `SITE_URL` and `BASE_URL` to override defaults. Configure the corresponding domain in your hosting provider and DNS; no domain is claimed by this repository.

- Build command: `npm run build`
- Output directory: `build`
- Node version: 24
- For a root custom domain: `BASE_URL=/`
- For a subdirectory: set that exact path, e.g. `/chuchu-lang-docs/`

Use the host's real 404 handling and directory index support. Do not rewrite all missing paths to the homepage. The build emits a 404 page.

## SEO

Docusaurus prerenders page content as HTML and generates canonical links, per-page titles/descriptions, social metadata, and `sitemap.xml`. English is explicitly configured. The supplied Chuchu logo is used for the navbar, favicon, and social preview. `robots.txt` permits crawling; submit the production sitemap to your search console after deployment. Search and 404 utility pages should not compete with documentation pages.

Set the real origin/base path before publishing. Do not use production indexing settings for publicly accessible previews: add host-level `X-Robots-Tag: noindex` headers or access controls for preview deployments. Useful content and external discovery still matter; framework configuration does not guarantee rankings.

## Versions

One development documentation line is maintained. When stable releases need separate references, Docusaurus can snapshot docs with `npm run docusaurus docs:version <version>`. Avoid versioning every development change.
