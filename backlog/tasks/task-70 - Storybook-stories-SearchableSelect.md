---
id: TASK-70
title: 'Storybook stories: SearchableSelect'
status: To Do
assignee: []
created_date: '2026-03-25 14:49'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 10800
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/SearchableSelect.stories.svelte` covering `src/lib/components/ui/SearchableSelect.svelte`.

## Stories to write

| Story name | Props / initial state |
|---|---|
| `Idle` | `placeholder="Select a task…"`, 5 options, no value selected |
| `PreSelected` | Same options, `value` set to one option |
| `WithSentinelOption` | Options include `{ value: "", label: "No specific task" }` as first item; demonstrates it is always shown in filter results |
| `Disabled` | `disabled={true}` |
| `ManyOptions` | 20+ options to show scrollable dropdown and filter behaviour |
| `NoResults` | Pre-set `query` via play function or story setup so the dropdown opens with no matching items, showing the "No results" fallback |

## Notes
- Story title: `PlanTracker/UI/SearchableSelect`.
- The `clickOutside` use-action requires a real DOM — Storybook's canvas provides this, so no mocking needed.
- Use `play` functions (from `@storybook/test`) for the `NoResults` story: focus the input, type a non-matching string.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Idle story shows closed combobox with placeholder text
- [ ] #2 Keyboard navigation (ArrowDown, Enter, Escape) works in the canvas
- [ ] #3 Disabled story shows greyed-out input that cannot be opened
- [ ] #4 NoResults story shows the 'No results' fallback list item
- [ ] #5 Sentinel option (value='') is never filtered out when typing
<!-- AC:END -->
