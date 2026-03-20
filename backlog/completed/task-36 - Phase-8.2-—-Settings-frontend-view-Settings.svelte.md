---
id: TASK-36
title: Phase 8.2 — Settings frontend view (Settings.svelte)
status: Done
assignee: []
created_date: '2026-03-19 14:34'
updated_date: '2026-03-19 14:52'
labels:
  - frontend
  - typescript
  - svelte
milestone: Phase 8
dependencies:
  - TASK-35
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the full Settings view with persistent settings storage via `tauri-plugin-store`, and wire the `entriesLimit` setting into the existing views.

## 1. Install frontend package

```bash
pnpm add @tauri-apps/plugin-store
```

## 2. Settings store (`src/lib/stores/settings.ts`)

Create a reactive settings module backed by `tauri-plugin-store`:

```typescript
import { Store } from '@tauri-apps/plugin-store';
import { writable } from 'svelte/store';

export type SyncFrequency = 'manual' | '30min' | '1hour';

export const entriesLimit = writable<number>(20);
export const syncFrequency = writable<SyncFrequency>('manual');
export const lastSyncedAt = writable<string | null>(null);

let _store: Store | null = null;

async function getStore(): Promise<Store> {
  if (!_store) _store = await Store.load('config.json');
  return _store;
}

export async function loadSettings(): Promise<void> {
  const store = await getStore();
  entriesLimit.set((await store.get<number>('entriesLimit')) ?? 20);
  syncFrequency.set((await store.get<SyncFrequency>('syncFrequency')) ?? 'manual');
  lastSyncedAt.set((await store.get<string>('lastSyncedAt')) ?? null);
}

export async function saveEntriesLimit(value: number): Promise<void> {
  entriesLimit.set(value);
  const store = await getStore();
  await store.set('entriesLimit', value);
  await store.save();
}

export async function saveSyncFrequency(value: SyncFrequency): Promise<void> {
  syncFrequency.set(value);
  const store = await getStore();
  await store.set('syncFrequency', value);
  await store.save();
}

export async function saveLastSyncedAt(value: string): Promise<void> {
  lastSyncedAt.set(value);
  const store = await getStore();
  await store.set('lastSyncedAt', value);
  await store.save();
}
```

## 3. Settings.svelte

Replace the stub. Sections:

### Entries section
- Label: "Recent Entries Limit"
- `<Input type="number">` bound to a local copy of `$entriesLimit`
- Saves on blur/change via `saveEntriesLimit()`

### Sync section
- Label: "Sync Frequency"
- `<Select>` with options: `[{value:'manual',label:'Manual only'},{value:'30min',label:'Every 30 minutes'},{value:'1hour',label:'Every hour'}]`
- Saves on change via `saveSyncFrequency()`
- "Sync Now" button (variant="primary") — calls `syncAndLoad()` from planner store, then saves `lastSyncedAt` = `new Date().toISOString()` via `saveLastSyncedAt()`
- Last synced timestamp displayed below: `"Last synced: Jan 19, 2:34 PM"` (use `toLocaleString`) or `"Never"` if null

### Data section
- Label: "Data Directory"
- Read-only text display of the path (loaded via `getDataDir()` on mount)
- "Open Folder" button (variant="ghost") — calls `open(dataDir)` from `@tauri-apps/plugin-opener`

### Account section
- User display name from `$authStatus` store
- "Sign Out" button (variant="danger") — calls `logout()` from auth store

## 4. Wire `entriesLimit` into views

In `TimeTracking.svelte` and `ManualEntry.svelte`, replace the hardcoded `limit: 20` in `getRecentEntries()` with `$entriesLimit` from the settings store:

```typescript
import { entriesLimit } from '$lib/stores/settings';
// ...
entries = await getRecentEntries({
  taskId: selectedTaskId || undefined,
  planId: !selectedTaskId && selectedPlanId ? selectedPlanId : undefined,
  limit: $entriesLimit,
});
```

## 5. Call `loadSettings()` on startup

In `src/routes/+page.svelte` `onMount`, add `await loadSettings()` alongside the existing `initAuth()` and `initTimer()` calls.

## 6. Save `lastSyncedAt` after sync

In `src/lib/stores/planner.ts`, after a successful `syncPlansAndTasks()` call, import and call `saveLastSyncedAt(new Date().toISOString())`.

## Notes
- Load `dataDir` on mount inside Settings.svelte, not at store level — it requires the app handle and does not need to be globally reactive.
- The "Sync Now" button should show a loading state and disable during the sync.
- Settings sections should be visually separated (use `<hr>` or top-border on each section heading).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Settings persist across app restarts (entriesLimit and syncFrequency survive reload)
- [x] #2 Recent entries limit controls how many entries appear in TimeTracking and ManualEntry
- [x] #3 Sync Now button triggers a sync and updates the last synced timestamp
- [x] #4 Data directory path displays correctly and Open Folder button opens it in the file manager
- [x] #5 Account section shows the signed-in user name with a working Sign Out button
- [x] #6 loadSettings() is called on app startup in +page.svelte onMount
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Installed `@tauri-apps/plugin-store 2.4.2`. Created `src/lib/stores/settings.ts` with `entriesLimit`, `syncFrequency`, `lastSyncedAt` writable stores backed by `Store.load('config.json')`, and `loadSettings` / `saveEntriesLimit` / `saveSyncFrequency` / `saveLastSyncedAt` helpers. Implemented full `Settings.svelte` with four sections: Entries (limit input, saves on blur), Sync (frequency select + Sync Now button with loading state + last synced timestamp), Storage (data directory path + Open Folder via `@tauri-apps/plugin-opener`), Account (user display name + Sign Out). Wired `$entriesLimit` into `TimeTracking.svelte` and `ManualEntry.svelte` (replacing hardcoded 20). Added `saveLastSyncedAt` call in `planner.ts` after successful sync. Added `loadSettings()` to `+page.svelte` onMount. Added `onblur` prop forwarding to `Input.svelte`.
<!-- SECTION:FINAL_SUMMARY:END -->
