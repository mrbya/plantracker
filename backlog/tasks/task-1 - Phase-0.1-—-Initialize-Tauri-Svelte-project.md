---
id: TASK-1
title: Phase 0.1 — Initialize Tauri + Svelte project
status: Done
assignee: []
created_date: '2026-03-18 22:50'
labels: []
milestone: Phase 0 — Project Scaffold
dependencies: []
priority: high
ordinal: 100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Bootstrap the project using `pnpm create tauri-app` with the Svelte TypeScript template and Tauri 2.

```bash
pnpm create tauri-app plantracker \
  --template svelte-ts \
  --manager pnpm \
  --tauri-version 2
cd plantracker
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Project directory created with Tauri 2 + Svelte TypeScript template
- [ ] #2 `cargo tauri dev` launches without errors
- [ ] #3 Svelte HMR works (edit App.svelte, see instant update)
<!-- AC:END -->
