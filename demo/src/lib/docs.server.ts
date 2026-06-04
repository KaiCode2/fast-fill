/**
 * Server-only docs helpers: read the markdown off disk and rewrite relative cross-links. Kept
 * separate from the pure `docs.ts` manifest so the manifest stays importable from client
 * components. `next build` statically generates every doc page (see the route's
 * `generateStaticParams`), so the markdown is baked into the build — there is no runtime
 * dependency on files outside the `demo/` project.
 */
import "server-only";
import { readFileSync } from "node:fs";
import { posix, resolve } from "node:path";
import { notFound } from "next/navigation";
import { DOCS, getDocBySlug, type DocMeta } from "./docs";

// The demo runs from `demo/` (process.cwd()), so the repo root — where the docs live — is one up.
// This mirrors how `scripts/gen-abis.ts` reaches the repo root for the Foundry artifacts.
const REPO_ROOT = resolve(process.cwd(), "..");

/** Read a doc by its URL slug parts; 404s for unknown slugs. */
export function getDoc(slugParts?: string[]): { meta: DocMeta; content: string } {
  const slug = (slugParts ?? []).join("/");
  const meta = getDocBySlug(slug);
  if (!meta) notFound();
  const content = readFileSync(resolve(REPO_ROOT, meta.file), "utf8");
  return { meta, content };
}

/**
 * Rewrite a relative markdown link (as authored in the docs) to an in-app docs href. Resolves the
 * link against the current doc's directory in repo-root space, then matches it to a manifest entry.
 * Returns null for links that don't map to an included doc (e.g. `src/*.sol`) so the caller can
 * degrade gracefully.
 */
export function resolveDocHref(currentFile: string, href: string): { slug: string; anchor: string } | null {
  const [pathPart, anchor = ""] = href.split("#");
  if (!pathPart) return null; // pure in-page anchors are handled by the caller
  const dir = posix.dirname(currentFile);
  const resolved = posix.normalize(posix.join(dir, pathPart));
  const meta = DOCS.find((d) => d.file === resolved);
  return meta ? { slug: meta.slug, anchor } : null;
}
