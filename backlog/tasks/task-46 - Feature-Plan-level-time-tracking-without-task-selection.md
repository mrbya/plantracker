---
id: TASK-46
title: 'Feature: Plan-level time tracking without task selection'
status: To Do
assignee: []
created_date: '2026-03-23 02:41'
updated_date: '2026-03-23 02:41'
labels:
  - feature
  - backend
  - frontend
  - database
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Allow users to track time and add manual entries against a plan without selecting a specific task. These "taskless" entries are stored with a plan reference but no task reference, and appear in the Reports view labelled "No specific task".

**Current behaviour**
- Selecting a task is required in both TimeTracking and ManualEntry before any action is possible.
- `time_entries.task_id` is `NOT NULL` with `ON DELETE CASCADE` to `tasks`.
- The timer store, API wrappers, and all Rust commands assume a task ID is always present.

**Desired behaviour**
- Selecting a plan (with no task) is sufficient to start a timer or save a manual entry.
- "Taskless" entries are stored with `plan_id` set and `task_id NULL`.
- The entries table in TimeTracking and ManualEntry shows "No specific task" in the Task column for these entries.
- The Reports view already uses `task_title` from the backend; that field will say "No specific task" for taskless entries.

**Subtask breakdown**
- TASK-47 — DB migration + Rust model (foundation layer)
- TASK-48 — Backend Tauri commands (depends on TASK-47)
- TASK-49 — Frontend changes (depends on TASK-48)
<!-- SECTION:DESCRIPTION:END -->
