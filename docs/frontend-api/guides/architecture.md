# Frontend architecture

## Overview

PlanTracker is a Tauri 2 desktop application. The frontend is a SvelteKit project compiled to a static SPA and loaded by the Tauri WebView. There is no server-side rendering and no network routing — `src/routes/+layout.ts` exports `ssr = false`, which in combination with `@sveltejs/adapter-static` tells SvelteKit to produce a single `index.html`.

All view switching is handled client-side inside `Layout.svelte` via an `activeView` state variable. There is only one SvelteKit route (`src/routes/+page.svelte`). The four views (Time Tracking, Manual Entry, Reports, Settings) live in `src/views/` and are rendered conditionally, not routed to.

Backend communication happens exclusively through `src/lib/api/index.ts`, which wraps each Tauri command in a typed async function. Views and stores never call `invoke()` directly.

---

## Route and layout composition

```
src/routes/+layout.ts          exports ssr = false
src/routes/+layout.svelte      imports app.css, renders slot
src/routes/+page.svelte        startup sequence + auth gate
  ├─ (unauthenticated) → src/views/Login.svelte
  └─ (authenticated)   → src/lib/components/Layout.svelte
                              ├─ sidebar nav (icon buttons)
                              └─ main content area
                                   ├─ src/views/TimeTracking.svelte
                                   ├─ src/views/ManualEntry.svelte
                                   ├─ src/views/Reports.svelte
                                   └─ src/views/Settings.svelte
```

`+page.svelte` is the application root. It:
1. Runs `loadTheme()` and `loadSettings()` to hydrate persisted preferences.
2. Calls `initAuth()`, which checks the OS keychain and conditionally triggers `syncAndLoad()`.
3. Calls `initTimer()`, which queries the backend for any open timer row and restores the tick loop.
4. Attaches a `MediaQueryList` listener so the resolved theme stays in sync if the OS colour scheme changes while the app is open.

The `$isAuthenticated` derived store from `src/lib/stores/auth.ts` controls whether `Layout` or `Login` is rendered.

---

## Reusable components vs views

**`src/lib/components/ui/`** contains headless primitive components: `Button`, `Select`, `Input`, `Card`, `Badge`, `Spinner`, `EmptyState`, and `SearchableSelect`. These have no awareness of domain logic, store state, or API calls. They accept props and emit events.

**`src/lib/components/`** contains structural shell components: `Layout` (sidebar + content area) and `ToastContainer` (fixed-position toast renderer). These read from stores but contain no business logic.

**`src/views/`** contains feature views. Each view can read from multiple stores, call store actions, and use any UI primitive. Views own the local UI state (loading flags, form fields, confirm dialogs) that does not need to survive navigation.

The rule: if a piece of state or logic is needed in more than one view, it belongs in a store. If it is specific to one view's rendering, it stays local.

---

## Data flow

```
Backend (SQLite / Graph API)
    ↕ Tauri IPC
src/lib/api/index.ts      (typed wrappers)
    ↕ async calls
src/lib/stores/           (reactive state)
    ↕ Svelte store subscriptions
src/views/ + Layout.svelte (UI rendering)
```

Stores are the only consumers of the API layer. Views subscribe to store exports (`$plans`, `$isRunning`, etc.) and call store actions (`start()`, `stop()`, `syncAndLoad()`). Views never fetch data themselves.

Data that does not change during a session (cached plans and tasks from SQLite) lives in stores and is refreshed only by explicit sync. Transient UI state (which entry is being confirmed for deletion, whether a form is loading) stays local to the view.

---

## Module responsibilities

