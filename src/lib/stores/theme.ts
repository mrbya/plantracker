import { Store } from "@tauri-apps/plugin-store";
import { writable, derived } from "svelte/store";

export type ThemeChoice = "dark" | "light" | "system";

export const themeChoice = writable<ThemeChoice>("system");

// Resolves 'system' to the actual OS preference.
export const resolvedTheme = derived(themeChoice, ($choice) => {
  if ($choice !== "system") return $choice;
  return window.matchMedia("(prefers-color-scheme: light)").matches
    ? "light"
    : "dark";
});

// Side-effect: keep <html> class in sync with resolved theme.
resolvedTheme.subscribe((resolved) => {
  if (typeof document === "undefined") return;
  document.documentElement.classList.toggle(
    "theme-light",
    resolved === "light",
  );
});

// ── Persistence ───────────────────────────────────────────────────────────────

let _store: Store | null = null;

async function getStore(): Promise<Store> {
  if (!_store) _store = await Store.load("config.json");
  return _store;
}

export async function loadTheme(): Promise<void> {
  const store = await getStore();
  const saved = await store.get<ThemeChoice>("theme");
  if (saved) themeChoice.set(saved);
}

export async function saveTheme(value: ThemeChoice): Promise<void> {
  themeChoice.set(value);
  const store = await getStore();
  await store.set("theme", value);
  await store.save();
}
