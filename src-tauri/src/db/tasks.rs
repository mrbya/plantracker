use sqlx::SqlitePool;

use crate::models::Task;

/// Inserts or updates a task row (conflict on `graph_id`).
///
/// # Errors
/// Returns an error if the DB upsert fails.
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

/// Returns all tasks for the given plan, ordered by title.
///
/// # Errors
/// Returns an error if the DB query fails.
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

/// Looks up a task by its Microsoft Graph ID, returning `None` if not found.
///
/// # Errors
/// Returns an error if the DB query fails.
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

/// Looks up a task by its local primary key, returning `None` if not found.
///
/// # Errors
/// Returns an error if the DB query fails.
pub async fn get_task(pool: &SqlitePool, id: &str) -> anyhow::Result<Option<Task>> {
    let row = sqlx::query_as!(
        Task,
        r#"SELECT id as "id!", graph_id as "graph_id!", plan_id as "plan_id!", title as "title!", synced_at as "synced_at!" FROM tasks WHERE id = ?"#,
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
    use crate::models::Task;
    use uuid::Uuid;

    async fn insert_plan(pool: &SqlitePool, id: &str) {
        sqlx::query("INSERT INTO plans (id, graph_id, title, synced_at) VALUES (?, ?, ?, ?)")
            .bind(id)
            .bind(format!("g-{id}"))
            .bind("Test Plan")
            .bind("2024-01-01T00:00:00Z")
            .execute(pool)
            .await
            .expect("failed to insert plan");
    }

    fn make_task(graph_id: &str, plan_id: &str, title: &str) -> Task {
        Task {
            id: Uuid::new_v4().to_string(),
            graph_id: graph_id.to_owned(),
            plan_id: plan_id.to_owned(),
            title: title.to_owned(),
            synced_at: "2024-01-01T00:00:00Z".to_owned(),
        }
    }

    #[tokio::test]
    async fn upsert_task_insert() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let task = make_task("gt1", "p1", "My Task");
        upsert_task(&pool, &task).await.expect("upsert task");

        let fetched = get_task(&pool, &task.id).await.expect("get task");
        assert!(fetched.is_some());
        assert_eq!(fetched.expect("task should be Some").graph_id, "gt1");
    }

    #[tokio::test]
    async fn upsert_task_update() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let task = make_task("gt1", "p1", "Original Title");
        upsert_task(&pool, &task).await.expect("upsert task");

        // Re-upsert same graph_id with updated title
        let updated = Task {
            id: Uuid::new_v4().to_string(),
            graph_id: "gt1".to_owned(),
            plan_id: "p1".to_owned(),
            title: "Updated Title".to_owned(),
            synced_at: "2024-06-01T00:00:00Z".to_owned(),
        };
        upsert_task(&pool, &updated).await.expect("upsert updated task");

        let fetched = get_task_by_graph_id(&pool, "gt1")
            .await
            .expect("get task by graph id")
            .expect("task should exist");
        assert_eq!(fetched.title, "Updated Title");
        assert_eq!(fetched.synced_at, "2024-06-01T00:00:00Z");
    }

    #[tokio::test]
    async fn list_tasks_for_plan_test() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;
        insert_plan(&pool, "p2").await;

        let t1 = make_task("gt1", "p1", "Task A");
        let t2 = make_task("gt2", "p1", "Task B");
        let t3 = make_task("gt3", "p2", "Task C"); // different plan — must not appear

        upsert_task(&pool, &t1).await.expect("upsert t1");
        upsert_task(&pool, &t2).await.expect("upsert t2");
        upsert_task(&pool, &t3).await.expect("upsert t3");

        let results = list_tasks_for_plan(&pool, "p1").await.expect("list tasks for plan");
        assert_eq!(results.len(), 2);
        let ids: Vec<&str> = results.iter().map(|t| t.id.as_str()).collect();
        assert!(ids.contains(&t1.id.as_str()));
        assert!(ids.contains(&t2.id.as_str()));
    }

    #[tokio::test]
    async fn get_task_by_graph_id_test() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let task = make_task("gt1", "p1", "My Task");
        upsert_task(&pool, &task).await.expect("upsert task");

        let fetched = get_task_by_graph_id(&pool, "gt1").await.expect("get task by graph id");
        assert!(fetched.is_some());
        assert_eq!(fetched.expect("task should be Some").id, task.id);
    }

    #[tokio::test]
    async fn cascade_delete() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let task = make_task("gt1", "p1", "Will Be Deleted");
        upsert_task(&pool, &task).await.expect("upsert task");

        // Deleting the plan must cascade-delete the task
        sqlx::query("DELETE FROM plans WHERE id = ?")
            .bind("p1")
            .execute(&pool)
            .await
            .expect("delete plan");

        let result = get_task(&pool, &task.id).await.expect("get task");
        assert!(result.is_none());
    }
}
