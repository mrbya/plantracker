---
id: TASK-65
title: 'Storybook stories: Badge'
status: To Do
assignee: []
created_date: '2026-03-25 14:49'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 10300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/Badge.stories.svelte` covering `src/lib/components/ui/Badge.svelte`.

## Stories to write

| Story name | Props | Slot |
|---|---|---|
| `Running` | `color="green"` | "Running" |
| `Stopped` | `color="red"` | "Stopped" |
| `Warning` | `color="yellow"` | "Warning" |
| `AllColors` | — | Render all three side-by-side |

## Notes
- Story title: `PlanTracker/UI/Badge`.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Green, red, and yellow badge variants all render with the correct Catppuccin tinted background and border
- [ ] #2 AllColors story shows all three variants side-by-side
<!-- AC:END -->
