---
id: TASK-78.2
title: 'Create translation message files for EN, SK, DE (§12.2.3)'
status: Done
assignee: []
created_date: '2026-03-26 07:15'
updated_date: '2026-03-26 07:27'
labels:
  - frontend
  - i18n
  - phase-12
dependencies: []
parent_task_id: TASK-78
priority: medium
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Populate `messages/en.json`, `messages/sk.json`, and `messages/de.json` with all user-visible strings in the app. Keys use `snake_case` namespaced by feature area. Interpolation uses `{variable}` syntax.

## Key areas covered

- `nav_*` — sidebar navigation labels + sign-out
- `login_*` — Login view
- `timer_*` / `label_*` / `placeholder_*` — TimeTracking view
- `entries_*` — Recent Entries table
- `manual_*` — ManualEntry view
- `reports_*` — Reports view + column headers
- `settings_*` — Settings sections, labels, descriptions, options
- `toast_*` — all toast notification messages

## Interpolated keys (require `{variable}` syntax)

- `timer_stop`: `{elapsed}`
- `settings_sync_last`: `{time}`
- `toast_export_saved`: `{path}`
- `toast_sign_out_failed`, `toast_sign_in_failed`, `toast_sync_failed`, `toast_load_failed`, `toast_delete_failed`: `{error}`

The complete string sets for all three locales are specified in `docs/phases/phase-12-quality-of-life.md` §12.2.3 — copy them verbatim.

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.2.3
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 messages/en.json contains all keys listed in §12.2.3 with correct $schema field
- [x] #2 messages/sk.json and messages/de.json contain the same keys with correct translations
- [x] #3 All interpolation variables ({elapsed}, {time}, {path}, {error}) are present in all three locales
- [x] #4 Running just i18n (after TASK-78.5 is done) compiles without errors
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Populated all three message files verbatim from the plan spec (§12.2.3).

**Keys written:** 83 keys across 9 namespaces (`nav_*`, `login_*`, `timer_*`, `label_*`, `placeholder_*`, `entries_*`, `manual_*`, `reports_*`, `settings_*`, `toast_*`).

**Interpolated keys verified:** `timer_stop` (`{elapsed}`), `settings_sync_last` (`{time}`), `toast_export_saved` (`{path}`), and the five `toast_*_failed` keys (`{error}`) — all present in all three locales.

**Compilation verified:** `paraglide-js compile` succeeded and generated 83 individual tree-shakable message modules in `src/lib/paraglide/messages/`. Each interpolated key compiled to a typed function (e.g. `timer_stop(inputs: { elapsed })`).
<!-- SECTION:FINAL_SUMMARY:END -->
