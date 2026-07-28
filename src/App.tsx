import { useEffect } from "react";
import { useAppStore } from "./store";
import { Sidebar } from "./components/Sidebar";
import { ToastContainer } from "./components/Toast";
import { t } from "./i18n";
import { Dashboard } from "./pages/Dashboard";
import { Import } from "./pages/Import";
import { PackDetail } from "./pages/PackDetail";
import { Settings } from "./pages/Settings";

export default function App() {
  const { config, currentPage } = useAppStore();

  // Apply theme to document root
  useEffect(() => {
    if (config.theme === "light") {
      document.documentElement.setAttribute("data-theme", "light");
    } else {
      document.documentElement.removeAttribute("data-theme");
    }
  }, [config.theme]);

  // Apply locale to document lang
  useEffect(() => {
    document.documentElement.lang = config.lang;
  }, [config.lang]);

  const renderPage = () => {
    switch (currentPage) {
      case "home":
        return <Dashboard />;
      case "import":
        return <Import />;
      case "pack-detail":
        return <PackDetail />;
      case "settings":
        return <Settings />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <div className="flex h-[100dvh] w-full overflow-hidden">
      <a
        href="#main-content"
        className="sr-only focus:not-sr-only focus:absolute focus:top-2 focus:left-2 focus:z-[100] focus:px-4 focus:py-2 focus:rounded-md focus:text-sm focus:font-medium"
        style={{ backgroundColor: "var(--color-primary)", color: "var(--color-on-primary)" }}
      >
        {t("a11y.skip_to_content")}
      </a>
      <Sidebar />
      <main
        id="main-content"
        className="flex-1 overflow-hidden"
        style={{ backgroundColor: "var(--color-canvas)" }}
      >
        {renderPage()}
      </main>
      <ToastContainer />
    </div>
  );
}
