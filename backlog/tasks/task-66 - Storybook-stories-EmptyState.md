---
id: TASK-66
title: 'Storybook stories: EmptyState'
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
ordinal: 10400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/EmptyState.stories.svelte` covering `src/lib/components/ui/EmptyState.svelte`.

## Stories to write

| Story name | `message` prop |
|---|---|
| `NoEntries` | "No entries yet. Start a timer to track time." |
| `NoTasks` | "No tasks found for this plan." |
| `NoResults` | "No results match your filter." |

## Notes
- Story title: `PlanTracker/UI/EmptyState`.
- `message` should be an Storybook arg so the controls panel can edit it freely.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Icon and message render centred with muted styling
- [x] #2 message arg is editable in the Storybook controls panel
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/stories/EmptyState.stories.svelte` with NoEntries, NoTasks, and NoResults stories. The `message` prop is exposed as a Storybook arg with a text control so it can be edited freely in the controls panel.
<!-- SECTION:FINAL_SUMMARY:END -->
