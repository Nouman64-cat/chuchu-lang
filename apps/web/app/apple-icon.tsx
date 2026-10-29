import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { ImageResponse } from "next/og";

export const size = { width: 180, height: 180 };
export const contentType = "image/png";

/**
 * Apple touch icon.
 *
 * iOS applies its own rounded-rectangle mask, so the logo is placed on the
 * site background and kept well inside the safe area. The asset is read once
 * at module scope so the route stays statically optimized.
 */
const logoSrc = `data:image/svg+xml;base64,${(
  await readFile(join(process.cwd(), "public/chuchu-logo-3.svg"))
).toString("base64")}`;

export default function Icon() {
  return new ImageResponse(
    (
      <div
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          width: "100%",
          height: "100%",
          backgroundColor: "#f7f8f3",
        }}
      >
        <img src={logoSrc} width={140} height={84} alt="" />
      </div>
    ),
    { ...size },
  );
}
