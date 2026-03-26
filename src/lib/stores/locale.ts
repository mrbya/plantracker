import { Store } from "@tauri-apps/plugin-store";
import { setLocale } from "$lib/paraglide/runtime";

export type AppLocale = "en" | "sk" | "de";

export const LOCALE_LABELS: Record<AppLocale, string> = {
  en: "English",
  sk: "Slovenčina",
  de: "Deutsch",
};

let _store: Store | null = null;

async function getStore(): Promise<Store> {
  if (!_store) _store = await Store.load("config.json");
  return _store;
}

/**
 * Reads the persisted locale from `config.json` and applies it via Paraglide's
 * `setLocale`. Should be called first in `onMount` in `+page.svelte`, before
 * any other startup functions, to minimise the flash of the default locale on
 * non-English sessions.
 *
 * If no locale has been saved yet, this function is a no-op — Paraglide falls
 * back to the `baseLocale` (`"en"`) automatically.
 */
export async function loadLocale(): Promise<void> {
  const store = await getStore();
  const saved = await store.get<AppLocale>("locale");
  if (saved && (["en", "sk", "de"] as string[]).includes(saved)) {
    setLocale(saved);
  }
}

/**
 * Applies `value` as the active locale immediately (synchronous Paraglide
 * side-effect) and persists it to `config.json` for the next session.
 */
export async function saveLocale(value: AppLocale): Promise<void> {
  setLocale(value);
  const store = await getStore();
  await store.set("locale", value);
  await store.save();
}
