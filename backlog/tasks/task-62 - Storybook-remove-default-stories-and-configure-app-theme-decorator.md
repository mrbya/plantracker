---
id: TASK-62
title: 'Storybook: remove default stories and configure app theme decorator'
status: To Do
assignee: []
created_date: '2026-03-25 14:48'
labels:
  - storybook
  - docs
dependencies: []
priority: high
ordinal: 10000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The default Storybook scaffold left several demo files in `src/stories/` that are unrelated to the app. Remove them and wire up the Catppuccin Mocha theme so that all subsequent app-component stories render with the correct CSS variables and font.

## Work items

1. **Delete default scaffold files** from `src/stories/`:
   - `Button.svelte`, `Button.stories.svelte`, `button.css`
   - `Header.svelte`, `Header.stories.svelte`, `header.css`
   - `Page.svelte`, `Page.stories.svelte`, `page.css`
   - `Configure.mdx`
   - `assets/` directory

2. **Add a global theme decorator** in `.storybook/preview.ts` (or a shared `src/stories/_decorators.ts`) that:
   - Imports `src/lib/theme/mocha.css` (and `latte.css` if light theme support is desired)
   - Imports `src/lib/theme/fonts.css` and `src/lib/theme/global.css`
   - Sets `background-color: var(--bg)` on the story canvas so components render on the correct base surface

3. **Update the docs landing page** `docs/frontend-ui/PlanTracker.mdx` if the `Configure.mdx` removal breaks any cross-references.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All default scaffold files removed from src/stories/
- [ ] #2 Running `just storybook` (or equivalent) shows no Example/* stories
- [ ] #3 All story canvases render with dark Catppuccin Mocha background and JetBrains Mono font
- [ ] #4 No broken imports or missing CSS variables in the Storybook preview
<!-- AC:END -->
