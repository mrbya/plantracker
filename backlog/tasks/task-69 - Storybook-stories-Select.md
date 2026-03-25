---
id: TASK-69
title: 'Storybook stories: Select'
status: Done
assignee: []
created_date: '2026-03-25 14:49'
updated_date: '2026-03-25 14:56'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 10700
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/Select.stories.svelte` covering `src/lib/components/ui/Select.svelte`.

## Stories to write

| Story name | Props |
|---|---|
| `WithPlaceholder` | `placeholder="Select a plan…"`, 3–4 options, no initial value |
| `PreSelected` | Same options, `value` set to one of them |
| `ManyOptions` | 10+ options to show scroll/overflow behaviour |

## Notes
- Story title: `PlanTracker/UI/Select`.
- Use realistic plan/task names as option labels.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Placeholder option renders as disabled/hidden when a real value is selected
- [x] #2 PreSelected story shows correct option highlighted on render
- [x] #3 Custom chevron icon visible in all stories
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/stories/Select.stories.svelte` with WithPlaceholder, PreSelected, and ManyOptions (12 options) stories. Uses realistic plan names as option labels.
<!-- SECTION:FINAL_SUMMARY:END -->
