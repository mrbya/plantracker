# Store layer

All reactive state is owned by the six stores in `src/lib/stores/`. Stores are the only modules allowed to call `src/lib/api/index.ts`. Views subscribe to store exports and call store actions; they never fetch data themselves.

---

## Store inventory

### `auth.ts`

**File:** `src/lib/stores/auth.ts`

**Owns:**
- `authStatus` (private `writable<AuthStatus | null>`) — the raw response from `get_auth_status` / `login`.

**Derives:**
- `isAuthenticated: Readable<boolean>` — `true` while a valid or refreshable token exists.
- `userDisplayName: Readable<string | null>` — the Microsoft Graph display name, or `null`.

**Exports (actions):**
- `initAuth()` — call once on startup; reads keychain state and triggers `syncAndLoad()` if authenticated.
- `login()` — opens the OAuth PKCE browser flow, updates the store, triggers `syncAndLoad()` on success.
- `logout()` — clears backend tokens, resets store to unauthenticated.

**Dependencies:**
- `src/lib/api` — `getAuthStatus`, `login`, `logout`
- `src/lib/stores/planner` — calls `syncAndLoad()` after successful auth
- `src/lib/stores/notifications` — surfaces errors as toasts

**Persistence:** none. The token itself is in the OS keychain (managed by Rust). The store is ephemerally populated on startup by `initAuth`.

**Consumed by:** `+page.svelte` (renders `Layout` vs `Login`), `Layout.svelte` (avatar initials), `Settings.svelte` (display name).

---

### `planner.ts`

**File:** `src/lib/stores/planner.ts`

**Owns:**
- `plans: Writable<Plan[]>` — all plans cached from the last SQLite load.
- `tasksByPlan: Writable<Record<string, Task[]>>` — tasks keyed by local plan UUID.
- `selectedPlan: Writable<Plan | null>` — the plan currently active in the UI.
- `selectedTask: Writable<Task | null>` — the task currently active in the UI.

**Derives:** nothing (all four are plain writables).

**Exports (actions):**
- `selectTask(task)` — sets `selectedTask` and also updates `selectedPlan` to the task's parent. Always use this instead of writing `selectedTask` directly to maintain the invariant that `selectedPlan.id === selectedTask.planId`.
- `syncAndLoad()` — calls `syncPlansAndTasks` (Graph sync), then reloads `plans` and `tasksByPlan` from SQLite. Shows a success or error toast. Continues to the local-load phase even if the Graph sync fails.

**Dependencies:**
- `src/lib/api` — `syncPlansAndTasks`, `listPlans`, `listTasksForPlan`
- `src/lib/stores/settings` — calls `saveLastSyncedAt` after a successful sync
- `src/lib/stores/notifications` — success/error toasts

**Persistence:** none at the store level. The underlying data lives in SQLite; the stores are an in-session cache.

**Consumed by:** `TimeTracking.svelte`, `ManualEntry.svelte`, `Reports.svelte` (plan/task dropdowns and entries loading).

**Important invariant:** `selectedTask.planId` must always match `selectedPlan.id`. The `selectTask()` export maintains this. Never call `selectedTask.set()` from a view without also updating `selectedPlan`.

---

### `timer.ts`

**File:** `src/lib/stores/timer.ts`

**Owns:**
- `activeEntry` (private `writable<TimeEntry | null>`) — the open `time_entries` row, or `null`.
- `elapsedSeconds: Writable<number>` — incremented every second by `setInterval` while running.
- An internal `intervalId` for the tick loop (module-level `let`, not a store).

**Derives:**
- `isRunning: Readable<boolean>` — `true` when `activeEntry !== null`.

**Exports (actions):**
- `initTimer()` — call once on startup; queries `getActiveTimer()` and restores state + tick loop if an open entry exists.
- `start(planId, taskId?)` — calls `startTimer`, sets `activeEntry`, resets `elapsedSeconds` to 0, starts the tick loop.
- `stop()` — calls `stopTimer`, stops the tick loop, clears `activeEntry`, returns the completed `TimeEntry`.

Both `start` and `stop` re-throw the backend error after surfacing it as a toast. Callers should handle the rejected promise to reset their local `busy` flag.

**Dependencies:**
- `src/lib/api` — `getActiveTimer`, `startTimer`, `stopTimer`
- `src/lib/stores/notifications` — error toasts

**Persistence:** none at the store level. Timer state is persisted in SQLite (`end_time = NULL`); `initTimer` restores it from there on startup.

**Consumed by:** `TimeTracking.svelte` (start/stop button, elapsed display).

**Side effect:** the `setInterval` tick loop runs as long as `activeEntry` is non-null. It is started inside `start()` and `initTimer()`, stopped inside `stop()`. The loop only increments `elapsedSeconds`; it never makes API calls.

---

### `notifications.ts`

**File:** `src/lib/stores/notifications.ts`

**Owns:**
- `notifications: Writable<Notification[]>` — the current queue of active toasts.

**Derives:** nothing.

**Exports (actions):**
- `addSuccess(msg)` — appends a green toast, auto-removes after 4 seconds.
- `addError(msg)` — appends a red toast, auto-removes after 4 seconds.
- `addWarning(msg)` — appends a yellow toast, auto-removes after 4 seconds.

**Dependencies:** none. This store is a leaf — nothing it does requires another store or the API layer.

**Persistence:** none. Toasts are ephemeral.

**Consumed by:** all other stores (error/success feedback), `ToastContainer.svelte` (rendering).

