"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { DOCS, DOC_GROUPS, docHref } from "@/lib/docs";

/** Grouped sidebar navigation for the docs viewer, with active-link highlighting. */
export function DocsNav() {
  const pathname = usePathname();
  return (
    <nav className="lg:sticky lg:top-20 space-y-6 text-sm">
      {DOC_GROUPS.map((group) => (
        <div key={group}>
          <div className="label">{group}</div>
          <ul className="mt-2 space-y-1">
            {DOCS.filter((d) => d.group === group).map((d) => {
              const href = docHref(d.slug);
              const active = pathname === href;
              return (
                <li key={d.slug || "index"}>
                  <Link
                    href={href}
                    className={`block rounded-md px-3 py-1.5 transition ${
                      active
                        ? "bg-accent font-semibold text-ink"
                        : "text-slate-400 hover:bg-edge/60 hover:text-slate-200"
                    }`}
                  >
                    {d.title}
                  </Link>
                </li>
              );
            })}
          </ul>
        </div>
      ))}
    </nav>
  );
}
