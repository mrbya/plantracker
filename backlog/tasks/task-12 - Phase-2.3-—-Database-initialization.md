---
id: TASK-12
title: Phase 2.3 — Database initialization
status: Done
assignee: []
created_date: '2026-03-19 09:27'
updated_date: '2026-03-19 10:54'
labels:
  - backend
  - database
  - phase-2
dependencies:
  - TASK-11
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement `init_db()` in `src-tauri/src/db/mod.rs` to open (or create) the SQLite database, run all pending migrations, and enable foreign key enforcement.

```rust
pub async fn init_db(app_handle: &tauri::AppHandle) -> anyhow::Result<SqlitePool> {
    let db_path = resolve_db_path(app_handle)?;
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display())).await?;
    sqlx::query!("PRAGMA foreign_keys = ON").execute(&pool).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
```

- `mode=rwc` creates the file if it does not exist
- `PRAGMA foreign_keys = ON` must be applied right after pool creation (SQLite does not enforce FK by default)
- Call `init_db()` in `main.rs` during Tauri setup and register the pool as managed state: `app.manage(pool)`
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 App starts and plantracker.db is created in the correct platform directory
- [x] #2 PRAGMA foreign_keys = ON is applied to the pool
- [x] #3 Migrations run automatically on startup via sqlx::migrate!()
- [x] #4 SqlitePool is registered as Tauri managed state
- [x] #5 cargo build succeeds (sqlx offline cache may need regenerating — run cargo sqlx prepare --workspace after)
<!-- AC:END -->
