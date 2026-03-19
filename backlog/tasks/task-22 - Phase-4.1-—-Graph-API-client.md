---
id: TASK-22
title: Phase 4.1 — Graph API client
status: Done
assignee: []
created_date: '2026-03-19 13:07'
updated_date: '2026-03-19 13:10'
labels:
  - backend
  - rust
  - graph
milestone: Phase 4
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/graph/client.rs` with a `GraphClient` struct that wraps `reqwest::Client` + `Arc<AuthManager>` and provides authenticated HTTP requests to Microsoft Graph.

## Requirements

- `GraphClient` holds `reqwest::Client` and `Arc<AuthManager>`
- `async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T>` — appends path to `https://graph.microsoft.com/v1.0`, attaches `Authorization: Bearer {token}` header
- On `401 Unauthorized` response: call `auth.get_valid_token()` to force a refresh, then retry the request once
- `async fn get_url<T: DeserializeOwned>(&self, url: &str) -> anyhow::Result<T>` — same as above but takes a full URL (needed for `@odata.nextLink` pagination)
- Parse non-success responses by reading the Graph error JSON body (`error.code` + `error.message`) and surfacing a human-readable error via `anyhow::bail!`
- Create `src-tauri/src/graph/mod.rs` declaring `pub mod client; pub mod models; pub mod planner;`
- Declare `pub mod graph;` in `src-tauri/src/lib.rs`
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 GraphClient struct exists in src-tauri/src/graph/client.rs
- [x] #2 get() attaches a valid bearer token to every request
- [x] #3 401 response triggers one silent retry after token refresh
- [x] #4 Non-2xx responses surface Graph error.message in the anyhow error
- [x] #5 get_url() accepts a full absolute URL for paginated next-link following
- [x] #6 graph module declared in lib.rs
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src-tauri/src/graph/` module with `mod.rs`, `client.rs`, `models.rs` (stub), and `planner.rs` (stub). `GraphClient` wraps `reqwest::Client` + `Arc<AuthManager>`, provides `get()` and `get_url()` with bearer auth, 401 retry, and structured Graph error surfacing. `pub mod graph;` added to `lib.rs`. Compiles cleanly.
<!-- SECTION:FINAL_SUMMARY:END -->
