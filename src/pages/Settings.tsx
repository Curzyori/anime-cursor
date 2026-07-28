import { useState } from "react";
import { useAppStore } from "../store";
import { t, LOCALES } from "../i18n";
import { restoreDefaultCursor, openExternalUrl } from "../backend";
import { Dropdown } from "../components/Dropdown";
import { Button } from "../components/Button";
import { Modal } from "../components/Modal";
import {
  IconGitHub,
  IconGlobe,
  IconHeart,
  IconExternalLink,
  IconRotateCcw,
  IconSun,
  IconMoon,
  IconFolder,
  FlagEN,
  FlagID,
  FlagCN,
  FlagJP,
} from "../components/icons";
import type { AppConfig } from "../types";

const LINKS = [
  {
    key: "settings.github",
    url: "https://github.com/Curzyori/anime-cursor",
    icon: IconGitHub,
  },
  {
    key: "settings.website",
    url: "https://anime-cursor.curzy.dev",
    icon: IconGlobe,
  },
  {
    key: "settings.donate",
    url: "https://donate.curzy.dev",
    icon: IconHeart,
  },
];

const FLAG_ICONS: Record<AppConfig["lang"], React.FC<{ size?: number }>> = {
  en: FlagEN,
  id: FlagID,
  zh: FlagCN,
  ja: FlagJP,
};

const PACK_PATH = "~/.anime-cursor/packs/";
const APP_VERSION = "v1.0.0";

