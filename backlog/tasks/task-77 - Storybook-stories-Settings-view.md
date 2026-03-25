---
id: TASK-77
title: 'Storybook stories: Settings view'
status: Done
assignee: []
created_date: '2026-03-25 14:50'
updated_date: '2026-03-25 15:04'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 11500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/views/Settings.stories.svelte` covering `src/views/Settings.svelte`.

## Approach
Mock `invoke('get_settings')` and `invoke('save_settings')`. Seed the `auth` store with a fixture user name for the account section.

## Stories to write

| Story name | State |
|---|---|
| `Default` | Settings loaded with default values; all controls enabled |
| `Saving` | Save button in loading state (simulate slow `invoke('save_settings')`) |
| `ValidationError` | An `Input` field shows an inline error (e.g. entries-limit field with a non-numeric value) |

## Notes
- Story title: `PlanTracker/Views/Settings`.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Default story renders all settings sections without errors
- [x] #2 Saving story shows disabled save button with Spinner
- [x] #3 ValidationError story shows inline error message on the affected Input
- [x] #4 No invoke() errors in Storybook console
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/stories/views/Settings.stories.svelte` with Default, SyncInProgress, and ValidationError stories. The plugin-store and plugin-opener mocks (from TASK-72) handle the Tauri dependencies. ValidationError uses a `play` function to type "-5" into the entries limit field and tab away, triggering the onblur reset.
<!-- SECTION:FINAL_SUMMARY:END -->
