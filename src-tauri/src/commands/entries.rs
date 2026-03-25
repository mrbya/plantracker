use chrono::DateTime;
use sqlx::SqlitePool;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{commands::timer::ActiveTimer, db, models::TimeEntry};

// ---------------------------------------------------------------------------
// Shared validation helper
// ---------------------------------------------------------------------------

/// Parses and validates a pair of ISO 8601 timestamp strings for a time entry.
///
/// Both strings are parsed as [`DateTime<chrono::Utc>`] and the relationship between
/// them is checked. This helper is called by both [`create_manual_entry`] and
/// [`update_entry`] before any database write.
///
/// # Arguments
///
/// - `start_time`: ISO 8601 timestamp string for the start of the interval.
/// - `end_time`: ISO 8601 timestamp string for the end of the interval.
///
/// # Returns
///
/// `Ok((start, end))` — the parsed timestamps as [`DateTime<chrono::Utc>`] values.
///
/// # Errors
///
/// Returns a string error if:
/// - `start_time` cannot be parsed as a valid `DateTime<Utc>`,
/// - `end_time` cannot be parsed as a valid `DateTime<Utc>`, or
/// - `end_time` is equal to or before `start_time` (durations of zero or negative length
///   are not permitted).
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

/// Creates a completed time entry manually, bypassing the live timer.
///
/// This command is used by the Manual Entry view to record time that was not tracked
/// in real time. It validates the provided timestamps, checks that no timer is currently
/// running (to avoid ambiguous state), creates a new [`TimeEntry`] with a fresh UUID,
/// and inserts it into `SQLite`.
///
/// # Arguments
///
/// - `plan_id`: Local UUID of the plan. Must not be empty.
/// - `task_id`: Optional local UUID of the task. `None` creates a plan-level entry.
/// - `start_time`: ISO 8601 start timestamp string.
/// - `end_time`: ISO 8601 end timestamp string.
/// - `notes`: Optional free-text notes.
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
/// - `timer`: Tauri managed state reference to the `Mutex<Option<ActiveTimer>>`.
///
/// # Returns
///
/// `Ok(TimeEntry)` — the newly created entry row.
///
/// # Errors
///
/// Returns a string error if:
/// - `plan_id` is empty,
/// - time validation fails (see [`parse_and_validate_times`]),
/// - a timer is currently running (manual entries are rejected while a timer is active), or
/// - the `SQLite` insert fails (e.g. `plan_id` foreign-key violation).
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
/// Validates the new timestamps before writing to ensure `end_time` is strictly after
/// `start_time`. After the update, the modified row is fetched from `SQLite` and
/// returned so the frontend always receives the persisted values.
///
/// This command only updates `start_time`, `end_time`, and `notes`. The `plan_id`,
/// `task_id`, and `created_at` fields cannot be changed via this command.
///
/// # Arguments
///
/// - `id`: Local UUID of the entry to update.
/// - `start_time`: New ISO 8601 start timestamp string.
/// - `end_time`: New ISO 8601 end timestamp string.
/// - `notes`: New optional free-text notes. `None` clears any existing notes.
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
///
/// # Returns
///
/// `Ok(TimeEntry)` — the entry row as it exists in `SQLite` after the update.
///
/// # Errors
///
/// Returns a string error if:
/// - time validation fails (see [`parse_and_validate_times`]),
/// - the `SQLite` update fails, or
/// - the entry is not found after the update (the ID does not exist).
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

/// Deletes a time entry by its local UUID.
///
/// Permanently removes the `time_entries` row with the given `id`. The deletion is
/// not reversible. No cascade effects occur from this deletion because no other table
/// references `time_entries` with a foreign key.
///
/// # Arguments
///
/// - `id`: Local UUID of the entry to delete.
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
///
/// # Returns
///
/// `Ok(())` on success.
///
/// # Errors
///
/// Returns a string error if the `SQLite` delete statement fails.
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
        assert!(result
            .expect_err("should reject end before start")
            .contains("after start_time"));
    }

    #[test]
    fn rejects_equal_times() {
        parse_and_validate_times("2024-03-15T10:00:00Z", "2024-03-15T10:00:00Z")
            .expect_err("should reject equal start and end times");
    }

    #[test]
    fn accepts_valid_range() {
        let (start, end) = parse_and_validate_times("2024-03-15T10:00:00Z", "2024-03-15T11:00:00Z")
            .expect("valid range should be accepted");
        assert!(end > start);
    }

    #[test]
    fn rejects_unparseable_start() {
        parse_and_validate_times("not-a-date", "2024-03-15T11:00:00Z")
            .expect_err("should reject unparseable start time");
    }
}
