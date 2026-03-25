---
id: TASK-68
title: 'Storybook stories: Input'
status: To Do
assignee: []
created_date: '2026-03-25 14:49'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 10600
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/Input.stories.svelte` covering `src/lib/components/ui/Input.svelte`.

## Stories to write

| Story name | Props |
|---|---|
| `Default` | `label="Notes"`, no error |
| `WithError` | `label="Duration"`, `error="Must be a positive number"` |
| `NumberType` | `type="number"`, `label="Entries limit"` |
| `DateTimeLocal` | `type="datetime-local"`, `label="Start time"` |
| `NoLabel` | No `label` prop |

## Notes
- Story title: `PlanTracker/UI/Input`.
- `label`, `error`, and `type` should all be Storybook args.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 WithError story renders a red border and error message below the input
- [ ] #2 Label story shows label above input with correct for/id wiring
- [ ] #3 All args are editable in the controls panel
<!-- AC:END -->
