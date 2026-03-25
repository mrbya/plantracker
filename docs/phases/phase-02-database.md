# Phase 2 — Database Layer

## 2.1 Configure sqlx and data directory

In `src-tauri/src/db/mod.rs`:
- Determine data directory using `tauri::path::app_data_dir()` on Linux, `documents_dir()` on Windows
- Create directory if it doesn't exist
- Return path to `plantracker.db`

## 2.2 Write migrations

Create `src-tauri/migrations/` with numbered files:

**`0001_initial.sql`**
```sql
CREATE TABLE IF NOT EXISTS plans (
    id        TEXT PRIMARY KEY,
    graph_id  TEXT NOT NULL UNIQUE,
    title     TEXT NOT NULL,
    synced_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id        TEXT PRIMARY KEY,
    graph_id  TEXT NOT NULL UNIQUE,
    plan_id   TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    title     TEXT NOT NULL,
    synced_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS time_entries (
    id         TEXT PRIMARY KEY,
    task_id    TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    start_time TEXT NOT NULL,
    end_time   TEXT,
    notes      TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_time_entries_task_id ON time_entries(task_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_start_time ON time_entries(start_time);
CREATE INDEX IF NOT EXISTS idx_tasks_plan_id ON tasks(plan_id);
```

**`0002_plan_level_entries.sql`**

Allows tracking time against a plan without a specific task (`task_id` nullable,
`plan_id` non-nullable added directly to `time_entries`). Uses the
table-recreation pattern because SQLite cannot drop NOT NULL via ALTER TABLE.

```sql
ALTER TABLE time_entries ADD COLUMN plan_id TEXT REFERENCES plans(id) ON DELETE CASCADE;
UPDATE time_entries SET plan_id = (SELECT plan_id FROM tasks WHERE tasks.id = time_entries.task_id) WHERE task_id IS NOT NULL;
CREATE TABLE time_entries_new (
    id         TEXT PRIMARY KEY,
    plan_id    TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    task_id    TEXT REFERENCES tasks(id) ON DELETE SET NULL,
    start_time TEXT NOT NULL,
    end_time   TEXT,
    notes      TEXT,
    created_at TEXT NOT NULL
);
INSERT INTO time_entries_new SELECT id, plan_id, task_id, start_time, end_time, notes, created_at FROM time_entries;
DROP TABLE time_entries;
ALTER TABLE time_entries_new RENAME TO time_entries;
CREATE INDEX IF NOT EXISTS idx_time_entries_plan_id ON time_entries(plan_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_task_id ON time_entries(task_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_start_time ON time_entries(start_time);
```

## 2.3 Database initialization

In `src-tauri/src/db/mod.rs`:
```rust
pub async fn init_db(app_handle: &tauri::AppHandle) -> anyhow::Result<SqlitePool> {
    let db_path = resolve_db_path(app_handle)?;
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display())).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
```

## 2.4 Repository functions

Create `src-tauri/src/db/plans.rs`, `tasks.rs`, `entries.rs` with these functions:

**plans.rs**
- `upsert_plan(pool, plan: &Plan) -> Result<()>`
- `list_plans(pool) -> Result<Vec<Plan>>`
- `get_plan(pool, id: &str) -> Result<Option<Plan>>`

**tasks.rs**
- `upsert_task(pool, task: &Task) -> Result<()>`
- `list_tasks_for_plan(pool, plan_id: &str) -> Result<Vec<Task>>`
- `get_task_by_graph_id(pool, graph_id: &str) -> Result<Option<Task>>`

**entries.rs**
- `insert_entry(pool, entry: &TimeEntry) -> Result<()>`
- `update_entry_end_time(pool, id: &str, end_time: DateTime<Utc>) -> Result<()>`
- `list_entries_for_task(pool, task_id: &str, limit: u32) -> Result<Vec<TimeEntry>>`
- `list_entries_for_plan(pool, plan_id: &str, limit: u32) -> Result<Vec<TimeEntry>>`
- `list_entries_in_range(pool, plan_id: Option<&str>, task_id: Option<&str>, from: NaiveDate, to: NaiveDate) -> Result<Vec<TimeEntry>>`
- `delete_entry(pool, id: &str) -> Result<()>`

## 2.5 Shared state

In `src-tauri/src/main.rs`, add `SqlitePool` to Tauri's managed state:
```rust
app.manage(db_pool);
```
Pass `State<SqlitePool>` into all command functions.

## Verification checklist
- [x] App starts and creates `plantracker.db` in correct platform directory
- [x] `sqlx migrate run` applies all migrations cleanly
- [x] `cargo sqlx prepare` generates `.sqlx/` query cache without errors
