use chrono::NaiveDate;
use sqlx::SqlitePool;
use tauri::State;

use crate::db;

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

/// A single enriched time-entry row in a generated report.
///
/// Unlike the raw [`crate::models::TimeEntry`] struct, `ReportEntry` includes resolved
/// display titles for the task and plan, and a pre-computed `duration_seconds` value.
/// Title resolution is performed inside [`generate_report`] using cached `SQLite` lookups
/// so that many entries sharing the same task or plan do not each require a separate query.
///
/// `ReportEntry` is also used as input to [`export_report_csv`] when the user exports
/// the report to a file.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportEntry {
    /// Display title of the associated task, or `"No specific task"` for plan-level entries.
    pub task_title: String,
    /// Display title of the associated plan.
    pub plan_title: String,
    /// ISO 8601 start timestamp.
    pub start_time: String,
    /// ISO 8601 end timestamp.
    pub end_time: String,
    /// Duration of the entry in whole seconds (`end_time - start_time`, minimum 0).
    pub duration_seconds: i64,
    /// Optional free-text notes attached to this entry by the user.
    pub notes: Option<String>,
}

/// The complete result of a [`generate_report`] call, as returned to the frontend.
///
/// This struct is passed directly to [`export_report_csv`] when the user chooses to
/// export the report — the frontend serialises the same `ReportResult` object it received
/// from `generate_report` and sends it back to the export command.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportResult {
    /// All matching completed [`ReportEntry`] rows within the requested date range,
    /// ordered by `start_time DESC`.
    pub entries: Vec<ReportEntry>,
    /// Sum of `duration_seconds` across all entries in `entries`.
    pub grand_total_seconds: i64,
    /// Human-readable label describing the scope of the report:
    /// `"Task: <title>"`, `"Plan: <title>"`, or `"All entries"`.
    pub subject_label: String,
}

// ---------------------------------------------------------------------------
// generate_report
// ---------------------------------------------------------------------------

