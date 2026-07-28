import type { Pack } from "../types";
import { t } from "../i18n";
import { Badge } from "./Badge";
import { IconPackage } from "./icons";

interface PackCardProps {
  pack: Pack;
  onClick: () => void;
}

export function PackCard({ pack, onClick }: PackCardProps) {
  const variantCount = pack.variants.length;
  const isNew = Date.now() - pack.created_at < 120000;

  return (
    <button
      onClick={onClick}
      className="text-left p-4 rounded-lg transition-all duration-200 ease-snap-out group w-full active:scale-[0.96]"
      style={{
        backgroundColor: "var(--color-surface-card)",
        border: "1px solid var(--color-hairline)",
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.borderColor = "var(--color-primary)";
        e.currentTarget.style.transform = "scale(1.02)";
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.borderColor = "var(--color-hairline)";
        e.currentTarget.style.transform = "scale(1)";
      }}
      aria-label={t("pack.open", { name: pack.name })}
    >
      {/* Thumbnail */}
      <div
        className="w-full rounded-md mb-3 flex items-center justify-center overflow-hidden"
        style={{
          aspectRatio: "1",
          backgroundColor: "var(--color-surface-elevated)",
        }}
      >
        {pack.thumbnail ? (
          <img src={pack.thumbnail} alt="" className="w-full h-full object-cover" />
        ) : (
          <IconPackage size={48} className="opacity-30" />
        )}
      </div>

      {/* Name */}
      <h2
        className="font-semibold mb-1 truncate"
        style={{ fontSize: "16px", color: "var(--color-body)" }}
      >
        {pack.name}
      </h2>

      {/* Author */}
      {pack.author && (
        <p
          className="text-xs mb-2 truncate"
          style={{ color: "var(--color-body-soft)" }}
        >
          {pack.author}
        </p>
      )}

      {/* Badges */}
      <div className="flex items-center gap-1.5 flex-wrap">
        <Badge variant="neutral">
          <span className="tabular-nums">{variantCount}</span> variants
        </Badge>
        {isNew && <Badge variant="success">New</Badge>}
      </div>
    </button>
  );
}
