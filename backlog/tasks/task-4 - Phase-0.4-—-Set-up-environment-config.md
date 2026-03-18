---
id: TASK-4
title: Phase 0.4 — Set up environment config
status: Done
assignee: []
created_date: '2026-03-18 22:50'
updated_date: '2026-03-18 22:56'
labels: []
milestone: Phase 0 — Project Scaffold
dependencies:
  - TASK-1
priority: high
ordinal: 400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create the environment variable template and the Rust build script to forward env vars at compile time.

**`.env.example`**
```env
VITE_AZURE_CLIENT_ID=
VITE_AZURE_TENANT_ID=common
```

**`src-tauri/build.rs`**
```rust
fn main() {
    println!("cargo:rerun-if-env-changed=VITE_AZURE_CLIENT_ID");
    tauri_build::build()
}
```

Developers copy `.env.example` to `.env` and fill in their Azure app credentials.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 .env.example exists with VITE_AZURE_CLIENT_ID and VITE_AZURE_TENANT_ID keys
- [x] #2 src-tauri/build.rs exists and calls tauri_build::build()
- [x] #3 .env is listed in .gitignore (secrets never committed)
- [x] #4 `import.meta.env.VITE_AZURE_CLIENT_ID` is accessible in frontend code when .env is populated
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created .env.example with VITE_AZURE_CLIENT_ID and VITE_AZURE_TENANT_ID. Updated build.rs to add cargo:rerun-if-env-changed directives for both vars. .gitignore already correctly excludes .env/.env.* and explicitly allows .env.example.
<!-- SECTION:FINAL_SUMMARY:END -->
