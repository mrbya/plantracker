---
id: TASK-46.2
title: 'Backend: update Tauri commands for plan-level entries'
status: Done
assignee: []
created_date: '2026-03-23 02:42'
updated_date: '2026-03-23 03:02'
labels:
  - backend
dependencies:
  - TASK-46.1
parent_task_id: TASK-46
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Update the Tauri command layer so that `start_timer`, `create_manual_entry`, and `get_active_timer` all accept a required `plan_id` and an optional `task_id`. Update `generate_report` to label taskless entries "No specific task". Depends on TASK-46.1 (model and DB query changes).

**`src-tauri/src/commands/timer.rs`**

`ActiveTimer` and `ActiveTimerInfo` gain `plan_id` and `task_id` becomes optional:
```rust
pub struct ActiveTimer {
    pub entry_id: String,
    pub plan_id: String,
    pub task_id: Option<String>,
    pub start_time: chrono::DateTime<Utc>,
}

#[serde(rename_all = "camelCase")]
pub struct ActiveTimerInfo {
    pub entry_id: String,
    pub plan_id: String,
    pub task_id: Option<String>,
    pub start_time: String,
    pub elapsed_seconds: i64,
}
```

`start_timer` signature changes:
```rust
pub async fn start_timer(
    plan_id: String,
    task_id: Option<String>,
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String>
```
- Validate `plan_id` is not empty; if `task_id` is `Some`, validate it is not empty.
- Build `TimeEntry` with `plan_id` and optional `task_id`.
- Store `plan_id` and `task_id` in `ActiveTimer`.

`stop_timer` — no signature change.

`get_active_timer` — build `ActiveTimerInfo` from the updated `ActiveTimer` fields.

`get_recent_entries` — no change needed.

**`src-tauri/src/commands/entries.rs`**

`create_manual_entry` signature changes:
```rust
pub async fn create_manual_entry(
    plan_id: String,
    task_id: Option<String>,
    start_time: String,
    end_time: String,
    notes: Option<String>,
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String>
```
- Guard: `plan_id` must not be empty.
- Build `TimeEntry` with `plan_id` and optional `task_id`.
- `update_entry` does not change `plan_id` or `task_id` — no change needed.

**`src-tauri/src/commands/reports.rs`**

In `generate_report`, `raw.task_id` is now `Option<String>`. Update the entry-building loop:
```rust
let (task_title, task_plan_id) = if let Some(ref tid) = raw.task_id {
    // existing cache lookup by tid
} else {
    ("No specific task".to_string(), raw.plan_id.clone())
};
```
When `task_id` is `None`, `plan_id` is already on the raw entry — no extra DB lookup is needed for the plan title (use the plan cache keyed on `raw.plan_id`).

**Startup timer recovery**

Wherever `find_active_entry` result is used to reconstruct `ActiveTimer` on startup (check `src-tauri/src/main.rs` or `src-tauri/src/lib.rs`), update the struct construction to include `plan_id` and the now-optional `task_id`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 start_timer accepts plan_id (required String) and task_id (Option<String>); returns Err if plan_id is empty.
- [ ] #2 create_manual_entry accepts plan_id (required) and task_id (optional); returns Err if plan_id is empty.
- [ ] #3 get_active_timer returns planId and optional taskId in ActiveTimerInfo.
- [ ] #4 generate_report sets task_title to 'No specific task' for entries where task_id IS NULL.
- [ ] #5 Startup timer recovery correctly restores plan_id and optional task_id into ActiveTimer.
- [ ] #6 cargo clippy passes with no warnings.
<!-- AC:END -->
