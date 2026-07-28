import { useEffect, useMemo, useState } from "react";
import { useAppStore } from "../store";
import { t } from "../i18n";
import { listPacks } from "../backend";
import { PackCard } from "../components/PackCard";
import { Dropdown } from "../components/Dropdown";
import { Button } from "../components/Button";
import { IconPackage, IconImport } from "../components/icons";
import type { SortMode } from "../types";

export function Dashboard() {
  const { packs, setPacks, navigate, sortMode, setSortMode, pushToast } = useAppStore();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    listPacks()
      .then((result) => {
        if (cancelled) return;
        setPacks(result);
      })
      .catch(() => {
        if (cancelled) return;
        pushToast("error", t("import.error.generic"));
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => {
      cancelled = true;
    };
  }, [setPacks, pushToast]);

  const sortedPacks = useMemo(() => {
    const sorted = [...packs];
    if (sortMode === "recent") {
      sorted.sort((a, b) => b.created_at - a.created_at);
    } else {
      sorted.sort((a, b) => a.name.localeCompare(b.name));
    }
    return sorted;
  }, [packs, sortMode]);

  const sortOptions = [
    { value: "recent" as SortMode, label: t("dashboard.sort.recent") },
    { value: "name" as SortMode, label: t("dashboard.sort.name") },
  ];

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* Header */}
      <div
        className="flex items-center justify-between px-6 py-4 shrink-0"
        style={{ borderBottom: "1px solid var(--color-hairline-soft)" }}
      >
        <div>
          <h1
            className="font-bold text-balance"
            style={{ fontSize: "24px", color: "var(--color-body)", letterSpacing: "-0.3px" }}
          >
            {t("dashboard.title")}
          </h1>
          {packs.length > 0 && (
            <p className="text-xs mt-0.5 tabular-nums" style={{ color: "var(--color-body-soft)" }}>
              {t("dashboard.count", { count: packs.length })}
            </p>
          )}
        </div>
        {packs.length > 0 && (
          <Dropdown
            value={sortMode}
            options={sortOptions}
            onChange={setSortMode}
            ariaLabel={t("dashboard.sort.recent")}
          />
        )}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-6 py-6">
        {loading ? (
          <div
            className="grid gap-5"
            style={{
              gridTemplateColumns: "repeat(auto-fill, minmax(180px, 1fr))",
            }}
          >
            {Array.from({ length: 6 }).map((_, i) => (
              <div
                key={i}
                className="rounded-lg p-4 animate-pulse"
                style={{
                  backgroundColor: "var(--color-surface-card)",
                  border: "1px solid var(--color-hairline)",
                }}
              >
                <div
                  className="rounded-md mb-3"
                  style={{
                    aspectRatio: "1",
                    backgroundColor: "var(--color-surface-elevated)",
                  }}
                />
                <div
                  className="h-4 rounded mb-2"
                  style={{ backgroundColor: "var(--color-surface-elevated)", width: "70%" }}
                />
                <div
                  className="h-3 rounded"
                  style={{ backgroundColor: "var(--color-surface-elevated)", width: "50%" }}
                />
              </div>
            ))}
          </div>
        ) : packs.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full py-12">
            <div
              className="rounded-2xl flex items-center justify-center mb-6 size-20"
              style={{
                backgroundColor: "var(--color-surface-elevated)",
              }}
            >
              <IconPackage size={40} className="opacity-30" />
            </div>
            <h2
              className="font-semibold mb-2 text-balance"
              style={{ fontSize: "20px", color: "var(--color-body)" }}
            >
              {t("dashboard.empty.title")}
            </h2>
            <p
              className="text-center mb-6 max-w-sm text-pretty"
              style={{ color: "var(--color-body-soft)" }}
            >
              {t("dashboard.empty.desc")}
            </p>
            <Button onClick={() => navigate("import")}>
              <IconImport size={16} />
              {t("dashboard.empty.cta")}
            </Button>
          </div>
        ) : (
          <div
            className="grid gap-5"
            style={{
              gridTemplateColumns: "repeat(auto-fill, minmax(180px, 1fr))",
            }}
          >
            {sortedPacks.map((pack) => (
              <PackCard
                key={pack.id}
                pack={pack}
                onClick={() => navigate("pack-detail", pack.id)}
              />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
