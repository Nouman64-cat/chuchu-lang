import type { MetadataRoute } from "next";
import { site } from "@/lib/site";

export default function manifest(): MetadataRoute.Manifest {
  return {
    name: "Chuchu — the Chuchu programming language",
    short_name: "Chuchu",
    description: site.description,
    start_url: "/",
    scope: "/",
    display: "standalone",
    background_color: "#f7f8f3",
    theme_color: "#10B23F",
    categories: ["developer", "productivity", "utilities"],
    icons: [
      {
        src: "/chuchu-logo-3.svg",
        sizes: "any",
        type: "image/svg+xml",
        purpose: "any",
      },
    ],
  };
}
