use chrono::{DateTime, NaiveDate, Utc};
use sqlx::SqlitePool;

use crate::models::TimeEntry;

/// Inserts a new [`TimeEntry`] row into the `time_entries` table.
///
/// The `plan_id` field must reference an existing row in the `plans` table.
/// If `task_id` is `Some`, it must reference an existing row in the `tasks` table.
/// Both constraints are enforced by `SQLite` foreign keys, provided
/// `PRAGMA foreign_keys = ON` was executed after opening the pool (see [`super::init_db`]).
///
/// An entry with `end_time = None` represents an active timer. The application
/// enforces an at-most-one-active invariant at the command layer: callers must check
/// [`find_active_entry`] before inserting a new active entry.
///
/// # Arguments
///
/// - `pool`: Reference to the shared `SQLite` connection pool.
/// - `entry`: The [`TimeEntry`] to insert. Its `id` must be a fresh UUID.
///
/// # Errors
///
/// Returns an error if:
/// - the `SQLite` insert fails,
/// - `entry.plan_id` does not reference an existing plan (foreign-key violation), or
/// - `entry.task_id` is `Some` but does not reference an existing task.
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

/// Sets the `end_time` of a [`TimeEntry`], marking its timer as stopped.
///
/// After this call the row's `end_time` column is set to an ISO 8601 string
/// derived from `end_time.to_rfc3339()`. The entry will no longer be returned by
/// [`find_active_entry`], which queries for rows with `end_time IS NULL`.
///
/// This function is called exclusively by [`crate::commands::timer::stop_timer`] after
/// the user stops a running timer.
///
/// # Arguments
///
/// - `pool`: Reference to the shared `SQLite` connection pool.
/// - `id`: The local UUID primary key of the entry to update.
/// - `end_time`: The UTC timestamp to record as the end of the interval.
///
/// # Errors
///
/// Returns an error if the `SQLite` update statement fails.
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

