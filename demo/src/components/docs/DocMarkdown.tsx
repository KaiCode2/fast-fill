import Link from "next/link";
import ReactMarkdown, { type Components } from "react-markdown";
import remarkGfm from "remark-gfm";
import rehypeRaw from "rehype-raw";
import rehypeSlug from "rehype-slug";
import rehypeHighlight from "rehype-highlight";
import { resolveDocHref } from "@/lib/docs.server";
import { docHref } from "@/lib/docs";
import { Mermaid } from "./Mermaid";

/**
 * Renders a markdown doc with GitHub-flavored markdown, raw HTML, heading anchors and code
 * highlighting. Two element overrides do the docs-specific work:
 *  - `pre`: ```` ```mermaid ```` fenced blocks become rendered diagrams (read straight from the
 *    hast node so highlighting can't interfere); everything else stays a normal code block.
 *  - `a`: relative links between docs are rewritten to in-app /docs routes; external links open
 *    in a new tab; links to non-doc repo files degrade to plain text.
 *
 * Server component — only the <Mermaid> leaf is client-rendered.
 */
export function DocMarkdown({ source, file }: { source: string; file: string }) {
  const components: Components = {
    pre(props) {
      const { node, children, ...rest } = props;
      const codeNode = node?.children?.[0];
      const className =
        codeNode && codeNode.type === "element" ? codeNode.properties?.className : undefined;
      const isMermaid =
        Array.isArray(className) && className.some((c) => String(c) === "language-mermaid");
      if (isMermaid && codeNode && codeNode.type === "element") {
        const textNode = codeNode.children[0];
        const raw = textNode && textNode.type === "text" ? textNode.value.replace(/\n$/, "") : "";
        return <Mermaid code={raw} />;
      }
      return <pre {...rest}>{children}</pre>;
    },
    a(props) {
      const { href, children, ...rest } = props;
      const h = href ?? "";
      // In-page anchor — leave as-is.
      if (h.startsWith("#")) {
        return (
          <a href={h} {...rest}>
            {children}
          </a>
        );
      }
      // Absolute / external (http:, https:, mailto:, //host…) — open in a new tab.
      if (/^[a-z][a-z0-9+.-]*:/i.test(h) || h.startsWith("//")) {
        return (
          <a href={h} target="_blank" rel="noreferrer" {...rest}>
            {children}
          </a>
        );
      }
      // Relative link — rewrite to an in-app docs route when it maps to an included doc.
      const resolved = resolveDocHref(file, h);
      if (resolved) {
        return <Link href={docHref(resolved.slug, resolved.anchor)}>{children}</Link>;
      }
      // Unresolvable relative link (e.g. a source file) — render the text without a dead link.
      return <span {...rest}>{children}</span>;
    },
  };

  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      rehypePlugins={[rehypeRaw, rehypeSlug, rehypeHighlight]}
      components={components}
    >
      {source}
    </ReactMarkdown>
  );
}
