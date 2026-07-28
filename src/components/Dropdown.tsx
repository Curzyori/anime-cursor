import { useEffect, useRef, useState } from "react";
import { IconChevronDown } from "./icons";

interface DropdownOption<T extends string> {
  value: T;
  label: string;
  icon?: React.ReactNode;
}

interface DropdownProps<T extends string> {
  value: T;
  options: DropdownOption<T>[];
  onChange: (value: T) => void;
  ariaLabel: string;
}

export function Dropdown<T extends string>({
  value,
  options,
  onChange,
  ariaLabel,
}: DropdownProps<T>) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) return;
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [open]);

  const selected = options.find((o) => o.value === value);

  return (
    <div className="relative" ref={ref}>
      <button
        onClick={() => setOpen(!open)}
        className="flex items-center justify-between gap-2 w-full px-4 py-2 rounded-md text-sm transition-colors"
        style={{
          backgroundColor: "var(--color-surface-elevated)",
          border: "1px solid var(--color-hairline)",
          color: "var(--color-body)",
          minWidth: "180px",
        }}
        aria-label={ariaLabel}
        aria-expanded={open}
      >
        <span className="flex items-center gap-2">
          {selected?.icon}
          {selected?.label}
        </span>
        <IconChevronDown size={16} className={open ? "rotate-180 transition-transform" : "transition-transform"} />
      </button>
      {open && (
        <div
          className="absolute top-full left-0 mt-1 w-full rounded-md overflow-hidden z-30"
          style={{
            backgroundColor: "var(--color-surface-elevated)",
            border: "1px solid var(--color-hairline)",
            boxShadow: "var(--shadow-toast)",
          }}
        >
          {options.map((opt) => (
            <button
              key={opt.value}
              onClick={() => {
                onChange(opt.value);
                setOpen(false);
              }}
              className="flex items-center gap-2 w-full px-4 py-2 text-left text-sm transition-colors"
              style={{
                color: opt.value === value ? "var(--color-primary)" : "var(--color-body)",
                backgroundColor:
                  opt.value === value ? "var(--color-primary-muted)" : "transparent",
              }}
              onMouseEnter={(e) => {
                if (opt.value !== value) {
                  e.currentTarget.style.backgroundColor = "var(--color-surface-hover)";
                }
              }}
              onMouseLeave={(e) => {
                if (opt.value !== value) {
                  e.currentTarget.style.backgroundColor = "transparent";
                }
              }}
            >
              {opt.icon}
              {opt.label}
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
