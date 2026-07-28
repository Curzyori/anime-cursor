import type { Pack, PackVariant } from "../types";

// Tauri API imports
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

// Preview paths (real extracted cursor icon files)
const madokaPreview = "./previews/madoka.ico";
const mikuPreview = "./previews/wait.ico";

const defaultImportSvg = `data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="100" height="100"><path d="M6 6 L26 16 L18 18 L26 26 L22 28 L14 20 L10 24 Z" fill="%23ff6b9d" stroke="%23ffffff" stroke-width="1.5"/></svg>`;

let mockPacks: Pack[] = [
  {
    id: "madoka-magica",
    name: "Madoka Magica — Miki Sayaka",
    author: "Unknown",
    source_url: "https://www.cursors-4u.com",
    created_at: Date.now() - 60000,
    thumbnail: madokaPreview,
    variants: [
      {
        id: "madoka-32",
        filename: "Madoka Magica - Miki Sayaka - Puella Magi Madoka M_32-48-64.ani",
        size_label: "32-48-64",
        ani_path: "mock://madoka/32-48-64.ani",
        preview_path: madokaPreview,
      },
      {
        id: "madoka-72",
        filename: "Madoka Magica - Miki Sayaka - Puella Magi Madoka M_72-96-128.ani",
        size_label: "72-96-128",
        ani_path: "mock://madoka/72-96-128.ani",
        preview_path: madokaPreview,
      },
      {
        id: "madoka-256",
        filename: "Madoka Magica - Miki Sayaka - Puella Magi Madoka M_256.ani",
        size_label: "256",
        ani_path: "mock://madoka/256.ani",
        preview_path: madokaPreview,
      },
    ],
  },
  {
    id: "wait-chibi",
    name: "Wait — Hatsune Miku Chibi",
    author: "supermariofps",
    source_url: "https://www.cursors-4u.com",
    created_at: Date.now() - 200000,
    thumbnail: mikuPreview,
    variants: [
      {
        id: "wait-32",
        filename: "Wait_96_32-48-64.ani",
        size_label: "32-48-64",
        ani_path: "mock://wait/32-48-64.ani",
        preview_path: mikuPreview,
      },
      {
        id: "wait-96",
        filename: "Wait_96_96.ani",
        size_label: "96",
        ani_path: "mock://wait/96.ani",
        preview_path: mikuPreview,
      },
    ],
  },
];

export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function listPacks(): Promise<Pack[]> {
  if (isTauri()) {
    try {
      const result = await invoke<Pack[]>("list_packs");
      // Map preview paths to asset URLs; backend created_at is in seconds → ms
      return result.map(p => ({
        ...p,
        created_at: p.created_at * 1000,
        thumbnail: p.thumbnail ? convertFileSrc(p.thumbnail) : undefined,
        variants: p.variants.map(v => ({
          ...v,
          preview_path: v.preview_path ? convertFileSrc(v.preview_path) : undefined
        }))
      }));
    } catch (err) {
      console.error("Tauri invoke list_packs failed, falling back to mock:", err);
    }
  }
  await delay(300);
  return [...mockPacks];
}

export async function importPack(
  filePath: string
): Promise<{ pack: Pack; duplicate: boolean }> {
  if (isTauri()) {
    const p = await invoke<Pack>("import_pack", { zipPath: filePath });
    const mapped: Pack = {
      ...p,
      created_at: p.created_at * 1000,
      thumbnail: p.thumbnail ? convertFileSrc(p.thumbnail) : undefined,
      variants: p.variants.map(v => ({
        ...v,
        preview_path: v.preview_path ? convertFileSrc(v.preview_path) : undefined
      }))
    };
    return { pack: mapped, duplicate: false };
  }
  await delay(1500);
  const pack: Pack = {
    id: `imported-${Date.now()}`,
    name: extractNameFromPath(filePath),
    author: undefined,
    source_url: undefined,
    created_at: Date.now(),
    thumbnail: defaultImportSvg,
    variants: [
      {
        id: `v-${Date.now()}`,
        filename: "cursor.ani",
        size_label: "32-48-64",
        ani_path: `mock://imported/${Date.now()}.ani`,
        preview_path: defaultImportSvg,
      },
    ],
  };
  mockPacks = [pack, ...mockPacks];
  return { pack, duplicate: false };
}

export async function applyCursor(variant: PackVariant): Promise<void> {
  if (isTauri()) {
    return await invoke<void>("apply_cursor", { aniPath: variant.ani_path });
  }
  await delay(600);
  document.body.style.cursor = "wait";
  await delay(200);
  document.body.style.cursor = "crosshair";
  setTimeout(() => { document.body.style.cursor = ""; }, 3000);
}

export async function restoreDefaultCursor(): Promise<void> {
  if (isTauri()) {
    return await invoke<void>("restore_default_cursor");
  }
  await delay(600);
  document.body.style.cursor = "default";
}

export async function removePack(packId: string): Promise<void> {
  if (isTauri()) {
    return await invoke<void>("remove_pack", { packId });
  }
  await delay(400);
  mockPacks = mockPacks.filter((p) => p.id !== packId);
}

export async function openExternalUrl(url: string): Promise<void> {
  if (isTauri()) {
    try {
      const { open } = await import("@tauri-apps/plugin-shell");
      await open(url);
      return;
    } catch (err) {
      console.error("Failed to open URL via Tauri shell plugin, falling back:", err);
    }
  }
  window.open(url, "_blank");
}

const delay = (ms: number): Promise<void> => new Promise<void>((r) => setTimeout(r, ms));

function extractNameFromPath(path: string): string {
  const name = path.split(/[\\/]/).pop() || "Unknown Pack";
  return name.replace(/\.zip$/i, "").replace(/\.ani$/i, "");
}
