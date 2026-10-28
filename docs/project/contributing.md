---
title: "Contributing"
description: "Contribute to the Chuchu compiler or documentation, run validation, and write implementation-backed examples."
---

Chuchu is open source. Contributions can improve language behavior, diagnostics, examples, or documentation.

- [Compiler repository](https://github.com/chuchu-lang/chuchu-core)
- [Documentation repository](https://github.com/chuchu-lang/chuchu-lang-docs)

## Work on the compiler

Follow the [installation guide](../getting-started/installation.md), then run from `chuchu-core`:

```bash
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets -- -D warnings
```

The native integration suites are currently macOS-gated. A passing suite on another platform does not establish native platform support.

For benchmark harness changes:

```bash
python3 -m unittest discover -s benchmarks -p test_runner.py
```

Performance experiments belong in separate benchmark runs, outside ordinary correctness checks.

## Work on the documentation

Use Node.js 22 or newer; the repository's `.nvmrc` selects Node 24.

```bash
git clone https://github.com/chuchu-lang/chuchu-lang-docs.git
cd chuchu-lang-docs
npm ci
npm start
```

Before submitting changes:

```bash
npm run check
```

This type-checks the site and builds production pages. Broken internal links and anchors fail the build. Search indexing is generated during production builds; use `npm run serve` after a build to test it.

## Writing guidance

Use Markdown files in `docs/`, give each page a specific title and description, and add new reference pages to `sidebars.ts`. Keep examples executable with the current compiler. Explicitly label invalid code and historical benchmark results.

Do not document planned features as available. When changing language semantics, update examples, reference text, and relevant compiler tests together. Each page's edit link opens its source on GitHub.
