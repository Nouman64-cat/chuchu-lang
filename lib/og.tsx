import { readFile } from "node:fs/promises";
import { join } from "node:path";

/**
 * Shared Open Graph / Twitter card renderer.
 *
 * Satori (the `next/og` renderer) supports a subset of CSS: flexbox only, no
 * grid, and no `filter`. Soft shapes are therefore drawn with radial gradients
 * rather than blurred divs.
 */

export const ogImageSize = { width: 1200, height: 630 };

const FONT_DIR = join(process.cwd(), "assets/fonts");

export type OgFont = {
  name: string;
  data: Buffer;
  weight: 400 | 500 | 700;
  style: "normal";
};

/**
 * Loads the brand fonts used on social cards.
 *
 * The files live in the repository so the build never depends on a font CDN.
 * The read happens once at module scope: the assets never change per request,
 * so this keeps the generated images statically optimized.
 */
async function loadOgFonts(): Promise<OgFont[]> {
  const [displayMedium, displayBold, mono] = await Promise.all([
    readFile(join(FONT_DIR, "BricolageGrotesque-Medium.ttf")),
    readFile(join(FONT_DIR, "BricolageGrotesque-Bold.ttf")),
    readFile(join(FONT_DIR, "GeistMono-Regular.ttf")),
  ]);

  return [
    { name: "Bricolage", data: displayMedium, weight: 500, style: "normal" },
    { name: "Bricolage", data: displayBold, weight: 700, style: "normal" },
    { name: "Geist Mono", data: mono, weight: 400, style: "normal" },
  ];
}

export const ogFonts: OgFont[] = await loadOgFonts();

const COLORS = {
  bg: "#f7f8f3",
  ink: "#14201a",
  muted: "#4f5a54",
  subtle: "#68726c",
  green: "#10b23f",
  yellow: "#fad705",
  pink: "#fd8ab9",
  hair: "rgba(20, 32, 26, 0.09)",
  term: "#0e1812",
};

type OgCardProps = {
  eyebrow: string;
  /** A string, or one entry per rendered line. */
  title: string | string[];
  description: string;
  pills?: string[];
  /** Optional code panel shown on the right of the card. */
  code?: { lines: string[]; output: string };
};

/**
 * Renders the standard Chuchu social card: a two column layout with the
 * headline on the left and a code sample on the right.
 */
