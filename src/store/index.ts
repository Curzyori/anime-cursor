import { create } from "zustand";
import type { AppConfig, Page, SortMode, ToastItem, ToastType, Pack } from "../types";

interface AppState {
  config: AppConfig;
  sidebarCollapsed: boolean;
  currentPage: Page;
  activePackId: string | null;
  sortMode: SortMode;
  activeVariantId: string | null;
  packs: Pack[];
  toasts: ToastItem[];
  setConfig: (config: Partial<AppConfig>) => void;
  toggleSidebar: () => void;
  navigate: (page: Page, packId?: string) => void;
  setSortMode: (mode: SortMode) => void;
  setActiveVariant: (id: string | null) => void;
  setPacks: (packs: Pack[]) => void;
  addPack: (pack: Pack) => void;
  removePack: (id: string) => void;
  pushToast: (type: ToastType, message: string, duration?: number) => void;
  dismissToast: (id: string) => void;
}

export const useAppStore = create<AppState>((set) => ({
  config: {
    lang: (localStorage.getItem("ac:lang") as AppConfig["lang"]) || "en",
    theme: (localStorage.getItem("ac:theme") as AppConfig["theme"]) || "dark",
  },
  sidebarCollapsed: localStorage.getItem("ac:sidebar") === "collapsed",
  currentPage: "home",
  activePackId: localStorage.getItem("ac:activePackId") || null,
  sortMode: "recent",
  activeVariantId: localStorage.getItem("ac:activeVariantId") || null,
  packs: [],
  toasts: [],
  setConfig: (config) =>
    set((s) => {
      const next = { ...s.config, ...config };
      if (config.lang) localStorage.setItem("ac:lang", config.lang);
      if (config.theme) localStorage.setItem("ac:theme", config.theme);
      return { config: next };
    }),
  toggleSidebar: () =>
    set((s) => {
      const next = !s.sidebarCollapsed;
      localStorage.setItem("ac:sidebar", next ? "collapsed" : "expanded");
      return { sidebarCollapsed: next };
    }),
  navigate: (page, packId) =>
    set((s) => {
      const nextPackId = packId !== undefined ? packId : s.activePackId;
      if (packId !== undefined) {
        if (packId === null) localStorage.removeItem("ac:activePackId");
        else localStorage.setItem("ac:activePackId", packId);
      }
      return { currentPage: page, activePackId: nextPackId };
    }),
  setSortMode: (mode) => set({ sortMode: mode }),
  setActiveVariant: (id) =>
    set(() => {
      if (id === null) localStorage.removeItem("ac:activeVariantId");
      else localStorage.setItem("ac:activeVariantId", id);
      return { activeVariantId: id };
    }),
  setPacks: (packs) => set({ packs }),
  addPack: (pack) => set((s) => ({ packs: [pack, ...s.packs] })),
  removePack: (id) =>
    set((s) => {
      // activePackId is a navigation pointer; applied-cursor ownership is per variant
      const removed = s.packs.find((p) => p.id === id);
      const ownsActiveVariant =
        removed?.variants.some((v) => v.id === s.activeVariantId) ?? false;
      if (s.activePackId === id) localStorage.removeItem("ac:activePackId");
      if (ownsActiveVariant) localStorage.removeItem("ac:activeVariantId");
      return {
        packs: s.packs.filter((p) => p.id !== id),
        activePackId: s.activePackId === id ? null : s.activePackId,
        activeVariantId: ownsActiveVariant ? null : s.activeVariantId,
      };
    }),
  pushToast: (type, message, duration = 3000) =>
    set((s) => {
      const id = `t-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
      return { toasts: [...s.toasts, { id, type, message, duration }] };
    }),
  dismissToast: (id) =>
    set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) })),
}));
