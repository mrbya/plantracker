use sqlx::SqlitePool;

use crate::models::Task;

pub async fn upsert_task(pool: &SqlitePool, task: &Task) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO tasks (id, graph_id, plan_id, title, synced_at)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(graph_id) DO UPDATE SET
            plan_id   = excluded.plan_id,
            title     = excluded.title,
            synced_at = excluded.synced_at
        "#,
        task.id,
        task.graph_id,
        task.plan_id,
        task.title,
        task.synced_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_tasks_for_plan(pool: &SqlitePool, plan_id: &str) -> anyhow::Result<Vec<Task>> {
    let rows = sqlx::query_as!(
        Task,
        r#"SELECT id as "id!", graph_id as "graph_id!", plan_id as "plan_id!", title as "title!", synced_at as "synced_at!" FROM tasks WHERE plan_id = ? ORDER BY title"#,
        plan_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_task_by_graph_id(
    pool: &SqlitePool,
    graph_id: &str,
) -> anyhow::Result<Option<Task>> {
    let row = sqlx::query_as!(
        Task,
        r#"SELECT id as "id!", graph_id as "graph_id!", plan_id as "plan_id!", title as "title!", synced_at as "synced_at!" FROM tasks WHERE graph_id = ?"#,
        graph_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
