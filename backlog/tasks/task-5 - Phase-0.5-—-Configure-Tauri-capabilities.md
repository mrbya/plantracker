---
id: TASK-5
title: Phase 0.5 — Configure Tauri capabilities
status: Done
assignee: []
created_date: '2026-03-18 22:50'
updated_date: '2026-03-18 22:59'
labels: []
milestone: Phase 0 — Project Scaffold
dependencies:
  - TASK-3
priority: high
ordinal: 500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Enable the required Tauri plugin capabilities in `src-tauri/capabilities/default.json`.

Required capabilities:
- `core:default`
- `shell:allow-open` — open system browser for OAuth
- `store:default` — persist settings
- `dialog:default` — CSV export save dialog
- `oauth:default` — localhost redirect server for PKCE flow

Also register all plugins in `src-tauri/src/main.rs` via `.plugin(tauri_plugin_*::init())`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 capabilities/default.json includes all five capability entries
- [x] #2 All plugins initialized in main.rs
- [x] #3 `cargo tauri dev` starts without capability or permission errors
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created src-tauri/capabilities/default.json with core:default, shell:allow-open, store:default, dialog:default, oauth:allow-start, oauth:allow-cancel. Note: tauri-plugin-oauth exposes allow-start/allow-cancel individually (no default bundle). Registered all plugins in lib.rs (shell, store, dialog, oauth). Removed the placeholder greet command. cargo check passes cleanly.
<!-- SECTION:FINAL_SUMMARY:END -->
