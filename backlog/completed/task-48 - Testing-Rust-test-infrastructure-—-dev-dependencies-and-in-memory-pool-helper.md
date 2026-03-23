---
id: TASK-48
title: 'Testing: Rust test infrastructure — dev-dependencies and in-memory pool helper'
status: Done
assignee: []
created_date: '2026-03-23 07:51'
updated_date: '2026-03-23 09:39'
labels:
  - testing
  - backend
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Foundation for all Rust unit tests. Covers sections 10.1 and 10.2 of Phase 10.

**10.1 — Dev-dependency**

Add to `src-tauri/Cargo.toml`:

```toml
[dev-dependencies]
tokio = { version = "1", features = ["full", "test-util"] }
```

`tokio` is already in `[dependencies]` with `features = ["full"]`, but `test-util` must be explicitly listed under `[dev-dependencies]` for `#[tokio::test]` to work.

**10.2 — In-memory pool helper**

Create `src-tauri/src/db/test_helpers.rs`:

```rust
use sqlx::SqlitePool;

/// Spin up a fully migrated, in-memory SQLite pool for use in tests.
/// Each call returns an independent pool — tests are fully isolated.
pub async fn test_pool() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("failed to open in-memory SQLite");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations failed");

    sqlx::query!("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .expect("failed to enable FK constraints");

    pool
}
```

Expose from `src-tauri/src/db/mod.rs` behind `#[cfg(test)]`:

```rust
#[cfg(test)]
pub mod test_helpers;
```

This keeps the helper out of the release binary entirely.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 tokio test-util feature added under [dev-dependencies] in Cargo.toml
- [x] #2 src-tauri/src/db/test_helpers.rs exists with test_pool() that returns a migrated in-memory pool
- [x] #3 test_helpers module gated behind #[cfg(test)] in db/mod.rs
- [x] #4 cargo clippy passes with no warnings
- [x] #5 SQLX_OFFLINE=true cargo test compiles and runs (no tests yet, just no compile errors)
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `tokio` with `test-util` feature under `[dev-dependencies]` in Cargo.toml. Created `src-tauri/src/db/test_helpers.rs` with `test_pool()` returning a fully migrated in-memory SQLite pool. Used `sqlx::query` (non-macro) for the PRAGMA to avoid needing an offline cache entry for test-only code. Exposed the module behind `#[cfg(test)]` in `db/mod.rs`. `SQLX_OFFLINE=true cargo test` compiles and runs cleanly; clippy reports no warnings.
<!-- SECTION:FINAL_SUMMARY:END -->
