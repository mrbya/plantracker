---
id: TASK-24
title: Phase 4.3 — Graph Planner query functions
status: Done
assignee: []
created_date: '2026-03-19 13:07'
updated_date: '2026-03-19 13:12'
labels:
  - backend
  - rust
  - graph
milestone: Phase 4
dependencies:
  - TASK-22
  - TASK-23
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/graph/planner.rs` with functions that fetch Planner data from Microsoft Graph, handling OData pagination transparently.

## Requirements

```rust
pub async fn fetch_my_plans(client: &GraphClient) -> anyhow::Result<Vec<GraphPlan>>
// GET /me/planner/plans — paginated

pub async fn fetch_tasks_for_plan(client: &GraphClient, plan_id: &str) -> anyhow::Result<Vec<GraphTask>>
// GET /planner/plans/{plan_id}/tasks — paginated

pub async fn fetch_user_info(client: &GraphClient) -> anyhow::Result<GraphUser>
// GET /me — single object
```

- Implement a shared `fetch_all_pages<T>` helper (or inline pagination loop) that follows `@odata.nextLink` until exhausted
- Use `client.get()` for the initial request and `client.get_url()` for subsequent pages
- Rate-limit handling: on `429 Too Many Requests`, read the `Retry-After` header (default 10 s) and wait before retrying. Per `graph.md` this must not retry immediately.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 fetch_my_plans returns all plans across all pages
- [x] #2 fetch_tasks_for_plan returns all tasks for a given plan ID across all pages
- [x] #3 fetch_user_info returns the signed-in user's display name
- [x] #4 Pagination loop terminates when next_link is None
- [x] #5 429 responses are retried after Retry-After seconds (default 10)
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented `src-tauri/src/graph/planner.rs` with `fetch_my_plans`, `fetch_tasks_for_plan`, and `fetch_user_info`. Shared `fetch_all_pages<T>` helper follows `@odata.nextLink` until exhausted. 429 rate-limit handling added to `GraphClient::get_url()` in client.rs (reads Retry-After header, defaults to 10s, waits then retries once). Compiles warning-free.
<!-- SECTION:FINAL_SUMMARY:END -->
