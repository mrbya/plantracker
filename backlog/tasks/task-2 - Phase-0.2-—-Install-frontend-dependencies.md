---
id: TASK-2
title: Phase 0.2 — Install frontend dependencies
status: Done
assignee: []
created_date: '2026-03-18 22:50'
updated_date: '2026-03-18 22:53'
labels: []
milestone: Phase 0 — Project Scaffold
dependencies:
  - TASK-1
priority: high
ordinal: 200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Install required frontend dev dependencies. No runtime UI framework — vanilla Svelte only.

```bash
pnpm add -D @types/node
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 @types/node installed as a dev dependency
- [x] #2 `pnpm tsc --noEmit` passes without errors
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
@types/node was already present at ^25.5.0. Removed a now-stale `@ts-expect-error` directive from vite.config.js (process is correctly typed once @types/node is installed). `pnpm tsc --noEmit` passes cleanly.
<!-- SECTION:FINAL_SUMMARY:END -->
