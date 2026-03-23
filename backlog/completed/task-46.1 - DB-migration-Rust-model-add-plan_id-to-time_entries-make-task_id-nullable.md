---
id: TASK-46.1
title: 'DB migration + Rust model: add plan_id to time_entries, make task_id nullable'
status: Done
assignee: []
created_date: '2026-03-23 02:41'
updated_date: '2026-03-23 03:02'
labels:
  - database
  - backend
dependencies: []
parent_task_id: TASK-46
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Foundation layer for plan-level time tracking. The current schema has `time_entries.task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE` with no `plan_id` column. This task adds `plan_id` as a required column and makes `task_id` optional.

**Migration file: `src-tauri/migrations/0002_plan_level_entries.sql`**

SQLite does not support dropping a NOT NULL constraint via ALTER TABLE, so a table recreation is required:

```sql
-- Step 1: add plan_id (nullable initially so the backfill can run)
ALTER TABLE time_entries
    ADD COLUMN plan_id TEXT REFERENCES plans(id) ON DELETE CASCADE;

-- Step 2: backfill plan_id from the task's plan for every existing entry
UPDATE time_entries
SET plan_id = (SELECT plan_id FROM tasks WHERE tasks.id = time_entries.task_id)
WHERE task_id IS NOT NULL;

-- Step 3: recreate the table with the correct constraints
CREATE TABLE time_entries_new (
    id         TEXT PRIMARY KEY,
    plan_id    TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    task_id    TEXT            REFERENCES tasks(id) ON DELETE SET NULL,
    start_time TEXT NOT NULL,
    end_time   TEXT,
    notes      TEXT,
    created_at TEXT NOT NULL
);

INSERT INTO time_entries_new (id, plan_id, task_id, start_time, end_time, notes, created_at)
SELECT id, plan_id, task_id, start_time, end_time, notes, created_at
FROM time_entries;

DROP TABLE time_entries;
ALTER TABLE time_entries_new RENAME TO time_entries;

-- Step 4: recreate indices (dropped with the old table)
CREATE INDEX IF NOT EXISTS idx_time_entries_plan_id    ON time_entries(plan_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_task_id    ON time_entries(task_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_start_time ON time_entries(start_time);
```

`ON DELETE SET NULL` on `task_id` means if a task is removed during a Graph sync, its entries are preserved as plan-level entries rather than deleted — better data retention than the previous CASCADE.

**`src-tauri/src/models.rs` — update `TimeEntry`**

```rust
pub struct TimeEntry {
    pub id: String,
    pub plan_id: String,          // NEW — always set
    pub task_id: Option<String>,  // CHANGED — NULL for plan-level entries
    pub start_time: String,
    pub end_time: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}
```

**`src-tauri/src/db/entries.rs` — update all queries**

Every query that SELECTs or INSERTs into `time_entries` must be updated:
- Add `plan_id` to SELECT lists and INSERT values.
- Change `task_id` aliases from `"task_id!"` (non-null) to `"task_id"` (nullable) in `query_as!` macros.
- `list_entries_for_plan`: replace the `INNER JOIN tasks` approach with a direct `WHERE te.plan_id = ?` — this naturally includes both task-level and plan-level entries for a plan.
- `list_entries_in_range` (plan branch): same — use `WHERE te.plan_id = ?` directly instead of joining through tasks.

After all query changes, run **`just precache`** to regenerate the sqlx offline query cache and commit the updated `.sqlx/` directory.

**Startup timer recovery (`src-tauri/src/main.rs` or wherever `find_active_entry` is called)**

The existing recovery code constructs an `ActiveTimer` from a found entry. After this task's model change the `task_id` field is `Option<String>`, so any code building `ActiveTimer` must use `entry.task_id.clone()` (which is already an Option) — no structural breakage, just type alignment once `ActiveTimer` is updated in TASK-48.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Migration file `0002_plan_level_entries.sql` exists and runs cleanly on a fresh DB and on a DB that already has entries (backfill must not fail).
- [ ] #2 After migration, `time_entries.plan_id` is NOT NULL and `time_entries.task_id` is nullable.
- [ ] #3 Existing entries retain correct plan_id values after the backfill step.
- [ ] #4 `TimeEntry` Rust struct has `plan_id: String` and `task_id: Option<String>`.
- [ ] #5 All `db/entries.rs` queries compile with the updated struct (sqlx offline cache regenerated and committed).
- [ ] #6 `list_entries_for_plan` and the plan branch of `list_entries_in_range` return plan-level (taskless) entries as well as task-level entries for a plan.
- [ ] #7 `cargo clippy` passes with no warnings.
- [ ] #8 Run `just precache` and commit the updated `.sqlx/` directory.
<!-- AC:END -->
