---
id: TASK-11
title: Phase 2.2 — Write initial SQL migration
status: Done
assignee: []
created_date: '2026-03-19 09:27'
updated_date: '2026-03-19 10:53'
labels:
  - backend
  - database
  - phase-2
dependencies:
  - TASK-10
priority: high
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create the numbered migration file that defines the full database schema.

**File:** `src-tauri/migrations/0001_initial.sql`

Schema to create:
- `plans` table: `id TEXT PRIMARY KEY`, `graph_id TEXT NOT NULL UNIQUE`, `title TEXT NOT NULL`, `synced_at TEXT NOT NULL`
- `tasks` table: `id TEXT PRIMARY KEY`, `graph_id TEXT NOT NULL UNIQUE`, `plan_id TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE`, `title TEXT NOT NULL`, `synced_at TEXT NOT NULL`
- `time_entries` table: `id TEXT PRIMARY KEY`, `task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE`, `start_time TEXT NOT NULL`, `end_time TEXT` (NULL = active timer), `notes TEXT`, `created_at TEXT NOT NULL`
- Indexes: `idx_time_entries_task_id`, `idx_time_entries_start_time`, `idx_tasks_plan_id`

All tables use `IF NOT EXISTS`. All datetimes stored as ISO 8601 `TEXT`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Migration file exists at src-tauri/migrations/0001_initial.sql
- [x] #2 All three tables created with correct columns and types
- [x] #3 Foreign key constraints defined with ON DELETE CASCADE
- [x] #4 All three indexes created
- [x] #5 sqlx::migrate!() applies the migration without errors
<!-- AC:END -->