export function Settings() {
  const { config, setConfig, pushToast, setActiveVariant } = useAppStore();
  const [showRestore, setShowRestore] = useState(false);
  const [restoring, setRestoring] = useState(false);

  const langOptions = LOCALES.map((l) => ({
    value: l.code,
    label: l.native,
    icon: (() => {
      const Flag = FLAG_ICONS[l.code];
      return <Flag size={22} />;
    })(),
  }));

  const themeOptions = [
    { value: "dark" as const, label: t("settings.theme.dark"), icon: <IconMoon size={16} /> },
    { value: "light" as const, label: t("settings.theme.light"), icon: <IconSun size={16} /> },
  ];

  const handleRestore = async () => {
    setRestoring(true);
    try {
      await restoreDefaultCursor();
      setActiveVariant(null);
      pushToast("success", t("toast.restore.success"));
    } catch (err) {
      pushToast("error", t("toast.restore.error", { error: String(err) }));
    } finally {
      setRestoring(false);
      setShowRestore(false);
    }
  };

  return (
    <div className="flex flex-col h-full overflow-hidden">
      {/* Header */}
      <div
        className="px-6 py-4 shrink-0"
        style={{ borderBottom: "1px solid var(--color-hairline-soft)" }}
      >
        <h1
          className="font-bold text-balance"
          style={{ fontSize: "24px", color: "var(--color-body)", letterSpacing: "-0.3px" }}
        >
          {t("settings.title")}
        </h1>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-6 py-6">
        <div className="max-w-2xl flex flex-col gap-8">
          {/* General */}
          <section>
            <h2
              className="font-semibold mb-4 text-balance"
              style={{ fontSize: "16px", color: "var(--color-body)" }}
            >
              {t("settings.general")}
            </h2>
            <div className="flex flex-col gap-4">
              {/* Language */}
              <div className="flex items-center justify-between">
                <span style={{ color: "var(--color-body)", fontSize: "14px" }}>
                  {t("settings.language")}
                </span>
                <Dropdown
                  value={config.lang}
                  options={langOptions}
                  onChange={(lang) => setConfig({ lang })}
                  ariaLabel={t("settings.language")}
                />
              </div>

              {/* Theme */}
              <div className="flex items-center justify-between">
                <span style={{ color: "var(--color-body)", fontSize: "14px" }}>
                  {t("settings.theme")}
                </span>
                <Dropdown
                  value={config.theme}
                  options={themeOptions}
                  onChange={(theme) => setConfig({ theme })}
                  ariaLabel={t("settings.theme")}
                />
              </div>
            </div>
          </section>

          {/* Divider */}
          <div style={{ height: 1, backgroundColor: "var(--color-hairline-soft)" }} />

          {/* Cursor */}
          <section>
            <h2
              className="font-semibold mb-4 text-balance"
              style={{ fontSize: "16px", color: "var(--color-body)" }}
            >
              {t("settings.cursor")}
            </h2>
            <div className="flex flex-col gap-4">
              {/* Restore */}
              <div className="flex items-center justify-between">
                <span style={{ color: "var(--color-body)", fontSize: "14px" }}>
                  {t("settings.restore")}
                </span>
                <Button
                  variant="secondary"
                  onClick={() => setShowRestore(true)}
                  disabled={restoring}
                >
                  <IconRotateCcw size={16} />
                  {t("settings.restore")}
                </Button>
              </div>

              {/* Pack path */}
              <div className="flex items-center justify-between">
                <span style={{ color: "var(--color-body)", fontSize: "14px" }}>
                  {t("settings.pack_path")}
                </span>
                <span
                  className="flex items-center gap-2 px-3 py-1.5 rounded-md font-mono text-[13px]"
                  style={{
                    backgroundColor: "var(--color-surface-elevated)",
                    color: "var(--color-body)",
                  }}
                >
                  <IconFolder size={14} />
                  {PACK_PATH}
                </span>
              </div>
            </div>
          </section>

          {/* Divider */}
          <div style={{ height: 1, backgroundColor: "var(--color-hairline-soft)" }} />

          {/* Links */}
          <section>
            <h2
              className="font-semibold mb-4 text-balance"
              style={{ fontSize: "16px", color: "var(--color-body)" }}
            >
              {t("settings.links")}
            </h2>
            <div className="flex flex-col">
              {LINKS.map(({ key, url, icon: Icon }, i) => (
                <button
                  key={key}
                  onClick={() => openExternalUrl(url)}
                  className="flex items-center gap-3 px-3 py-2.5 transition w-full text-left min-h-[40px] active:scale-[0.96]"
                  style={{
                    color: "var(--color-body)",
                    borderBottom: i < LINKS.length - 1 ? "1px solid var(--color-hairline-soft)" : "none",
                    borderRadius: i === 0 ? "var(--radius-md) var(--radius-md) 0 0" : i === LINKS.length - 1 ? "0 0 var(--radius-md) var(--radius-md)" : "0",
                  }}
                  onMouseEnter={(e) => {
                    e.currentTarget.style.backgroundColor = "var(--color-surface-hover)";
                  }}
                  onMouseLeave={(e) => {
                    e.currentTarget.style.backgroundColor = "transparent";
                  }}
                >
                  <Icon size={20} className="shrink-0" />
                  <span className="flex-1 text-sm">{t(key)}</span>
                  <IconExternalLink size={16} style={{ color: "var(--color-body)" }} />
                </button>
              ))}
            </div>
          </section>

          {/* Divider */}
          <div style={{ height: 1, backgroundColor: "var(--color-hairline-soft)" }} />

          {/* About */}
          <section>
            <h2
              className="font-semibold mb-4 text-balance"
              style={{ fontSize: "16px", color: "var(--color-body)" }}
            >
              {t("settings.about")}
            </h2>
            <div className="flex items-center justify-between">
              <span style={{ color: "var(--color-body)", fontSize: "14px" }}>
                {t("settings.version")}
              </span>
              <span
                className="px-3 py-1 rounded-full text-xs font-medium tabular-nums"
                style={{
                  backgroundColor: "var(--color-primary-muted)",
                  color: "var(--color-primary)",
                }}
              >
                {APP_VERSION}
              </span>
            </div>
          </section>
        </div>
      </div>

      {/* Restore modal */}
      <Modal
        open={showRestore}
        title={t("modal.restore.title")}
        body={t("modal.restore.body")}
        confirmLabel={t("modal.restore.confirm")}
        onConfirm={handleRestore}
        onClose={() => setShowRestore(false)}
        danger
      />
    </div>
  );
}
