---
id: TASK-71
title: 'Storybook stories: ToastContainer'
status: Done
assignee: []
created_date: '2026-03-25 14:49'
updated_date: '2026-03-25 15:01'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 10900
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/ToastContainer.stories.svelte` covering `src/lib/components/ToastContainer.svelte`.

`ToastContainer` reads directly from the `notifications` store, so stories need to seed the store with fixture data rather than passing props.

## Approach

Create a thin story wrapper that:
1. Imports the `notifications` writable store.
2. Manually sets its value to an array of fixture `Notification` objects before rendering `ToastContainer`.
3. Resets the store in a cleanup function or via a Storybook `play` function.

## Stories to write

| Story name | Store seed |
|---|---|
| `SuccessToast` | One `{ id, type: 'success', message: 'Entry saved.' }` |
| `ErrorToast` | One `{ id, type: 'error', message: 'Sync failed: 401 Unauthorized' }` |
| `WarningToast` | One `{ id, type: 'warning', message: 'Timer was restored from a previous session.' }` |
| `MultipleToasts` | Three toasts (one of each type) stacked |
| `Empty` | Store empty — renders nothing (documents the absence state) |

## Notes
- Story title: `PlanTracker/Components/ToastContainer`.
- Auto-dismiss timer should be disabled or suspended for story viewing (either pause it or just not call `addSuccess`/`addError` helpers — write directly to the store).
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Success/error/warning dots render in the correct Catppuccin colours
- [x] #2 MultipleToasts shows all three stacked in bottom-right corner
- [x] #3 Empty story renders without errors and shows nothing
- [x] #4 Toasts do not auto-dismiss during story viewing
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/stories/ToastContainer.stories.svelte` with SuccessToast, ErrorToast, WarningToast, MultipleToasts, and Empty stories. Each story seeds the `notifications` writable store directly (bypassing the 4-second auto-dismiss) so toasts remain visible for inspection.
<!-- SECTION:FINAL_SUMMARY:END -->
