---
id: TASK-43
title: 'Reports: Replace monthly subtotals with grand total + individual entries list'
status: Done
assignee: []
created_date: '2026-03-23 01:55'
updated_date: '2026-03-23 02:05'
labels:
  - frontend
  - backend
  - reports
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Reports view currently shows a table of monthly subtotals with the grand total in the footer. The desired layout is: grand total displayed at the top, followed by a flat list of individual time entries matching the selected plan/task within the chosen date range — similar to the entries table in TimeTracking and ManualEntry views.

**Backend changes required (`src-tauri/src/commands/reports.rs`)**

Add a `ReportEntry` struct:
```rust
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportEntry {
    pub task_title: String,
    pub plan_title: String,
    pub start_time: String,   // ISO 8601
    pub end_time: String,     // ISO 8601 (entries without end_time are already filtered out by list_entries_in_range)
    pub duration_seconds: i64,
    pub notes: Option<String>,
}
```

Update `ReportResult`: remove `monthly_totals`, add `entries: Vec<ReportEntry>`.

In `generate_report`: after calling `list_entries_in_range`, look up each entry's task title via `db::tasks::get_task` and the task's plan title via `db::plans::get_plan`, build `ReportEntry` items, and compute `grand_total_seconds` by summing their `duration_seconds`. No SQL queries change, so `just precache` is not needed.

Update `export_report_csv`: export individual entries with columns: Task, Plan, Date, Start, End, Duration (HH:MM:SS), Notes. Remove the month-based grouping rows.

**Frontend changes required**

`src/lib/types.ts`:
- Add `ReportEntry` interface (matching the new Rust struct in camelCase).
- Update `ReportResult`: remove `monthlyTotals: MonthlyTotal[]`, add `entries: ReportEntry[]`.
- Remove the `MonthlyTotal` interface if it is no longer referenced anywhere.

`src/views/Reports.svelte`:
- Remove the monthly totals table and its derived `grandHours`/`grandMinutes` values.
- Show the grand total (formatted as `Xh Ym` using `formatDuration` from `$lib/utils/duration`) at the top of the results section.
- Below it, render an entries table with columns: Task, Plan, Date, Start, End, Duration, Notes — matching the visual style of the TimeTracking entries table.
- Keep the "No entries found for this period." empty state.
- Remove unused imports (`SHORT_MONTHS`, etc.).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 When a report is generated and entries exist, the grand total (formatted as 'Xh Ym') is displayed above the entries table.
- [x] #2 The entries table shows columns: Task, Plan, Date, Start, End, Duration, Notes.
- [x] #3 Entries are listed in descending chronological order (most recent first), consistent with list_entries_in_range ordering.
- [x] #4 The 'No entries found for this period.' empty state is shown when there are no matching entries.
- [x] #5 The MonthlyTotal interface and monthly subtotals table are removed from the frontend.
- [x] #6 The ReportResult type no longer contains monthlyTotals; it contains entries: ReportEntry[] instead.
- [x] #7 The CSV export writes individual entry rows (Task, Plan, Date, Start, End, Duration, Notes) plus a Grand Total row at the end.
- [x] #8 The app builds without errors (cargo + svelte-check).
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced the monthly-subtotals table with a grand total + individual entries layout.

**Backend (`src-tauri/src/commands/reports.rs`)**
- Added `ReportEntry` struct with `task_title`, `plan_title`, `start_time`, `end_time`, `duration_seconds`, `notes`.
- Replaced `monthly_totals: Vec<MonthlyTotal>` in `ReportResult` with `entries: Vec<ReportEntry>`.
- `generate_report` now builds `ReportEntry` items by looking up task/plan titles from SQLite (with an in-memory cache to avoid duplicate queries). `grand_total_seconds` is summed from `duration_seconds`. Removed the `BTreeMap`-based monthly aggregation and the `MonthlyTotal` struct.
- `export_report_csv` now writes per-entry rows (Task, Plan, Date, Start, End, Duration, Notes) plus a Grand Total row. Removed `month_name` helper. Fixed two clippy `needless_borrows_for_generic_args` warnings.

**Frontend (`src/lib/types.ts`)**
- Removed `MonthlyTotal` interface.
- Added `ReportEntry` interface.
- Updated `ReportResult`: `monthlyTotals` → `entries: ReportEntry[]`.

**Frontend (`src/views/Reports.svelte`)**
- Removed `SHORT_MONTHS` array and `grandHours`/`grandMinutes` derived values.
- Added `formatDuration` and `formatDateTime` imports.
- Results section now shows grand total (`Total: Xh Ym`) above an entries table (Task, Plan, Start, End, Duration, Notes columns).
- Empty state preserved.

Both `svelte-check` and `cargo clippy` pass with zero warnings.
<!-- SECTION:FINAL_SUMMARY:END -->
