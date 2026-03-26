---
id: TASK-78.4
title: Replace all hardcoded strings in views and stores with m.* calls (§12.2.6)
status: Done
assignee: []
created_date: '2026-03-26 07:16'
updated_date: '2026-03-26 07:50'
labels:
  - frontend
  - i18n
  - phase-12
dependencies: []
parent_task_id: TASK-78
priority: medium
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Replace every hardcoded English UI string in views, components, and stores with a `m.*()` call from Paraglide. Import `m` from `$lib/paraglide/messages` at the top of each file.

```typescript
import * as m from '$lib/paraglide/messages';
```

## Files to update

### `src/lib/components/Layout.svelte`

Convert `navItems` to a `$derived` array so labels re-evaluate on locale change:

```typescript
const navItems = $derived([
    { id: 'time-tracking', icon: Timer,        label: m.nav_time_tracking() },
    { id: 'manual-entry',  icon: PencilLine,   label: m.nav_manual_entry()  },
    { id: 'reports',       icon: BarChart2,    label: m.nav_reports()       },
    { id: 'settings',      icon: SettingsIcon, label: m.nav_settings()      },
]);
```

Sign-out button: `title={m.nav_sign_out()}`.

### `src/views/Login.svelte`

Replace title, tagline, and button label with `m.login_title()`, `m.login_tagline()`, `m.login_sign_in_btn()`.

### `src/views/TimeTracking.svelte`

Replace all labels, placeholders, table headers, timer button text, entry state text, and delete confirmation strings. Full mapping in §12.2.6 of the plan doc. Note interpolated key:
```typescript
m.timer_stop({ elapsed: formatDuration($elapsedSeconds) })
```

### `src/views/ManualEntry.svelte`

Same label/placeholder keys as TimeTracking, plus manual entry-specific keys (`manual_new_entry`, `manual_edit_entry`, `manual_save`, `manual_update`, `manual_cancel`, `label_notes`).

### `src/views/Reports.svelte`

Replace all filter labels, column headers, button text, and empty state. Full mapping in §12.2.6.

### `src/views/Settings.svelte`

Replace all section titles, labels, descriptions, button text, and `<Select>` option labels for sync frequency and theme. Keys follow the `settings_*` namespace in the message files.

### Stores: `src/lib/stores/notifications.ts` call-sites

`addSuccess`/`addError`/`addWarning` accept plain strings — the store itself is unchanged. Update all callers that pass hardcoded strings in `planner.ts`, `timer.ts`, `auth.ts`, and any views. Example:

```typescript
// Before:
addSuccess('Sync complete');
addError('Sync failed: ' + e);

// After:
addSuccess(m.toast_sync_success());
addError(m.toast_sync_failed({ error: String(e) }));
```

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.2.6
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 No hardcoded English UI strings remain in any .svelte file or store file
- [x] #2 navItems in Layout.svelte is a $derived array using m.nav_*() calls
- [x] #3 timer_stop interpolation uses m.timer_stop({ elapsed: ... })
- [x] #4 All toast call-sites in planner.ts, timer.ts, auth.ts, and views use m.toast_*() keys
- [x] #5 svelte-check passes with no type errors on m.* calls (all keys are type-checked by Paraglide)
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced all hardcoded English UI strings across 8 files with `m.*()` calls.

**Files changed:**

- `src/lib/components/Layout.svelte` — `navItems` converted to `$derived` with `m.nav_*()` labels; sign-out `title` uses `m.nav_sign_out()`; removed now-unused `Icon` import
- `src/views/Login.svelte` — title, tagline, button text
- `src/views/TimeTracking.svelte` — labels, placeholders, table headers, timer button text (`m.timer_stop({ elapsed })`), Running… state, delete confirm/cancel, empty state, loading, toast calls
- `src/views/ManualEntry.svelte` — same as TimeTracking plus form title, field labels (Start/End/Notes), action buttons (Save/Update/Cancel), toast calls
- `src/views/Reports.svelte` — range labels, filter labels, button text, table headers, grand total label, empty state; `planOptions`/`taskOptions` use `m.reports_all_plans/tasks()` in `$derived`; export toast uses `m.toast_export_saved({ path })`
- `src/views/Settings.svelte` — all section titles, labels, descs, button text; `THEME_OPTIONS` and `SYNC_OPTIONS` converted to `$derived`; `formatLastSynced` uses `m.settings_sync_never()` / `m.settings_sync_last({ time })`
- `src/lib/stores/planner.ts` — sync success/failure toasts
- `src/lib/stores/auth.ts` — sign-in/sign-out failure toasts

**Verified:** `pnpm check` (ESLint + svelte-check) — 0 errors, 0 warnings.
<!-- SECTION:FINAL_SUMMARY:END -->
