use chrono::DateTime;
use sqlx::SqlitePool;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{commands::timer::ActiveTimer, db, models::TimeEntry};

// ---------------------------------------------------------------------------
// Shared validation helper
// ---------------------------------------------------------------------------

fn parse_and_validate_times(
    start_time: &str,
    end_time: &str,
) -> Result<(DateTime<chrono::Utc>, DateTime<chrono::Utc>), String> {
    let start = start_time
        .parse::<DateTime<chrono::Utc>>()
        .map_err(|_| format!("Invalid start_time: {start_time}"))?;
    let end = end_time
        .parse::<DateTime<chrono::Utc>>()
        .map_err(|_| format!("Invalid end_time: {end_time}"))?;
    if end <= start {
        return Err("end_time must be after start_time".to_string());
    }
    Ok((start, end))
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn create_manual_entry(
    plan_id: String,
    task_id: Option<String>,
    start_time: String,
    end_time: String,
    notes: Option<String>,
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String> {
    if plan_id.is_empty() {
        return Err("plan_id must not be empty".to_string());
    }

    let (start, _end) = parse_and_validate_times(&start_time, &end_time)?;

    // Guard: reject if a timer is currently running.
    let timer_running = {
        let guard = timer.lock().await;
        guard.is_some()
    };
    if timer_running {
        return Err("Cannot add a manual entry while a timer is running".to_string());
    }

    let entry = TimeEntry {
        id: Uuid::new_v4().to_string(),
        plan_id,
        task_id,
        start_time: start.to_rfc3339(),
        end_time: Some(end_time),
        notes,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    db::entries::insert_entry(&pool, &entry)
        .await
        .map_err(|e| e.to_string())?;

    tracing::info!(entry_id = %entry.id, "Manual entry created");
    Ok(entry)
}

#[tauri::command]
pub async fn update_entry(
    id: String,
    start_time: String,
    end_time: String,
    notes: Option<String>,
    pool: State<'_, SqlitePool>,
) -> Result<TimeEntry, String> {
    parse_and_validate_times(&start_time, &end_time)?;

    db::entries::update_entry(&pool, &id, &start_time, &end_time, notes.as_deref())
        .await
        .map_err(|e| e.to_string())?;

    let entry = db::entries::get_entry(&pool, &id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Entry {id} not found after update"))?;

    tracing::info!(entry_id = %id, "Entry updated");
    Ok(entry)
}

#[tauri::command]
pub async fn delete_entry(id: String, pool: State<'_, SqlitePool>) -> Result<(), String> {
    db::entries::delete_entry(&pool, &id)
        .await
        .map_err(|e| e.to_string())?;
    tracing::info!(entry_id = %id, "Entry deleted");
    Ok(())
}
