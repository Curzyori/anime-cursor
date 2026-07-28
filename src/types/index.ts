export interface Pack {
  id: string;
  name: string;
  author?: string;
  source_url?: string;
  variants: PackVariant[];
  thumbnail?: string;
  created_at: number;
}

export interface PackVariant {
  id: string;
  filename: string;
  size_label: string;
  ani_path: string;
  preview_path?: string;
}

export interface AppConfig {
  lang: "en" | "id" | "zh" | "ja";
  theme: "dark" | "light";
}

export type Page = "home" | "import" | "pack-detail" | "settings";

export type SortMode = "recent" | "name";

export type ToastType = "success" | "error" | "warning" | "info";

export interface ToastItem {
  id: string;
  type: ToastType;
  message: string;
  duration: number;
}

export type ImportState = "idle" | "extracting" | "success" | "error";
