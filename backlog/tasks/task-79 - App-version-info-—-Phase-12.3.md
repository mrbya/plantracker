---
id: TASK-79
title: App version info — Phase 12.3
status: Done
assignee: []
created_date: '2026-03-26 12:54'
updated_date: '2026-03-26 13:23'
labels:
  - frontend
  - backend
  - phase-12
dependencies: []
documentation:
  - docs/phases/phase-12-quality-of-life.md
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Display the app version in the Settings view footer.

One new Rust command reads the version from `AppHandle` at runtime (keeping `Cargo.toml` as the single source of truth), one API wrapper exposes it to the frontend, a Storybook mock provides a predictable value for stories, and a footer line is added to `Settings.svelte`.

No new stores, no migrations, no new components.

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.3
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Version string displayed in Settings matches the version field in Cargo.toml
- [x] #2 Bumping the version in Cargo.toml and rebuilding updates the displayed version without any frontend change
- [x] #3 Footer renders in both Mocha (dark) and Latte (light) themes with muted text colour
- [x] #4 Version string is selectable (copyable) with the cursor
- [x] #5 Footer does not render if getAppVersion() throws
- [x] #6 Storybook Settings story renders v0.0.0-storybook in the footer without a console error
- [x] #7 pnpm tsc --noEmit passes — no type errors introduced
- [x] #8 cargo clippy -- -D warnings passes — no new Rust warnings
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
All three subtasks complete. `get_app_version` Rust command added and registered. `getAppVersion` TS wrapper and Storybook mock added. Settings.svelte footer implemented with guard, theme-aware styling, and copyable version string. All checks pass: `cargo clippy -- -D warnings`, `pnpm tsc --noEmit`, `pnpm svelte-check`.
<!-- SECTION:FINAL_SUMMARY:END -->
