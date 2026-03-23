use sqlx::SqlitePool;

use crate::models::Plan;

pub async fn upsert_plan(pool: &SqlitePool, plan: &Plan) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO plans (id, graph_id, title, synced_at)
        VALUES (?, ?, ?, ?)
        ON CONFLICT(graph_id) DO UPDATE SET
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

pub async fn get_plan_by_graph_id(
    pool: &SqlitePool,
    graph_id: &str,
) -> anyhow::Result<Option<Plan>> {
    let row = sqlx::query_as!(
        Plan,
        r#"SELECT id as "id!", graph_id as "graph_id!", title as "title!", synced_at as "synced_at!" FROM plans WHERE graph_id = ?"#,
        graph_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::test_pool;
    use crate::models::Plan;
    use uuid::Uuid;

    fn make_plan(graph_id: &str, title: &str) -> Plan {
        Plan {
            id: Uuid::new_v4().to_string(),
            graph_id: graph_id.to_string(),
            title: title.to_string(),
            synced_at: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    #[tokio::test]
    async fn upsert_plan_insert() {
        let pool = test_pool().await;
        let plan = make_plan("g1", "My Plan");
        upsert_plan(&pool, &plan).await.unwrap();

        let fetched = get_plan(&pool, &plan.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().graph_id, "g1");
    }

    #[tokio::test]
    async fn upsert_plan_update() {
        let pool = test_pool().await;
        let plan = make_plan("g1", "Original Title");
        upsert_plan(&pool, &plan).await.unwrap();

        // Re-upsert same graph_id with a new title and synced_at
        let updated = Plan {
            id: Uuid::new_v4().to_string(), // different local id — conflict is on graph_id
            graph_id: "g1".to_string(),
            title: "Updated Title".to_string(),
            synced_at: "2024-06-01T00:00:00Z".to_string(),
        };
        upsert_plan(&pool, &updated).await.unwrap();

        let fetched = get_plan_by_graph_id(&pool, "g1").await.unwrap().unwrap();
        assert_eq!(fetched.title, "Updated Title");
        assert_eq!(fetched.synced_at, "2024-06-01T00:00:00Z");
    }

    #[tokio::test]
    async fn list_plans_empty() {
        let pool = test_pool().await;
        let plans = list_plans(&pool).await.unwrap();
        assert!(plans.is_empty());
    }

    #[tokio::test]
    async fn list_plans_multiple() {
        let pool = test_pool().await;
        let p1 = make_plan("g1", "Alpha");
        let p2 = make_plan("g2", "Beta");
        upsert_plan(&pool, &p1).await.unwrap();
        upsert_plan(&pool, &p2).await.unwrap();

        let plans = list_plans(&pool).await.unwrap();
        assert_eq!(plans.len(), 2);
        let ids: Vec<&str> = plans.iter().map(|p| p.id.as_str()).collect();
        assert!(ids.contains(&p1.id.as_str()));
        assert!(ids.contains(&p2.id.as_str()));
    }

    #[tokio::test]
    async fn get_plan_missing() {
        let pool = test_pool().await;
        let result = get_plan(&pool, "nonexistent-id").await.unwrap();
        assert!(result.is_none());
    }
}
