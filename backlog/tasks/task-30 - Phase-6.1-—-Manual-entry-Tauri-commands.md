---
id: TASK-30
title: Phase 6.1 — Manual entry Tauri commands
status: Done
assignee: []
created_date: '2026-03-19 13:44'
updated_date: '2026-03-19 13:48'
labels:
  - backend
  - rust
milestone: Phase 6
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/commands/entries.rs` with three Tauri commands for creating, updating, and deleting time entries manually. Register them in `lib.rs`.

## Commands

**`create_manual_entry`**
```rust
#[tauri::command]
pub async fn create_manual_entry(
    task_id: String,
    start_time: String,       // ISO 8601
    end_time: String,         // ISO 8601
    notes: Option<String>,
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String>
```
Validations (return `Err(String)` on failure):
- `task_id` must not be empty
- Parse both `start_time` and `end_time` as `DateTime<Utc>` — return error on bad format
- `end_time` must be strictly after `start_time`
- Check that no timer is currently running (guard against overlapping the active entry)

On success: call `db::entries::insert_entry` with a fresh UUID and the provided fields.

**`update_entry`**
```rust
#[tauri::command]
pub async fn update_entry(
    id: String,
    start_time: String,
    end_time: String,
    notes: Option<String>,
    pool: State<'_, SqlitePool>,
) -> Result<TimeEntry, String>
```
- Same time validation as `create_manual_entry`
- Update the row via a new `db::entries::update_entry` function (UPDATE … WHERE id = ?)
- Return the updated `TimeEntry` via `db::entries::get_entry`

**`delete_entry`**
```rust
#[tauri::command]
pub async fn delete_entry(
    id: String,
    pool: State<'_, SqlitePool>,
) -> Result<(), String>
```
- Call `db::entries::delete_entry` — already implemented.

## DB additions

Add to `src-tauri/src/db/entries.rs`:
```rust
pub async fn update_entry(
    pool: &SqlitePool,
    id: &str,
    start_time: &str,
    end_time: &str,
    notes: Option<&str>,
) -> anyhow::Result<()>
// UPDATE time_entries SET start_time=?, end_time=?, notes=? WHERE id=?
```

After adding the query, run `cargo sqlx prepare --workspace` to regenerate the `.sqlx/` offline cache.

## Registration
- Declare `pub mod entries;` in `commands/mod.rs`
- Register `create_manual_entry`, `update_entry`, `delete_entry` in `lib.rs` invoke_handler
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 create_manual_entry rejects empty task_id, unparseable datetimes, and end_time <= start_time
- [x] #2 create_manual_entry returns an error if a timer is currently running
- [x] #3 create_manual_entry inserts a completed entry (both start_time and end_time set) and returns it
- [x] #4 update_entry applies the same time validations and returns the updated TimeEntry
- [x] #5 delete_entry removes the row and returns Ok(())
- [x] #6 db::entries::update_entry function added and .sqlx/ cache regenerated
- [x] #7 All 3 commands registered in lib.rs invoke_handler
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `commands/entries.rs` with `create_manual_entry` (validates task_id, parses+validates times, guards against active timer), `update_entry` (same time validation, updates row, returns refreshed entry), `delete_entry` (delegates to db). Added `db::entries::update_entry` SQL function. Regenerated `.sqlx/` cache. Registered all 3 commands in `lib.rs`. Compiles cleanly.
<!-- SECTION:FINAL_SUMMARY:END -->
