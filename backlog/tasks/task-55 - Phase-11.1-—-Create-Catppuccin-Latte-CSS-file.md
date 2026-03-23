---
id: TASK-55
title: Phase 11.1 — Create Catppuccin Latte CSS file
status: Done
assignee: []
created_date: '2026-03-23 18:47'
updated_date: '2026-03-23 18:48'
labels:
  - frontend
  - theme
  - phase-11
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/lib/theme/latte.css` with the full Catppuccin Latte colour palette and semantic aliases, scoped to the `:root.theme-light` selector.

This file is the light-mode counterpart to the existing `mocha.css`. It must define **exactly the same set of CSS custom property names** so that swapping the active theme only requires toggling a class on `<html>` — no component styles need to change.

## Context

- The app currently only supports Catppuccin Mocha (dark). All CSS variables are defined in `src/lib/theme/mocha.css` under `:root`.
- Phase 11 adds a Dark / Light / System Default toggle. The mechanism is: when the user picks Light, the class `theme-light` is added to `document.documentElement`; when Dark is active, the class is absent. `latte.css` overrides the Mocha defaults via a higher-specificity selector (`:root.theme-light`).
- Mocha stays the default (no class needed) so there is no flash of unstyled content on first load.

## File to create

`src/lib/theme/latte.css`

Selector: `:root.theme-light { … }`

### Latte palette values

```css
--ctp-base:      #eff1f5;
--ctp-mantle:    #e6e9ef;
--ctp-crust:     #dce0e8;
--ctp-surface0:  #ccd0da;
--ctp-surface1:  #bcc0cc;
--ctp-surface2:  #acb0be;
--ctp-overlay0:  #9ca0b0;
--ctp-overlay1:  #8c8fa1;
--ctp-overlay2:  #7c7f93;
--ctp-subtext0:  #6c6f85;
--ctp-subtext1:  #5c5f77;
--ctp-text:      #4c4f69;
--ctp-lavender:  #7287fd;
--ctp-blue:      #1e66f5;
--ctp-sapphire:  #209fb5;
--ctp-sky:       #04a5e5;
--ctp-teal:      #179299;
--ctp-green:     #40a02b;
--ctp-yellow:    #df8e1d;
--ctp-peach:     #fe640b;
--ctp-maroon:    #e64553;
--ctp-red:       #d20f39;
--ctp-mauve:     #8839ef;
--ctp-pink:      #ea76cb;
--ctp-flamingo:  #dd7878;
--ctp-rosewater: #dc8a78;
```

### Semantic aliases (identical names to mocha.css)

```css
--bg:           var(--ctp-base);
--bg-raised:    var(--ctp-mantle);
--bg-input:     var(--ctp-surface0);
--border:       var(--ctp-surface1);
--text:         var(--ctp-text);
--text-muted:   var(--ctp-subtext0);
--accent:       var(--ctp-mauve);
--accent-hover: var(--ctp-lavender);
--success:      var(--ctp-green);
--warning:      var(--ctp-yellow);
--danger:       var(--ctp-red);
--timer-active: var(--ctp-green);
```

No other files need to change in this task.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 File `src/lib/theme/latte.css` exists and is non-empty
- [x] #2 All CSS custom properties are scoped to `:root.theme-light { … }` — not `:root`
- [x] #3 Every palette variable from `--ctp-base` through `--ctp-rosewater` is defined with the correct Latte hex value
- [x] #4 All 12 semantic alias variables (`--bg`, `--bg-raised`, `--bg-input`, `--border`, `--text`, `--text-muted`, `--accent`, `--accent-hover`, `--success`, `--warning`, `--danger`, `--timer-active`) are present and map to the correct Latte palette variables
- [x] #5 The set of variable names matches `mocha.css` exactly — no missing or extra names
- [x] #6 No hardcoded hex values appear outside the palette block (semantic aliases use `var(--ctp-*)`)
<!-- AC:END -->
