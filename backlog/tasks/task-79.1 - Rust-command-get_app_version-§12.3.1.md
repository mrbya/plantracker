---
id: TASK-79.1
title: Rust command get_app_version (§12.3.1)
status: Done
assignee: []
created_date: '2026-03-26 12:54'
updated_date: '2026-03-26 12:58'
labels:
  - backend
  - phase-12
dependencies: []
references:
  - docs/phases/phase-12-quality-of-life.md
parent_task_id: TASK-79
priority: medium
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add `get_app_version` to `src-tauri/src/commands/settings.rs` and register it in the invoke handler.

## Implementation

Add alongside the existing `get_data_dir` command:

```rust
/// Returns the application version string from `tauri.conf.json` / `Cargo.toml`.
///
/// Tauri reads the version from `Cargo.toml` at build time and exposes it on
/// the `AppHandle`. This command surfaces it to the frontend so the Settings
/// view can display it without hardcoding.
///
/// # Returns
/// Version string in SemVer format, e.g. `"0.1.2"`.
#[tauri::command]
pub async fn get_app_version(app: tauri::AppHandle) -> Result<String, String> {
    Ok(app.package_info().version.to_string())
}
```

Register in `src-tauri/src/lib.rs`:

```rust
tauri::generate_handler![
    // ... existing commands ...
    commands::settings::get_data_dir,
    commands::settings::get_app_version,   // ← add
]
```

No new capability or permission entry required — reads only from `AppHandle` metadata.

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.3.1
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 get_app_version is defined in src-tauri/src/commands/settings.rs
- [x] #2 Command is registered in the tauri::generate_handler! macro in lib.rs
- [x] #3 cargo clippy -- -D warnings passes with no new warnings
- [x] #4 Invoking get_app_version from the frontend returns the version string from Cargo.toml
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `get_app_version` to `src-tauri/src/commands/settings.rs` alongside `get_data_dir`. Registered the command in the `tauri::generate_handler![]` macro in `src-tauri/src/lib.rs`. Fixed a `clippy::doc_markdown` lint (`SemVer` needs backticks). `cargo clippy -- -D warnings` passes clean.
<!-- SECTION:FINAL_SUMMARY:END -->
