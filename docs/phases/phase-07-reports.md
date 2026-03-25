# Phase 7 — Reports View

## 7.1 Report query (Rust)

Create `src-tauri/src/commands/reports.rs`:

```rust
#[derive(Serialize)]
pub struct MonthlyTotal {
    pub year: i32,
    pub month: u32,
    pub total_seconds: i64,
}

#[derive(Serialize)]
pub struct ReportResult {
    pub monthly_totals: Vec<MonthlyTotal>,
    pub grand_total_seconds: i64,
    pub subject_label: String,  // plan name or task name
}

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

SQL approach: group `time_entries` by `strftime('%Y-%m', start_time)`, filter by
`plan_id` or `task_id`, compute `SUM(unixepoch(end_time) - unixepoch(start_time))`.
Taskless entries are included in plan-level reports and display as "No specific task".

## 7.2 CSV export (Rust)

```rust
#[tauri::command]
pub async fn export_report_csv(
    report: ReportResult,
    app: tauri::AppHandle,
) -> Result<String, String>
```
- Use `tauri-plugin-dialog` to show save file dialog (default: `~/Downloads/plantracker-report-{date}.csv`)
- Write CSV using the `csv` crate
- Return saved file path for confirmation toast

## 7.3 Reports view

Create `src/views/Reports.svelte`:

**Controls row**
- From: month `<Select>` + year `<Input type="number">`
- To: month `<Select>` + year `<Input type="number">`
- Plan dropdown → Task dropdown (task optional)
- Generate button
- Export CSV button (disabled until report is generated)

**Report table**
- Columns: Month | Total Hours | Total Minutes
- Footer row: Grand Total
- Subject header: "Plan: {name}" or "Task: {name}"
- `EmptyState` if no entries in range

**Duration formatting utility** (`src/lib/utils/duration.ts`):
- `formatDuration(seconds: number): string` → `"2h 34m"`
- `formatDurationCSV(seconds: number): string` → `"2:34:00"`

## Verification checklist
- [x] Report generates correct totals (manually verify with known entries)
- [x] Selecting task only shows entries for that task
- [x] Selecting plan with no task shows all entries for that plan (including taskless entries)
- [x] CSV export produces correct file and opens save dialog
- [x] Date range spanning year boundary works correctly