/// Generates a time report for the specified month range.
///
/// The scoping priority for entries is: task > plan > all entries. Specifically:
/// - If `task_id` is `Some`, only entries for that task are included.
/// - If `task_id` is `None` but `plan_id` is `Some`, all entries for that plan are included.
/// - If both are `None`, all entries in the date range are included.
///
/// The date range is specified as year/month pairs (`from_year`/`from_month` through
/// `to_year`/`to_month`). The range is inclusive at both ends: it spans from the first
/// day of `from_year`/`from_month` through the last day of `to_year`/`to_month`.
///
/// Task and plan titles are resolved from `SQLite` with an in-memory cache to avoid
/// redundant queries when many entries share the same task or plan.
///
/// # Arguments
///
/// - `plan_id`: Optional local UUID to scope results to a specific plan.
/// - `task_id`: Optional local UUID to scope results to a specific task.
/// - `from_year`: Start year (e.g. `2024`).
/// - `from_month`: Start month, 1–12.
/// - `to_year`: End year (e.g. `2024`).
/// - `to_month`: End month, 1–12.
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
///
/// # Returns
///
/// `Ok(ReportResult)` containing all matching entries with resolved titles, the grand
/// total duration, and a human-readable scope label.
///
/// # Errors
///
/// Returns a string error if:
/// - the `from` or `to` date arguments produce an invalid date (e.g. month 0 or 13),
/// - an arithmetic overflow occurs when computing the last day of the `to` month, or
/// - any `SQLite` query fails.
#[tauri::command]
pub async fn generate_report(
    plan_id: Option<String>,
    task_id: Option<String>,
    from_year: i32,
    from_month: u32,
    to_year: i32,
    to_month: u32,
    pool: State<'_, SqlitePool>,
) -> Result<ReportResult, String> {
    let from = NaiveDate::from_ymd_opt(from_year, from_month, 1)
        .ok_or_else(|| format!("Invalid from date: {from_year}-{from_month}"))?;

    let to = {
        let (next_year, next_month) = if to_month == 12 {
            (
                to_year
                    .checked_add(1)
                    .ok_or_else(|| format!("Year overflow computing end of {to_year}-{to_month}"))?,
                1,
            )
        } else {
            (
                to_year,
                to_month
                    .checked_add(1)
                    .ok_or_else(|| format!("Month overflow computing end of {to_year}-{to_month}"))?,
            )
        };
        NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .ok_or_else(|| format!("Invalid to date: {to_year}-{to_month}"))?
            .pred_opt()
            .ok_or_else(|| format!("Date underflow computing end of {to_year}-{to_month}"))?
    };

    let raw_entries =
        db::entries::list_entries_in_range(&pool, plan_id.as_deref(), task_id.as_deref(), from, to)
            .await
            .map_err(|e| e.to_string())?;

    // Cache lookups to avoid redundant DB queries when many entries share the
    // same task or plan.
    let mut task_cache: std::collections::HashMap<String, (String, String)> =
        std::collections::HashMap::default();
    let mut plan_cache: std::collections::HashMap<String, String> =
        std::collections::HashMap::default();

    let mut entries: Vec<ReportEntry> = Vec::with_capacity(raw_entries.len());

    for raw in &raw_entries {
        let (task_title, plan_id_for_lookup) = if let Some(ref tid) = raw.task_id {
            if let Some(cached) = task_cache.get(tid) {
                cached.clone()
            } else {
                let (title, pid) = match db::tasks::get_task(&pool, tid).await {
                    Ok(Some(t)) => (t.title, t.plan_id),
                    _ => (tid.clone(), raw.plan_id.clone()),
                };
                task_cache.insert(tid.clone(), (title.clone(), pid.clone()));
                (title, pid)
            }
        } else {
            ("No specific task".to_owned(), raw.plan_id.clone())
        };

        let plan_title = if let Some(cached) = plan_cache.get(&plan_id_for_lookup) {
            cached.clone()
        } else {
            let title = match db::plans::get_plan(&pool, &plan_id_for_lookup).await {
                Ok(Some(p)) => p.title,
                _ => plan_id_for_lookup.clone(),
            };
            plan_cache.insert(plan_id_for_lookup.clone(), title.clone());
            title
        };

        let end_time = raw.end_time.clone().unwrap_or_default();

        let start = raw
            .start_time
            .parse::<chrono::DateTime<chrono::Utc>>()
            .map_err(|e| format!("Bad start_time '{}': {e}", raw.start_time))?;
        let end = end_time
            .parse::<chrono::DateTime<chrono::Utc>>()
            .map_err(|e| format!("Bad end_time '{end_time}': {e}"))?;

        let duration_seconds = end.signed_duration_since(start).num_seconds().max(0);

        entries.push(ReportEntry {
            task_title,
            plan_title,
            start_time: raw.start_time.clone(),
            end_time,
            duration_seconds,
            notes: raw.notes.clone(),
        });
    }

    let grand_total_seconds = entries.iter().map(|e| e.duration_seconds).sum();

    let subject_label = if let Some(ref tid) = task_id {
        match db::tasks::get_task(&pool, tid).await {
            Ok(Some(t)) => format!("Task: {}", t.title),
            _ => format!("Task: {tid}"),
        }
    } else if let Some(ref pid) = plan_id {
        match db::plans::get_plan(&pool, pid).await {
            Ok(Some(p)) => format!("Plan: {}", p.title),
            _ => format!("Plan: {pid}"),
        }
    } else {
        "All entries".to_owned()
    };

    Ok(ReportResult {
        entries,
        grand_total_seconds,
        subject_label,
    })
}

// ---------------------------------------------------------------------------
// export_report_csv
// ---------------------------------------------------------------------------

