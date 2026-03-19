---
id: TASK-37
title: Phase 9.1 — Generate and commit sqlx offline query cache
status: Done
assignee: []
created_date: '2026-03-19 16:13'
updated_date: '2026-03-19 16:20'
labels:
  - backend
  - rust
  - ci
milestone: Phase 9
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The `.sqlx/` offline query cache does not exist in the repository. CI builds run with `SQLX_OFFLINE=true` (no live database), so without this cache the build will fail to compile.

## Steps

1. Create a temporary SQLite database and run migrations against it:
   ```bash
   export DATABASE_URL="sqlite:///tmp/plantracker_sqlx.db"
   cargo sqlx database create
   cargo sqlx migrate run --source src-tauri/migrations
   ```

2. Generate the cache from within `src-tauri/`:
   ```bash
   cd src-tauri
   DATABASE_URL="sqlite:///tmp/plantracker_sqlx.db" cargo sqlx prepare --workspace
   ```
   This writes JSON files to `.sqlx/` in the workspace root.

3. Verify the generated cache is complete by compiling without a live DB:
   ```bash
   SQLX_OFFLINE=true cargo check
   ```
   This must produce zero errors.

4. Stage and commit the `.sqlx/` directory:
   ```bash
   git add .sqlx/
   git commit -m "Add sqlx offline query cache"
   ```

## Notes
- The `.sqlx/` directory must be committed — it is NOT a build artefact to gitignore.
- Every time a sqlx query is added or changed, this cache must be regenerated and recommitted. The CI job in TASK-38 uses `cargo sqlx prepare --check` to catch stale caches.
- The temp database at `/tmp/plantracker_sqlx.db` can be deleted afterwards.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 SQLX_OFFLINE=true cargo check passes without errors in src-tauri/
- [x] #2 .sqlx/ directory is committed to the repository
- [x] #3 The cache covers all queries in db/plans.rs, db/tasks.rs, and db/entries.rs
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
The `.sqlx/` offline query cache was already committed to the repository from a prior session (19 query JSON files in `src-tauri/.sqlx/`). Regenerated the cache against a fresh temp database to confirm it matches the current schema, then verified `SQLX_OFFLINE=true cargo check` compiles cleanly with zero errors. No new commit was needed — the cache was up to date.
<!-- SECTION:FINAL_SUMMARY:END -->
