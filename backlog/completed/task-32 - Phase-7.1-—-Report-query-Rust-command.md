---
id: TASK-32
title: Phase 7.1 — Report query Rust command
status: Done
assignee: []
created_date: '2026-03-19 14:12'
updated_date: '2026-03-19 14:24'
labels:
  - backend
  - rust
milestone: Phase 7
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/commands/reports.rs` with the `generate_report` Tauri command that aggregates time entry data into monthly totals.

## Structs

```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyTotal {
    pub year: i32,
    pub month: u32,
    pub total_seconds: i64,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportResult {
    pub monthly_totals: Vec<MonthlyTotal>,
    pub grand_total_seconds: i64,
    pub subject_label: String,  // e.g. "Plan: My Plan" or "Task: Fix bug"
}
```

Note: `MonthlyTotal` and `ReportResult` already exist in `src/lib/types.ts` as TypeScript interfaces — the Rust structs must serialize to the same camelCase shape.

## Command

```rust
#[tauri::command]
pub async fn generate_report(
    plan_id: Option<String>,
    task_id: Option<String>,
    from_year: i32,
    from_month: u32,
    to_year: i32,
    to_month: u32,
    pool: State<'_, SqlitePool>,
) -> Result<ReportResult, String>
```

### Implementation approach

Use `db::entries::list_entries_in_range` (already implemented, takes `plan_id`, `task_id`, `NaiveDate` bounds) to fetch completed entries, then aggregate in Rust:

1. Compute `from` as `NaiveDate` for the 1st of `from_year`/`from_month`, and `to` as the last day of `to_year`/`to_month` (use the 1st of the following month as the exclusive upper bound in `list_entries_in_range`).
2. Fetch entries — only completed ones are returned (the existing query excludes `end_time IS NULL`).
3. Group entries by `(year, month)` extracted from `start_time`, summing `(end_time - start_time)` in seconds.
4. Build sorted `Vec<MonthlyTotal>` and compute `grand_total_seconds`.
5. Build `subject_label`:
   - If `task_id` is Some: query `db::tasks::get_task_by_graph_id` won't help — instead do a plain `sqlx::query_scalar!` to select `title FROM tasks WHERE id = ?`, or look up the title from a `list_tasks_for_plan` call. Simpler: add a `get_task(pool, id)` helper to `db/tasks.rs` that selects by local `id` (not `graph_id`), OR use `"Task: {task_id}"` as fallback if no lookup function exists yet.
   - If `plan_id` is Some: use `db::plans::get_plan` → `"Plan: {title}"`.
   - Neither: `"All entries"`.

Prefer adding a small `get_task(pool, id)` function to `db/tasks.rs` to look up by local id. This requires updating the `.sqlx/` cache.

## Registration
- Declare `pub mod reports;` in `commands/mod.rs`
- Register `generate_report` in `lib.rs` invoke_handler
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 generate_report returns correct MonthlyTotal entries for a date range filtered by task_id
- [x] #2 generate_report returns correct totals when filtered by plan_id only
- [x] #3 grand_total_seconds equals the sum of all monthly total_seconds
- [x] #4 subject_label reads 'Plan: {title}' or 'Task: {title}' or 'All entries'
- [x] #5 Date range spanning a year boundary (e.g. Nov 2024 – Feb 2025) produces correct month groupings
- [x] #6 Command registered in lib.rs invoke_handler
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `get_task(pool, id)` to `db/tasks.rs` (lookup by local id). Created `commands/reports.rs` with `MonthlyTotal`, `ReportResult` structs and `generate_report` command — fetches entries via `list_entries_in_range`, aggregates in Rust using a `BTreeMap<(year, month), seconds>`, builds sorted `monthly_totals`, computes `grand_total_seconds`, and constructs `subject_label` by looking up task/plan titles. Registered in `commands/mod.rs` and `lib.rs`. Added `generateReport` wrapper to `src/lib/api/index.ts`. Regenerated `.sqlx/` cache.
<!-- SECTION:FINAL_SUMMARY:END -->
