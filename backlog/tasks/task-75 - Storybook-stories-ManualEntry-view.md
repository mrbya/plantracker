---
id: TASK-75
title: 'Storybook stories: ManualEntry view'
status: To Do
assignee: []
created_date: '2026-03-25 14:50'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 11300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/views/ManualEntry.stories.svelte` covering `src/views/ManualEntry.svelte`.

## Approach
Seed the `planner` store with fixture plans/tasks. Mock `invoke('create_entry')` and `invoke('update_entry')` to return success without hitting SQLite.

## Stories to write

| Story name | State |
|---|---|
| `EmptyForm` | Plans loaded, no values filled in the form fields |
| `FormFilled` | All fields pre-populated with realistic values (plan, task, start/end datetime, notes) |
| `NoPlans` | `plans = []` — shows empty plan dropdown or EmptyState |
| `Saving` | Form filled, save button in loading state (simulate slow `invoke`) |

## Notes
- Story title: `PlanTracker/Views/ManualEntry`.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 FormFilled story shows all inputs populated with values
- [ ] #2 Saving story shows disabled save button with Spinner
- [ ] #3 No invoke() errors in Storybook console
<!-- AC:END -->
