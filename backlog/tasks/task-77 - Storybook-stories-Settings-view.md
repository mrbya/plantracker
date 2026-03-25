---
id: TASK-77
title: 'Storybook stories: Settings view'
status: To Do
assignee: []
created_date: '2026-03-25 14:50'
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
- [ ] #1 Default story renders all settings sections without errors
- [ ] #2 Saving story shows disabled save button with Spinner
- [ ] #3 ValidationError story shows inline error message on the affected Input
- [ ] #4 No invoke() errors in Storybook console
<!-- AC:END -->
