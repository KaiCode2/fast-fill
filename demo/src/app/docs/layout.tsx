import type { ReactNode } from "react";
import "highlight.js/styles/github-dark.css";
import "./docs.css";
import { ConnectBar } from "@/components/ConnectBar";
import { DocsNav } from "@/components/docs/DocsNav";

export default function DocsLayout({ children }: { children: ReactNode }) {
  return (
    <>
      <ConnectBar />
      <div className="mx-auto flex max-w-6xl flex-col gap-8 px-4 py-8 lg:flex-row">
        <aside className="lg:w-56 lg:shrink-0">
          <DocsNav />
        </aside>
        <main className="min-w-0 flex-1">{children}</main>
      </div>
    </>
  );
}
