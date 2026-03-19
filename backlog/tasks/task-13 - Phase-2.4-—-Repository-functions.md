---
id: TASK-13
title: Phase 2.4 — Repository functions
status: Done
assignee: []
created_date: '2026-03-19 09:27'
updated_date: '2026-03-19 11:00'
labels:
  - backend
  - database
  - phase-2
dependencies:
  - TASK-12
priority: high
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the three database repository modules, each taking `&SqlitePool` as their first argument. Use `sqlx::query_as!` for row-returning queries and `sqlx::query!` for mutations. Never build queries via string concatenation.

**`src-tauri/src/db/plans.rs`**
- `upsert_plan(pool: &SqlitePool, plan: &Plan) -> anyhow::Result<()>` — INSERT OR REPLACE
- `list_plans(pool: &SqlitePool) -> anyhow::Result<Vec<Plan>>`
- `get_plan(pool: &SqlitePool, id: &str) -> anyhow::Result<Option<Plan>>`

**`src-tauri/src/db/tasks.rs`**
- `upsert_task(pool: &SqlitePool, task: &Task) -> anyhow::Result<()>` — INSERT OR REPLACE
- `list_tasks_for_plan(pool: &SqlitePool, plan_id: &str) -> anyhow::Result<Vec<Task>>`
- `get_task_by_graph_id(pool: &SqlitePool, graph_id: &str) -> anyhow::Result<Option<Task>>`

**`src-tauri/src/db/entries.rs`**
- `insert_entry(pool: &SqlitePool, entry: &TimeEntry) -> anyhow::Result<()>`
- `update_entry_end_time(pool: &SqlitePool, id: &str, end_time: DateTime<Utc>) -> anyhow::Result<()>` — store as `.to_rfc3339()`
- `find_active_entry(pool: &SqlitePool) -> anyhow::Result<Option<TimeEntry>>` — WHERE end_time IS NULL
- `list_entries_for_task(pool: &SqlitePool, task_id: &str, limit: u32) -> anyhow::Result<Vec<TimeEntry>>`
- `list_entries_for_plan(pool: &SqlitePool, plan_id: &str, limit: u32) -> anyhow::Result<Vec<TimeEntry>>`
- `list_entries_in_range(pool: &SqlitePool, plan_id: Option<&str>, task_id: Option<&str>, from: NaiveDate, to: NaiveDate) -> anyhow::Result<Vec<TimeEntry>>`
- `delete_entry(pool: &SqlitePool, id: &str) -> anyhow::Result<()>`

**Expose all three modules from `src-tauri/src/db/mod.rs`** via `pub mod plans; pub mod tasks; pub mod entries;`

After writing all queries, run `cargo sqlx prepare --workspace` and commit the updated `.sqlx/` cache.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All three repository files exist and compile cleanly
- [x] #2 sqlx compile-time query macros are used throughout — no raw string queries
- [x] #3 find_active_entry returns a row when end_time IS NULL, None otherwise
- [x] #4 list_entries_in_range filters correctly by plan_id or task_id (either optional)
- [x] #5 cargo sqlx prepare --workspace succeeds and .sqlx/ cache is committed
- [x] #6 cargo clippy -- -D warnings passes with no warnings
<!-- AC:END -->
