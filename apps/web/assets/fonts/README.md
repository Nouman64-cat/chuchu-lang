# Fonts

These font files are vendored so that generating Open Graph and Twitter cards
never depends on a font CDN at build time. They are only used by the
`next/og` image routes — the site itself still loads its fonts through
`next/font/google`.

| File | Family | Weight | Source |
| --- | --- | --- | --- |
| `BricolageGrotesque-Medium.ttf` | Bricolage Grotesque | 500 | [google/fonts](https://github.com/google/fonts/tree/main/ofl/bricolagegrotesque) |
| `BricolageGrotesque-Bold.ttf` | Bricolage Grotesque | 700 | [google/fonts](https://github.com/google/fonts/tree/main/ofl/bricolagegrotesque) |
| `GeistMono-Regular.ttf` | Geist Mono | 400 | [google/fonts](https://github.com/google/fonts/tree/main/ofl/geistmono) |

## Licence

Both families are released under the **SIL Open Font License 1.1**.

- Bricolage Grotesque — Copyright The Bricolage Grotesque Project Authors
- Geist Mono — Copyright 2024 The Geist Project Authors

The OFL permits redistribution of the font files with the software. See
<https://openfontlicense.org> for the full terms.
