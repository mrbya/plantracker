---
id: TASK-78
title: Multi-language locales (EN / SK / DE) — Phase 12.2
status: Done
assignee: []
created_date: '2026-03-26 07:14'
updated_date: '2026-03-26 07:57'
labels:
  - frontend
  - i18n
  - phase-12
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement i18n support for PlanTracker using Paraglide JS 2.0. Ship three locales: English (default), Slovak, and German.

This is a **pure frontend change** — no Rust commands, no database migrations. The locale preference is persisted via the existing `tauri-plugin-store` (`config.json`) and applied at startup before the first render.

## Why Paraglide JS

Paraglide 2.0 is SvelteKit's officially recommended i18n library. It compiles message files into tree-shakable TypeScript functions — unused locale strings are never bundled. It is fully type-safe with IDE autocomplete, has zero async waterfalls, and supports reactive locale switching without a page reload.

## Locale resolution strategy

`['cookie', 'baseLocale']` — reads a cookie named `locale` on startup; falls back to `en` if absent.

## Reference

`docs/phases/phase-12-quality-of-life.md`, section 12.2 (§12.2.1–12.2.9)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Changing the language in Settings → Language immediately updates all visible strings without a page reload or app restart
- [ ] #2 The selected language persists across app restarts (stored under key 'locale' in config.json)
- [ ] #3 First launch with no saved locale defaults to English
- [ ] #4 All three locales (EN / SK / DE) are selectable and render correctly
- [ ] #5 No hardcoded English strings remain in any .svelte file or store
- [ ] #6 All toast messages respect the active locale
- [ ] #7 All table column headers, button labels, placeholders, and section titles respect the active locale
<!-- AC:END -->
