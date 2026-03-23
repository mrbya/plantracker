use chrono::NaiveDate;
use sqlx::SqlitePool;
use tauri::State;

use crate::db;

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportEntry {
    pub task_title: String,
    pub plan_title: String,
    pub start_time: String,
    pub end_time: String,
    pub duration_seconds: i64,
    pub notes: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportResult {
    pub entries: Vec<ReportEntry>,
    pub grand_total_seconds: i64,
    pub subject_label: String,
}

// ---------------------------------------------------------------------------
// generate_report
// ---------------------------------------------------------------------------

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
            (to_year + 1, 1u32)
        } else {
            (to_year, to_month + 1)
        };
        NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .ok_or_else(|| format!("Invalid to date: {to_year}-{to_month}"))?
            - chrono::Duration::days(1)
    };

    let raw_entries =
        db::entries::list_entries_in_range(&pool, plan_id.as_deref(), task_id.as_deref(), from, to)
            .await
            .map_err(|e| e.to_string())?;

    // Cache lookups to avoid redundant DB queries when many entries share the
    // same task or plan.
    let mut task_cache: std::collections::HashMap<String, (String, String)> = Default::default();
    let mut plan_cache: std::collections::HashMap<String, String> = Default::default();

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
            ("No specific task".to_string(), raw.plan_id.clone())
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

        let duration_seconds = (end - start).num_seconds().max(0);

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
        "All entries".to_string()
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

fn format_duration_csv(seconds: i64) -> String {
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    format!("{h}:{m:02}:{s:02}")
}

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

    let file_path = match file_path {
        Some(p) => p,
        None => return Err("Export cancelled".to_string()),
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
