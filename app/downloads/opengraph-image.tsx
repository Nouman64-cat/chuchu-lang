import { ImageResponse } from "next/og";
import { ogFonts, OgCard, ogImageSize } from "@/lib/og";

export const alt =
  "Download Cheeku, the compiler for the high-performance Chuchu language, for macOS, Linux and Windows.";
export const size = ogImageSize;
export const contentType = "image/png";

export default async function Image() {
  return new ImageResponse(
    (
      <OgCard
        eyebrow="DOWNLOADS"
        title={["Get Cheeku, the", "Chuchu compiler."]}
        description="Native archives for macOS, Linux and Windows, or build Cheeku from source with Rust and LLVM."
        pills={["macOS", "Linux", "Windows"]}
      />
    ),
    {
      ...size,
      fonts: ogFonts,
    },
  );
}
