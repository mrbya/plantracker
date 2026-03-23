use chrono::Utc;
use sqlx::SqlitePool;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{db, models::TimeEntry};

// ---------------------------------------------------------------------------
// In-memory active timer state
// ---------------------------------------------------------------------------

pub struct ActiveTimer {
    pub entry_id: String,
    pub plan_id: String,
    pub task_id: Option<String>,
    pub start_time: chrono::DateTime<Utc>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveTimerInfo {
    pub entry_id: String,
    pub plan_id: String,
    pub task_id: Option<String>,
    pub start_time: String,
    pub elapsed_seconds: i64,
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn start_timer(
    plan_id: String,
    task_id: Option<String>,
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String> {
    if plan_id.is_empty() {
        return Err("plan_id must not be empty".to_string());
    }

    // Check for duplicate — extract and drop lock before any await.
    let already_running = {
        let guard = timer.lock().await;
        guard.is_some()
    };
    if already_running {
        return Err("A timer is already running".to_string());
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

#[tauri::command]
pub async fn stop_timer(
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String> {
    // Extract active timer info, then drop the lock before any await.
    let entry_id = {
        let guard = timer.lock().await;
        match guard.as_ref() {
            None => return Err("No timer is currently running".to_string()),
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
            elapsed_seconds: (Utc::now() - t.start_time).num_seconds(),
        })
    };
    Ok(info)
}

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
