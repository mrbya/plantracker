---
id: TASK-64
title: 'Storybook stories: Spinner'
status: To Do
assignee: []
created_date: '2026-03-25 14:49'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 10200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/Spinner.stories.svelte` covering `src/lib/components/ui/Spinner.svelte`.

## Stories to write

| Story name | Props |
|---|---|
| `Medium` | `size="md"` (default, used in content areas) |
| `Small` | `size="sm"` (used inline inside buttons) |
| `InsideButton` | Render a `Button` with `loading={true}` to show `sm` spinner in its natural context |

## Notes
- Story title: `PlanTracker/UI/Spinner`.
- The spinner uses `--border` for the track and `--accent` for the rotating arc — both visible only with the theme decorator from TASK-62.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Both size stories render the correct pixel dimensions (12px sm, 18px md)
- [ ] #2 Spinner animates continuously in the canvas
- [ ] #3 InsideButton story shows the spinner rendered inside a disabled Button
<!-- AC:END -->
