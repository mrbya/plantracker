---
id: TASK-6
title: Phase 1.1 — Define Catppuccin Mocha CSS variables
status: Done
assignee: []
created_date: '2026-03-18 23:01'
updated_date: '2026-03-19 06:57'
labels: []
milestone: Phase 1 — Theme &amp; Design System
dependencies: []
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/lib/theme/mocha.css` with the full Catppuccin Mocha palette and semantic alias variables.

**Full palette variables** (`--ctp-base` through `--ctp-rosewater`) plus **semantic aliases**:

```css
--bg:           var(--ctp-base)
--bg-raised:    var(--ctp-mantle)
--bg-input:     var(--ctp-surface0)
--border:       var(--ctp-surface1)
--text:         var(--ctp-text)
--text-muted:   var(--ctp-subtext0)
--accent:       var(--ctp-mauve)
--accent-hover: var(--ctp-lavender)
--success:      var(--ctp-green)
--warning:      var(--ctp-yellow)
--danger:       var(--ctp-red)
--timer-active: var(--ctp-green)
```

**Typography variables:**
```css
--font:           'JetBrainsMono Nerd Font', 'JetBrains Mono', monospace;
--font-size-sm:   0.75rem;
--font-size-base: 0.875rem;
--font-size-lg:   1rem;
--font-size-xl:   1.25rem;
```

**Spacing variables:**
```css
--radius:    6px;
--radius-lg: 10px;
--gap:       0.75rem;
```

Rules:
- Raw palette variables are defined only in this file
- All component styles reference semantic aliases, never raw `--ctp-*` variables directly
- Semantic aliases are defined only in this file via `var(--ctp-*)` references
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 src/lib/theme/mocha.css exists with all Catppuccin Mocha palette variables under :root
- [x] #2 All semantic alias variables defined (--bg, --bg-raised, --bg-input, --border, --text, --text-muted, --accent, --accent-hover, --success, --warning, --danger, --timer-active)
- [x] #3 Typography variables defined (--font, --font-size-sm/base/lg/xl)
- [x] #4 Spacing variables defined (--radius, --radius-lg, --gap)
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created src/lib/theme/mocha.css with all 26 Catppuccin Mocha palette variables, 12 semantic aliases, typography variables, and spacing variables under :root.
<!-- SECTION:FINAL_SUMMARY:END -->
