import { useCallback, useRef, useState, useEffect } from "react";
import { useAppStore } from "../store";
import { t } from "../i18n";
import { importPack, isTauri } from "../backend";
import { ProgressBar } from "../components/ProgressBar";
import { Button } from "../components/Button";
import { IconUpload, IconCheck } from "../components/icons";
import type { ImportState } from "../types";

export function Import() {
  const { addPack, pushToast, navigate } = useAppStore();
  const [state, setState] = useState<ImportState>("idle");
  const [progress, setProgress] = useState(0);
  const [dragActive, setDragActive] = useState(false);
  const [importedName, setImportedName] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!isTauri()) return;

    let unlistenDragDrop: (() => void) | undefined;

    async function setupDragDrop() {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      const appWindow = getCurrentWindow();

      unlistenDragDrop = await appWindow.onDragDropEvent((event) => {
        if (event.payload.type === "enter" || event.payload.type === "over") {
          setDragActive(true);
        } else if (event.payload.type === "leave") {
          setDragActive(false);
        } else if (event.payload.type === "drop") {
          setDragActive(false);
          const paths = event.payload.paths;
          if (paths && paths.length > 0) {
            const filePath = paths[0];
            const isZip = filePath.toLowerCase().endsWith(".zip");
            const isAni = filePath.toLowerCase().endsWith(".ani");
            if (!isZip && !isAni) {
              pushToast("error", t("import.error.corrupt"));
              return;
            }

            setState("extracting");
            setProgress(10);
            const interval = setInterval(() => {
              setProgress((p) => Math.min(90, p + Math.random() * 15));
            }, 150);

            (async () => {
              try {
                const result = await importPack(filePath);
                clearInterval(interval);
                setProgress(100);

                if (result.duplicate) {
                  setState("error");
                  pushToast("warning", t("import.error.duplicate"));
                  setTimeout(() => setState("idle"), 2000);
                  return;
                }

                addPack(result.pack);
                setImportedName(result.pack.name);
                setState("success");
                pushToast("success", t("import.success.title"));
              } catch (err) {
                clearInterval(interval);
                setState("error");
                pushToast("error", t("import.error.generic") + `: ${String(err)}`);
                setTimeout(() => setState("idle"), 2000);
              }
            })();
          }
        }
      });
    }

    setupDragDrop();

    return () => {
      if (unlistenDragDrop) unlistenDragDrop();
    };
  }, [addPack, pushToast]);

  const handleFile = useCallback(
    async (file: File) => {
      const isZip = file.name.toLowerCase().endsWith(".zip");
      const isAni = file.name.toLowerCase().endsWith(".ani");
      if (!isZip && !isAni) {
        pushToast("error", t("import.error.corrupt"));
        return;
      }

      setState("extracting");
      setProgress(0);

      const interval = setInterval(() => {
        setProgress((p) => Math.min(90, p + Math.random() * 20));
      }, 200);

      try {
        if (isTauri()) {
          throw new Error("Browser file API not available in Tauri mode; use native drag-drop instead");
        }
        const result = await importPack(file.name);
        clearInterval(interval);
        setProgress(100);

        if (result.duplicate) {
          setState("error");
          pushToast("warning", t("import.error.duplicate"));
          setTimeout(() => setState("idle"), 2000);
          return;
        }

        addPack(result.pack);
        setImportedName(result.pack.name);
        setState("success");
        pushToast("success", t("import.success.title"));
      } catch {
        clearInterval(interval);
        setState("error");
        pushToast("error", t("import.error.generic"));
        setTimeout(() => setState("idle"), 2000);
      }
    },
    [addPack, pushToast]
  );

  const handleZoneClick = useCallback(async () => {
    try {
      if (isTauri()) {
        const { open } = await import("@tauri-apps/plugin-dialog");
        const selected = await open({
          multiple: false,
          filters: [
            { name: "Cursor Packs", extensions: ["zip", "ani"] },
          ],
        });
        if (!selected) return; // user cancelled

        setState("extracting");
        setProgress(10);
        const interval = setInterval(() => {
          setProgress((p) => Math.min(90, p + Math.random() * 15));
        }, 150);

        try {
          const result = await importPack(selected);
          clearInterval(interval);
          setProgress(100);

          if (result.duplicate) {
            setState("error");
            pushToast("warning", t("import.error.duplicate"));
            setTimeout(() => setState("idle"), 2000);
            return;
          }

          addPack(result.pack);
          setImportedName(result.pack.name);
          setState("success");
          pushToast("success", t("import.success.title"));
        } catch (err) {
          clearInterval(interval);
          setState("error");
          pushToast("error", t("import.error.generic") + `: ${String(err)}`);
          setTimeout(() => setState("idle"), 2000);
        }
      } else {
        inputRef.current?.click();
      }
    } catch (err) {
      setState("error");
      pushToast("error", t("import.error.generic") + `: ${String(err)}`);
      setTimeout(() => setState("idle"), 2000);
    }
  }, [addPack, pushToast]);

  const handleDrop = useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      setDragActive(false);
      const file = e.dataTransfer.files[0];
      if (file) handleFile(file);
    },
    [handleFile]
  );

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setDragActive(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    setDragActive(false);
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) handleFile(file);
    e.target.value = "";
  };

  const reset = () => {
    setState("idle");
    setProgress(0);
    setImportedName("");
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
          {t("import.title")}
        </h1>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-6 py-6">
        {state === "success" ? (
          <div className="flex flex-col items-center justify-center h-full">
            <div
              className="rounded-full flex items-center justify-center mb-6 size-16"
              style={{
                backgroundColor: "rgba(74, 222, 128, 0.15)",
                color: "var(--color-success)",
              }}
            >
              <IconCheck size={32} />
            </div>
            <h2
              className="font-semibold mb-2 text-balance"
              style={{ fontSize: "20px", color: "var(--color-body)" }}
            >
              {t("import.success.title")}
            </h2>
            <p
              className="text-center mb-6 max-w-sm text-pretty"
              style={{ color: "var(--color-body-soft)" }}
            >
              {t("import.success.desc", { name: importedName })}
            </p>
            <div className="flex gap-3">
              <Button variant="secondary" onClick={reset}>
                {t("import.success.again")}
              </Button>
              <Button onClick={() => navigate("home")}>
                {t("nav.home")}
              </Button>
            </div>
          </div>
        ) : (
          <div className="flex flex-col items-center justify-center h-full">
            {/* Drop zone */}
            <div
              onClick={handleZoneClick}
              onDrop={handleDrop}
              onDragOver={handleDragOver}
              onDragLeave={handleDragLeave}
              className="w-full max-w-xl rounded-xl flex flex-col items-center justify-center cursor-pointer transition duration-200 ease-snap-out active:scale-[0.98]"
              style={{
                padding: "48px",
                backgroundColor: "var(--color-surface-elevated)",
                border: `2px dashed ${dragActive ? "var(--color-primary)" : "var(--color-hairline)"}`,
                boxShadow: dragActive ? "inset 0 0 40px rgba(255,107,157,0.2)" : "none",
                animation: dragActive ? "pulse-glow 1s infinite alternate" : "none",
              }}
              role="button"
              tabIndex={0}
              aria-label={t("import.dropzone.title")}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") handleZoneClick();
              }}
            >
              <div
                className="rounded-full flex items-center justify-center mb-4 size-14"
                style={{
                  backgroundColor: dragActive
                    ? "var(--color-primary-muted)"
                    : "var(--color-surface-hover)",
                  color: dragActive ? "var(--color-primary)" : "var(--color-body-soft)",
                  transition: "all 200ms cubic-bezier(0.16,1,0.3,1)",
                }}
              >
                <IconUpload size={28} />
              </div>
              <p
                className="font-semibold mb-1 text-balance"
                style={{ fontSize: "16px", color: "var(--color-body)" }}
              >
                {dragActive ? t("import.dropzone.active") : t("import.dropzone.title")}
              </p>
              <p className="text-xs text-pretty" style={{ color: "var(--color-body-soft)" }}>
                {t("import.dropzone.subtitle")}
              </p>
              <input
                ref={inputRef}
                type="file"
                accept=".zip,.ani"
                onChange={handleInputChange}
                className="hidden"
              />
            </div>

            {/* Progress bar */}
            {state === "extracting" && (
              <div className="w-full max-w-xl mt-6">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm" style={{ color: "var(--color-body-soft)" }}>
                    {t("import.progress")}
                  </span>
                  <span className="text-sm font-medium" style={{ color: "var(--color-primary)" }}>
                    {Math.round(progress)}%
                  </span>
                </div>
                <ProgressBar progress={progress} label={t("import.progress")} />
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
