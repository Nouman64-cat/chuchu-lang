/**
 * JSON-LD (schema.org) helpers.
 *
 * Structured data is rendered as a plain `<script type="application/ld+json">`
 * — `next/script` is for executable JavaScript, not inert metadata.
 */

type Json = Record<string, unknown> | Json[];

/**
 * Escapes characters that could break out of the script tag or the JSON
 * payload, so untrusted text can never be injected into the page.
 */
function serialize(data: Json): string {
  return JSON.stringify(data)
    .replace(/</g, "\\u003c")
    .replace(/>/g, "\\u003e")
    .replace(/&/g, "\\u0026")
    .replace(/\u2028/g, "\\u2028")
    .replace(/\u2029/g, "\\u2029");
}

export function JsonLd({ data }: { data: Json }) {
  return (
    <script
      type="application/ld+json"
      // Serialized above with `<`, `>` and `&` escaped, so the payload cannot
      // terminate the script element.
      dangerouslySetInnerHTML={{ __html: serialize(data) }}
    />
  );
}
