---
id: TASK-49
title: 'Testing: Rust DB layer tests — db::entries'
status: To Do
assignee: []
created_date: '2026-03-23 07:51'
updated_date: '2026-03-23 07:52'
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
Add inline `#[cfg(test)]` module to `src-tauri/src/db/entries.rs`. Covers section 10.3 of Phase 10.

All tests use `test_pool()` from `db::test_helpers` for full isolation. Each test inserts its own plan (and task where needed) to satisfy FK constraints.

**Test cases:**

| Test | What it asserts |
|---|---|
| `insert_and_fetch_entry` | `insert_entry` succeeds; `get_entry` returns the same row |
| `find_active_entry_none` | Returns `None` when no row has `end_time IS NULL` |
| `find_active_entry_some` | Returns the in-progress entry after `insert_entry` with `end_time: None` |
| `update_entry_end_time` | Sets `end_time`; `find_active_entry` returns `None` afterwards |
| `list_entries_for_task` | Returns only entries matching `task_id`, ordered by `start_time DESC`, capped by `limit` |
| `list_entries_for_plan` | Returns entries for all tasks in a plan, plus taskless entries |
| `list_entries_in_range_by_task` | Filters correctly by task within date window |
| `list_entries_in_range_by_plan` | Filters correctly by plan within date window |
| `list_entries_in_range_excludes_active` | In-progress entries (`end_time IS NULL`) are excluded |
| `delete_entry` | Row is gone; subsequent `get_entry` returns `None` |
| `fk_constraint_fires` | Inserting an entry with a nonexistent `plan_id` returns an error |

**Test structure pattern:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::test_pool;
    use crate::models::TimeEntry;
    use uuid::Uuid;

    fn make_entry(plan_id: &str, task_id: Option<&str>, end_time: Option<&str>) -> TimeEntry {
        TimeEntry {
            id: Uuid::new_v4().to_string(),
            plan_id: plan_id.to_string(),
            task_id: task_id.map(str::to_string),
            start_time: "2024-03-15T10:00:00Z".to_string(),
            end_time: end_time.map(str::to_string),
            notes: None,
            created_at: "2024-03-15T10:00:00Z".to_string(),
        }
    }

    #[tokio::test]
    async fn insert_and_fetch_entry() {
        let pool = test_pool().await;
        sqlx::query!("INSERT INTO plans (id, graph_id, title, synced_at) VALUES (?, ?, ?, ?)",
            "p1", "g1", "Test Plan", "2024-01-01T00:00:00Z")
            .execute(&pool).await.unwrap();

        let entry = make_entry("p1", None, Some("2024-03-15T11:00:00Z"));
        insert_entry(&pool, &entry).await.unwrap();

        let fetched = get_entry(&pool, &entry.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, entry.id);
    }

    // ... remaining tests
}
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 All 11 test cases listed above are implemented
- [ ] #2 Tests use test_pool() — no shared state between tests
- [ ] #3 list_entries_for_plan test verifies taskless entries are included alongside task-level entries
- [ ] #4 list_entries_in_range_excludes_active confirms entries with end_time IS NULL do not appear in range results
- [ ] #5 fk_constraint_fires test confirms inserting with a bogus plan_id returns Err
- [ ] #6 SQLX_OFFLINE=true cargo test passes with all tests green
- [ ] #7 cargo clippy passes with no warnings
<!-- AC:END -->
