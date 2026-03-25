# Frontend API docs

Generated from the TypeScript modules under `src/lib/` and `src/routes/`.
This section covers types, store interfaces, API wrappers, and utility functions.
Svelte component markup is excluded; for component visual behaviour and usage examples see the Storybook UI docs.

---

## What lives here

| Scope | Location | Docs |
|---|---|---|
| TypeScript types (mirrors Rust structs) | `src/lib/types.ts` | This site |
| Tauri command wrappers | `src/lib/api/index.ts` | This site |
| Reactive stores (auth, planner, timer, ...) | `src/lib/stores/` | This site |
| Utility functions | `src/lib/utils/` | This site |
| Svelte use-actions | `src/lib/actions/` | This site |
| Route config (SPA mode flag) | `src/routes/+layout.ts` | This site |
| Reusable UI components | `src/lib/components/ui/` | Storybook |
| Full view layouts | `src/views/` | Storybook |
| Rust backend (commands, db, auth, Graph) | `src-tauri/src/` | cargo doc |

Storybook and cargo doc are served alongside this site from the top-level `docs-page/` output.

---

## Frontend layout at a glance

```
src/
  app.css                      CSS import chain (fonts → mocha palette → global reset)
  lib/
    api/
      index.ts                 Single invoke boundary — all Tauri IPC calls
    actions/
      clickOutside.ts          Svelte use-action for dropdown dismissal
    components/
      Layout.svelte            App shell: sidebar nav + content area
      ToastContainer.svelte    Fixed-position toast renderer
      ui/                      Headless reusable primitives (Button, Input, Select, ...)
    stores/
      auth.ts                  Auth state, login/logout actions
      notifications.ts         Toast queue
      planner.ts               Plans, tasks, selection state, sync
      settings.ts              User preferences, config persistence
      theme.ts                 Theme choice, HTML class side effect
      timer.ts                 Active timer, tick loop, elapsed seconds
    types.ts                   All shared TypeScript interfaces
    utils/
      datetime.ts              formatDateTime
      duration.ts              formatDuration, formatDurationCSV
  routes/
    +layout.ts                 Disables SSR (SPA mode for Tauri)
    +layout.svelte             Imports global CSS, renders slot
    +page.svelte               App root: startup sequence + auth gate
  views/
    Login.svelte               OAuth sign-in screen
    ManualEntry.svelte         Create and edit time entries
    Reports.svelte             Generate and export reports
    Settings.svelte            User preferences UI
    TimeTracking.svelte        Timer controls and recent entries
```

---

## Key modules

**`src/lib/types.ts`**
Single source of truth for all TypeScript interfaces. Every type that crosses the Tauri IPC boundary is defined here. Field names are `camelCase` to match Rust structs serialised with `#[serde(rename_all = "camelCase")]`.

**`src/lib/api/index.ts`**
The only file allowed to call `invoke()`. Exports one typed async function per Tauri command, grouped by domain (auth, sync, timer, entries, reports, settings). Views and stores import from here.

**`src/lib/stores/`**
Six Svelte writable/derived stores that own all reactive state. Stores are the only consumers of `src/lib/api`. Views bind to store exports and call store actions.

**`src/lib/utils/`**
Pure functions with no side effects. `formatDateTime` and the two `formatDuration` variants cover all date and duration display needs.

---

## Guides

- [Architecture](guides/architecture.md) — end-to-end walkthrough of the frontend structure, data flow, and conventions
- [Stores](guides/stores.md) — per-store reference: ownership, derivations, actions, persistence
- [Tauri bridge](guides/tauri-bridge.md) — the `src/lib/api/index.ts` pattern, type contracts, testing

---

## How to read these docs

TypeDoc expands each entry point into a module page. Navigate by module in the left sidebar.
Within each module, exported functions show their full JSDoc including `@param`, `@returns`, and `@throws`.
Interfaces show field-level documentation.

The guides above are narrative and architecture-focused; the API reference pages are the authoritative source for exact signatures and semantics.
