---
id: TASK-34
title: Phase 7.3 — Reports frontend view
status: Done
assignee: []
created_date: '2026-03-19 14:12'
updated_date: '2026-03-19 14:28'
labels:
  - frontend
  - typescript
  - svelte
milestone: Phase 7
dependencies:
  - TASK-32
  - TASK-33
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement `src/views/Reports.svelte` — date-range controls, plan/task filters, a generated report table, and a CSV export button.

## API wrappers (add to `src/lib/api/index.ts`)

```typescript
export async function generateReport(params: {
  planId?: string;
  taskId?: string;
  fromYear: number;
  fromMonth: number;
  toYear: number;
  toMonth: number;
}): Promise<ReportResult>

export async function exportReportCsv(report: ReportResult): Promise<string>
// Returns the saved file path
```

## Layout

**Controls row**
- From: month `<Select>` (1–12, label "January"…"December") + year `<Input type="number">` (default: current year)
- To: month `<Select>` + year `<Input type="number">` (default: current month/year)
- Plan `<Select>` (optional, from `plans` store) → Task `<Select>` (optional, filtered by plan)
- "Generate" `<Button variant="primary">` — disabled while loading
- "Export CSV" `<Button variant="ghost">` — disabled until a report has been generated; disabled while exporting

**Report table** (shown only after generation)
- Subject header above the table: `"Plan: {name}"` or `"Task: {name}"` from `report.subjectLabel`
- Columns: Month | Hours | Minutes
  - Month: formatted as "Jan 2024", "Feb 2024", etc. from `year` + `month` fields
  - Hours: `Math.floor(totalSeconds / 3600)`
  - Minutes: `Math.floor((totalSeconds % 3600) / 60)`
- Footer row: **Grand Total** | total hours | total minutes
- `<EmptyState message="No entries found for this period.">` when `monthlyTotals` is empty but report was generated

## State handling
- `<Spinner>` inside Generate button while loading
- Success toast with `"Exported to {path}"` after CSV export
- Error toast if generate or export fails
- Export button remains disabled until a report exists in state

## Notes
- `ReportResult` and `MonthlyTotal` types already exist in `src/lib/types.ts`.
- `formatDuration` from `$lib/utils/duration.ts` is available but the table uses raw hours/minutes columns for readability rather than the `Xh Ym` format.
- When the plan selection changes, clear the task selection (same pattern as TimeTracking/ManualEntry).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 From/To month+year controls default to the current month and year
- [x] #2 Plan and task dropdowns filter the report (task selection clears when plan changes)
- [x] #3 Generate button triggers the report and renders the table
- [x] #4 Table shows one row per month with correct Hours and Minutes columns
- [x] #5 Grand Total footer row shows the sum across all months
- [x] #6 EmptyState shown when report is generated but contains no entries
- [x] #7 Export CSV button is disabled until a report is generated
- [x] #8 Clicking Export CSV opens a save dialog and shows a success toast with the saved path
- [x] #9 All invoke calls go through api/index.ts
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented `Reports.svelte` replacing the Phase 7 stub. Controls row: From/To month selects (MONTH_OPTIONS 1–12) + year number inputs defaulting to current month/year; optional plan/task filter selects following the same store-sync pattern as TimeTracking/ManualEntry; Generate (primary) and Export CSV (ghost, disabled until report exists) buttons. Report section shows subject label from `report.subjectLabel`, a monthly table (Month | Hours | Minutes columns with `SHORT_MONTHS` formatting like "Jan 2024") and a grand total tfoot row, or `<EmptyState>` when no entries returned. Export calls `exportReportCsv`, shows success toast with path or error toast (silently ignores "Export cancelled"). All calls go through `$lib/api`. Layout already imports and renders Reports — no changes needed there.
<!-- SECTION:FINAL_SUMMARY:END -->
