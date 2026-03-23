---
id: TASK-45
title: Update documentation to reflect TASK-43 and TASK-44 changes
status: To Do
assignee: []
created_date: '2026-03-23 01:57'
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
- [ ] #1 `.claude/rules/frontend.md` Types section no longer shows `MonthlyTotal` or `monthlyTotals`.
- [ ] #2 `.claude/rules/frontend.md` Types section shows the updated `ReportResult` interface (with `entries: ReportEntry[]`) and the new `ReportEntry` interface.
- [ ] #3 No other file in `.claude/rules/` contains stale references to `MonthlyTotal` or the old monthly-subtotals report structure.
- [ ] #4 `CLAUDE.md` is verified and updated if any report-related content is stale.
- [ ] #5 Backlog documents (if any) describing the reports feature are verified and updated.
<!-- AC:END -->
