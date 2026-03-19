---
id: TASK-8
title: Phase 1.3 — Base UI components
status: Done
assignee: []
created_date: '2026-03-18 23:01'
updated_date: '2026-03-19 07:03'
labels: []
milestone: Phase 1 — Theme &amp; Design System
dependencies:
  - TASK-7
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create all reusable primitive components in `src/lib/components/ui/`. These are headless, theme-aware building blocks used across all views.

| Component | Props | Behaviour |
|---|---|---|
| `Button.svelte` | `variant: 'primary'\|'ghost'\|'danger'`, `disabled: boolean`, `loading: boolean` | Primary uses `--accent`; danger uses `--danger`; ghost is transparent. Shows `Spinner` inside when `loading=true` and disables itself. |
| `Select.svelte` | `options: {value: string, label: string}[]`, `value: string`, `placeholder?: string` | Styled `<select>` wrapper using `--bg-input` / `--border`. |
| `Input.svelte` | `type: string`, `value: string`, `label?: string`, `error?: string` | Text/datetime input. Shows red border + error text when `error` is set. |
| `Card.svelte` | `title?: string` | `--ctp-surface0` background, `--radius` border-radius, optional header. |
| `Badge.svelte` | `color: 'green'\|'red'\|'yellow'` | Small pill indicator mapping to `--success` / `--danger` / `--warning`. |
| `Spinner.svelte` | `size: 'sm'\|'md'` | Pure CSS rotating animation using `--accent` color. No JS. |
| `EmptyState.svelte` | `message: string` | Centered, `--text-muted` colored text for empty list states. |

Rules:
- All colours via CSS custom properties — no hardcoded hex values
- No inline `style="..."` attributes unless the value is genuinely dynamic (e.g. computed width)
- Use Svelte 5 runes syntax (`$props()`, `$state()`, `$derived()`) — no legacy `export let`
- Every interactive element has a visible `:focus-visible` ring
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All 7 components exist in src/lib/components/ui/
- [x] #2 Button renders all three variants (primary, ghost, danger) with correct colors
- [x] #3 Button shows Spinner and disables itself when loading=true
- [x] #4 Select renders styled dropdown with placeholder support
- [x] #5 Input shows error state (red border + message) when error prop is set
- [x] #6 Card renders with surface0 background and optional title
- [x] #7 Badge renders in green/red/yellow using semantic CSS variables
- [x] #8 Spinner animates with CSS only (no JS timers)
- [x] #9 EmptyState renders centered muted text
- [x] #10 All components use Svelte 5 runes syntax
- [x] #11 No hardcoded hex color values anywhere in component styles
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created all 7 components in src/lib/components/ui/: Spinner (CSS-only animation), Button (3 variants, embeds Spinner when loading), Select (styled with SVG chevron), Input (label + error state), Card (optional title header), Badge (color-mix tinted pills), EmptyState (centered muted message). All use Svelte 5 $props()/$bindable(), no hardcoded hex values. pnpm tsc --noEmit passes.
<!-- SECTION:FINAL_SUMMARY:END -->
