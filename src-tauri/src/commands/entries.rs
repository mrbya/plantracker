use chrono::DateTime;
use sqlx::SqlitePool;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{commands::timer::ActiveTimer, db, models::TimeEntry};

// ---------------------------------------------------------------------------
// Shared validation helper
// ---------------------------------------------------------------------------

/// Parses and validates start/end time strings, ensuring end is strictly after start.
fn parse_and_validate_times(
    start_time: &str,
    end_time: &str,
) -> Result<(DateTime<chrono::Utc>, DateTime<chrono::Utc>), String> {
    let start = start_time
        .parse::<DateTime<chrono::Utc>>()
        .map_err(|_e| format!("Invalid start_time: {start_time}"))?;
    let end = end_time
        .parse::<DateTime<chrono::Utc>>()
        .map_err(|_e| format!("Invalid end_time: {end_time}"))?;
    if end <= start {
        return Err("end_time must be after start_time".to_owned());
    }
    Ok((start, end))
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Creates a time entry manually (not via the live timer).
///
/// # Errors
/// Returns a string error if validation fails, a timer is running, or the DB insert fails.
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
        return Err("plan_id must not be empty".to_owned());
    }

    let (start, _end) = parse_and_validate_times(&start_time, &end_time)?;

    // Guard: reject if a timer is currently running.
    let timer_running = {
        let guard = timer.lock().await;
        guard.is_some()
    };
    if timer_running {
        return Err("Cannot add a manual entry while a timer is running".to_owned());
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

/// Updates an existing time entry's start/end times and notes.
///
/// # Errors
/// Returns a string error if validation fails, the DB update fails, or the entry is not found.
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

/// Deletes a time entry by ID.
///
/// # Errors
/// Returns a string error if the DB delete fails.
#[tauri::command]
pub async fn delete_entry(id: String, pool: State<'_, SqlitePool>) -> Result<(), String> {
    db::entries::delete_entry(&pool, &id)
        .await
        .map_err(|e| e.to_string())?;
    tracing::info!(entry_id = %id, "Entry deleted");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_and_validate_times;

    #[test]
    fn rejects_end_before_start() {
        let result = parse_and_validate_times("2024-03-15T11:00:00Z", "2024-03-15T10:00:00Z");
        assert!(result.expect_err("should reject end before start").contains("after start_time"));
    }

    #[test]
    fn rejects_equal_times() {
        parse_and_validate_times("2024-03-15T10:00:00Z", "2024-03-15T10:00:00Z")
            .expect_err("should reject equal start and end times");
    }

    #[test]
    fn accepts_valid_range() {
        let (start, end) =
            parse_and_validate_times("2024-03-15T10:00:00Z", "2024-03-15T11:00:00Z")
                .expect("valid range should be accepted");
        assert!(end > start);
    }

    #[test]
    fn rejects_unparseable_start() {
        parse_and_validate_times("not-a-date", "2024-03-15T11:00:00Z")
            .expect_err("should reject unparseable start time");
    }
}
