---
id: TASK-27
title: Phase 5.1 — Timer Rust commands and managed state
status: Done
assignee: []
created_date: '2026-03-19 13:24'
updated_date: '2026-03-19 13:28'
labels:
  - backend
  - rust
milestone: Phase 5
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/commands/timer.rs` with in-memory `ActiveTimer` state and all timer-related Tauri commands. Wire everything into `lib.rs`.

## ActiveTimer state

```rust
pub struct ActiveTimer {
    pub entry_id: String,
    pub task_id: String,
    pub start_time: DateTime<Utc>,
}
```

Register in `lib.rs` setup:
```rust
app.manage(tokio::sync::Mutex::new(None::<ActiveTimer>));
```

## Commands

**`start_timer(task_id, pool, timer) -> Result<TimeEntry, String>`**
- Check for an existing active timer — return an error if one is already running (prevents duplicates, see pitfalls.md)
- Insert a `time_entries` row with `end_time = NULL` and a fresh UUID
- Store the entry details in `ActiveTimer` managed state
- Return the created `TimeEntry`

**`stop_timer(pool, timer) -> Result<TimeEntry, String>`**
- Return error if no timer is running
- Set `end_time = Utc::now()` on the in-progress DB row via `db::entries::update_entry_end_time`
- Clear `ActiveTimer` from managed state
- Return the completed `TimeEntry`

**`get_active_timer(timer) -> Result<Option<ActiveTimerInfo>, String>`**
```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveTimerInfo {
    pub entry_id: String,
    pub task_id: String,
    pub start_time: String,     // ISO 8601
    pub elapsed_seconds: i64,   // Utc::now() - start_time
}
```

**`get_recent_entries(task_id, plan_id, limit, pool) -> Result<Vec<TimeEntry>, String>`**
- If `task_id` is Some: call `db::entries::list_entries_for_task`
- If `plan_id` is Some: call `db::entries::list_entries_for_plan`
- Otherwise return empty vec

## Startup restoration (pitfalls.md)

In `lib.rs` setup, after init_db and before returning `Ok(())`:
```rust
if let Some(entry) = db::entries::find_active_entry(&pool).await? {
    *timer_state.lock().await = Some(ActiveTimer {
        entry_id: entry.id.clone(),
        task_id: entry.task_id.clone(),
        start_time: entry.start_time.parse()?,
    });
}
```

This covers the "Active Timer Lost on App Restart" pitfall.

## Mutex safety
All Mutex locks must be released before any `.await` point (rust.md rule). Extract or clone the needed value before awaiting.

## Registration
- Declare `pub mod timer;` in `commands/mod.rs`
- Register `start_timer`, `stop_timer`, `get_active_timer`, `get_recent_entries` in `lib.rs` invoke_handler
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 ActiveTimer struct and Mutex<Option<ActiveTimer>> registered in lib.rs managed state
- [x] #2 start_timer inserts a time_entries row with end_time = NULL and stores ActiveTimer
- [x] #3 start_timer returns an error if a timer is already running
- [x] #4 stop_timer sets end_time, clears ActiveTimer, returns completed TimeEntry
- [x] #5 get_active_timer returns elapsed_seconds computed from start_time to now
- [x] #6 get_recent_entries returns entries filtered by task_id or plan_id
- [x] #7 On app startup, any existing active entry (end_time IS NULL) restores ActiveTimer state
- [x] #8 No tokio::sync::Mutex lock is held across an .await point
- [x] #9 All 4 commands registered in lib.rs invoke_handler
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `commands/timer.rs` with `ActiveTimer` struct, `start_timer`, `stop_timer`, `get_active_timer`, `get_recent_entries`. Added `get_entry` to `db/entries.rs` (used by `stop_timer` to return the completed row). Registered `Mutex<Option<ActiveTimer>>` in managed state. On startup, queries DB for any `end_time IS NULL` entry and restores `ActiveTimer` state. All Mutex locks released before every `.await`. Updated `.sqlx/` cache. All 4 commands registered in `lib.rs`. Compiles cleanly.
<!-- SECTION:FINAL_SUMMARY:END -->
