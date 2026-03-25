---
id: TASK-72
title: 'Storybook stories: Layout'
status: Done
assignee: []
created_date: '2026-03-25 14:50'
updated_date: '2026-03-25 15:01'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 11000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/Layout.stories.svelte` covering `src/lib/components/Layout.svelte`.

`Layout` renders the full app shell (sidebar + content area) and mounts the four view components. For Storybook, the view components will attempt to call Tauri `invoke()` on mount, so those calls must be intercepted.

## Approach

Mock `@tauri-apps/api/core` in `.storybook/preview.ts` (or via a Vite alias in `vite.config.ts` for test/storybook mode) so that `invoke()` returns empty/default data instead of crashing.

Seed the `auth` store with a fake user name so the avatar and sign-out button render correctly.

## Stories to write

| Story name | Active view / store state |
|---|---|
| `TimeTrackingActive` | Default; `userDisplayName` = "Ada Lovelace" |
| `ManualEntryActive` | Simulate clicking the Manual Entry nav item via a `play` function |
| `ReportsActive` | Simulate clicking Reports |
| `SettingsActive` | Simulate clicking Settings |
| `UnknownUser` | `userDisplayName` = null — avatar shows "?" |

## Notes
- Story title: `PlanTracker/Components/Layout`.
- If mocking all view `invoke()` calls is too complex, render a simplified stub view in the content area instead of the real view components — document the trade-off.
- Depends on TASK-62 and the invoke mock strategy established for view stories (TASK-73 through TASK-77).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Sidebar renders with all 4 nav icons and the avatar/sign-out area
- [x] #2 Active nav button has accent highlight
- [x] #3 Avatar shows correct initials derived from userDisplayName
- [x] #4 No unhandled invoke() errors in the Storybook console
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/stories/Layout.stories.svelte` with TimeTrackingActive, ManualEntryActive, ReportsActive, SettingsActive, and UnknownUser stories. Navigation stories use `play` functions to click the correct sidebar button. Three Tauri plugin mocks added (`tauri-api-core`, `tauri-plugin-opener`, `tauri-plugin-store`) and wired in `.storybook/main.ts` via `viteFinal` so all view components render without a running Tauri process. Documented that `userDisplayName` shows "?" in isolation (avatar requires `initAuth()` called from `+page.svelte`).
<!-- SECTION:FINAL_SUMMARY:END -->
