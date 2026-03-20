use std::collections::BTreeMap;

use chrono::NaiveDate;
use sqlx::SqlitePool;
use tauri::State;

use crate::db;

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyTotal {
    pub year: i32,
    pub month: u32,
    pub total_seconds: i64,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportResult {
    pub monthly_totals: Vec<MonthlyTotal>,
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

    let entries = db::entries::list_entries_in_range(
        &pool,
        plan_id.as_deref(),
        task_id.as_deref(),
        from,
        to,
    )
    .await
    .map_err(|e| e.to_string())?;

    let mut totals: BTreeMap<(i32, u32), i64> = BTreeMap::new();

    for entry in &entries {
        let start = entry
            .start_time
            .parse::<chrono::DateTime<chrono::Utc>>()
            .map_err(|e| format!("Bad start_time '{}': {e}", entry.start_time))?;

        let end_str = entry
            .end_time
            .as_deref()
            .ok_or_else(|| format!("Entry {} has no end_time", entry.id))?;
        let end = end_str
            .parse::<chrono::DateTime<chrono::Utc>>()
            .map_err(|e| format!("Bad end_time '{end_str}': {e}"))?;

        let seconds = (end - start).num_seconds().max(0);
        let year = start.format("%Y").to_string().parse::<i32>().unwrap_or(0);
        let month = start.format("%m").to_string().parse::<u32>().unwrap_or(0);

        *totals.entry((year, month)).or_insert(0) += seconds;
    }

    let monthly_totals: Vec<MonthlyTotal> = totals
        .into_iter()
        .map(|((year, month), total_seconds)| MonthlyTotal {
            year,
            month,
            total_seconds,
        })
        .collect();

    let grand_total_seconds = monthly_totals.iter().map(|m| m.total_seconds).sum();

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
        monthly_totals,
        grand_total_seconds,
        subject_label,
    })
}

// ---------------------------------------------------------------------------
// export_report_csv
// ---------------------------------------------------------------------------

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
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

    let file = std::fs::File::create(&path)
        .map_err(|e| format!("Could not create file: {e}"))?;

    let mut writer = csv::Writer::from_writer(file);

    writer
        .write_record(["Month", "Year", "Total Hours", "Total Minutes", "Total Seconds"])
        .map_err(|e| e.to_string())?;

    for row in &report.monthly_totals {
        let hours = row.total_seconds / 3600;
        let minutes = (row.total_seconds % 3600) / 60;
        let seconds = row.total_seconds % 60;
        writer
            .write_record(&[
                month_name(row.month).to_string(),
                row.year.to_string(),
                hours.to_string(),
                minutes.to_string(),
                seconds.to_string(),
            ])
            .map_err(|e| e.to_string())?;
    }

    writer
        .write_record(&[
            "Grand Total".to_string(),
            String::new(),
            String::new(),
            String::new(),
            report.grand_total_seconds.to_string(),
        ])
        .map_err(|e| e.to_string())?;

    writer.flush().map_err(|e| e.to_string())?;

    Ok(path.display().to_string())
}
