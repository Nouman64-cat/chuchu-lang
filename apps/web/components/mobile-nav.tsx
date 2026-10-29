"use client";

import Link from "next/link";
import { useEffect, useId, useRef, useState, type ReactNode } from "react";

export type MobileNavItem = {
  label: string;
  href: string;
  /** External links open as plain anchors; internal ones use client routing. */
  external?: boolean;
  current?: boolean;
};

type MobileNavProps = {
  items: MobileNavItem[];
  /** Target of the "Get started" button, in the bar and in the menu. */
  installHref: string;
  /** Static bar content: the brand and the inline desktop links. */
  children: ReactNode;
};

// Matches the CSS breakpoint where the inline links take over.
const DESKTOP_QUERY = "(min-width: 900px)";

/**
 * Navigation frame with the menu button and dropdown shown instead of the
 * inline links on narrow screens. The dropdown is a sibling of the glass bar,
 * not a child, so its backdrop blur sees the page rather than the bar.
 * Closes on link selection, Escape, any outside press and when the viewport
 * grows past the breakpoint; locks page scroll while open.
 */
export default function MobileNav({ items, installHref, children }: MobileNavProps) {
  const [open, setOpen] = useState(false);
  const button = useRef<HTMLButtonElement>(null);
  const panel = useRef<HTMLDivElement>(null);
  const panelId = useId();

  useEffect(() => {
    if (!open) return;
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setOpen(false);
        button.current?.focus();
      }
    };
    const onPress = (event: PointerEvent) => {
      const target = event.target as Node;
      if (!button.current?.contains(target) && !panel.current?.contains(target)) {
        setOpen(false);
      }
    };
    const desktop = window.matchMedia(DESKTOP_QUERY);
    const onResize = () => {
      if (desktop.matches) setOpen(false);
    };

    // Lock page scroll; pad for the scrollbar so nothing shifts sideways.
    const html = document.documentElement;
    const scrollbar = window.innerWidth - html.clientWidth;
    html.style.overflow = "hidden";
    if (scrollbar > 0) html.style.paddingRight = `${scrollbar}px`;

    document.addEventListener("keydown", onKey);
    document.addEventListener("pointerdown", onPress);
    desktop.addEventListener("change", onResize);
    return () => {
      html.style.overflow = "";
      html.style.paddingRight = "";
      document.removeEventListener("keydown", onKey);
      document.removeEventListener("pointerdown", onPress);
      desktop.removeEventListener("change", onResize);
    };
  }, [open]);

  const close = () => setOpen(false);

  return (
    <nav className="navwrap" aria-label="Main navigation">
      <div className="glass navpill">
        {children}
        <div className="navend">
          <a className="btn btn-primary btn-nav" href={installHref}>
            Get started{" "}
            <span className="arr" aria-hidden="true">
              &rarr;
            </span>
          </a>
          <div className="mobilenav">
            <button
              ref={button}
              type="button"
              className="menu-btn"
              aria-expanded={open}
              aria-controls={panelId}
              aria-label={open ? "Close menu" : "Open menu"}
              onClick={() => setOpen((value) => !value)}
            >
              <span className={`menu-icon${open ? " is-open" : ""}`} aria-hidden="true">
                <i />
                <i />
                <i />
              </span>
            </button>
          </div>
        </div>
      </div>
      <div ref={panel} id={panelId} className="glass menu-panel" hidden={!open}>
        {items.map((item) =>
          item.external ? (
            <a key={item.href} className="menu-link" href={item.href} onClick={close}>
              {item.label}
            </a>
          ) : (
            <Link
              key={item.href}
              className="menu-link"
              href={item.href}
              aria-current={item.current ? "page" : undefined}
              onClick={close}
            >
              {item.label}
            </Link>
          ),
        )}
        <a className="btn btn-primary menu-cta" href={installHref} onClick={close}>
          Get started{" "}
          <span className="arr" aria-hidden="true">
            &rarr;
          </span>
        </a>
      </div>
    </nav>
  );
}
