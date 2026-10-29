import { ImageResponse } from "next/og";
import { ogFonts, OgCard, ogImageSize } from "@/lib/og";

export const alt =
  "Chuchu community — report issues and contribute to the high-performance Chuchu language and the Cheeku compiler.";
export const size = ogImageSize;
export const contentType = "image/png";

export default async function Image() {
  return new ImageResponse(
    (
      <OgCard
        eyebrow="COMMUNITY"
        title={["Build a high-performance", "language, in the open."]}
        description="Report issues, propose changes and contribute to Chuchu and the Cheeku compiler on GitHub."
        pills={["Open source", "MIT licensed", "GitHub"]}
      />
    ),
    {
      ...size,
      fonts: ogFonts,
    },
  );
}
