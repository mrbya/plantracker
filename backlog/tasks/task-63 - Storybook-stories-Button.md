---
id: TASK-63
title: 'Storybook stories: Button'
status: To Do
assignee: []
created_date: '2026-03-25 14:48'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 10100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/Button.stories.svelte` covering the app's `src/lib/components/ui/Button.svelte` component.

## Stories to write

| Story name | Props |
|---|---|
| `Primary` | `variant="primary"`, label text in slot |
| `Ghost` | `variant="ghost"` |
| `Danger` | `variant="danger"` |
| `Success` | `variant="success"` |
| `Loading` | `variant="primary"`, `loading={true}` — spinner visible, button non-interactive |
| `Disabled` | `variant="primary"`, `disabled={true}` |
| `AllVariants` | Render all four variants side-by-side for a visual overview |

## Notes
- Use `@storybook/addon-svelte-csf` `defineMeta` / `Story` pattern (same as existing stories).
- Story title: `PlanTracker/UI/Button`.
- `onclick` arg should use `fn()` so interactions panel captures clicks.
- Depends on TASK-62 (theme decorator) being in place.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All 7 stories render without errors
- [ ] #2 Loading story shows spinner and button is non-interactive
- [ ] #3 Disabled story shows reduced opacity and not-allowed cursor
- [ ] #4 All four variant colours match the Catppuccin Mocha palette
<!-- AC:END -->
