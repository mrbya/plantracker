---
id: TASK-10
title: Phase 2.1 — Configure sqlx and data directory
status: Done
assignee: []
created_date: '2026-03-19 09:27'
updated_date: '2026-03-19 10:53'
labels:
  - backend
  - database
  - phase-2
dependencies: []
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Set up the database module foundation: resolve the platform-specific data directory and return the path to `plantracker.db`.

**File:** `src-tauri/src/db/mod.rs`

- On Linux: use `app.path().app_local_data_dir()` → `~/.local/share/PlanTracker/`
- On Windows: use `app.path().document_dir()` → `%USERPROFILE%\Documents\PlanTracker\`
- Create the directory if it does not exist (`std::fs::create_dir_all`)
- Return `base.join("plantracker.db")`

```rust
fn resolve_db_path(app: &tauri::AppHandle) -> anyhow::Result<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    let base = app.path().document_dir()?;
    #[cfg(not(target_os = "windows"))]
    let base = app.path().app_local_data_dir()?;
    std::fs::create_dir_all(&base)?;
    Ok(base.join("plantracker.db"))
}
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 resolve_db_path() returns the correct path on Linux (~/.local/share/PlanTracker/plantracker.db)
- [x] #2 Directory is created automatically if it does not exist
- [x] #3 Function compiles and is accessible from db/mod.rs
<!-- AC:END -->
