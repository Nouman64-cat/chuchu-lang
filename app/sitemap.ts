import type { MetadataRoute } from "next";
import { absoluteUrl } from "@/lib/site";

export default function sitemap(): MetadataRoute.Sitemap {
  return [
    {
      url: absoluteUrl("/"),
      // Fixed value: new Date() is not allowed here with cacheComponents enabled.
      lastModified: "2026-10-29",
      changeFrequency: "monthly",
      priority: 1,
    },
    {
      url: absoluteUrl("/about"),
      // Fixed value: new Date() is not allowed here with cacheComponents enabled.
      lastModified: "2026-10-29",
      changeFrequency: "monthly",
      priority: 0.8,
    },
    {
      url: absoluteUrl("/downloads"),
      // Fixed value: new Date() is not allowed here with cacheComponents enabled.
      lastModified: "2026-10-29",
      changeFrequency: "weekly",
      priority: 0.9,
    },
    {
      url: absoluteUrl("/community"),
      // Fixed value: new Date() is not allowed here with cacheComponents enabled.
      lastModified: "2026-10-29",
      changeFrequency: "monthly",
      priority: 0.7,
    },
  ];
}
