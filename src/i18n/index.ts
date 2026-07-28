import { useAppStore } from "../store";
import type { AppConfig } from "../types";
import { en } from "./en";
import { id } from "./id";
import { zh } from "./zh";
import { ja } from "./ja";

export type Locale = AppConfig["lang"];

const dicts: Record<Locale, Record<string, string>> = {
  en,
  id,
  zh,
  ja,
};

export function t(key: string, params?: Record<string, string | number>): string {
  const lang = useAppStore.getState().config.lang;
  const dict = dicts[lang] || en;
  const template = dict[key] ?? en[key] ?? key;
  if (!params) return template;
  return template.replace(/\{(\w+)\}/g, (_, k: string) =>
    k in params ? String(params[k]) : `{${k}}`
  );
}

export const LOCALES: { code: Locale; label: string; native: string }[] = [
  { code: "en", label: "EN", native: "English" },
  { code: "id", label: "ID", native: "Bahasa Indonesia" },
  { code: "zh", label: "ZH", native: "中文" },
  { code: "ja", label: "JA", native: "日本語" },
];
