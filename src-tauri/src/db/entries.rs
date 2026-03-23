use chrono::{DateTime, Duration, NaiveDate, Utc};
use sqlx::SqlitePool;

use crate::models::TimeEntry;

pub async fn insert_entry(pool: &SqlitePool, entry: &TimeEntry) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO time_entries (id, plan_id, task_id, start_time, end_time, notes, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        entry.id,
        entry.plan_id,
        entry.task_id,
        entry.start_time,
        entry.end_time,
        entry.notes,
        entry.created_at,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_entry_end_time(
    pool: &SqlitePool,
    id: &str,
    end_time: DateTime<Utc>,
) -> anyhow::Result<()> {
    let end_time_str = end_time.to_rfc3339();
    sqlx::query!(
        "UPDATE time_entries SET end_time = ? WHERE id = ?",
        end_time_str,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn find_active_entry(pool: &SqlitePool) -> anyhow::Result<Option<TimeEntry>> {
    let row = sqlx::query_as!(
        TimeEntry,
        r#"SELECT id as "id!", plan_id as "plan_id!", task_id, start_time as "start_time!", end_time, notes, created_at as "created_at!" FROM time_entries WHERE end_time IS NULL LIMIT 1"#,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn list_entries_for_task(
    pool: &SqlitePool,
    task_id: &str,
    limit: u32,
) -> anyhow::Result<Vec<TimeEntry>> {
    let limit = limit as i64;
    let rows = sqlx::query_as!(
        TimeEntry,
        r#"SELECT id as "id!", plan_id as "plan_id!", task_id, start_time as "start_time!", end_time, notes, created_at as "created_at!" FROM time_entries WHERE task_id = ? ORDER BY start_time DESC LIMIT ?"#,
        task_id,
        limit,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn list_entries_for_plan(
    pool: &SqlitePool,
    plan_id: &str,
    limit: u32,
) -> anyhow::Result<Vec<TimeEntry>> {
    let limit = limit as i64;
    let rows = sqlx::query_as!(
        TimeEntry,
        r#"SELECT id as "id!", plan_id as "plan_id!", task_id, start_time as "start_time!", end_time, notes, created_at as "created_at!" FROM time_entries WHERE plan_id = ? ORDER BY start_time DESC LIMIT ?"#,
        plan_id,
        limit,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// List completed entries within an inclusive date range.
/// Filters by `task_id` if provided, otherwise by `plan_id` if provided,
/// otherwise returns all entries in the range.
pub async fn list_entries_in_range(
    pool: &SqlitePool,
    plan_id: Option<&str>,
    task_id: Option<&str>,
    from: NaiveDate,
    to: NaiveDate,
) -> anyhow::Result<Vec<TimeEntry>> {
    // Build ISO 8601 range bounds for lexicographic TEXT comparison in SQLite.
    let from_str = format!("{}T00:00:00Z", from.format("%Y-%m-%d"));
    let to_str = format!("{}T00:00:00Z", (to + Duration::days(1)).format("%Y-%m-%d"));

    if let Some(tid) = task_id {
        let rows = sqlx::query_as!(
            TimeEntry,
            r#"
            SELECT id as "id!", plan_id as "plan_id!", task_id, start_time as "start_time!", end_time, notes, created_at as "created_at!"
            FROM time_entries
            WHERE task_id = ?
              AND start_time >= ?
              AND start_time <  ?
              AND end_time IS NOT NULL
            ORDER BY start_time DESC
            "#,
            tid,
            from_str,
            to_str,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    if let Some(pid) = plan_id {
        let rows = sqlx::query_as!(
            TimeEntry,
            r#"
            SELECT id as "id!", plan_id as "plan_id!", task_id, start_time as "start_time!", end_time, notes, created_at as "created_at!"
            FROM time_entries
            WHERE plan_id = ?
              AND start_time >= ?
              AND start_time <  ?
              AND end_time IS NOT NULL
            ORDER BY start_time DESC
            "#,
            pid,
            from_str,
            to_str,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    let rows = sqlx::query_as!(
        TimeEntry,
        r#"
        SELECT id as "id!", plan_id as "plan_id!", task_id, start_time as "start_time!", end_time, notes, created_at as "created_at!"
        FROM time_entries
        WHERE start_time >= ?
          AND start_time <  ?
          AND end_time IS NOT NULL
        ORDER BY start_time DESC
        "#,
        from_str,
        to_str,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn update_entry(
    pool: &SqlitePool,
    id: &str,
    start_time: &str,
    end_time: &str,
    notes: Option<&str>,
) -> anyhow::Result<()> {
    sqlx::query!(
        "UPDATE time_entries SET start_time = ?, end_time = ?, notes = ? WHERE id = ?",
        start_time,
        end_time,
        notes,
        id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_entry(pool: &SqlitePool, id: &str) -> anyhow::Result<Option<TimeEntry>> {
    let row = sqlx::query_as!(
        TimeEntry,
        r#"SELECT id as "id!", plan_id as "plan_id!", task_id, start_time as "start_time!", end_time, notes, created_at as "created_at!" FROM time_entries WHERE id = ?"#,
        id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn delete_entry(pool: &SqlitePool, id: &str) -> anyhow::Result<()> {
    sqlx::query!("DELETE FROM time_entries WHERE id = ?", id)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::test_pool;
    use crate::models::TimeEntry;
    use chrono::NaiveDate;
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

    async fn insert_task(pool: &SqlitePool, id: &str, plan_id: &str) {
        sqlx::query(
            "INSERT INTO tasks (id, graph_id, plan_id, title, synced_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(format!("g-{id}"))
        .bind(plan_id)
        .bind("Test Task")
        .bind("2024-01-01T00:00:00Z")
        .execute(pool)
        .await
        .expect("failed to insert task");
    }

    fn make_entry(
        plan_id: &str,
        task_id: Option<&str>,
        start_time: &str,
        end_time: Option<&str>,
    ) -> TimeEntry {
        TimeEntry {
            id: Uuid::new_v4().to_string(),
            plan_id: plan_id.to_string(),
            task_id: task_id.map(str::to_string),
            start_time: start_time.to_string(),
            end_time: end_time.map(str::to_string),
            notes: None,
            created_at: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    #[tokio::test]
    async fn insert_and_fetch_entry() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let entry = make_entry(
            "p1",
            None,
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );
        insert_entry(&pool, &entry).await.unwrap();

        let fetched = get_entry(&pool, &entry.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, entry.id);
    }

    #[tokio::test]
    async fn find_active_entry_none() {
        let pool = test_pool().await;
        let result = find_active_entry(&pool).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn find_active_entry_some() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let entry = make_entry("p1", None, "2024-03-15T10:00:00Z", None);
        insert_entry(&pool, &entry).await.unwrap();

        let result = find_active_entry(&pool).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, entry.id);
    }

    #[tokio::test]
    async fn update_entry_end_time_test() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let entry = make_entry("p1", None, "2024-03-15T10:00:00Z", None);
        insert_entry(&pool, &entry).await.unwrap();

        let end: DateTime<Utc> = "2024-03-15T11:00:00Z".parse().unwrap();
        update_entry_end_time(&pool, &entry.id, end).await.unwrap();

        assert!(find_active_entry(&pool).await.unwrap().is_none());
        let fetched = get_entry(&pool, &entry.id).await.unwrap().unwrap();
        assert!(fetched.end_time.is_some());
    }

    #[tokio::test]
    async fn list_entries_for_task_test() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;
        insert_task(&pool, "t1", "p1").await;
        insert_task(&pool, "t2", "p1").await;

        // Three entries for t1 with distinct start times
        let e1 = make_entry(
            "p1",
            Some("t1"),
            "2024-03-15T08:00:00Z",
            Some("2024-03-15T09:00:00Z"),
        );
        let e2 = make_entry(
            "p1",
            Some("t1"),
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );
        let e3 = make_entry(
            "p1",
            Some("t1"),
            "2024-03-15T12:00:00Z",
            Some("2024-03-15T13:00:00Z"),
        );
        // Entry for t2 — must not appear in t1 results
        let e4 = make_entry(
            "p1",
            Some("t2"),
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );

        insert_entry(&pool, &e1).await.unwrap();
        insert_entry(&pool, &e2).await.unwrap();
        insert_entry(&pool, &e3).await.unwrap();
        insert_entry(&pool, &e4).await.unwrap();

        // limit=2 returns the 2 most recent entries for t1 in DESC order
        let results = list_entries_for_task(&pool, "t1", 2).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, e3.id);
        assert_eq!(results[1].id, e2.id);
    }

    #[tokio::test]
    async fn list_entries_for_plan_test() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;
        insert_plan(&pool, "p2").await;
        insert_task(&pool, "t1", "p1").await;

        // Task-level entry for p1
        let e1 = make_entry(
            "p1",
            Some("t1"),
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );
        // Taskless (plan-level) entry for p1
        let e2 = make_entry(
            "p1",
            None,
            "2024-03-15T12:00:00Z",
            Some("2024-03-15T13:00:00Z"),
        );
        // Entry for a different plan — must not appear
        let e3 = make_entry(
            "p2",
            None,
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );

        insert_entry(&pool, &e1).await.unwrap();
        insert_entry(&pool, &e2).await.unwrap();
        insert_entry(&pool, &e3).await.unwrap();

        let results = list_entries_for_plan(&pool, "p1", 100).await.unwrap();
        assert_eq!(results.len(), 2);
        let ids: Vec<&str> = results.iter().map(|e| e.id.as_str()).collect();
        assert!(ids.contains(&e1.id.as_str()));
        assert!(ids.contains(&e2.id.as_str()));
    }

    #[tokio::test]
    async fn list_entries_in_range_by_task() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;
        insert_task(&pool, "t1", "p1").await;

        let in_range = make_entry(
            "p1",
            Some("t1"),
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );
        let out_of_range = make_entry(
            "p1",
            Some("t1"),
            "2024-04-01T10:00:00Z",
            Some("2024-04-01T11:00:00Z"),
        );

        insert_entry(&pool, &in_range).await.unwrap();
        insert_entry(&pool, &out_of_range).await.unwrap();

        let from = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let results = list_entries_in_range(&pool, None, Some("t1"), from, to)
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, in_range.id);
    }

    #[tokio::test]
    async fn list_entries_in_range_by_plan() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;
        insert_plan(&pool, "p2").await;

        let in_range = make_entry(
            "p1",
            None,
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );
        let out_of_range = make_entry(
            "p1",
            None,
            "2024-04-01T10:00:00Z",
            Some("2024-04-01T11:00:00Z"),
        );
        let other_plan = make_entry(
            "p2",
            None,
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );

        insert_entry(&pool, &in_range).await.unwrap();
        insert_entry(&pool, &out_of_range).await.unwrap();
        insert_entry(&pool, &other_plan).await.unwrap();

        let from = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let results = list_entries_in_range(&pool, Some("p1"), None, from, to)
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, in_range.id);
    }

    #[tokio::test]
    async fn list_entries_in_range_excludes_active() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let completed = make_entry(
            "p1",
            None,
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );
        // Active entry (end_time IS NULL) — must be excluded from range results
        let active = make_entry("p1", None, "2024-03-15T12:00:00Z", None);

        insert_entry(&pool, &completed).await.unwrap();
        insert_entry(&pool, &active).await.unwrap();

        let from = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let results = list_entries_in_range(&pool, Some("p1"), None, from, to)
            .await
            .unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, completed.id);
    }

    #[tokio::test]
    async fn delete_entry_test() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let entry = make_entry(
            "p1",
            None,
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );
        insert_entry(&pool, &entry).await.unwrap();

        delete_entry(&pool, &entry.id).await.unwrap();

        let result = get_entry(&pool, &entry.id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fk_constraint_fires() {
        let pool = test_pool().await;
        let entry = make_entry(
            "nonexistent-plan",
            None,
            "2024-03-15T10:00:00Z",
            Some("2024-03-15T11:00:00Z"),
        );
        let result = insert_entry(&pool, &entry).await;
        assert!(result.is_err());
    }
}