| Module | Responsibility |
|---|---|
| `src/lib/types.ts` | Defines all interfaces shared between frontend modules and the API layer. Single source of truth for type shapes. |
| `src/lib/api/index.ts` | Wraps every `invoke()` call. No state, no side effects beyond the IPC call itself. |
| `src/lib/stores/auth.ts` | Auth state and actions. Triggers `syncAndLoad` on successful authentication. |
| `src/lib/stores/planner.ts` | Plans, tasks, current selection. Owns the two-phase sync (Graph → SQLite → stores). |
| `src/lib/stores/timer.ts` | Active timer state. Owns the `setInterval` tick loop and the `stop`/`start` actions. |
| `src/lib/stores/notifications.ts` | Toast queue. Self-contained; no external dependencies. |
| `src/lib/stores/theme.ts` | Theme preference. Side-effects the `<html>` class. Persists to `config.json`. |
| `src/lib/stores/settings.ts` | User-configurable preferences. Persists to `config.json`. |
| `src/lib/utils/datetime.ts` | Locale-aware display formatting for ISO timestamps. |
| `src/lib/utils/duration.ts` | Duration formatting for UI (`Xh Ym`) and CSV export (`H:MM:SS`). |
| `src/lib/actions/clickOutside.ts` | Svelte use-action for closing dropdowns. |

---

## Typical feature flow

### Starting a timer

1. The user selects a plan and optionally a task in `TimeTracking.svelte` via `SearchableSelect` dropdowns. These update `selectedPlanId` / `selectedTaskId` local state and the `planner` store.
2. The user clicks "Start Timer". The view calls `start(planId, taskId)` from `src/lib/stores/timer.ts`.
3. `start()` calls `startTimer(planId, taskId)` from `src/lib/api/index.ts`, which calls `invoke('start_timer', ...)`.
4. The backend inserts a `time_entries` row with `end_time = NULL` and returns the new `TimeEntry`.
5. `start()` sets `activeEntry` in the timer store, resets `elapsedSeconds` to 0, and starts a `setInterval` that increments `elapsedSeconds` every second.
6. `$isRunning` (derived from `activeEntry`) becomes `true`. The view re-renders showing the "Stop — Xh Ym" button with the live elapsed time.

### Generating a report

1. The user selects a date range and optionally a plan/task in `Reports.svelte`, then clicks "Generate".
2. The view calls `generateReport(params)` from `src/lib/api/index.ts` directly (reports are a read-only query; no store owns report state).
3. The backend queries SQLite for matching completed entries and returns a `ReportResult`.
4. The view stores the result in local `$state` and renders the table. The `Export CSV` button becomes enabled.
5. If the user clicks "Export CSV", the view calls `exportReportCsv(report)`. The backend opens a native save-file dialog and writes the file. If the user cancels, the backend throws an error containing `"Export cancelled"`, which the view detects and silently swallows.

---

## Conventions

**`undefined` → `null` at the IPC boundary.**
Tauri serialises TypeScript `undefined` as JSON `null`, but for clarity all optional arguments are explicitly coerced to `null` before the `invoke` call (`taskId: taskId ?? null`). Rust command signatures use `Option<String>` for these parameters.

**Stores call the API; views call stores.**
A view should never import from `src/lib/api/index.ts` directly (with the exception of `Reports.svelte`, which fetches report data locally because no store owns report state — this is the one sanctioned deviation from the pattern).

**Backend as source of truth.**
Plans, tasks, and time entries are stored in SQLite by the backend. The frontend caches them in stores only for the current session. On next launch, `initAuth` triggers `syncAndLoad` to repopulate from SQLite.

**Config persistence is frontend-only.**
Theme choice, entries limit, and sync frequency are stored in `config.json` via `tauri-plugin-store`. These are UI preferences with no counterpart in the Rust backend.

**SvelteKit is the build tool, not the router.**
`src/routes/` contains exactly one page. Do not add additional SvelteKit routes for new views; add a new view component to `src/views/` and a new entry in `Layout.svelte`'s `navItems` array.

---

## Where to put new code

| What you are adding | Where it goes |
|---|---|
| A new Tauri command result type | `src/lib/types.ts` |
| A wrapper for a new Tauri command | `src/lib/api/index.ts` |
| Reactive state shared between views | New or existing store in `src/lib/stores/` |
| Local UI state (loading, form fields) | `$state` inside the view |
| A new full-page view | `src/views/NewView.svelte`, registered in `Layout.svelte` |
| A reusable stateless UI primitive | `src/lib/components/ui/` |
| A pure display-formatting function | `src/lib/utils/` |
| A Svelte use-action | `src/lib/actions/` |
