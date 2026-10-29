import { ImageResponse } from "next/og";
import { ogFonts, OgCard, ogImageSize } from "@/lib/og";

export const alt =
  "About Chuchu — a high-performance, statically typed programming language with inferred types, and Cheeku, the Rust compiler that turns it into native programs through LLVM.";
export const size = ogImageSize;
export const contentType = "image/png";

export default async function Image() {
  return new ImageResponse(
    (
      <OgCard
        eyebrow="ABOUT THE PROJECT"
        title={["A high-performance language,", "built carefully."]}
        description="Chuchu is the language. Cheeku is the Rust compiler that turns it into native programs through LLVM. Both are open source and MIT licensed."
        pills={["MIT licensed", "Built in Rust", "Open source"]}
      />
    ),
    {
      ...size,
      fonts: ogFonts,
    },
  );
}
