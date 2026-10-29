import Image from "next/image";
import Link from "next/link";
import { site } from "@/lib/site";

// Logo intrinsic size from public/chuchu-logo-3.svg (976 x 586).
const LOGO_RATIO = 586 / 976;

type SiteNavProps = {
  /** Marks the active section for assistive technology. */
  current?: "home" | "about";
};

/**
 * Primary navigation, shared by every route so the header never drifts
 * between pages.
 */
export default function SiteNav({ current }: SiteNavProps) {
  return (
    <div className="navwrap">
      <nav className="glass navpill" aria-label="Main navigation">
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
          <Link className="navlink" href="/#how-it-works">
            How it works
          </Link>
          <Link className="navlink" href="/#features">
            Features
          </Link>
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
          <a className="navlink" href={site.links.source}>
            GitHub
          </a>
        </div>
        <a className="btn btn-primary btn-nav" href={site.links.install}>
          Get started{" "}
          <span className="arr" aria-hidden="true">
            &rarr;
          </span>
        </a>
      </nav>
    </div>
  );
}
