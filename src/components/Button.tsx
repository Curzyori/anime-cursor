import type { ButtonHTMLAttributes, ReactNode } from "react";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "ghost" | "danger";
  children: ReactNode;
}

export function Button({ variant = "primary", children, className = "", ...props }: ButtonProps) {
  const variants: Record<string, string> = {
    primary: "ac-btn-primary",
    secondary: "ac-btn-secondary",
    ghost: "ac-btn-ghost",
    danger: "ac-btn-danger",
  };

  return (
    <button
      className={`inline-flex items-center justify-center gap-2 px-[18px] rounded-md text-sm font-medium min-h-[40px] transition active:scale-[0.96] disabled:opacity-50 disabled:pointer-events-none ${variants[variant]} ${className}`}
      {...props}
    >
      {children}
    </button>
  );
}
