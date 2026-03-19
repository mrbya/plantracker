---
id: TASK-7
title: Phase 1.2 — Global reset and base styles
status: Done
assignee: []
created_date: '2026-03-18 23:01'
updated_date: '2026-03-19 06:58'
labels: []
milestone: Phase 1 — Theme &amp; Design System
dependencies:
  - TASK-6
priority: high
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/lib/theme/global.css` and wire both theme files into the app entry point.

**`global.css` must include:**
- Box-sizing reset: `*, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }`
- Base body styles using CSS variables: `background: var(--bg)`, `color: var(--text)`, `font-family: var(--font)`, `font-size: var(--font-size-base)`
- Thin scrollbar styling using Catppuccin surface/overlay colors
- Global focus ring: `:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }` — never `outline: none` without an alternative

**`src/app.css` (or the SvelteKit entry point):**
- Import `$lib/theme/mocha.css` first, then `$lib/theme/global.css`

The app entry point (`src/app.html` or the root layout) must load `app.css` so styles apply globally.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 src/lib/theme/global.css exists with box-sizing reset, body base styles, scrollbar styling, and focus ring
- [x] #2 app.css imports mocha.css then global.css in that order
- [x] #3 App background renders as Catppuccin Mocha dark (#1e1e2e) when launched
- [x] #4 Font is JetBrains Mono throughout the app
- [x] #5 Focus ring (2px solid accent color) visible on interactive elements
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created src/lib/theme/global.css (reset, body styles, focus ring, thin scrollbar). Created src/app.css importing mocha.css then global.css. Created src/routes/+layout.svelte to load app.css globally using Svelte 5 $props()/children rune pattern. pnpm tsc --noEmit passes.
<!-- SECTION:FINAL_SUMMARY:END -->
