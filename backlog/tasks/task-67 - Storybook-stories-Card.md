---
id: TASK-67
title: 'Storybook stories: Card'
status: Done
assignee: []
created_date: '2026-03-25 14:49'
updated_date: '2026-03-25 14:55'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 10500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/Card.stories.svelte` covering `src/lib/components/ui/Card.svelte`.

## Stories to write

| Story name | Props | Slot content |
|---|---|---|
| `WithTitle` | `title="Active Timer"` | A short paragraph of body text |
| `WithoutTitle` | _(no title)_ | A short paragraph of body text |
| `WithComplexContent` | `title="Entries"` | A small table or list to show the card in a realistic layout |

## Notes
- Story title: `PlanTracker/UI/Card`.
- `title` should be a Storybook arg.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 WithTitle renders a header row separated from the body by a border
- [x] #2 WithoutTitle renders body only with no header
- [x] #3 title arg is editable in controls panel
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/stories/Card.stories.svelte` with WithTitle, WithoutTitle, and WithComplexContent (table of entries) stories. The `title` arg is editable in the controls panel.
<!-- SECTION:FINAL_SUMMARY:END -->
