import Image from "next/image";
import Link from "next/link";
import MobileNav, { type MobileNavItem } from "@/components/mobile-nav";
import { site } from "@/lib/site";

// Logo intrinsic size from public/chuchu-logo-3.svg (976 x 586).
const LOGO_RATIO = 586 / 976;

type SiteNavProps = {
  /** Marks the active section for assistive technology. */
  current?: "home" | "about" | "community" | "downloads";
};

/**
 * Primary navigation, shared by every route so the header never drifts
 * between pages.
 */
export default function SiteNav({ current }: SiteNavProps) {
  const mobileItems: MobileNavItem[] = [
    { label: "About", href: "/about", current: current === "about" },
    { label: "Documentation", href: site.links.docs, external: true },
    { label: "Community", href: "/community", current: current === "community" },
    { label: "Downloads", href: "/downloads", current: current === "downloads" },
    { label: "GitHub", href: site.links.source, external: true },
  ];
  return (
    <MobileNav items={mobileItems} installHref={site.links.install}>
      <Link className="brand" href="/" aria-label="Chuchu home">
        <Image
          className="brand-logo"
          src="/chuchu-logo-3.svg"
          alt=""
          width={50}
          height={Math.round(50 * LOGO_RATIO)}
          loading="eager"
        />
        <span className="brand-name">chuchu</span>
      </Link>
      <div className="navlinks">
        <Link
          className="navlink"
          href="/about"
          aria-current={current === "about" ? "page" : undefined}
        >
          About
        </Link>
        <a className="navlink" href={site.links.docs}>
          Documentation
        </a>
        <Link
          className="navlink"
          href="/community"
          aria-current={current === "community" ? "page" : undefined}
        >
          Community
        </Link>
        <Link
          className="navlink"
          href="/downloads"
          aria-current={current === "downloads" ? "page" : undefined}
        >
          Downloads
        </Link>
        <a className="navlink" href={site.links.source}>
          GitHub
        </a>
      </div>
    </MobileNav>
  );
}
