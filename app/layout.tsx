import type { Metadata, Viewport } from "next";
import { Bricolage_Grotesque, Geist, Geist_Mono } from "next/font/google";
import Script from "next/script";
import { JsonLd } from "@/lib/json-ld";
import { site, siteKeywords, SITE_URL } from "@/lib/site";
import { siteStructuredData } from "@/lib/structured-data";
import "./globals.css";

const bricolage = Bricolage_Grotesque({
  variable: "--font-display",
  subsets: ["latin"],
  // Turbopack's next/font rejects weight ranges like "500 800", so load the
  // variable font (200-800). The page uses 500-800, including 650.
  weight: "variable",
  axes: ["opsz"],
});

const geist = Geist({
  variable: "--font-text",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-mono",
  subsets: ["latin"],
});

const gaId = process.env.NEXT_PUBLIC_GA_ID;

const TITLE = "Chuchu — A high-performance, statically typed programming language";

export const metadata: Metadata = {
  // Resolves every relative URL below (canonical, OG images, manifest) against
  // the production origin.
  metadataBase: new URL(SITE_URL),
  title: {
    default: TITLE,
    template: site.titleTemplate,
  },
  description: site.description,
  applicationName: site.name,
  keywords: [...siteKeywords],
  authors: [{ name: site.author.name, url: site.author.url }],
  creator: site.name,
  publisher: site.name,
  category: "technology",
  alternates: {
    canonical: "/",
  },
  openGraph: {
    type: "website",
    locale: site.locale,
    url: "/",
    siteName: site.name,
    title: TITLE,
    description: site.description,
  },
  twitter: {
    card: "summary_large_image",
    title: TITLE,
    description: site.description,
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-image-preview": "large",
      "max-snippet": -1,
      "max-video-preview": -1,
    },
  },
  icons: {
    icon: [{ url: "/chuchu-logo-3.svg", type: "image/svg+xml" }],
    // Served by the app/apple-icon.tsx route handler.
    apple: [{ url: "/apple-icon", type: "image/png", sizes: "180x180" }],
  },
  manifest: "/manifest.webmanifest",
};

export const viewport: Viewport = {
  themeColor: "#10B23F",
  colorScheme: "light",
};

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html
      lang="en"
      className={`${bricolage.variable} ${geist.variable} ${geistMono.variable}`}
    >
      <body>
        <JsonLd data={siteStructuredData()} />
        {children}
        {gaId && (
          <>
            <Script
              src={`https://www.googletagmanager.com/gtag/js?id=${gaId}`}
              strategy="afterInteractive"
            />
            <Script id="google-analytics" strategy="afterInteractive">
              {`
                window.dataLayer = window.dataLayer || [];
                function gtag(){dataLayer.push(arguments);}
                gtag('js', new Date());
                gtag('config', '${gaId}');
              `}
            </Script>
          </>
        )}
      </body>
    </html>
  );
}
