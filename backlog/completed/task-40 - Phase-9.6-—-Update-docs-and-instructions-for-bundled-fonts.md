---
id: TASK-40
title: Phase 9.6 — Update docs and instructions for bundled fonts
status: Done
assignee: []
created_date: '2026-03-21 10:53'
updated_date: '2026-03-21 11:02'
labels:
  - docs
  - fonts
  - phase-9
milestone: Phase 9
dependencies:
  - TASK-39
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Fonts are now bundled with the app (TASK-39). Update all documentation and Claude instructions to reflect that users no longer need to install JetBrainsMono Nerd Font system-wide, and update the implementation plan with a note on this decision.

## Changes required

### `CLAUDE.md`
- Remove JetBrains Mono Nerd Font from the Stack table (it is an implementation detail, not a user-facing stack entry)
- Add `static/fonts/` to the File Structure tree with a note that it contains the bundled fonts

### `.claude/rules/ui.md`
- Update the Typography section to note that `JetBrainsMono Nerd Font` is loaded from `static/fonts/` via `@font-face` in `src/lib/theme/fonts.css` — **users do not need to install the font**
- Remove or amend any wording implying the font must be present on the host system

### `docs/IMPLEMENTATION_PLAN.md`
- Add a note under the relevant phase that fonts are self-hosted: bundled in `static/fonts/`, loaded via `@font-face`, no system font dependency

### Backlog plan document
- Create a plan document (`backlog/docs/plan.md`) that captures the high-level implementation phases and add a note under Phase 9 that fonts are self-hosted: bundled in `static/fonts/`, loaded via `@font-face`, no system font dependency.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 CLAUDE.md file structure includes static/fonts/ entry
- [x] #2 .claude/rules/ui.md typography section states fonts are loaded from static/fonts/ via @font-face
- [x] #3 .claude/rules/ui.md makes clear users do not need to install the font system-wide
- [x] #4 backlog/docs/plan.md exists and includes a Phase 9 note on bundled fonts
- [x] #5 docs/IMPLEMENTATION_PLAN.md includes a note that fonts are bundled in static/fonts/ and loaded via @font-face with no system install required
<!-- AC:END -->
