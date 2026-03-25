/**
 * Settings store.
 *
 * Manages all user-configurable preferences that are persisted to `config.json`
 * via `tauri-plugin-store`.  The same `config.json` file is shared with
 * `theme.ts`.
 *
 * Stored keys and their defaults:
 *
 * | Key              | Store          | Default    | Description                             |
 * |------------------|----------------|------------|-----------------------------------------|
 * | `entriesLimit`   | `entriesLimit` | `20`       | Max rows shown in recent-entry tables   |
 * | `syncFrequency`  | `syncFrequency`| `"manual"` | Auto-sync interval                      |
 * | `lastSyncedAt`   | `lastSyncedAt` | `null`     | ISO 8601 timestamp of the last sync     |
 *
 * All `save*` functions update the in-memory store first so the UI reflects
 * the change immediately, then flush to disk asynchronously.
 */
import { Store } from "@tauri-apps/plugin-store";
import { writable } from "svelte/store";

/** The three valid auto-sync frequency values. */
export type SyncFrequency = "manual" | "30min" | "1hour";

/**
 * Maximum number of recent entries shown in the Time Tracking and Manual
 * Entry views.  Controlled by the "Recent Entries Limit" setting.
 */
export const entriesLimit = writable<number>(20);

/**
 * How often the app should automatically sync plans and tasks from Microsoft
 * Graph in the background.  `"manual"` disables automatic sync entirely.
 */
export const syncFrequency = writable<SyncFrequency>("manual");

/**
 * ISO 8601 timestamp of the most recent successful sync, or `null` if no sync
 * has been performed in this session.  Displayed as "Last synced: …" in the
 * Settings view.
 */
export const lastSyncedAt = writable<string | null>(null);

/** Cached `tauri-plugin-store` handle; initialised lazily on first access. */
let _store: Store | null = null;

/**
 * Returns the shared `Store` instance, opening `config.json` on first call.
 */
async function getStore(): Promise<Store> {
  if (!_store) _store = await Store.load("config.json");
  return _store;
}

/**
 * Reads all settings from `config.json` and hydrates the stores.
 *
 * Must be called once during app startup (inside `onMount` in `+page.svelte`).
 * Missing keys fall back to their declared defaults rather than throwing.
 */
export async function loadSettings(): Promise<void> {
  const store = await getStore();
  entriesLimit.set((await store.get<number>("entriesLimit")) ?? 20);
  syncFrequency.set(
    (await store.get<SyncFrequency>("syncFrequency")) ?? "manual",
  );
  lastSyncedAt.set((await store.get<string>("lastSyncedAt")) ?? null);
}

/**
 * Updates `entriesLimit` in-memory and persists it to `config.json`.
 *
 * @param value - The new maximum number of recent entries to display.
 *   Must be a positive integer; validation is the caller's responsibility.
 */
export async function saveEntriesLimit(value: number): Promise<void> {
  entriesLimit.set(value);
  const store = await getStore();
  await store.set("entriesLimit", value);
  await store.save();
}

/**
 * Updates `syncFrequency` in-memory and persists it to `config.json`.
 *
 * @param value - The new auto-sync interval.
 */
export async function saveSyncFrequency(value: SyncFrequency): Promise<void> {
  syncFrequency.set(value);
  const store = await getStore();
  await store.set("syncFrequency", value);
  await store.save();
}

/**
 * Updates `lastSyncedAt` in-memory and persists it to `config.json`.
 *
 * Called by `syncAndLoad()` in the planner store and by the "Sync Now" button
 * in the Settings view after a successful sync.
 *
 * @param value - ISO 8601 UTC timestamp of the sync completion time.
 */
export async function saveLastSyncedAt(value: string): Promise<void> {
  lastSyncedAt.set(value);
  const store = await getStore();
  await store.set("lastSyncedAt", value);
  await store.save();
}
