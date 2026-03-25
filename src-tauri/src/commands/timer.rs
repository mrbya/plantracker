use chrono::Utc;
use sqlx::SqlitePool;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{db, models::TimeEntry};

// ---------------------------------------------------------------------------
// In-memory active timer state
// ---------------------------------------------------------------------------

/// In-memory representation of the currently running timer.
///
/// An `ActiveTimer` is stored inside `Mutex<Option<ActiveTimer>>` registered as Tauri
/// managed state. The `Option` wrapper means `None` when no timer is running and
/// `Some(ActiveTimer)` when one is active.
///
/// At application startup, [`crate::run`] queries `SQLite` for any `time_entries` row
/// with `end_time IS NULL`. If found, it reconstructs an `ActiveTimer` from that row so
/// the in-memory state is consistent with the database after a force-quit restart.
///
/// The struct is intentionally not `Serialize` — it is never sent to the frontend
/// directly. Instead, [`ActiveTimerInfo`] is constructed from it on demand inside
/// [`get_active_timer`] with `elapsed_seconds` computed at call time.
pub struct ActiveTimer {
    /// ID of the `time_entries` row created when the timer was started.
    ///
    /// Used by [`stop_timer`] to locate the row that needs its `end_time` filled in.
    pub entry_id: String,
    /// Local UUID of the plan being tracked.
    pub plan_id: String,
    /// Local UUID of the task being tracked, or `None` for plan-level entries.
    pub task_id: Option<String>,
    /// UTC timestamp when the timer was started, used to compute `elapsed_seconds`.
    pub start_time: chrono::DateTime<Utc>,
}

