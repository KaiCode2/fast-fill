import Link from "next/link";
import type { ReactNode } from "react";

/**
 * Small inline link from a demo UI element to the relevant docs section. No "use client" so it can
 * be dropped into both server and client components.
 */
export function DocLink({
  href,
  children,
  className = "",
}: {
  href: string;
  children: ReactNode;
  className?: string;
}) {
  return (
    <Link
      href={href}
      className={`inline-flex items-center gap-1 text-accent transition hover:underline ${className}`}
    >
      {children}
      <span aria-hidden>→</span>
    </Link>
  );
}
