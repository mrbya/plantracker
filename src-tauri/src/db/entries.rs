use chrono::{DateTime, Duration, NaiveDate, Utc};
use sqlx::SqlitePool;

use crate::models::TimeEntry;


pub async fn insert_entry(pool: &SqlitePool, entry: &TimeEntry) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO time_entries (id, task_id, start_time, end_time, notes, created_at)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        entry.id,
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
        r#"SELECT id as "id!", task_id as "task_id!", start_time as "start_time!", end_time, notes, created_at as "created_at!" FROM time_entries WHERE end_time IS NULL LIMIT 1"#,
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
        r#"SELECT id as "id!", task_id as "task_id!", start_time as "start_time!", end_time, notes, created_at as "created_at!" FROM time_entries WHERE task_id = ? ORDER BY start_time DESC LIMIT ?"#,
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
        r#"
        SELECT te.id as "id!", te.task_id as "task_id!", te.start_time as "start_time!", te.end_time, te.notes, te.created_at as "created_at!"
        FROM time_entries te
        INNER JOIN tasks t ON t.id = te.task_id
        WHERE t.plan_id = ?
        ORDER BY te.start_time DESC
        LIMIT ?
        "#,
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
    let to_str = format!(
        "{}T00:00:00Z",
        (to + Duration::days(1)).format("%Y-%m-%d")
    );

    if let Some(tid) = task_id {
        let rows = sqlx::query_as!(
            TimeEntry,
            r#"
            SELECT id as "id!", task_id as "task_id!", start_time as "start_time!", end_time, notes, created_at as "created_at!"
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
            SELECT te.id as "id!", te.task_id as "task_id!", te.start_time as "start_time!", te.end_time, te.notes, te.created_at as "created_at!"
            FROM time_entries te
            INNER JOIN tasks t ON t.id = te.task_id
            WHERE t.plan_id = ?
              AND te.start_time >= ?
              AND te.start_time <  ?
              AND te.end_time IS NOT NULL
            ORDER BY te.start_time DESC
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
        SELECT id as "id!", task_id as "task_id!", start_time as "start_time!", end_time, notes, created_at as "created_at!"
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
        r#"SELECT id as "id!", task_id as "task_id!", start_time as "start_time!", end_time, notes, created_at as "created_at!" FROM time_entries WHERE id = ?"#,
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