/// Formats a duration in seconds as `H:MM:SS` for CSV export.
///
/// The hour component is not zero-padded (e.g. `1:05:03` for 1 hour, 5 minutes, 3 seconds),
/// but minutes and seconds are always two digits. This matches the `HH:MM:SS` style
/// recommended in the UI conventions for CSV exports.
///
/// # Arguments
///
/// - `seconds`: Total duration in whole seconds.
///
/// # Returns
///
/// A string in `H:MM:SS` format (e.g. `"2:34:00"`, `"0:05:30"`, `"10:00:00"`).
fn format_duration_csv(seconds: i64) -> String {
    let h = seconds.div_euclid(3600);
    let m = seconds.rem_euclid(3600).div_euclid(60);
    let s = seconds.rem_euclid(60);
    format!("{h}:{m:02}:{s:02}")
}

/// Exports a [`ReportResult`] as a CSV file, prompting the user to choose a save path.
///
/// Opens a native save-file dialog (via `tauri-plugin-dialog`) pre-filled with a default
/// filename of `plantracker-report-YYYY-MM-DD.csv`. If the user dismisses the dialog
/// without choosing a path, the command returns an error with the sentinel string
/// `"Export cancelled"` so the frontend can distinguish cancellation from a real error.
///
/// The CSV includes columns: `Task`, `Plan`, `Date`, `Start`, `End`, `Duration`, `Notes`,
/// and a final `Grand Total` row with the summed duration in `H:MM:SS` format.
///
/// # Arguments
///
/// - `report`: The [`ReportResult`] to export, typically the value returned by a prior
///   call to [`generate_report`].
/// - `app`: Tauri application handle, used to access the dialog plugin.
///
/// # Returns
///
/// `Ok(path)` — the absolute path of the file that was written, as a `String`.
///
/// # Errors
///
/// Returns a string error (or the sentinel `"Export cancelled"`) if:
/// - the user dismisses the file dialog without choosing a path,
/// - the chosen path cannot be resolved to a filesystem path,
/// - the CSV file cannot be created (e.g. permission denied), or
/// - writing any CSV record fails.
#[tauri::command]
pub async fn export_report_csv(
    report: ReportResult,
    app: tauri::AppHandle,
) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;

    let default_name = format!(
        "plantracker-report-{}.csv",
        chrono::Local::now().format("%Y-%m-%d")
    );

    let file_path = app
        .dialog()
        .file()
        .set_file_name(&default_name)
        .add_filter("CSV", &["csv"])
        .blocking_save_file();

    let Some(file_path) = file_path else {
        return Err("Export cancelled".to_owned());
    };

    let path = file_path
        .into_path()
        .map_err(|e| format!("Could not resolve save path: {e}"))?;

    let file = std::fs::File::create(&path).map_err(|e| format!("Could not create file: {e}"))?;

    let mut writer = csv::Writer::from_writer(file);

    writer
        .write_record(["Task", "Plan", "Date", "Start", "End", "Duration", "Notes"])
        .map_err(|e| e.to_string())?;

    for entry in &report.entries {
        let start_dt = entry
            .start_time
            .parse::<chrono::DateTime<chrono::Utc>>()
            .unwrap_or_default();
        let end_dt = entry
            .end_time
            .parse::<chrono::DateTime<chrono::Utc>>()
            .unwrap_or_default();

        let date = start_dt.format("%Y-%m-%d").to_string();
        let start_str = start_dt.format("%H:%M:%S").to_string();
        let end_str = end_dt.format("%H:%M:%S").to_string();
        let duration = format_duration_csv(entry.duration_seconds);
        let notes = entry.notes.as_deref().unwrap_or("");

        writer
            .write_record([
                entry.task_title.as_str(),
                entry.plan_title.as_str(),
                date.as_str(),
                start_str.as_str(),
                end_str.as_str(),
                duration.as_str(),
                notes,
            ])
            .map_err(|e| e.to_string())?;
    }

    let grand_duration = format_duration_csv(report.grand_total_seconds);
    writer
        .write_record(["Grand Total", "", "", "", "", grand_duration.as_str(), ""])
        .map_err(|e| e.to_string())?;

    writer.flush().map_err(|e| e.to_string())?;

    Ok(path.display().to_string())
}
