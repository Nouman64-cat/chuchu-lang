import { ImageResponse } from "next/og";
import { ogFonts, OgCard, ogImageSize } from "@/lib/og";

export const alt =
  "Chuchu — a high-performance, statically typed programming language with inferred types, compiled to native programs through LLVM.";
export const size = ogImageSize;
export const contentType = "image/png";

export default async function Image() {
  return new ImageResponse(
    (
      <OgCard
        eyebrow="THE CHUCHU PROGRAMMING LANGUAGE"
        title={["Simple syntax.", "Native possibilities."]}
        description="A high-performance, statically typed language with inferred types. Its Rust compiler, Cheeku, turns your code into native programs through LLVM."
        pills={["Static types", "Type inference", "Native binaries"]}
        code={{
          lines: ["assign x to 5", "reassign x to 6", "show x"],
          output: "6",
        }}
      />
    ),
    {
      ...size,
      fonts: ogFonts,
    },
  );
}
