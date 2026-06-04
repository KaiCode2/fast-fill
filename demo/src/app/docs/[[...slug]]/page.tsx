import type { Metadata } from "next";
import { DOCS, getDocBySlug } from "@/lib/docs";
import { getDoc } from "@/lib/docs.server";
import { DocMarkdown } from "@/components/docs/DocMarkdown";

type Params = { slug?: string[] };

// Statically generate a page for every doc in the manifest (index === empty slug). This bakes the
// markdown into the build, so there's no runtime dependency on files outside the demo project.
export function generateStaticParams(): Params[] {
  return DOCS.map((d) => ({ slug: d.slug ? d.slug.split("/") : [] }));
}

export async function generateMetadata({ params }: { params: Promise<Params> }): Promise<Metadata> {
  const { slug } = await params;
  const meta = getDocBySlug((slug ?? []).join("/"));
  return { title: meta ? `${meta.title} — fast-fill docs` : "Docs — fast-fill" };
}

export default async function DocPage({ params }: { params: Promise<Params> }) {
  const { slug } = await params;
  const { meta, content } = getDoc(slug);
  return (
    <article className="doc-prose prose prose-invert max-w-none prose-headings:scroll-mt-24 prose-a:text-accent prose-a:no-underline hover:prose-a:underline prose-pre:rounded-lg prose-pre:border prose-pre:border-edge prose-pre:bg-ink prose-code:before:content-none prose-code:after:content-none prose-img:rounded-lg">
      <DocMarkdown source={content} file={meta.file} />
    </article>
  );
}
