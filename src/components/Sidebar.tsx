import { useAppStore } from "../store";
import { t } from "../i18n";
import type { Page } from "../types";
import { IconHome, IconImport, IconSettings, IconChevronLeft, IconChevronRight } from "./icons";

const NAV_ITEMS: { page: Page; icon: React.FC<{ size?: number }>; labelKey: string }[] = [
  { page: "home", icon: IconHome, labelKey: "nav.home" },
  { page: "import", icon: IconImport, labelKey: "nav.import" },
  { page: "settings", icon: IconSettings, labelKey: "nav.settings" },
];

export function Sidebar() {
  const { sidebarCollapsed, toggleSidebar, currentPage, navigate } = useAppStore();

  const isActive = (page: Page) => {
    if (page === "pack-detail") return currentPage === page;
    return currentPage === page;
  };

  const handleNav = (page: Page) => {
    navigate(page);
  };

  return (
    <aside
      className="flex flex-col shrink-0 transition-[width] duration-200 ease-snap-out overflow-hidden app-region-drag"
      aria-label="Sidebar"
      style={{
        width: sidebarCollapsed ? 56 : 220,
        backgroundColor: "var(--color-surface)",
        borderRight: "1px solid var(--color-hairline-soft)",
      }}
    >
      {/* Logo + app name */}
      <div
        className="flex items-center gap-3 px-4 h-14 shrink-0"
        style={{ borderBottom: "1px solid var(--color-hairline-soft)" }}
      >
        <div
          className="shrink-0 rounded-lg flex items-center justify-center"
          style={{
            width: 32,
            height: 32,
            background: "var(--color-gradient)",
          }}
        >
          <img src="./logo.svg" alt="Anime Cursor" className="w-5 h-5" />
        </div>
        {!sidebarCollapsed && (
          <span
            className="font-semibold whitespace-nowrap"
            style={{ fontSize: "14px", color: "var(--color-body)" }}
          >
            Anime Cursor
          </span>
        )}
      </div>

      {/* Nav items */}
      <nav className="flex-1 py-3 px-2 flex flex-col gap-1 app-region-no-drag" aria-label="Main navigation">
        {NAV_ITEMS.map(({ page, icon: Icon, labelKey: lk }) => {
          const active = isActive(page);
          return (
            <button
              key={page}
              onClick={() => handleNav(page)}
              className="flex items-center gap-3 px-3 py-2 rounded-md text-sm font-medium transition active:scale-[0.96] relative min-h-[40px]"
              style={{
                backgroundColor: active ? "var(--color-surface-card)" : "transparent",
                color: active ? "var(--color-primary)" : "var(--color-body)",
              }}
              aria-label={t(lk)}
              aria-current={active ? "page" : undefined}
              onMouseEnter={(e) => {
                if (!active) e.currentTarget.style.backgroundColor = "var(--color-surface-hover)";
              }}
              onMouseLeave={(e) => {
                if (!active) e.currentTarget.style.backgroundColor = "transparent";
              }}
            >
              {active && (
                <span
                  className="absolute left-0 top-1/2 -translate-y-1/2 rounded-r-full"
                  style={{
                    width: 3,
                    height: 20,
                    backgroundColor: "var(--color-primary)",
                  }}
                />
              )}
              <Icon size={20} />
              {!sidebarCollapsed && <span className="whitespace-nowrap">{t(lk)}</span>}
            </button>
          );
        })}
      </nav>

      {/* Collapse toggle */}
      <div
        className="p-2 app-region-no-drag"
        style={{ borderTop: "1px solid var(--color-hairline-soft)" }}
      >
        <button
          onClick={toggleSidebar}
          className="flex items-center justify-center w-full py-2 rounded-md transition active:scale-[0.96] min-h-[40px]"
          style={{ color: "var(--color-body-soft)" }}
          aria-label={sidebarCollapsed ? "Expand sidebar" : "Collapse sidebar"}
          onMouseEnter={(e) => {
            e.currentTarget.style.backgroundColor = "var(--color-surface-hover)";
          }}
          onMouseLeave={(e) => {
            e.currentTarget.style.backgroundColor = "transparent";
          }}
        >
          {sidebarCollapsed ? (
            <IconChevronRight size={18} />
          ) : (
            <IconChevronLeft size={18} />
          )}
        </button>
      </div>
    </aside>
  );
}
