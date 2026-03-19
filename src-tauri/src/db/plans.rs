use sqlx::SqlitePool;

use crate::models::Plan;

pub async fn upsert_plan(pool: &SqlitePool, plan: &Plan) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO plans (id, graph_id, title, synced_at)
        VALUES (?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            graph_id  = excluded.graph_id,
            title     = excluded.title,
            synced_at = excluded.synced_at
        "#,
        plan.id,
        plan.graph_id,
        plan.title,
        plan.synced_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_plans(pool: &SqlitePool) -> anyhow::Result<Vec<Plan>> {
    let rows = sqlx::query_as!(
        Plan,
        r#"SELECT id as "id!", graph_id as "graph_id!", title as "title!", synced_at as "synced_at!" FROM plans ORDER BY title"#
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_plan(pool: &SqlitePool, id: &str) -> anyhow::Result<Option<Plan>> {
    let row = sqlx::query_as!(
        Plan,
        r#"SELECT id as "id!", graph_id as "graph_id!", title as "title!", synced_at as "synced_at!" FROM plans WHERE id = ?"#,
        id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}
