---
id: TASK-3
title: Phase 0.3 — Install Rust crates
status: Done
assignee: []
created_date: '2026-03-18 22:50'
updated_date: '2026-03-18 22:56'
labels: []
milestone: Phase 0 — Project Scaffold
dependencies:
  - TASK-1
priority: high
ordinal: 300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add all required Rust dependencies to `src-tauri/Cargo.toml`.

```toml
[dependencies]
tauri = { version = "2", features = ["protocol-asset"] }
tauri-plugin-store = "2"
tauri-plugin-shell = "2"
tauri-plugin-oauth = "2"
tauri-plugin-dialog = "2"
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-rustls", "chrono", "macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
keyring = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
anyhow = "1"
base64 = "0.22"
sha2 = "0.10"
rand = "0.8"
csv = "1"
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All crates listed in Cargo.toml under [dependencies]
- [x] #2 `cargo build` (or `cargo check`) compiles without errors
- [x] #3 No duplicate or conflicting dependency versions
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added all required crates to src-tauri/Cargo.toml. Also enabled `assetProtocol` in tauri.conf.json (required to match the `protocol-asset` Cargo feature). `cargo check` passes cleanly.
<!-- SECTION:FINAL_SUMMARY:END -->
