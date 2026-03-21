---
id: TASK-39
title: Phase 9.5 — Bundle JetBrainsMono Nerd Font into frontend
status: Done
assignee: []
created_date: '2026-03-21 10:53'
updated_date: '2026-03-21 10:58'
labels:
  - frontend
  - fonts
  - phase-9
milestone: Phase 9
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The font files have been added to `static/fonts/`. Wire them into the CSS so the app loads them from the bundle rather than relying on system-installed fonts.

## What to do

Create `src/lib/theme/fonts.css` with `@font-face` declarations for every weight/style combination present in `static/fonts/`:

| File | weight | style |
|---|---|---|
| JetBrainsMonoNerdFont-Thin.ttf | 100 | normal |
| JetBrainsMonoNerdFont-ThinItalic.ttf | 100 | italic |
| JetBrainsMonoNerdFont-ExtraLight.ttf | 200 | normal |
| JetBrainsMonoNerdFont-ExtraLightItalic.ttf | 200 | italic |
| JetBrainsMonoNerdFont-Light.ttf | 300 | normal |
| JetBrainsMonoNerdFont-LightItalic.ttf | 300 | italic |
| JetBrainsMonoNerdFont-Regular.ttf | 400 | normal |
| JetBrainsMonoNerdFont-Italic.ttf | 400 | italic |
| JetBrainsMonoNerdFont-Medium.ttf | 500 | normal |
| JetBrainsMonoNerdFont-MediumItalic.ttf | 500 | italic |
| JetBrainsMonoNerdFont-SemiBold.ttf | 600 | normal |
| JetBrainsMonoNerdFont-SemiBoldItalic.ttf | 600 | italic |
| JetBrainsMonoNerdFont-Bold.ttf | 700 | normal |
| JetBrainsMonoNerdFont-BoldItalic.ttf | 700 | italic |
| JetBrainsMonoNerdFont-ExtraBold.ttf | 800 | normal |
| JetBrainsMonoNerdFont-ExtraBoldItalic.ttf | 800 | italic |

Each declaration should use `font-display: swap` and reference the file via `/fonts/<filename>`.

Import `fonts.css` into `src/app.html` or the root CSS entry point so it is loaded globally before any component styles.

The `--font` variable in `mocha.css` already declares `"JetBrainsMono Nerd Font"` as the first family — the `font-family` name in every `@font-face` must match this exactly.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 fonts.css exists in src/lib/theme/ with @font-face declarations for all 16 weight/style combinations
- [x] #2 font-family name in every @font-face matches the --font CSS variable value exactly: "JetBrainsMono Nerd Font"
- [x] #3 fonts.css is imported globally so it applies before any component styles
- [x] #4 font-display: swap is set on every @font-face
- [x] #5 App renders with the bundled font when system font is not installed
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/lib/theme/fonts.css` with 16 `@font-face` declarations covering all weight/style combinations (100–800, normal + italic) from `static/fonts/`. Every declaration uses `font-family: "JetBrainsMono Nerd Font"` to match the existing `--font` CSS variable, `font-display: swap`, and a `/fonts/<filename>` URL path. Imported as the first entry in `src/app.css` so it loads before `mocha.css` and `global.css`.
<!-- SECTION:FINAL_SUMMARY:END -->
