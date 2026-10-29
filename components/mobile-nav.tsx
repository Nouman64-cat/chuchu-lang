"use client";

import Link from "next/link";
import { useEffect, useId, useRef, useState } from "react";

export type MobileNavItem = {
  label: string;
  href: string;
  /** External links open as plain anchors; internal ones use client routing. */
  external?: boolean;
  current?: boolean;
};

/**
 * Menu button and dropdown shown instead of the inline links on narrow
 * screens. Closes on link selection, Escape and any outside press.
 */
export default function MobileNav({ items }: { items: MobileNavItem[] }) {
  const [open, setOpen] = useState(false);
  const root = useRef<HTMLDivElement>(null);
  const panelId = useId();

  useEffect(() => {
    if (!open) return;
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") setOpen(false);
    };
    const onPress = (event: PointerEvent) => {
      if (!root.current?.contains(event.target as Node)) setOpen(false);
    };
    document.addEventListener("keydown", onKey);
    document.addEventListener("pointerdown", onPress);
    return () => {
      document.removeEventListener("keydown", onKey);
      document.removeEventListener("pointerdown", onPress);
    };
  }, [open]);

  const close = () => setOpen(false);

  return (
    <div className="mobilenav" ref={root}>
      <button
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
      <div id={panelId} className="glass menu-panel" hidden={!open}>
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
      </div>
    </div>
  );
}
