---
id: TASK-45
title: Update documentation to reflect TASK-43 and TASK-44 changes
status: Done
assignee: []
created_date: '2026-03-23 01:57'
updated_date: '2026-03-23 02:24'
labels:
  - documentation
dependencies:
  - TASK-43
  - TASK-44
priority: low
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After TASK-43 and TASK-44 are implemented, several documentation files will be stale and must be brought up to date.

**Files to review and update**

`.claude/rules/frontend.md` — the Types section (lines 47–48) shows the old `ReportResult` and `MonthlyTotal` interfaces inline. These must be updated to reflect the new shapes:
- Remove `MonthlyTotal` (deleted from the codebase by TASK-43).
- Update `ReportResult` to show `entries: ReportEntry[]` instead of `monthlyTotals: MonthlyTotal[]`.
- Add `ReportEntry` with its fields: `taskTitle`, `planTitle`, `startTime`, `endTime`, `durationSeconds`, `notes`.

**Other files to check** (update only if content is outdated after the tasks):
- `CLAUDE.md` — does not currently describe report-specific types or UI behaviour, but verify nothing references monthly totals or the old results layout.
- Other `.claude/rules/` files — scan for any mention of `MonthlyTotal`, `monthlyTotals`, or the old reports table structure and fix if found.
- Any project backlog documents (`backlog/docs/`) that describe the reports feature.

Do not change rule content that remains accurate — only update what is made incorrect or incomplete by the two tasks.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `.claude/rules/frontend.md` Types section no longer shows `MonthlyTotal` or `monthlyTotals`.
- [x] #2 `.claude/rules/frontend.md` Types section shows the updated `ReportResult` interface (with `entries: ReportEntry[]`) and the new `ReportEntry` interface.
- [x] #3 No other file in `.claude/rules/` contains stale references to `MonthlyTotal` or the old monthly-subtotals report structure.
- [x] #4 `CLAUDE.md` is verified and updated if any report-related content is stale.
- [x] #5 Backlog documents (if any) describing the reports feature are verified and updated.
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Updated four stale references across three live documentation files.

**`.claude/rules/frontend.md`**
- Replaced `ReportResult { monthlyTotals: MonthlyTotal[]; ... }` and `MonthlyTotal { year; month; totalSeconds; }` with `ReportEntry { taskTitle; planTitle; startTime; endTime; durationSeconds; notes; }` and `ReportResult { entries: ReportEntry[]; grandTotalSeconds; subjectLabel; }`.

**`README.md`**
- Features list (line 41): "Monthly breakdowns, plan/task totals" → "Grand total + per-entry breakdown, plan/task filter".
- Reports view description (line 97): "Table shows: monthly totals, grand total..." → "Table shows: grand total at top, then individual entries (task, plan, start, end, duration, notes)".

**`docs/IMPLEMENTATION_PLAN.md` — Appendix C type definitions**
- Same interface update as `frontend.md` — the authoritative type reference now matches the codebase.

**Not changed:**
- Phase 7 planning section of `IMPLEMENTATION_PLAN.md` — these are historical plan descriptions (future tense, "Create ... with ..."), not a living reference.
- `backlog/completed/` task files — immutable historical records of original implementations.
- `backlog/tasks/task-43` and `task-45` — these correctly describe the changes made, not the old state.
<!-- SECTION:FINAL_SUMMARY:END -->
