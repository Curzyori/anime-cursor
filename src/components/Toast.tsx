import { useEffect, useRef, useState } from "react";
import { useAppStore } from "../store";
import { t } from "../i18n";
import type { ToastItem } from "../types";
import { IconCheck, IconAlertTriangle, IconX, IconGlobe } from "./icons";

const TOAST_STYLES: Record<string, { bg: string; fg: string; icon: React.ReactNode }> = {
  success: {
    bg: "var(--color-success)",
    fg: "#0F0F23",
    icon: <IconCheck size={16} />,
  },
  error: {
    bg: "var(--color-error)",
    fg: "#FFFFFF",
    icon: <IconAlertTriangle size={16} />,
  },
  warning: {
    bg: "var(--color-warning)",
    fg: "#0F0F23",
    icon: <IconAlertTriangle size={16} />,
  },
  info: {
    bg: "var(--color-info)",
    fg: "#0F0F23",
    icon: <IconGlobe size={16} />,
  },
};

function ToastEntry({ toast }: { toast: ToastItem }) {
  const dismiss = useAppStore((s) => s.dismissToast);
  const style = TOAST_STYLES[toast.type] || TOAST_STYLES.info;
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    if (toast.type === "error") return;
    timerRef.current = setTimeout(() => dismiss(toast.id), toast.duration);
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, [toast, dismiss]);

  return (
    <div
      className="flex items-center gap-2 px-4 py-3 rounded-md animate-slide-in-right min-w-[280px] max-w-[400px] shadow-lg"
      style={{ backgroundColor: style.bg, color: style.fg, boxShadow: "var(--shadow-toast)" }}
      role={toast.type === "error" ? "alert" : "status"}
      aria-live={toast.type === "error" ? "assertive" : "polite"}
    >
      {style.icon}
      <span className="text-sm font-medium flex-1">{toast.message}</span>
      <button
        onClick={() => dismiss(toast.id)}
        className="shrink-0 opacity-70 hover:opacity-100 transition-opacity"
        aria-label="Dismiss"
      >
        <IconX size={14} />
      </button>
    </div>
  );
}

export function ToastContainer() {
  const toasts = useAppStore((s) => s.toasts);
  const [mounted, setMounted] = useState(false);
  useEffect(() => setMounted(true), []);

  if (!mounted) return null;

  return (
    <div
      className="fixed top-4 right-4 z-50 flex flex-col gap-2 items-end"
      role="region"
      aria-label="Notifications"
    >
      {toasts.map((toast) => (
        <ToastEntry key={toast.id} toast={toast} />
      ))}
    </div>
  );
}

export { t };
