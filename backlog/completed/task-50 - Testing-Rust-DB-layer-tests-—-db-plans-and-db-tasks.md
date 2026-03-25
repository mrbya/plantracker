---
id: TASK-50
title: 'Testing: Rust DB layer tests — db::plans and db::tasks'
status: Done
assignee: []
created_date: '2026-03-23 07:51'
updated_date: '2026-03-23 10:49'
labels:
  - testing
  - backend
  - database
dependencies:
  - TASK-48
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add inline `#[cfg(test)]` modules to `src-tauri/src/db/plans.rs` and `src-tauri/src/db/tasks.rs`. Covers section 10.4 of Phase 10.

**`db::plans` test cases:**

| Test | What it asserts |
|---|---|
| `upsert_plan_insert` | New plan is retrievable after upsert |
| `upsert_plan_update` | Upserting with same `graph_id` updates `title` and `synced_at` |
| `list_plans_empty` | Returns empty vec when no plans exist |
| `list_plans_multiple` | Returns all inserted plans |
| `get_plan_missing` | Returns `None` for an unknown ID |

**`db::tasks` test cases:**

| Test | What it asserts |
|---|---|
| `upsert_task_insert` | New task is retrievable |
| `upsert_task_update` | Updates `title` on re-upsert with same `graph_id` |
| `list_tasks_for_plan` | Only tasks belonging to the given plan are returned |
| `get_task_by_graph_id` | Lookup by external Graph ID works |
| `cascade_delete` | Deleting a plan deletes its tasks (FK `ON DELETE CASCADE`) |

All tests use `test_pool()` from `db::test_helpers`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All 5 db::plans test cases implemented
- [x] #2 All 5 db::tasks test cases implemented
- [x] #3 cascade_delete test inserts a plan + task, deletes the plan, then asserts the task no longer exists
- [x] #4 upsert_plan_update and upsert_task_update verify the field was actually changed, not just that upsert didn't error
- [x] #5 SQLX_OFFLINE=true cargo test passes with all tests green
- [x] #6 cargo clippy passes with no warnings
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `#[cfg(test)] mod tests` to `plans.rs` (5 tests: upsert_plan_insert, upsert_plan_update, list_plans_empty, list_plans_multiple, get_plan_missing) and `tasks.rs` (5 tests: upsert_task_insert, upsert_task_update, list_tasks_for_plan_test, get_task_by_graph_id_test, cascade_delete). The cascade_delete test deletes a plan via raw sqlx::query and asserts the FK ON DELETE CASCADE removes the task. The update tests verify the actual field values changed. All 21 tests pass under SQLX_OFFLINE=true; clippy is clean.
<!-- SECTION:FINAL_SUMMARY:END -->
