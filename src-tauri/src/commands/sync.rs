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

#[tauri::command]
pub async fn list_plans(pool: State<'_, SqlitePool>) -> Result<Vec<Plan>, String> {
    db::plans::list_plans(&pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_tasks_for_plan(
    plan_id: String,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Task>, String> {
    db::tasks::list_tasks_for_plan(&pool, &plan_id)
        .await
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResult {
    pub plans_count: usize,
    pub tasks_count: usize,
    pub synced_at: String,
}

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
    let mut tasks_count = 0usize;
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
            tracing::warn!("Plan {} not found in DB after upsert — skipping tasks", gp.id);
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
            tasks_count += 1;
        }
    }

    tracing::info!("Sync complete: {plans_count} plans, {tasks_count} tasks");

    Ok(SyncResult {
        plans_count,
        tasks_count,
        synced_at,
    })
}