/// Serialisable snapshot of the active timer returned to the frontend.
///
/// Constructed from [`ActiveTimer`] on every call to [`get_active_timer`]. The
/// `elapsed_seconds` field is computed at the moment the command is invoked, so it
/// always reflects how long the timer has been running up to that instant.
///
/// The frontend uses this struct to display a live elapsed-time counter and to know
/// which plan/task is currently being tracked.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveTimerInfo {
    /// ID of the associated `time_entries` row.
    pub entry_id: String,
    /// Local UUID of the plan being tracked.
    pub plan_id: String,
    /// Local UUID of the task being tracked, or `None` for plan-level entries.
    pub task_id: Option<String>,
    /// ISO 8601 start timestamp (stored in the `time_entries` row).
    pub start_time: String,
    /// Number of whole seconds elapsed since `start_time`, computed at call time.
    pub elapsed_seconds: i64,
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Starts a new timer for the given plan (and optionally a specific task).
///
/// Enforces the at-most-one-active timer invariant by checking the in-memory
/// `Mutex<Option<ActiveTimer>>` before inserting. If a timer is already running the
/// command returns an error rather than inserting a duplicate active entry.
///
/// The `time_entries` row is inserted first (with `end_time = NULL`) and the in-memory
/// state is updated after the database write succeeds. This ordering ensures that if
/// the application crashes between the insert and the state update, the row is still
/// in `SQLite` and can be recovered on the next startup.
///
/// # Arguments
///
/// - `plan_id`: The local UUID of the plan to track. Must not be empty.
/// - `task_id`: Optional local UUID of the task to track. `None` creates a plan-level entry.
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
/// - `timer`: Tauri managed state reference to the `Mutex<Option<ActiveTimer>>`.
///
/// # Returns
///
/// `Ok(TimeEntry)` — the newly created entry row (with `end_time = None`).
///
/// # Errors
///
/// Returns a string error if:
/// - `plan_id` is empty,
/// - a timer is already running (in-memory check), or
/// - the `SQLite` insert fails (e.g. foreign-key violation if `plan_id` does not exist).
///
/// # Mutex safety
///
/// The lock is acquired briefly to check whether a timer is running, then released
/// before the `SQLite` `.await`. It is acquired again after the DB write to update the
/// state. No lock is held across any `.await` point.
#[tauri::command]
pub async fn start_timer(
    plan_id: String,
    task_id: Option<String>,
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String> {
    if plan_id.is_empty() {
        return Err("plan_id must not be empty".to_owned());
    }

    // Check for duplicate — extract and drop lock before any await.
    let already_running = {
        let guard = timer.lock().await;
        guard.is_some()
    };
    if already_running {
        return Err("A timer is already running".to_owned());
    }

    let now = Utc::now();
    let entry = TimeEntry {
        id: Uuid::new_v4().to_string(),
        plan_id: plan_id.clone(),
        task_id: task_id.clone(),
        start_time: now.to_rfc3339(),
        end_time: None,
        notes: None,
        created_at: now.to_rfc3339(),
    };

    db::entries::insert_entry(&pool, &entry)
        .await
        .map_err(|e| e.to_string())?;

    // Store in managed state — re-acquire lock after the DB await.
    *timer.lock().await = Some(ActiveTimer {
        entry_id: entry.id.clone(),
        plan_id: plan_id.clone(),
        task_id: task_id.clone(),
        start_time: now,
    });

    tracing::info!(entry_id = %entry.id, plan_id = %plan_id, task_id = ?task_id, "Timer started");
    Ok(entry)
}

/// Stops the currently running timer and records the end time.
///
/// Reads the `entry_id` from the in-memory [`ActiveTimer`], writes the current UTC
/// time as `end_time` to `SQLite`, clears the in-memory state, and then fetches the
/// completed entry from `SQLite` to return to the frontend with the accurate timestamps.
///
/// # Arguments
///
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
/// - `timer`: Tauri managed state reference to the `Mutex<Option<ActiveTimer>>`.
///
/// # Returns
///
/// `Ok(TimeEntry)` — the completed entry row with `end_time` set.
///
/// # Errors
///
/// Returns a string error if:
/// - no timer is currently running (in-memory state is `None`),
/// - the `SQLite` update fails, or
/// - the entry cannot be fetched from `SQLite` after the update.
///
/// # Mutex safety
///
/// The lock is acquired to read `entry_id`, then released before the `SQLite` `.await`.
/// It is re-acquired after the DB write to clear the state.
#[tauri::command]
pub async fn stop_timer(
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String> {
    // Extract active timer info, then drop the lock before any await.
    let entry_id = {
        let guard = timer.lock().await;
        match guard.as_ref() {
            None => return Err("No timer is currently running".to_owned()),
            Some(t) => t.entry_id.clone(),
        }
    };

    let end_time = Utc::now();
    db::entries::update_entry_end_time(&pool, &entry_id, end_time)
        .await
        .map_err(|e| e.to_string())?;

    // Clear managed state — re-acquire lock after the DB await.
    *timer.lock().await = None;

    // Fetch the completed entry from DB for an accurate return value.
    let entry = db::entries::get_entry(&pool, &entry_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Entry {entry_id} not found after stop"))?;

    tracing::info!(entry_id = %entry_id, "Timer stopped");
    Ok(entry)
}

/// Returns the currently active timer's state, or `None` if no timer is running.
///
/// Reads entirely from the in-memory `Mutex<Option<ActiveTimer>>` — no `SQLite` query
/// is made. This makes the command lightweight and safe to call on every frontend tick
/// (e.g. to update an elapsed-seconds display). The `elapsed_seconds` value is computed
/// freshly on each call as `Utc::now() - start_time`.
///
/// # Arguments
///
/// - `timer`: Tauri managed state reference to the `Mutex<Option<ActiveTimer>>`.
///
/// # Returns
///
/// - `Ok(Some(ActiveTimerInfo))` if a timer is running, with `elapsed_seconds` set to the
///   number of whole seconds elapsed since the timer started.
/// - `Ok(None)` if no timer is running.
///
/// # Errors
///
/// This command is infallible in practice; the `Err` variant is never returned.
///
/// # Mutex safety
///
/// The lock is acquired, the snapshot is computed synchronously (no `.await` inside the
/// lock), and the lock is released before the function returns.
#[tauri::command]
pub async fn get_active_timer(
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<Option<ActiveTimerInfo>, String> {
    // Extract what we need, then drop the lock — no awaits needed after this.
    let info = {
        let guard = timer.lock().await;
        guard.as_ref().map(|t| ActiveTimerInfo {
            entry_id: t.entry_id.clone(),
            plan_id: t.plan_id.clone(),
            task_id: t.task_id.clone(),
            start_time: t.start_time.to_rfc3339(),
            elapsed_seconds: Utc::now().signed_duration_since(t.start_time).num_seconds(),
        })
    };
    Ok(info)
}

/// Returns the most recent completed time entries for the given task or plan.
///
/// The scoping priority mirrors [`crate::db::entries::list_entries_in_range`]:
/// - If `task_id` is `Some`, entries are scoped to that task.
/// - If `task_id` is `None` but `plan_id` is `Some`, entries are scoped to that plan.
/// - If both are `None`, an empty `Vec` is returned (no global listing is provided
///   by this command).
///
/// Only completed entries (`end_time IS NOT NULL`) are returned, ordered newest-first.
///
/// # Arguments
///
/// - `task_id`: Optional local UUID of the task to scope results to.
/// - `plan_id`: Optional local UUID of the plan to scope results to (used only when
///   `task_id` is `None`).
/// - `limit`: Maximum number of entries to return.
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
///
/// # Returns
///
/// `Ok(entries)` — up to `limit` completed [`TimeEntry`] rows, ordered by
/// `start_time DESC`.
///
/// # Errors
///
/// Returns a string error if the `SQLite` query fails.
#[tauri::command]
pub async fn get_recent_entries(
    task_id: Option<String>,
    plan_id: Option<String>,
    limit: u32,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<TimeEntry>, String> {
    if let Some(ref tid) = task_id {
        return db::entries::list_entries_for_task(&pool, tid, limit)
            .await
            .map_err(|e| e.to_string());
    }
    if let Some(ref pid) = plan_id {
        return db::entries::list_entries_for_plan(&pool, pid, limit)
            .await
            .map_err(|e| e.to_string());
    }
    Ok(vec![])
}
