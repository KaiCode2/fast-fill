/**
 * Docs manifest — the single source of truth for which markdown files the in-app docs viewer
 * exposes and how their slugs map back to files in the repo. This module is intentionally pure
 * (no `fs`, no `server-only`) so it can be imported from both client components (the sidebar nav)
 * and server components (the route + renderer). The filesystem read and link rewriting (both of
 * which touch node APIs) live in `docs.server.ts`.
 */

export type DocGroup = "Overview" | "Design" | "Reference";

export interface DocMeta {
  /** URL slug under /docs ("" is the index page). */
  slug: string;
  /** Sidebar/title label. */
  title: string;
  /** Sidebar grouping. */
  group: DocGroup;
  /** Path to the markdown file, relative to the repository root. */
  file: string;
}

/** The core protocol docs, in sidebar order. */
export const DOCS: DocMeta[] = [
  { slug: "", title: "Overview", group: "Overview", file: "README.md" },
  { slug: "demo", title: "Live demo", group: "Overview", file: "DEMO.md" },
  { slug: "architecture", title: "Architecture", group: "Design", file: "docs/ARCHITECTURE.md" },
  { slug: "pricing", title: "Pricing", group: "Design", file: "docs/PRICING.md" },
  { slug: "gateway", title: "Gateway (design)", group: "Design", file: "docs/GATEWAY.md" },
  { slug: "deployments", title: "Deployments", group: "Reference", file: "DEPLOYMENTS.md" },
  { slug: "gas", title: "Gas benchmarks", group: "Reference", file: "docs/GAS.md" },
];

/** Group order for the sidebar. */
export const DOC_GROUPS: DocGroup[] = ["Overview", "Design", "Reference"];

/** Build an in-app docs href from a slug (and optional anchor). */
export function docHref(slug: string, anchor = ""): string {
  const base = slug ? `/docs/${slug}` : "/docs";
  return anchor ? `${base}#${anchor}` : base;
}

export function getDocBySlug(slug: string): DocMeta | undefined {
  return DOCS.find((d) => d.slug === slug);
}
