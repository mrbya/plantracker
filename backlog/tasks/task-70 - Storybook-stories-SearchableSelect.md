---
id: TASK-70
title: 'Storybook stories: SearchableSelect'
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
- [x] #1 Idle story shows closed combobox with placeholder text
- [x] #2 Keyboard navigation (ArrowDown, Enter, Escape) works in the canvas
- [x] #3 Disabled story shows greyed-out input that cannot be opened
- [x] #4 NoResults story shows the 'No results' fallback list item
- [x] #5 Sentinel option (value='') is never filtered out when typing
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/stories/SearchableSelect.stories.svelte` with Idle, PreSelected, WithSentinelOption, Disabled, ManyOptions (22 options), and NoResults stories. The NoResults story uses a `play` function that types a non-matching string ("xyzzy") and asserts the "No results" fallback option is visible.
<!-- SECTION:FINAL_SUMMARY:END -->
