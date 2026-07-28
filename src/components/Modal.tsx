import { useEffect } from "react";
import { t } from "../i18n";
import { IconX } from "./icons";

interface ModalProps {
  open: boolean;
  title: string;
  body: string;
  confirmLabel: string;
  cancelLabel?: string;
  onConfirm: () => void;
  onClose: () => void;
  danger?: boolean;
}

export function Modal({
  open,
  title,
  body,
  confirmLabel,
  cancelLabel,
  onConfirm,
  onClose,
  danger,
}: ModalProps) {
  useEffect(() => {
    if (!open) return;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, [open, onClose]);

  if (!open) return null;

  const cancel = cancelLabel || t("btn.cancel");

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-6"
      style={{ backgroundColor: "var(--color-overlay)" }}
      onClick={onClose}
      role="dialog"
      aria-modal="true"
      aria-label={title}
    >
      <div
        className="relative w-full max-w-md rounded-lg p-6 animate-slide-in-right"
        style={{
          backgroundColor: "var(--color-surface-card)",
          boxShadow: "var(--shadow-modal)",
          animation: "slide-in-right 200ms cubic-bezier(0.16,1,0.3,1)",
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <button
          onClick={onClose}
          className="absolute top-4 right-4 opacity-50 hover:opacity-100 transition-opacity"
          aria-label={t("a11y.close")}
        >
          <IconX size={18} />
        </button>
        <h2
          className="font-semibold mb-2"
          style={{ fontSize: "20px", color: "var(--color-body)" }}
        >
          {title}
        </h2>
        <p
          className="mb-6"
          style={{ fontSize: "14px", color: "var(--color-body-soft)", lineHeight: 1.55, overflowWrap: "break-word" }}
        >
          {body}
        </p>
        <div className="flex justify-end gap-3">
          <button
            onClick={onClose}
            className="px-5 py-2 rounded-md text-sm font-medium transition-colors"
            style={{
              backgroundColor: "transparent",
              color: "var(--color-body-soft)",
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.backgroundColor = "var(--color-surface-hover)";
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.backgroundColor = "transparent";
            }}
          >
            {cancel}
          </button>
          <button
            onClick={onConfirm}
            className="px-5 py-2 rounded-md text-sm font-medium transition-colors"
            style={{
              backgroundColor: danger ? "var(--color-error)" : "var(--color-primary)",
              color: danger ? "#FFFFFF" : "var(--color-on-primary)",
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.backgroundColor = danger
                ? "#E0405A"
                : "var(--color-primary-active)";
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.backgroundColor = danger
                ? "var(--color-error)"
                : "var(--color-primary)";
            }}
          >
            {confirmLabel}
          </button>
        </div>
      </div>
    </div>
  );
}