/// Finds the currently active timer entry (the row with `end_time IS NULL`), if any.
///
/// At most one such row should exist at any given time. This function is called in two
/// contexts:
///
/// 1. **Application startup** — [`crate::run`] calls this to detect a timer that was
///    left running when the app was previously force-quit. If found, the
///    [`crate::commands::timer::ActiveTimer`] in-memory state is restored from the row,
///    ensuring the elapsed duration is computed correctly.
///
/// 2. **Timer start guard** — [`crate::commands::timer::start_timer`] calls this to
///    enforce the at-most-one-active invariant before inserting a new active entry.
///
/// # Arguments
///
/// - `pool`: Reference to the shared `SQLite` connection pool.
///
/// # Returns
///
/// - `Ok(Some(entry))` if an active timer row is found.
/// - `Ok(None)` if no row has `end_time IS NULL`.
///
/// # Errors
///
/// Returns an error if the `SQLite` query fails.
pub async fn find_active_entry(pool: &SqlitePool) -> anyhow::Result<Option<TimeEntry>> {
    let row = sqlx::query_as!(
        TimeEntry,
        r#"SELECT id as "id!", plan_id as "plan_id!", task_id, start_time as "start_time!", end_time, notes, created_at as "created_at!" FROM time_entries WHERE end_time IS NULL LIMIT 1"#,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// Returns the most recent completed entries for a given task, limited to `limit` rows.
///
/// Only rows with a non-`NULL` `end_time` are returned — active timers are excluded.
/// Results are ordered by `start_time DESC` so the most recently started entry appears
/// first. This function is called by [`crate::commands::timer::get_recent_entries`] when
/// a `task_id` is provided.
///
/// # Arguments
///
/// - `pool`: Reference to the shared `SQLite` connection pool.
/// - `task_id`: The local UUID of the task whose entries to fetch.
/// - `limit`: Maximum number of rows to return.
///
/// # Returns
///
/// `Ok(entries)` — up to `limit` completed [`TimeEntry`] rows for the given task,
/// ordered newest-first.
///
/// # Errors
///
/// Returns an error if the `SQLite` query fails.
pub async fn list_entries_for_task(
    pool: &SqlitePool,
    task_id: &str,
    limit: u32,
) -> anyhow::Result<Vec<TimeEntry>> {
    let limit = i64::from(limit);
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

/// Returns the most recent completed entries for a given plan, limited to `limit` rows.
///
/// This includes both task-level entries (where `task_id IS NOT NULL`) and plan-level
/// entries (where `task_id IS NULL`) that belong to the plan. Only rows with a non-`NULL`
/// `end_time` are returned — active timers are excluded. Results are ordered by
/// `start_time DESC`. This function is called by
/// [`crate::commands::timer::get_recent_entries`] when only a `plan_id` is provided.
///
/// # Arguments
///
/// - `pool`: Reference to the shared `SQLite` connection pool.
/// - `plan_id`: The local UUID of the plan whose entries to fetch.
/// - `limit`: Maximum number of rows to return.
///
/// # Returns
///
/// `Ok(entries)` — up to `limit` completed [`TimeEntry`] rows for the given plan,
/// ordered newest-first.
///
/// # Errors
///
/// Returns an error if the `SQLite` query fails.
pub async fn list_entries_for_plan(
    pool: &SqlitePool,
    plan_id: &str,
    limit: u32,
) -> anyhow::Result<Vec<TimeEntry>> {
    let limit = i64::from(limit);
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

/// Lists all completed entries whose `start_time` falls within an inclusive date range.
///
/// The scoping priority is: task > plan > all entries. Specifically:
/// - If `task_id` is `Some`, only entries for that task are returned (regardless of `plan_id`).
/// - If `task_id` is `None` but `plan_id` is `Some`, all entries for that plan are returned.
/// - If both are `None`, all entries in the date range are returned.
///
/// The date range is converted to ISO 8601 strings for lexicographic comparison in
/// `SQLite` (which stores datetimes as `TEXT`). Because `SQLite` compares `TEXT`
/// lexicographically, ISO 8601 format (`YYYY-MM-DDTHH:MM:SSZ`) is required for correct
/// ordering and range filtering. Active timers (`end_time IS NULL`) are always excluded.
///
/// Results are ordered by `start_time DESC`.
///
/// # Arguments
///
/// - `pool`: Reference to the shared `SQLite` connection pool.
/// - `plan_id`: Optional local UUID to scope results to a specific plan.
/// - `task_id`: Optional local UUID to scope results to a specific task (takes priority over `plan_id`).
/// - `from`: Inclusive start date of the range (converted to `YYYY-MM-DDT00:00:00Z`).
/// - `to`: Inclusive end date of the range (converted to the first moment of the following day).
///
/// # Errors
///
/// Returns an error if:
/// - `to` is the maximum representable [`NaiveDate`] (overflow when computing the next day), or
/// - any `SQLite` query fails.
pub async fn list_entries_in_range(
    pool: &SqlitePool,
    plan_id: Option<&str>,
    task_id: Option<&str>,
    from: NaiveDate,
    to: NaiveDate,
) -> anyhow::Result<Vec<TimeEntry>> {
    // Build ISO 8601 range bounds for lexicographic TEXT comparison in SQLite.
    let from_str = format!("{}T00:00:00Z", from.format("%Y-%m-%d"));
    let to_next = to
        .succ_opt()
        .ok_or_else(|| anyhow::anyhow!("Date out of range: {to}"))?;
    let to_str = format!("{}T00:00:00Z", to_next.format("%Y-%m-%d"));

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

/// Updates a time entry's `start_time`, `end_time`, and `notes` fields.
///
/// Used by [`crate::commands::entries::update_entry`] when the user edits a previously
/// recorded (completed) entry from the manual entry or report view. The `id` column is
/// not changed. Callers are responsible for validating that `end_time` is strictly after
/// `start_time` before calling this function.
///
/// # Arguments
///
/// - `pool`: Reference to the shared `SQLite` connection pool.
/// - `id`: The local UUID primary key of the entry to update.
/// - `start_time`: ISO 8601 start timestamp string.
/// - `end_time`: ISO 8601 end timestamp string.
/// - `notes`: Optional free-text notes; `None` clears any existing notes.
///
/// # Errors
///
/// Returns an error if the `SQLite` update statement fails.
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

/// Fetches a single [`TimeEntry`] by its local UUID primary key.
///
/// Used after [`update_entry_end_time`] or [`update_entry`] to return the freshly
/// modified row to the caller without a separate query, and by
/// [`crate::commands::timer::stop_timer`] to return the completed entry to the frontend.
///
/// # Arguments
///
/// - `pool`: Reference to the shared `SQLite` connection pool.
/// - `id`: The local UUID primary key of the entry to fetch.
///
/// # Returns
///
/// - `Ok(Some(entry))` if a row with the given `id` exists.
/// - `Ok(None)` if no such row is found.
///
/// # Errors
///
/// Returns an error if the `SQLite` query fails.
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

/// Deletes a time entry by its local UUID primary key.
///
/// Called by [`crate::commands::entries::delete_entry`] when the user removes an entry
/// from the UI. The deletion is permanent and cannot be undone. No cascade effects
/// occur from this deletion because no other table references `time_entries`.
///
/// # Arguments
///
/// - `pool`: Reference to the shared `SQLite` connection pool.
/// - `id`: The local UUID primary key of the entry to delete.
///
/// # Errors
///
/// Returns an error if the `SQLite` delete statement fails.
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
            plan_id: plan_id.to_owned(),
            task_id: task_id.map(str::to_owned),
            start_time: start_time.to_owned(),
            end_time: end_time.map(str::to_owned),
            notes: None,
            created_at: "2024-01-01T00:00:00Z".to_owned(),
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
        insert_entry(&pool, &entry).await.expect("insert entry");

        let fetched = get_entry(&pool, &entry.id).await.expect("get entry");
        assert!(fetched.is_some());
        assert_eq!(fetched.expect("entry should be Some").id, entry.id);
    }

    #[tokio::test]
    async fn find_active_entry_none() {
        let pool = test_pool().await;
        let result = find_active_entry(&pool).await.expect("find active entry");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn find_active_entry_some() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let entry = make_entry("p1", None, "2024-03-15T10:00:00Z", None);
        insert_entry(&pool, &entry).await.expect("insert entry");

        let result = find_active_entry(&pool).await.expect("find active entry");
        assert!(result.is_some());
        assert_eq!(result.expect("active entry should be Some").id, entry.id);
    }

    #[tokio::test]
    async fn update_entry_end_time_test() {
        let pool = test_pool().await;
        insert_plan(&pool, "p1").await;

        let entry = make_entry("p1", None, "2024-03-15T10:00:00Z", None);
        insert_entry(&pool, &entry).await.expect("insert entry");

        let end: DateTime<Utc> = "2024-03-15T11:00:00Z".parse().expect("parse end time");
        update_entry_end_time(&pool, &entry.id, end)
            .await
            .expect("update entry end time");

        assert!(find_active_entry(&pool)
            .await
            .expect("find active entry")
            .is_none());
        let fetched = get_entry(&pool, &entry.id)
            .await
            .expect("get entry")
            .expect("entry should exist after update");
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

        insert_entry(&pool, &e1).await.expect("insert e1");
        insert_entry(&pool, &e2).await.expect("insert e2");
        insert_entry(&pool, &e3).await.expect("insert e3");
        insert_entry(&pool, &e4).await.expect("insert e4");

        // limit=2 returns the 2 most recent entries for t1 in DESC order
        let results = list_entries_for_task(&pool, "t1", 2)
            .await
            .expect("list entries for task");
        assert_eq!(results.len(), 2);
        assert_eq!(results.first().expect("result[0]").id, e3.id);
        assert_eq!(results.get(1).expect("result[1]").id, e2.id);
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

        insert_entry(&pool, &e1).await.expect("insert e1");
        insert_entry(&pool, &e2).await.expect("insert e2");
        insert_entry(&pool, &e3).await.expect("insert e3");

        let results = list_entries_for_plan(&pool, "p1", 100)
            .await
            .expect("list entries for plan");
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

        insert_entry(&pool, &in_range).await.expect("insert in_range");
        insert_entry(&pool, &out_of_range).await.expect("insert out_of_range");

        let from = NaiveDate::from_ymd_opt(2024, 3, 15).expect("valid date");
        let to = NaiveDate::from_ymd_opt(2024, 3, 15).expect("valid date");
        let results = list_entries_in_range(&pool, None, Some("t1"), from, to)
            .await
            .expect("list entries in range");

        assert_eq!(results.len(), 1);
        assert_eq!(results.first().expect("result[0]").id, in_range.id);
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

        insert_entry(&pool, &in_range).await.expect("insert in_range");
        insert_entry(&pool, &out_of_range).await.expect("insert out_of_range");
        insert_entry(&pool, &other_plan).await.expect("insert other_plan");

        let from = NaiveDate::from_ymd_opt(2024, 3, 15).expect("valid date");
        let to = NaiveDate::from_ymd_opt(2024, 3, 15).expect("valid date");
        let results = list_entries_in_range(&pool, Some("p1"), None, from, to)
            .await
            .expect("list entries in range");

        assert_eq!(results.len(), 1);
        assert_eq!(results.first().expect("result[0]").id, in_range.id);
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

        insert_entry(&pool, &completed).await.expect("insert completed");
        insert_entry(&pool, &active).await.expect("insert active");

        let from = NaiveDate::from_ymd_opt(2024, 3, 15).expect("valid date");
        let to = NaiveDate::from_ymd_opt(2024, 3, 15).expect("valid date");
        let results = list_entries_in_range(&pool, Some("p1"), None, from, to)
            .await
            .expect("list entries in range");

        assert_eq!(results.len(), 1);
        assert_eq!(results.first().expect("result[0]").id, completed.id);
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
        insert_entry(&pool, &entry).await.expect("insert entry");

        delete_entry(&pool, &entry.id).await.expect("delete entry");

        let result = get_entry(&pool, &entry.id).await.expect("get entry");
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
        insert_entry(&pool, &entry).await.expect_err("should reject missing plan FK");
    }
}
