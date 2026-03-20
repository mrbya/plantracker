# Database Conventions

## Driver

All database access uses `sqlx` with SQLite. No ORMs (Diesel, SeaORM, etc.).

Always use the compile-time checked macros:
- `sqlx::query_as!(MyStruct, "SELECT ...")` — for queries that return rows
- `sqlx::query!("INSERT ...")` — for queries that return no rows or use `.fetch_one()` inline

**Never build queries by string concatenation or format!().**

## Offline Query Cache

`sqlx` checks queries against the database schema at compile time using a `.sqlx/` cache directory.

**After adding or changing any query:**
```bash
just precache
```

Commit the updated `.sqlx/` directory. CI builds without a live database will fail if the cache is stale.

## Migrations

All migrations live in `src-tauri/migrations/` and are numbered sequentially:
```
0001_initial.sql
0002_add_notes_to_entries.sql
```

Never apply migrations manually. Always use `sqlx::migrate!()` at startup:
```rust
pub async fn init_db(app_handle: &tauri::AppHandle) -> anyhow::Result<SqlitePool> {
    let path = resolve_db_path(app_handle)?;
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", path.display())).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
```

`?mode=rwc` creates the file if it does not exist.

## Foreign Keys

SQLite does not enforce foreign keys by default. Enable them at connection time in a connection hook or immediately after creating the pool:
```rust
sqlx::query!("PRAGMA foreign_keys = ON").execute(&pool).await?;
```

## Schema Reference

```sql
CREATE TABLE plans (
    id        TEXT PRIMARY KEY,          -- local UUID
    graph_id  TEXT NOT NULL UNIQUE,      -- Microsoft Graph ID
    title     TEXT NOT NULL,
    synced_at TEXT NOT NULL              -- ISO 8601
);

CREATE TABLE tasks (
    id        TEXT PRIMARY KEY,
    graph_id  TEXT NOT NULL UNIQUE,
    plan_id   TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    title     TEXT NOT NULL,
    synced_at TEXT NOT NULL
);

CREATE TABLE time_entries (
    id         TEXT PRIMARY KEY,
    task_id    TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    start_time TEXT NOT NULL,            -- ISO 8601, required
    end_time   TEXT,                     -- NULL means timer is active
    notes      TEXT,
    created_at TEXT NOT NULL
);
```

A `NULL` `end_time` signals an in-progress timer. There must be at most one such row at any time — enforce this check in the `start_timer` command before inserting.

## DateTime Queries

SQLite compares `TEXT` lexicographically. ISO 8601 (`YYYY-MM-DDTHH:MM:SSZ`) produces correct chronological ordering. Use `strftime` for grouping:

```sql
-- Monthly report grouping
SELECT strftime('%Y-%m', start_time) as month,
       SUM(unixepoch(end_time) - unixepoch(start_time)) as total_seconds
FROM time_entries
WHERE task_id = ?
  AND start_time >= ?
  AND start_time < ?
  AND end_time IS NOT NULL
GROUP BY month
ORDER BY month;
```

## Repository Layout

```
src-tauri/src/db/
├── mod.rs        ← init_db(), resolve_db_path()
├── plans.rs      ← upsert_plan(), list_plans(), get_plan()
├── tasks.rs      ← upsert_task(), list_tasks_for_plan(), get_task_by_graph_id()
└── entries.rs    ← insert_entry(), update_entry_end_time(), list_entries_for_task(),
                     list_entries_for_plan(), list_entries_in_range(), delete_entry()
```

Each file takes `&SqlitePool` as its first argument. Pool is stored in Tauri managed state and injected into commands via `tauri::State<'_, SqlitePool>`.

## Platform Data Directory

```rust
use tauri::Manager;

fn resolve_db_path(app: &tauri::AppHandle) -> anyhow::Result<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    let base = app.path().document_dir()?;

    #[cfg(not(target_os = "windows"))]
    let base = app.path().app_local_data_dir()?;
    // resolves to ~/.local/share/PlanTracker on Linux

    std::fs::create_dir_all(&base)?;
    Ok(base.join("plantracker.db"))
}
```

