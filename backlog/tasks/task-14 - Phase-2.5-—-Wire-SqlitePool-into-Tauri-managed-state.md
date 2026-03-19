---
id: TASK-14
title: Phase 2.5 — Wire SqlitePool into Tauri managed state
status: Done
assignee: []
created_date: '2026-03-19 09:28'
updated_date: '2026-03-19 11:00'
labels:
  - backend
  - database
  - phase-2
dependencies:
  - TASK-13
priority: high
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Call `init_db()` during the Tauri app setup in `src-tauri/src/main.rs` and register the resulting pool as managed state so all commands can access it.

```rust
// In the Tauri Builder setup block:
let pool = db::init_db(&app.handle()).await
    .expect("Failed to initialise database");
app.manage(pool);
```

Commands receive the pool via:
```rust
#[tauri::command]
pub async fn some_command(pool: tauri::State<'_, SqlitePool>) -> Result<..., String> {
    db::plans::list_plans(&pool).await.map_err(|e| e.to_string())
}
```

Also verify the startup sequence handles the pitfall from `pitfalls.md`: after `init_db()`, query for any `time_entries` row where `end_time IS NULL` and restore the `ActiveTimer` state if one is found. (ActiveTimer managed state will be added in Phase 5 \u2014 for now just document the hook point with a `// TODO: restore active timer` comment.)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 init_db() is called during Tauri setup before any commands can be invoked
- [x] #2 SqlitePool is registered with app.manage()
- [x] #3 App starts without panics and the database file is created
- [x] #4 A // TODO: restore active timer comment marks the hook point in main.rs
- [x] #5 cargo tauri dev runs cleanly end-to-end
<!-- AC:END -->
