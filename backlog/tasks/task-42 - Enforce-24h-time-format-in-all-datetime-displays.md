---
id: TASK-42
title: Enforce 24h time format in all datetime displays
status: Done
assignee: []
created_date: '2026-03-21 16:43'
updated_date: '2026-03-21 16:58'
labels:
  - frontend
  - ux
dependencies: []
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Date/time strings displayed in the entries tables and Settings view use `toLocaleString(undefined, { hour: "2-digit", minute: "2-digit" })` without an explicit `hour12` override. On systems with a 12h locale (e.g. en-US) this renders AM/PM times. The app should always display times in 24h format regardless of locale.

## Affected sites

| File | Usage |
|---|---|
| `src/views/TimeTracking.svelte` | Local `formatDateTime()` function, called in entries table |
| `src/views/ManualEntry.svelte` | Local `formatDateTime()` function, called in entries table |
| `src/views/Settings.svelte` | Inline `toLocaleString` call for last-sync timestamp display |

## Fix

Extract `formatDateTime` into a shared utility (`src/lib/utils/datetime.ts`) and add `hour12: false` to the `Intl.DateTimeFormatOptions`. Remove the duplicate local implementations from `TimeTracking.svelte` and `ManualEntry.svelte`, and update `Settings.svelte` to use the shared function.

```ts
// src/lib/utils/datetime.ts
export function formatDateTime(iso: string): string {
  return new Date(iso).toLocaleString(undefined, {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  });
}
```

Replace all three usage sites to import from `$lib/utils/datetime`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 src/lib/utils/datetime.ts exists and exports formatDateTime with hour12: false
- [x] #2 TimeTracking.svelte imports formatDateTime from $lib/utils/datetime and has no local definition
- [x] #3 ManualEntry.svelte imports formatDateTime from $lib/utils/datetime and has no local definition
- [x] #4 Settings.svelte uses formatDateTime from $lib/utils/datetime instead of an inline toLocaleString call
- [x] #5 pnpm check passes with no errors
- [x] #6 The Start and End time inputs in ManualEntry.svelte render in 24h format regardless of system locale
<!-- AC:END -->
