import { absoluteUrl, SITE_URL, site } from "./site";

/**
 * schema.org entities describing the project.
 *
 * These are referenced across pages by `@id`, so the same nodes are described
 * once and linked rather than duplicated.
 */

export const ORGANIZATION_ID = `${SITE_URL}/#organization`;
export const WEBSITE_ID = `${SITE_URL}/#website`;
export const LANGUAGE_ID = `${SITE_URL}/#chuchu`;
export const SOURCE_ID = `${SITE_URL}/#chuchu-source`;
export const COMPILER_ID = `${SITE_URL}/#cheeku`;

/** The site itself and the project behind it. Rendered once in the root layout. */
export function siteStructuredData() {
  return {
    "@context": "https://schema.org",
    "@graph": [
      {
        "@type": "WebSite",
        "@id": WEBSITE_ID,
        url: absoluteUrl("/"),
        name: site.name,
        description: site.description,
        inLanguage: site.language,
        publisher: { "@id": ORGANIZATION_ID },
      },
      {
        "@type": "Organization",
        "@id": ORGANIZATION_ID,
        name: site.name,
        url: absoluteUrl("/"),
        description: site.description,
        logo: {
          "@type": "ImageObject",
          url: absoluteUrl("/chuchu-logo-3.svg"),
        },
        sameAs: [
          site.links.source,
          site.links.docsSource,
          site.links.webSource,
        ],
      },
    ],
  };
}

/** The Chuchu language and the Cheeku compiler that targets it. */
export function projectStructuredData() {
  return {
    "@context": "https://schema.org",
    "@graph": [
      {
        "@type": "ComputerLanguage",
        "@id": LANGUAGE_ID,
        name: "Chuchu",
        alternateName: "Chuchu programming language",
        url: absoluteUrl("/"),
        description: site.description,
        inLanguage: site.language,
        creator: { "@id": ORGANIZATION_ID },
      },
      {
        "@type": "SoftwareSourceCode",
        "@id": SOURCE_ID,
        name: "Chuchu",
        description: site.description,
        codeRepository: site.links.source,
        programmingLanguage: { "@id": LANGUAGE_ID },
        runtimePlatform: "macOS",
        license: "https://opensource.org/license/mit",
        author: { "@id": ORGANIZATION_ID },
        isPartOf: { "@id": WEBSITE_ID },
      },
      {
        "@type": "SoftwareApplication",
        "@id": COMPILER_ID,
        name: "Cheeku",
        description:
          "The Chuchu compiler. Cheeku type-checks .chuchu source files and lowers them to LLVM IR, then links a native executable.",
        applicationCategory: "DeveloperApplication",
        operatingSystem: "macOS",
        softwareVersion: "0.1.0",
        programmingLanguage: { "@type": "ComputerLanguage", name: "Rust" },
        codeRepository: site.links.source,
        license: "https://opensource.org/license/mit",
        downloadUrl: site.links.install,
        offers: { "@type": "Offer", price: "0", priceCurrency: "USD" },
        author: { "@id": ORGANIZATION_ID },
      },
    ],
  };
}

/** The About route, describing itself and the project it documents. */
export function aboutStructuredData() {
  return {
    "@context": "https://schema.org",
    "@type": "AboutPage",
    "@id": `${absoluteUrl("/about")}#aboutpage`,
    url: absoluteUrl("/about"),
    name: "About Chuchu",
    description:
      "What Chuchu is, how the Cheeku compiler works, the design principles behind the language, and where the project stands today.",
    inLanguage: site.language,
    isPartOf: { "@id": WEBSITE_ID },
    mainEntity: { "@id": LANGUAGE_ID },
    about: { "@id": LANGUAGE_ID },
    breadcrumb: {
      "@type": "BreadcrumbList",
      itemListElement: [
        {
          "@type": "ListItem",
          position: 1,
          name: "Home",
          item: absoluteUrl("/"),
        },
        {
          "@type": "ListItem",
          position: 2,
          name: "About",
          item: absoluteUrl("/about"),
        },
      ],
    },
  };
}
