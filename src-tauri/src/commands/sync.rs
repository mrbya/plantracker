use std::sync::Arc;

use chrono::Utc;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

use crate::{
    auth::manager::AuthManager,
    db,
    graph::{
        client::GraphClient,
        planner::{fetch_my_plans, fetch_tasks_for_plan, fetch_user_info},
    },
    models::{Plan, Task},
};

// ---------------------------------------------------------------------------
// Read-only list commands (used by frontend to hydrate stores from SQLite)
// ---------------------------------------------------------------------------

/// Returns all plans stored in the local `SQLite` database.
///
/// This is a lightweight cache read — no Microsoft Graph request is made. It is called
/// by the frontend on application startup and after a sync to populate the plan dropdown.
///
/// # Arguments
///
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
///
/// # Returns
///
/// `Ok(plans)` — a `Vec<Plan>` ordered alphabetically by title, possibly empty.
///
/// # Errors
///
/// Returns a string error if the `SQLite` query fails.
#[tauri::command]
pub async fn list_plans(pool: State<'_, SqlitePool>) -> Result<Vec<Plan>, String> {
    db::plans::list_plans(&pool)
        .await
        .map_err(|e| e.to_string())
}

/// Returns all tasks for the given plan stored in the local `SQLite` database.
///
/// This is a lightweight cache read — no Microsoft Graph request is made. It is called
/// by the frontend whenever the user selects a different plan in the plan dropdown.
///
/// # Arguments
///
/// - `plan_id`: The local UUID of the plan whose tasks to return.
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
///
/// # Returns
///
/// `Ok(tasks)` — a `Vec<Task>` ordered alphabetically by title, possibly empty.
///
/// # Errors
///
/// Returns a string error if the `SQLite` query fails.
#[tauri::command]
pub async fn list_tasks_for_plan(
    plan_id: String,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Task>, String> {
    db::tasks::list_tasks_for_plan(&pool, &plan_id)
        .await
        .map_err(|e| e.to_string())
}

/// Summary statistics returned by [`sync_plans_and_tasks`].
///
/// Passed to the frontend as the result of the sync command so the UI can display
/// a success toast with the number of plans and tasks that were upserted.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResult {
    /// Number of plans fetched from Graph and upserted into `SQLite`.
    pub plans_count: usize,
    /// Total number of tasks fetched from Graph and upserted into `SQLite` across all plans.
    pub tasks_count: usize,
    /// ISO 8601 timestamp recorded at the start of the sync operation.
    pub synced_at: String,
}

/// Fetches plans and tasks from Microsoft Graph and upserts them into the local `SQLite` database.
///
/// The sync proceeds in the following five steps:
///
/// 1. **User info** — `GET /me` is called to fetch the signed-in user's display name.
///    The name is cached in [`AuthManager`]. Failure is non-fatal; the sync continues.
/// 2. **Plan fetch** — `GET /me/planner/plans` (with pagination) returns all plans
///    visible to the user.
/// 3. **Plan upsert** — each plan is upserted into `SQLite` with a freshly generated
///    local UUID (the conflict key is `graph_id`, so existing local UUIDs are preserved).
/// 4. **Task fetch** — for each plan, `GET /planner/plans/{id}/tasks` (with pagination)
///    returns all tasks. If fetching tasks for a specific plan fails, a warning is logged
///    and the sync continues with the remaining plans (partial-failure behaviour).
/// 5. **Task upsert** — each task is upserted into `SQLite` with the resolved local
///    plan UUID as the `plan_id` foreign key.
///
/// # Arguments
///
/// - `auth`: Tauri managed state reference to the shared [`AuthManager`].
/// - `pool`: Tauri managed state reference to the shared `SQLite` connection pool.
///
/// # Returns
///
/// `Ok(SyncResult)` containing counts of upserted plans and tasks, plus the sync timestamp.
///
/// # Errors
///
/// Returns a string error if:
/// - authentication fails or token refresh fails,
/// - the plan fetch from Graph fails, or
/// - a plan or task upsert into `SQLite` fails.
///
/// Note: task-fetch failures for individual plans are treated as warnings (logged) rather
/// than hard errors — the command will still succeed if at least the plan list is fetched.
#[tauri::command]
pub async fn sync_plans_and_tasks(
    auth: State<'_, Arc<AuthManager>>,
    pool: State<'_, SqlitePool>,
) -> Result<SyncResult, String> {
    let client = GraphClient::new(Arc::clone(&auth));
    let synced_at = Utc::now().to_rfc3339();

    // Fetch user info and cache the display name.
    match fetch_user_info(&client).await {
        Ok(user) => {
            auth.set_display_name(user.display_name).await;
        }
        Err(e) => {
            tracing::warn!("Could not fetch user info: {e}");
        }
    }

    // Fetch and upsert plans.
    let graph_plans = fetch_my_plans(&client).await.map_err(|e| e.to_string())?;
    let plans_count = graph_plans.len();

    for gp in &graph_plans {
        let plan = Plan {
            id: Uuid::new_v4().to_string(),
            graph_id: gp.id.clone(),
            title: gp.title.clone(),
            synced_at: synced_at.clone(),
        };
        db::plans::upsert_plan(&pool, &plan)
            .await
            .map_err(|e| e.to_string())?;
    }

    // Fetch and upsert tasks for every plan.
    let mut tasks_count: usize = 0;
    for gp in &graph_plans {
        let graph_tasks = match fetch_tasks_for_plan(&client, &gp.id).await {
            Ok(t) => t,
            Err(e) => {
                tracing::warn!("Failed to fetch tasks for plan {}: {e}", gp.id);
                continue;
            }
        };

        // Resolve the local plan id for FK reference.
        let local_plan = db::plans::get_plan_by_graph_id(&pool, &gp.id)
            .await
            .map_err(|e| e.to_string())?;

        let Some(local_plan) = local_plan else {
            tracing::warn!(
                "Plan {} not found in DB after upsert — skipping tasks",
                gp.id
            );
            continue;
        };

        for gt in graph_tasks {
            let task = Task {
                id: Uuid::new_v4().to_string(),
                graph_id: gt.id.clone(),
                plan_id: local_plan.id.clone(),
                title: gt.title.clone(),
                synced_at: synced_at.clone(),
            };
            db::tasks::upsert_task(&pool, &task)
                .await
                .map_err(|e| e.to_string())?;
            tasks_count = tasks_count.saturating_add(1);
        }
    }

    tracing::info!("Sync complete: {plans_count} plans, {tasks_count} tasks");

    Ok(SyncResult {
        plans_count,
        tasks_count,
        synced_at,
    })
}
