---
id: TASK-56
title: Phase 11.2 — Wire latte.css into app.css
status: Done
assignee: []
created_date: '2026-03-23 18:47'
updated_date: '2026-03-23 18:48'
labels:
  - frontend
  - theme
  - phase-11
dependencies:
  - TASK-55
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add an `@import` for `latte.css` in `src/app.css` so the Latte overrides are available globally.

## Context

`src/app.css` currently imports theme files in this order:

```css
@import "$lib/theme/fonts.css";
@import "$lib/theme/mocha.css";
@import "$lib/theme/global.css";
```

Import order matters:
- `mocha.css` defines the `:root` defaults (Mocha, always active).
- `latte.css` overrides them via `:root.theme-light` (higher specificity, only when the class is present).
- `global.css` consumes the resolved variables — it must come last.

## Change required

Insert `@import "$lib/theme/latte.css";` **after** `mocha.css` and **before** `global.css`:

```css
@import "$lib/theme/fonts.css";
@import "$lib/theme/mocha.css";
@import "$lib/theme/latte.css";   /* ← add this line */
@import "$lib/theme/global.css";
```

This is the only change needed in this task. No component files need to be touched.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The import `@import "$lib/theme/latte.css";` is present in `src/app.css`
- [x] #2 The import appears after `mocha.css` and before `global.css`
- [x] #3 No other lines in `app.css` are modified
- [ ] #4 `pnpm tsc --noEmit` passes with no new type errors
- [ ] #5 Running `cargo tauri dev` starts cleanly (CSS is loaded, no console errors about missing files)
<!-- AC:END -->
