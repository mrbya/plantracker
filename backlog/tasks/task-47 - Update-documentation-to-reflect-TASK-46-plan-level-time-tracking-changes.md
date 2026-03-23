---
id: TASK-47
title: Update documentation to reflect TASK-46 plan-level time tracking changes
status: To Do
assignee: []
created_date: '2026-03-23 02:45'
labels:
  - documentation
dependencies:
  - TASK-46.1
  - TASK-46.2
  - TASK-46.3
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After TASK-46.1–46.3 are implemented, several rule and reference files will contain stale type signatures, schema definitions, and code snippets. Update every live reference to match the new reality.

**`.claude/rules/database.md` — Schema Reference section**

The `time_entries` table definition currently shows:
```sql
task_id    TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
```
Replace with the post-migration schema:
```sql
plan_id    TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
task_id    TEXT            REFERENCES tasks(id) ON DELETE SET NULL,
```
Also add `idx_time_entries_plan_id` to the index list.

**`.claude/rules/rust.md` — Serialisation section**

The `TimeEntry` example struct currently has `pub task_id: String`. Update to:
```rust
pub struct TimeEntry {
    pub id: String,
    pub plan_id: String,
    pub task_id: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}
```

**`.claude/rules/pitfalls.md` — "Active Timer Lost on App Restart" section**

The recovery code snippet currently constructs `ActiveTimer` with only `entry_id`, `task_id`, and `start_time`. Update to include `plan_id` and reflect `task_id` as optional:
```rust
*timer.lock().await = Some(ActiveTimer {
    entry_id: entry.id.clone(),
    plan_id:  entry.plan_id.clone(),
    task_id:  entry.task_id.clone(),   // Option<String>
    start_time: entry.start_time.parse()?,
});
```

**`.claude/rules/frontend.md` — Types section and Invoke Boundary example**

- `TimeEntry` interface: add `planId: string`, change `taskId: string` → `taskId: string | null`.
- `startTimer` example: update signature to `startTimer(planId: string, taskId?: string): Promise<TimeEntry>` with body `invoke<TimeEntry>('start_timer', { planId, taskId: taskId ?? null })`.
- Update both the correct and incorrect example if `taskId` appears in the "WRONG" example.

**`docs/IMPLEMENTATION_PLAN.md` — Appendix C only**

- `TimeEntry` interface (line ~810): add `planId`, change `taskId` to nullable.
- `startTimer` wrapper (line ~794): update signature.

Do **not** edit Phase 5 (timer commands) or Phase 6 (entries commands) planning sections — those are historical plan descriptions.

**Files confirmed not to need changes**
- `CLAUDE.md` — no schema or type snippets referencing `task_id`.
- `README.md` — no type-level detail; high-level feature descriptions remain accurate.
- `backlog/completed/` — immutable historical records.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 `.claude/rules/database.md` schema shows plan_id NOT NULL and task_id nullable with ON DELETE SET NULL, and includes idx_time_entries_plan_id.
- [ ] #2 `.claude/rules/rust.md` TimeEntry struct has plan_id: String and task_id: Option<String>.
- [ ] #3 `.claude/rules/pitfalls.md` ActiveTimer recovery snippet includes plan_id and uses task_id as Option<String>.
- [ ] #4 `.claude/rules/frontend.md` TimeEntry interface has planId: string and taskId: string | null.
- [ ] #5 `.claude/rules/frontend.md` startTimer example reflects the new (planId, taskId?) signature.
- [ ] #6 `docs/IMPLEMENTATION_PLAN.md` Appendix C TimeEntry and startTimer references are updated.
- [ ] #7 No other live documentation file contains stale task_id-as-required or missing plan_id references for TimeEntry or ActiveTimer.
<!-- AC:END -->