export function OgCard({ eyebrow, title, description, pills, code }: OgCardProps) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        width: "100%",
        height: "100%",
        backgroundColor: COLORS.bg,
        color: COLORS.ink,
        fontFamily: "Bricolage",
        position: "relative",
        overflow: "hidden",
        padding: "60px 64px",
      }}
    >
      {/* Grid paper */}
      <div
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          backgroundImage:
            "linear-gradient(rgba(20,32,26,0.055) 1px, transparent 1px), linear-gradient(90deg, rgba(20,32,26,0.055) 1px, transparent 1px)",
          backgroundSize: "56px 56px",
        }}
      />

      {/* Ambient colour */}
      <div
        style={{
          position: "absolute",
          top: -140,
          right: -60,
          width: 460,
          height: 460,
          borderRadius: 999,
          backgroundImage:
            "radial-gradient(circle, rgba(16,178,63,0.34), rgba(16,178,63,0))",
        }}
      />
      <div
        style={{
          position: "absolute",
          bottom: -180,
          left: 180,
          width: 420,
          height: 420,
          borderRadius: 999,
          backgroundImage:
            "radial-gradient(circle, rgba(253,138,185,0.32), rgba(253,138,185,0))",
        }}
      />
      <div
        style={{
          position: "absolute",
          top: 120,
          left: -120,
          width: 320,
          height: 320,
          borderRadius: 999,
          backgroundImage:
            "radial-gradient(circle, rgba(250,215,5,0.3), rgba(250,215,5,0))",
        }}
      />

      {/* Header row */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          position: "relative",
        }}
      >
        <div style={{ display: "flex", alignItems: "center", gap: 14 }}>
          <div
            style={{
              display: "flex",
              width: 34,
              height: 34,
              borderRadius: 10,
              backgroundColor: COLORS.green,
            }}
          />
          <div
            style={{
              display: "flex",
              fontSize: 30,
              fontWeight: 700,
              letterSpacing: "-0.04em",
            }}
          >
            chuchu
          </div>
        </div>
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: 10,
            fontFamily: "Geist Mono",
            fontSize: 17,
            letterSpacing: "0.14em",
            color: COLORS.green,
          }}
        >
          <div
            style={{
              display: "flex",
              width: 10,
              height: 10,
              borderRadius: 999,
              backgroundColor: COLORS.green,
            }}
          />
          OPEN SOURCE
        </div>
      </div>

      {/* Body */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 56,
          flex: 1,
          position: "relative",
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            flex: 1,
            minWidth: 0,
          }}
        >
          <div
            style={{
              display: "flex",
              fontFamily: "Geist Mono",
              fontSize: 18,
              letterSpacing: "0.16em",
              color: COLORS.green,
              marginBottom: 22,
            }}
          >
            {eyebrow}
          </div>
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              fontSize: 62,
              fontWeight: 700,
              lineHeight: 1.04,
              letterSpacing: "-0.045em",
              marginBottom: 24,
            }}
          >
            {(Array.isArray(title) ? title : [title]).map((line) => (
              <div key={line} style={{ display: "flex" }}>
                {line}
              </div>
            ))}
          </div>
          <div
            style={{
              display: "flex",
              fontSize: 24,
              lineHeight: 1.5,
              color: COLORS.muted,
              marginBottom: 32,
            }}
          >
            {description}
          </div>
          {pills ? (
            <div style={{ display: "flex", gap: 10, flexWrap: "wrap" }}>
              {pills.map((pill) => (
                <div
                  key={pill}
                  style={{
                    display: "flex",
                    alignItems: "center",
                    padding: "10px 20px",
                    borderRadius: 999,
                    border: `1px solid ${COLORS.hair}`,
                    backgroundColor: "rgba(255,255,255,0.72)",
                    fontFamily: "Geist Mono",
                    fontSize: 17,
                  }}
                >
                  {pill}
                </div>
              ))}
            </div>
          ) : null}
        </div>

        {code ? (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              width: 430,
              flexShrink: 0,
              borderRadius: 22,
              overflow: "hidden",
              backgroundColor: COLORS.term,
              boxShadow: "0 40px 70px -30px rgba(14,24,18,0.7)",
            }}
          >
            <div
              style={{
                display: "flex",
                alignItems: "center",
                justifyContent: "space-between",
                padding: "16px 22px",
                borderBottom: `1px solid rgba(255,255,255,0.09)`,
              }}
            >
              <div style={{ display: "flex", gap: 8 }}>
                {["#fd8ab9", "#fad705", "#10b23f"].map((color) => (
                  <div
                    key={color}
                    style={{
                      display: "flex",
                      width: 11,
                      height: 11,
                      borderRadius: 999,
                      backgroundColor: color,
                    }}
                  />
                ))}
              </div>
              <div
                style={{
                  display: "flex",
                  fontFamily: "Geist Mono",
                  fontSize: 14,
                  color: "#8fb09b",
                }}
              >
                hello.chuchu
              </div>
            </div>
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                gap: 8,
                padding: "26px 24px",
              }}
            >
              {code.lines.map((line) => (
                <div
                  key={line}
                  style={{
                    display: "flex",
                    fontFamily: "Geist Mono",
                    fontSize: 19,
                    lineHeight: 1.5,
                    color: "#dcebe1",
                  }}
                >
                  {line}
                </div>
              ))}
            </div>
            <div
              style={{
                display: "flex",
                alignItems: "center",
                gap: 10,
                padding: "18px 24px",
                borderTop: "1px solid rgba(255,255,255,0.09)",
                backgroundColor: "rgba(255,255,255,0.03)",
                fontFamily: "Geist Mono",
                fontSize: 18,
                color: "#6fe59b",
              }}
            >
              {code.output}
            </div>
          </div>
        ) : null}
      </div>
    </div>
  );
}
