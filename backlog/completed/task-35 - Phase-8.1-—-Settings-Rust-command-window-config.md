---
id: TASK-35
title: Phase 8.1 — Settings Rust command + window config
status: Done
assignee: []
created_date: '2026-03-19 14:34'
updated_date: '2026-03-19 14:46'
labels:
  - backend
  - rust
  - config
milestone: Phase 8
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add the backend pieces needed by the Settings view and update the window configuration.

## 1. `get_data_dir` command

Create `src-tauri/src/commands/settings.rs`:

```rust
#[tauri::command]
pub async fn get_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    // Re-use the same resolution logic as db::mod::resolve_db_path
    // but return the directory (not the .db file path).
    use tauri::Manager;
    #[cfg(target_os = "windows")]
    let base = app.path().document_dir().map_err(|e| e.to_string())?;
    #[cfg(not(target_os = "windows"))]
    let base = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    Ok(base.display().to_string())
}
```

## 2. Register module

- Add `pub mod settings;` to `commands/mod.rs`
- Register `commands::settings::get_data_dir` in `lib.rs` invoke_handler

## 3. API wrapper

Add to `src/lib/api/index.ts`:

```typescript
// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------
export async function getDataDir(): Promise<string> {
  return invoke<string>('get_data_dir');
}
```

## 4. Opener capability

Add `"opener:default"` to `src-tauri/capabilities/default.json` permissions array so the frontend `@tauri-apps/plugin-opener` JS package can open filesystem paths. This enables the "Open Folder" button in Settings.

## 5. Window configuration

Update `src-tauri/tauri.conf.json` `app.windows[0]`:
```json
{
  "title": "PlanTracker",
  "width": 1024,
  "height": 700,
  "minWidth": 800,
  "minHeight": 600,
  "decorations": true
}
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 get_data_dir command returns the correct platform data directory path string
- [x] #2 Command registered in lib.rs invoke_handler
- [x] #3 getDataDir API wrapper added to src/lib/api/index.ts
- [x] #4 opener:default added to capabilities so the JS opener plugin works
- [x] #5 tauri.conf.json window has title PlanTracker and minWidth 800 / minHeight 600
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `commands/settings.rs` with `get_data_dir` (platform-aware: `app_local_data_dir` on Linux, `document_dir` on Windows). Registered in `commands/mod.rs` and `lib.rs`. Added `getDataDir` to `src/lib/api/index.ts`. Added `opener:default` to capabilities. Updated `tauri.conf.json` window: title "PlanTracker", 1024×700, minWidth 800, minHeight 600, decorations true.
<!-- SECTION:FINAL_SUMMARY:END -->