**Implementation note:** each toast gets an auto-incrementing integer `id`. The `setTimeout` dismiss callback filters by that `id`, so concurrent toasts do not interfere.

---

### `theme.ts`

**File:** `src/lib/stores/theme.ts`

**Owns:**
- `themeChoice: Writable<ThemeChoice>` — one of `"dark"`, `"light"`, or `"system"`. Defaults to `"system"`.

**Derives:**
- `resolvedTheme: Readable<"dark" | "light">` — resolves `"system"` to the actual OS preference via `window.matchMedia`.

**Exports:**
- `loadTheme()` — reads from `config.json` and hydrates `themeChoice`. Call once on startup.
- `saveTheme(value)` — sets `themeChoice` and persists to `config.json`.

**Dependencies:** `@tauri-apps/plugin-store` (via the lazy `getStore()` singleton).

**Persistence:** `config.json` via `tauri-plugin-store`, key `"theme"`.

**Side effect:** `resolvedTheme.subscribe` runs on every change and toggles the `theme-light` class on `document.documentElement`. This is a top-level module-scope subscription that fires as soon as the module is imported. The CSS override rules for the light theme live in `src/lib/theme/mocha.css` under `.theme-light`.

**Consumed by:** `Settings.svelte` (theme selector), `+page.svelte` (OS preference change listener).

---

### `settings.ts`

**File:** `src/lib/stores/settings.ts`

**Owns:**
- `entriesLimit: Writable<number>` — maximum rows shown in recent-entry tables. Default: `20`.
- `syncFrequency: Writable<SyncFrequency>` — auto-sync interval (`"manual"`, `"30min"`, `"1hour"`). Default: `"manual"`.
- `lastSyncedAt: Writable<string | null>` — ISO 8601 timestamp of the last successful sync.

**Derives:** nothing.

**Exports:**
- `loadSettings()` — reads all three keys from `config.json`. Call once on startup.
- `saveEntriesLimit(value)` — updates store and persists.
- `saveSyncFrequency(value)` — updates store and persists.
- `saveLastSyncedAt(value)` — updates store and persists. Called by `planner.syncAndLoad()` and the Settings "Sync Now" button.

**Dependencies:** `@tauri-apps/plugin-store` (via the same lazy `getStore()` singleton pattern as `theme.ts`; however, the two stores each maintain their own `_store` reference — both point to the same `config.json` file but are not shared instances).

**Persistence:** `config.json` via `tauri-plugin-store`, keys `"entriesLimit"`, `"syncFrequency"`, `"lastSyncedAt"`.

**Consumed by:** `Settings.svelte` (all controls), `TimeTracking.svelte` and `ManualEntry.svelte` (`$entriesLimit`), `planner.ts` (`saveLastSyncedAt`).

---

## Store dependency graph

```
notifications   (leaf — no outbound deps)
settings        (leaf — tauri-plugin-store only)
theme           (leaf — tauri-plugin-store + DOM side effect)
timer           <- api, notifications
planner         <- api, settings, notifications
auth            <- api, planner, notifications
```

Views and Layout are consumers only; they have no outbound store dependencies of their own.

---

## Persistence summary

| Store | Persisted? | Mechanism | Keys |
|---|---|---|---|
| `auth` | No | — | — |
| `planner` | No | SQLite (backend) | — |
| `timer` | No | SQLite (backend) | — |
| `notifications` | No | — | — |
| `theme` | Yes | `tauri-plugin-store` | `theme` |
| `settings` | Yes | `tauri-plugin-store` | `entriesLimit`, `syncFrequency`, `lastSyncedAt` |

"SQLite (backend)" means the data survives app restarts via the backend database; the store itself is just an in-session cache that is rehydrated on startup by `initTimer` / `initAuth` + `syncAndLoad`.

---

## Anti-patterns to avoid

**Do not call `invoke()` inside a store.** Stores must use `src/lib/api/index.ts`. This keeps the IPC surface in one place and makes it testable via `mockIPC`.

**Do not call API functions from views.** Views call store actions. The one exception is `Reports.svelte`, which calls `generateReport` and `exportReportCsv` directly because no store owns report state — this is an acknowledged deviation, acceptable for pure read-only queries that carry no shared state.

**Do not write `selectedTask` without also writing `selectedPlan`.** Use `selectTask()` to maintain the plan/task invariant. Views that update the task dropdown should call `selectTask(task)` from the planner store, not `selectedTask.set(task)`.

**Do not read from `tauri-plugin-store` in views.** All config access must go through `settings.ts` or `theme.ts`. Direct `Store.load` calls outside of the store files are not permitted.

**Do not hold `setInterval` handles in component scope.** Timer ticking is owned by `timer.ts`. If a view needs to display elapsed time it subscribes to `$elapsedSeconds`.

---

## Contributor guidance

**Add a new store when:**
- State needs to survive navigation between views (e.g., if a new feature has a selection that multiple views share).
- Async actions have shared loading/error states that multiple views must reflect.
- A new persistent preference needs to be read and written from more than one place.

**Do not add a new store when:**
- State is only used in one view (form fields, loading flags, inline confirm state).
- State can be derived from existing stores without side effects.
- You only need a utility function — those belong in `src/lib/utils/`.

**Keep state local to a view when:**
- It tracks transient UI interactions: which row is being confirmed for delete, whether a form is in edit mode, whether a button is busy.
- It only exists while the view is mounted and does not need to persist or be shared.
