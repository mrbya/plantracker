---
id: TASK-23
title: Phase 4.2 — Graph API response models
status: Done
assignee: []
created_date: '2026-03-19 13:07'
updated_date: '2026-03-19 13:11'
labels:
  - backend
  - rust
  - graph
milestone: Phase 4
dependencies:
  - TASK-22
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/graph/models.rs` with Rust structs that deserialize Microsoft Graph API responses for Planner resources.

## Requirements

Structs (all derive `Deserialize`):

```rust
pub struct GraphPlan {
    pub id: String,
    pub title: String,
}

pub struct GraphTask {
    pub id: String,
    pub title: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
}

pub struct GraphUser {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
}

pub struct GraphPagedResponse<T> {
    pub value: Vec<T>,
    #[serde(rename = "@odata.nextLink")]
    pub next_link: Option<String>,
}
```

- All camelCase Graph field names must be mapped correctly via `#[serde(rename = "...")]` or `#[serde(rename_all = "camelCase")]`
- `GraphPagedResponse<T>` must be generic so it works for both plans and tasks
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 GraphPlan deserializes from Graph /me/planner/plans response
- [x] #2 GraphTask deserializes planId correctly (camelCase rename)
- [x] #3 GraphUser deserializes displayName and userPrincipalName
- [x] #4 GraphPagedResponse<T> captures @odata.nextLink as next_link: Option<String>
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Populated `src-tauri/src/graph/models.rs` with `GraphPlan`, `GraphTask` (camelCase via `rename_all`), `GraphUser`, and `GraphPagedResponse<T>`. All fields map correctly to Graph API JSON keys. Compiles cleanly.
<!-- SECTION:FINAL_SUMMARY:END -->
