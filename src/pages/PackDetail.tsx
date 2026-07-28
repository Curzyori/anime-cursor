import { useState } from "react";
import { useAppStore } from "../store";
import { t } from "../i18n";
import { applyCursor, removePack, restoreDefaultCursor } from "../backend";
import { Badge } from "../components/Badge";
import { Button } from "../components/Button";
import { Modal } from "../components/Modal";
import { IconChevronLeft, IconExternalLink, IconTrash, IconPackage, IconCheck } from "../components/icons";

export function PackDetail() {
  const {
    packs,
    activePackId,
    navigate,
    activeVariantId,
    setActiveVariant,
    removePack: removePackStore,
    pushToast,
  } = useAppStore();

  const [showRemove, setShowRemove] = useState(false);
  const [applying, setApplying] = useState(false);

  const pack = packs.find((p) => p.id === activePackId);
  const sourceUrl = pack?.source_url && /^(https?):\/\//i.test(pack.source_url)
    ? pack.source_url
    : undefined;

  if (!pack) {
    return (
      <div className="flex flex-col items-center justify-center h-full">
        <p style={{ color: "var(--color-body-soft)" }}>{t("pack.no_variants")}</p>
        <Button variant="secondary" onClick={() => navigate("home")} className="mt-4">
          <IconChevronLeft size={16} />
          {t("detail.back")}
        </Button>
      </div>
    );
  }

  const handleApply = async (variantId: string) => {
    const variant = pack.variants.find((v) => v.id === variantId);
    if (!variant) return;
    setApplying(true);
    try {
      await applyCursor(variant);
      setActiveVariant(variantId);
      pushToast("success", t("toast.apply.success"));
    } catch (err) {
      pushToast("error", t("toast.apply.error", { error: String(err) }));
    } finally {
      setApplying(false);
    }
  };

  const handleRemove = async () => {
    try {
      // Only reset the OS cursor if the applied variant belongs to this pack
      const ownsActiveVariant = pack.variants.some((v) => v.id === activeVariantId);
      await removePack(pack.id);
      removePackStore(pack.id);

      if (ownsActiveVariant) {
        try {
          await restoreDefaultCursor();
          setActiveVariant(null);
        } catch (err) {
          console.error("Failed to automatically restore default cursor:", err);
        }
      }

      pushToast("success", t("toast.remove.success"));
      navigate("home");
    } catch (err) {
      pushToast("error", t("toast.remove.error", { error: String(err) }));
    }
    setShowRemove(false);
  };

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* Header bar */}
      <div
        className="flex items-center gap-4 px-6 py-3 shrink-0"
        style={{ borderBottom: "1px solid var(--color-hairline-soft)" }}
      >
        <button
          onClick={() => navigate("home")}
          className="p-2 rounded-md transition-colors inline-flex items-center justify-center min-h-[40px] min-w-[40px]"
          style={{ color: "var(--color-body-soft)" }}
          aria-label={t("detail.back")}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = "var(--color-surface-hover)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "transparent";
          }}
        >
          <IconChevronLeft size={20} />
        </button>
        <span className="text-sm" style={{ color: "var(--color-body-soft)" }}>
          {t("detail.back")}
        </span>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-6 py-6">
        {/* Pack header */}
        <div className="mb-6">
          <div className="flex items-start justify-between gap-4 mb-2">
            <div className="flex-1 min-w-0">
              <h1
                className="font-semibold mb-1 text-balance"
                style={{ fontSize: "24px", color: "var(--color-body)", letterSpacing: "-0.3px" }}
              >
                {pack.name}
              </h1>
              {pack.author && (
                <p className="text-sm mb-1 text-pretty" style={{ color: "var(--color-body-soft)" }}>
                  {t("detail.author", { author: pack.author })}
                </p>
              )}
              {sourceUrl && (
                <a
                  href={sourceUrl}
                  target="_blank"
                  rel="noopener noreferrer"
                  aria-label={`${t("detail.source")} (opens in new tab)`}
                  className="inline-flex items-center gap-1 text-xs transition-colors hover:underline py-1 min-h-[40px]"
                  style={{ color: "var(--color-primary)" }}
                >
                  <IconExternalLink size={16} />
                  {t("detail.source")}
                </a>
              )}
            </div>
            <Button
              variant="secondary"
              onClick={() => setShowRemove(true)}
              className="shrink-0"
              style={{
                color: "var(--color-error)",
                borderColor: "var(--color-error)",
              }}
            >
              <IconTrash size={16} />
              {t("pack.remove")}
            </Button>
          </div>
        </div>

        {/* Variants */}
        <h2
          className="font-semibold mb-3 text-balance"
          style={{ fontSize: "16px", color: "var(--color-body)" }}
        >
          {t("pack.variants")} (<span className="tabular-nums">{pack.variants.length}</span>)
        </h2>

        {pack.variants.length === 0 ? (
          <p style={{ color: "var(--color-body-soft)" }}>{t("pack.no_variants")}</p>
        ) : (
          <div className="flex flex-col gap-3">
            {pack.variants.map((variant) => {
              const isActive = activeVariantId === variant.id;
              return (
                <div
                  key={variant.id}
                  className="flex items-center gap-4 p-3 rounded-lg transition"
                  style={{
                    backgroundColor: "var(--color-surface-elevated)",
                    border: "1px solid var(--color-hairline)",
                    borderLeft: isActive
                      ? "3px solid var(--color-primary)"
                      : "1px solid var(--color-hairline)",
                  }}
                  onMouseEnter={(e) => {
                    if (!isActive) e.currentTarget.style.backgroundColor = "var(--color-surface-hover)";
                  }}
                  onMouseLeave={(e) => {
                    if (!isActive) e.currentTarget.style.backgroundColor = "var(--color-surface-elevated)";
                  }}
                >
                  {/* Thumbnail */}
                  <div
                    className="shrink-0 rounded-md flex items-center justify-center size-12 overflow-hidden border border-hairline-soft"
                    style={{
                      backgroundColor: "var(--color-surface-card)",
                    }}
                  >
                    {variant.preview_path ? (
                      <img src={variant.preview_path} alt="" className="size-full object-cover" />
                    ) : (
                      <IconPackage size={24} className="opacity-30" />
                    )}
                  </div>

                  {/* Name + size */}
                  <div className="flex-1 min-w-0">
                    <p
                      className="text-sm font-medium truncate"
                      style={{ color: "var(--color-body)" }}
                    >
                      {variant.filename}
                    </p>
                    <div className="flex items-center gap-2 mt-1">
                      <span className="text-xs" style={{ color: "var(--color-body-soft)" }}>
                        {t("pack.size_label")}: <span className="tabular-nums">{variant.size_label}</span>
                      </span>
                    </div>
                  </div>

                  {/* Active badge or Apply button */}
                  {isActive ? (
                    <Badge variant="success" size="md">
                      <IconCheck size={12} className="mr-1" />
                      {t("pack.active")}
                    </Badge>
                  ) : (
                    <Button
                      onClick={() => handleApply(variant.id)}
                      disabled={applying}
                      className="shrink-0"
                    >
                      {t("pack.apply")}
                    </Button>
                  )}
                </div>
              );
            })}
          </div>
        )}
      </div>

      {/* Remove modal */}
      <Modal
        open={showRemove}
        title={t("modal.remove.title")}
        body={t("modal.remove.body", { name: pack.name })}
        confirmLabel={t("modal.remove.confirm")}
        onConfirm={handleRemove}
        onClose={() => setShowRemove(false)}
        danger
      />
    </div>
  );
}
