import type { ReactNode } from "react";

interface BadgeProps {
  children: ReactNode;
  variant?: "primary" | "success" | "neutral";
  size?: "sm" | "md";
}

export function Badge({ children, variant = "primary", size = "sm" }: BadgeProps) {
  const styles: Record<string, { bg: string; fg: string }> = {
    primary: {
      bg: "var(--color-primary-muted)",
      fg: "var(--color-primary)",
    },
    success: {
      bg: "rgba(74, 222, 128, 0.15)",
      fg: "var(--color-success)",
    },
    neutral: {
      bg: "var(--color-surface-hover)",
      fg: "var(--color-body-soft)",
    },
  };
  const style = styles[variant];
  const padding = size === "sm" ? "2px 8px" : "4px 12px";

  return (
    <span
      className="inline-flex items-center font-medium uppercase tracking-wide whitespace-nowrap"
      style={{
        backgroundColor: style.bg,
        color: style.fg,
        fontSize: "11px",
        fontWeight: 500,
        letterSpacing: "0.5px",
        padding,
        borderRadius: "var(--radius-pill)",
        lineHeight: 1.4,
      }}
    >
      {children}
    </span>
  );
}
