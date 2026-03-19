---
id: TASK-9
title: Phase 1.4 — Layout shell
status: Done
assignee: []
created_date: '2026-03-18 23:02'
updated_date: '2026-03-19 07:27'
labels: []
milestone: Phase 1 — Theme &amp; Design System
dependencies:
  - TASK-8
references:
  - src/lib/stores/auth.ts
priority: high
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/lib/components/Layout.svelte` — the persistent app shell that wraps all authenticated views.

**Structure:**
- Left sidebar: 64px wide, icon-only vertical navigation
- Main content area: fills remaining horizontal space, scrollable

**Sidebar nav items** (top-aligned, using Nerd Font glyphs):
| Icon glyph | View | Route/identifier |
|---|---|---|
| `󰥔` (clock) | Time Tracking | `time-tracking` |
| `󰃰` (calendar-edit) | Manual Entry | `manual-entry` |
| `󰈈` (chart-bar) | Reports | `reports` |
| `󰒓` (cog) | Settings | `settings` |

**Bottom of sidebar:**
- User avatar or initials display
- Sign-out button

**Active state:** current nav item highlighted with `--accent` color background or left border.

**Navigation:** clicking a nav item updates the active view shown in the main content area. Use a `$state` variable for the active route — no SvelteKit router needed (single-window desktop app).

**`App.svelte` integration:**
- Import `isAuthenticated` from `$lib/stores/auth`
- Conditionally render `<Login>` when not authenticated, `<Layout>` when authenticated
- Each nav item renders the corresponding placeholder view component inside the layout's content slot

Create placeholder view files (`src/views/TimeTracking.svelte`, `ManualEntry.svelte`, `Reports.svelte`, `Settings.svelte`, `Login.svelte`) with minimal content so Layout can render without errors.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Layout.svelte exists with 64px left sidebar and main content area
- [x] #2 Sidebar shows 4 nav items with Nerd Font icons
- [x] #3 Active nav item is visually highlighted with accent color
- [x] #4 Clicking nav items switches the rendered view in main content area
- [x] #5 User info and sign-out button appear at the bottom of the sidebar
- [x] #6 App.svelte renders Login when not authenticated, Layout when authenticated
- [x] #7 All 5 placeholder view files exist in src/views/
- [x] #8 pnpm tsc --noEmit passes after all files are created
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created: src/lib/types.ts (all shared TS types), src/lib/stores/auth.ts (stub with isAuthenticated derived store), all 5 placeholder views in src/views/, src/lib/components/Layout.svelte (64px sidebar with 4 Nerd Font nav icons + avatar + sign-out, $state-based routing), and replaced +page.svelte with auth-gated Login/Layout rendering. Ran svelte-kit sync to register the $lib alias. pnpm tsc --noEmit passes.
<!-- SECTION:FINAL_SUMMARY:END -->
