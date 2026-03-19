---
id: TASK-33
title: Phase 7.2 — CSV export Rust command
status: Done
assignee: []
created_date: '2026-03-19 14:12'
updated_date: '2026-03-19 14:26'
labels:
  - backend
  - rust
milestone: Phase 7
dependencies:
  - TASK-32
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add an `export_report_csv` Tauri command to `src-tauri/src/commands/reports.rs` that serialises a `ReportResult` to a CSV file using a save dialog.

## Command

```rust
#[tauri::command]
pub async fn export_report_csv(
    report: ReportResult,
    app: tauri::AppHandle,
) -> Result<String, String>
// Returns the path of the saved file as a confirmation string.
```

## Implementation

1. **Default filename**: `plantracker-report-YYYY-MM-DD.csv` using `chrono::Local::now()`.
2. **Save dialog**: use `tauri_plugin_dialog::DialogExt` to open a save-file dialog:
   ```rust
   use tauri_plugin_dialog::DialogExt;
   let path = app.dialog()
       .file()
       .set_file_name(&default_name)
       .add_filter("CSV", &["csv"])
       .blocking_save_file();
   ```
   Return `Err("Export cancelled".to_string())` if the user dismisses the dialog (`path` is `None`).
3. **Write CSV** using the `csv` crate:
   - Header row: `Month,Year,Total Hours,Total Minutes,Total Seconds`
   - One data row per `MonthlyTotal`
   - Final row: `Grand Total,,,,{grand_total_seconds}`
4. Return the saved file path as a `String` for a confirmation toast on the frontend.

## Notes
- The `csv` crate is already in `Cargo.toml`.
- `tauri_plugin_dialog` is already initialised in `lib.rs`.
- Do **not** use `async` dialog APIs — the blocking variant avoids executor complexity inside a Tauri command.

## Registration
Register `export_report_csv` in `lib.rs` invoke_handler.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Save dialog opens with a default filename of plantracker-report-YYYY-MM-DD.csv
- [x] #2 Dismissing the dialog returns Err('Export cancelled') without writing any file
- [x] #3 Written CSV has a header row and one data row per MonthlyTotal
- [x] #4 Grand Total row appears as the last row
- [x] #5 Return value is the absolute path of the saved file
- [x] #6 Command registered in lib.rs invoke_handler
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `serde::Deserialize` to `MonthlyTotal` and `ReportResult` (needed as command input). Added `export_report_csv` command using `tauri_plugin_dialog::DialogExt::blocking_save_file()` — opens save dialog with default name `plantracker-report-YYYY-MM-DD.csv`, returns `Err("Export cancelled")` on dismiss, writes CSV with header row + monthly rows (hours/minutes/seconds breakdown) + grand total row, returns absolute path on success. Registered in `lib.rs`. Added `exportReportCsv` API wrapper.
<!-- SECTION:FINAL_SUMMARY:END -->
