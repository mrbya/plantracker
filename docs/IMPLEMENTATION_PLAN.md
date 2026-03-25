# PlanTracker — Implementation Plan

> Structured step-by-step plan for code-generation agents.
> Each phase is self-contained. Complete and verify each phase before starting the next.
>
> Detailed phase specifications live in `docs/phases/`. This file is a navigable index.

---

## Conventions

- **Frontend**: Svelte 5 + TypeScript, located in `src/`
- **Backend**: Rust + Tauri 2, located in `src-tauri/`
- **DB**: SQLite accessed via `sqlx` with compile-time checked queries
- **Commands**: Tauri `#[tauri::command]` functions bridge frontend ↔ Rust
- **Stores**: Svelte stores hold reactive UI state; backend is source of truth
- **Theme**: All colors from Catppuccin Mocha palette via CSS custom properties
- **Font**: JetBrains Mono Nerd Font throughout — bundled in `static/fonts/`, loaded via `@font-face` in `src/lib/theme/fonts.css`; no system install required

---

## Phases

| Phase | Title | Detail |
|---|---|---|
| 0 | Project Scaffold | [phase-00-scaffold.md](phases/phase-00-scaffold.md) |
| 1 | Theme & Design System | [phase-01-theme.md](phases/phase-01-theme.md) |
| 2 | Database Layer | [phase-02-database.md](phases/phase-02-database.md) |
| 3 | Authentication | [phase-03-auth.md](phases/phase-03-auth.md) |
| 4 | Microsoft Graph Integration | [phase-04-graph.md](phases/phase-04-graph.md) |
| 5 | Time Tracking View | [phase-05-time-tracking.md](phases/phase-05-time-tracking.md) |
| 6 | Manual Entry View | [phase-06-manual-entry.md](phases/phase-06-manual-entry.md) |
| 7 | Reports View | [phase-07-reports.md](phases/phase-07-reports.md) |
| 8 | Settings & Polish | [phase-08-settings.md](phases/phase-08-settings.md) |
| 9 | Build & Distribution | [phase-09-build.md](phases/phase-09-build.md) |
| 10 | Testing | [phase-10-testing.md](phases/phase-10-testing.md) |
| 11 | Dark / Light Theme Toggle | [phase-11-theme-toggle.md](phases/phase-11-theme-toggle.md) |
| 12 | Quality of Life | [phase-12-quality-of-life.md](phases/phase-12-quality-of-life.md) |

---

## Phase Summaries

**Phase 0 — Project Scaffold**
Initialize the Tauri + Svelte project, install all Rust and JS dependencies, configure environment variables and Tauri capabilities.

**Phase 1 — Theme & Design System**
Define Catppuccin Mocha CSS variables and semantic aliases, create global reset styles, implement base UI components (`Button`, `Select`, `Input`, `Card`, `Badge`, `Spinner`, `EmptyState`), and build the sidebar layout shell.

**Phase 2 — Database Layer**
Write SQLite migrations, implement `sqlx` repository functions for plans, tasks, and time entries, configure the offline query cache, and wire the pool into Tauri managed state.

**Phase 3 — Authentication**
Implement OAuth 2.0 PKCE flow via system browser and `tauri-plugin-oauth`, store tokens in the OS keychain, build the `AuthManager` with silent refresh, and create the Login view.

**Phase 4 — Microsoft Graph Integration**
Build the `GraphClient` with pagination and 401 retry, implement plan/task/user fetching from Microsoft Graph, add the sync command, and populate the `planner` frontend store.

**Phase 5 — Time Tracking View**
Implement the `start_timer`/`stop_timer` commands with in-memory `ActiveTimer` state, build the timer store with live elapsed-seconds counting, and create the Time Tracking view with plan/task dropdowns and recent entries table.

**Phase 6 — Manual Entry View**
Add `create_manual_entry`, `update_entry`, and `delete_entry` commands, and build the Manual Entry view with form validation and an inline edit flow.

**Phase 7 — Reports View**
Implement monthly aggregation queries grouped by `strftime('%Y-%m', start_time)`, CSV export via `tauri-plugin-dialog`, and the Reports view with date range controls and a totals table.

**Phase 8 — Settings & Polish**
Create the Settings view (entries limit, sync frequency, account info), implement the app initialization sequence, add toast notifications, and handle loading/error states throughout the app.

**Phase 9 — Build & Distribution**
Configure production builds, generate app icons, set up the GitLab CI pipeline with Linux and Windows cross-compilation jobs, and validate the sqlx offline query cache in CI.

**Phase 10 — Testing**
Add Rust in-memory test infrastructure (`test_pool` helper), write DB layer and command unit tests, set up Vitest for the frontend, write utility and API wrapper tests, and integrate both suites into the justfile and CI pipeline.

**Phase 11 — Dark / Light Theme Toggle**
Create `latte.css` with Catppuccin Latte variables scoped to `:root.theme-light`, add a `theme` store with OS-preference detection and `config.json` persistence, and add a Theme selector to the Settings view.

**Phase 12 — Quality of Life**
Replace plan/task `<Select>` dropdowns with a `SearchableSelect` combobox in TimeTracking, ManualEntry, and Reports views, enabling live filtering of long lists without any backend changes.

---

## Appendices

Reference material and shared contracts — see [docs/phases/appendices.md](phases/appendices.md).

| Appendix | Content |
|---|---|
| A | Tauri command registration (`main.rs` handler list) |
| B | Frontend ↔ Backend invoke contract and typed wrapper pattern |
| C | TypeScript type definitions mirroring Rust structs |

---

## Implementation Order Summary

| Phase | Deliverable | Complexity |
|---|---|---|
| 0 | Project scaffold | Low |
| 1 | Theme & components | Medium |
| 2 | Database layer | Medium |
| 3 | Authentication | High |
| 4 | Graph API sync | Medium |
| 5 | Time tracking | Medium |
| 6 | Manual entry | Low |
| 7 | Reports | Medium |
| 8 | Settings & polish | Medium |
| 9 | Build & CI | Low |
| 10 | Testing | Medium |
| 11 | Dark / Light Theme Toggle | Low |
| 12 | Quality of Life | Medium |
