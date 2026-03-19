---
id: TASK-25
title: Phase 4.4 — Sync Tauri command
status: Done
assignee: []
created_date: '2026-03-19 13:07'
updated_date: '2026-03-19 13:18'
labels:
  - backend
  - rust
  - graph
  - database
milestone: Phase 4
dependencies:
  - TASK-24
  - TASK-22
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/commands/sync.rs` with a `sync_plans_and_tasks` Tauri command that fetches data from Microsoft Graph and upserts it into local SQLite.

## Requirements

```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResult {
    pub plans_count: usize,
    pub tasks_count: usize,
    pub synced_at: String,  // ISO 8601
}

#[tauri::command]
pub async fn sync_plans_and_tasks(
    auth: tauri::State<'_, Arc<AuthManager>>,
    pool: tauri::State<'_, SqlitePool>,
) -> Result<SyncResult, String>
```

Steps:
1. Create a `GraphClient` from `auth`
2. Call `fetch_user_info` and set `auth.set_display_name()` with the result
3. Call `fetch_my_plans`, map each `GraphPlan` → `Plan` (generate local UUID), upsert via `db::plans::upsert_plan`
4. For each plan, call `fetch_tasks_for_plan`, map `GraphTask` → `Task`, upsert via `db::tasks::upsert_task`
5. Return `SyncResult` with counts and current UTC timestamp

- Register `commands::sync::sync_plans_and_tasks` in `lib.rs` `invoke_handler`
- Declare `pub mod sync;` in `commands/mod.rs`
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 sync_plans_and_tasks command is registered in lib.rs invoke_handler
- [x] #2 Plans and tasks are upserted into SQLite after a successful sync
- [x] #3 SyncResult returns correct plans_count, tasks_count, and synced_at timestamp
- [x] #4 User display name is set on AuthManager after fetching /me
- [x] #5 Graph or DB errors are mapped to String and returned as Err
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src-tauri/src/commands/sync.rs` with `sync_plans_and_tasks` command. Fetches user info (sets display name on AuthManager), fetches all plans/tasks from Graph, upserts into SQLite. Fixed `upsert_plan` and `upsert_task` to conflict on `graph_id` (not local `id`) so re-syncs are idempotent. Added `get_plan_by_graph_id` to `db/plans.rs` for FK resolution. Regenerated `.sqlx/` offline cache. Command registered in `lib.rs`. Compiles cleanly.
<!-- SECTION:FINAL_SUMMARY:END -->
