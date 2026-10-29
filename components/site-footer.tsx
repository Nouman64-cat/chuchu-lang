import Image from "next/image";
import Link from "next/link";
import { site } from "@/lib/site";

// Logo intrinsic size from public/chuchu-logo-3.svg (976 x 586).
const LOGO_RATIO = 586 / 976;

/** Site footer, shared by every route. */
export default function SiteFooter() {
  return (
    <div className="wrap">
      <footer className="footer">
        <Link className="footer-brand" href="/" aria-label="Chuchu home">
          <Image
            className="footer-logo"
            src="/chuchu-logo-3.svg"
            alt=""
            width={40}
            height={Math.round(40 * LOGO_RATIO)}
            loading="eager"
          />
          chuchu
        </Link>
        <span>{site.tagline}</span>
        <nav className="footer-nav" aria-label="Footer navigation">
          <Link className="navlink" href="/about">
            About
          </Link>
          <Link className="navlink" href="/community">
            Community
          </Link>
          <Link className="navlink" href="/downloads">
            Downloads
          </Link>
          <a className="navlink" href={site.links.docs}>
            Documentation
          </a>
          <a className="navlink" href={site.links.source}>
            GitHub
          </a>
        </nav>
      </footer>
    </div>
  );
}
