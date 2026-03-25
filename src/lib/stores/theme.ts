/**
 * Theme store.
 *
 * Manages the user's colour scheme preference and persists it to
 * `config.json` via `tauri-plugin-store`.
 *
 * Three valid choices:
 * - `"dark"`   — Catppuccin Mocha (default CSS variable values in `mocha.css`)
 * - `"light"`  — Catppuccin Latte (overrides applied via the `theme-light`
 *                HTML class in `mocha.css`)
 * - `"system"` — delegates to the OS `prefers-color-scheme` media query
 *
 * Side effect:
 *   The `resolvedTheme` store subscription keeps the `<html>` element's
 *   `theme-light` class in sync with the resolved value.  This subscription
 *   fires synchronously on any theme change, so the page never flickers.
 *
 * Persistence singleton:
 *   `_store` caches the `Store` instance after the first `getStore()` call so
 *   subsequent reads/writes reuse the same handle rather than reopening the
 *   file on every operation.
 */
import { Store } from "@tauri-apps/plugin-store";
import { writable, derived } from "svelte/store";

/** The three valid theme choices. */
export type ThemeChoice = "dark" | "light" | "system";

/**
 * The user's raw theme preference.  May be `"system"`, in which case the
 * actual rendered theme is determined by `resolvedTheme`.
 *
 * Defaults to `"system"` until `loadTheme()` reads the persisted value.
 */
export const themeChoice = writable<ThemeChoice>("system");

/**
 * The effective theme after resolving `"system"` against the OS preference.
 *
 * Always either `"dark"` or `"light"`.  Used by the `<html>` class toggle
 * and can be consumed by components that need to know the concrete theme.
 */
export const resolvedTheme = derived(themeChoice, ($choice) => {
  if ($choice !== "system") return $choice;
  return window.matchMedia("(prefers-color-scheme: light)").matches
    ? "light"
    : "dark";
});

/**
 * Side-effect subscription: keeps the `theme-light` class on `<html>` in sync
 * with the resolved theme so the CSS variable overrides in `mocha.css` apply
 * immediately on any theme change.
 */
resolvedTheme.subscribe((resolved) => {
  if (typeof document === "undefined") return;
  document.documentElement.classList.toggle(
    "theme-light",
    resolved === "light",
  );
});

// ── Persistence ───────────────────────────────────────────────────────────────

/** Cached `tauri-plugin-store` handle; initialised lazily on first access. */
let _store: Store | null = null;

/**
 * Returns the shared `Store` instance, opening `config.json` on first call.
 *
 * The same `config.json` file is shared with `settings.ts` so all persistent
 * preferences are stored in a single file.
 */
async function getStore(): Promise<Store> {
  if (!_store) _store = await Store.load("config.json");
  return _store;
}

/**
 * Reads the persisted theme choice from `config.json` and hydrates the store.
 *
 * Should be called once during app startup (inside `onMount` in
 * `+page.svelte`) before the first render to avoid a flash of the wrong
 * theme.  If no value has been saved yet the store keeps its default
 * (`"system"`).
 */
export async function loadTheme(): Promise<void> {
  const store = await getStore();
  const saved = await store.get<ThemeChoice>("theme");
  if (saved) themeChoice.set(saved);
}

/**
 * Updates the theme choice in-memory and persists it to `config.json`.
 *
 * The `resolvedTheme` derived store (and its `<html>` class side effect) will
 * fire synchronously as a result of `themeChoice.set()`.
 *
 * @param value - The new theme choice to apply and persist.
 */
export async function saveTheme(value: ThemeChoice): Promise<void> {
  themeChoice.set(value);
  const store = await getStore();
  await store.set("theme", value);
  await store.save();
}
