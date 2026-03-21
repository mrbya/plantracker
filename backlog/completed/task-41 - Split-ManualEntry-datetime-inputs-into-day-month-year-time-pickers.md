---
id: TASK-41
title: Split ManualEntry datetime inputs into day / month / year / time pickers
status: Done
assignee: []
created_date: '2026-03-21 15:14'
updated_date: '2026-03-21 15:17'
labels:
  - frontend
  - manual-entry
  - ux
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Start and End fields in the ManualEntry form currently use `<Input type="datetime-local">`, which renders a single browser-native datetime widget. Replace them with separate, individually styled pickers — day (number), month (Select), year (number), and time (`<input type="time">`) — consistent with the date range controls in the Reports view.

## Current state

```
[Start: datetime-local input          ]
[End:   datetime-local input          ]
```

`startTime` and `endTime` are stored as datetime-local strings (`YYYY-MM-DDTHH:MM`) and converted to/from ISO 8601 via `localToIso()` / `isoToLocal()`.

## Target state

Each of Start and End is replaced by a row of four fields:

```
[Day ↕] [Month ▾          ] [Year ↕  ] [Time  ]
```

- **Day** — `<input type="number">`, range 1–31, width ~3.5rem
- **Month** — `<Select>` using the same `MONTH_OPTIONS` array already defined in `Reports.svelte`; move `MONTH_OPTIONS` to a shared utility or re-declare locally
- **Year** — `<input type="number">`, 4-digit, width ~5rem (same as Reports)
- **Time** — `<input type="time">`, width ~6rem, step 60 (minute precision)

Layout: the four fields sit side by side in a single flex row, aligned to baseline, with the field label (`Start` / `End`) above the row — matching the `.range-group` / `.range-fields` pattern in `Reports.svelte`.

## State changes

Replace `let startTime = $state("")` / `let endTime = $state("")` with structured state:

```ts
let startDay   = $state("")
let startMonth = $state("")
let startYear  = $state("")
let startHour  = $state("")   // from <input type="time"> value, e.g. "14:30"

let endDay     = $state("")
let endMonth   = $state("")
let endYear    = $state("")
let endHour    = $state("")
```

## Helper changes

Replace `localToIso(local: string)` with `fieldsToIso(day, month, year, time)` that assembles an ISO 8601 UTC string from the four parts. Validate that all four parts are non-empty and form a valid date before assembling.

Replace `isoToLocal(iso: string)` with `isoToFields(iso: string)` that returns `{ day, month, year, time }` and is used in `handleEdit()` to populate the six state variables.

## Validation changes

Update `validate()` to check each structured field individually and produce appropriate error messages (e.g. "Start day is required", "Start month is required", etc.). The cross-field check (end must be after start) remains — assemble both ISO strings first, then compare.

## Error display

Show inline error messages below each picker row (Start row errors / End row errors), matching the existing `.error-msg` style.

## resetForm changes

Reset all eight structured state variables (plus errors) instead of the two datetime-local strings.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Start and End fields each render as a flex row: day number input, month Select, year number input, time input
- [x] #2 MONTH_OPTIONS is available in ManualEntry (copied locally or extracted to a shared utility)
- [x] #3 Submitting a valid form constructs the correct ISO 8601 UTC start and end times and calls the API
- [x] #4 Editing an existing entry populates all eight picker fields correctly via isoToFields()
- [x] #5 resetForm() clears all picker fields and error state
- [x] #6 validate() reports per-field errors for missing day, month, year, or time on either Start or End
- [x] #7 End-before-start validation still works
- [x] #8 pnpm check passes with no errors
<!-- AC:END -->
