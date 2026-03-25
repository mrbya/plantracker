---
id: TASK-76
title: 'Storybook stories: Reports view'
status: To Do
assignee: []
created_date: '2026-03-25 14:50'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 11400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/views/Reports.stories.svelte` covering `src/views/Reports.svelte`.

## Approach
Seed the `planner` store with fixture plans/tasks. Mock `invoke('get_report')` to return fixture `ReportResult` data without hitting SQLite.

Fixture data:
```ts
const fixtureReport: ReportResult = {
  subjectLabel: 'Sprint 12',
  grandTotalSeconds: 14520,
  entries: [
    { taskTitle: 'Implement auth flow', planTitle: 'Sprint 12', startTime: '2026-03-20T09:00:00Z', endTime: '2026-03-20T11:30:00Z', durationSeconds: 9000, notes: null },
    { taskTitle: 'Write migration', planTitle: 'Sprint 12', startTime: '2026-03-21T13:00:00Z', endTime: '2026-03-21T14:30:00Z', durationSeconds: 5400, notes: 'Added indexes' },
    { taskTitle: 'Write migration', planTitle: 'Sprint 12', startTime: '2026-03-22T10:00:00Z', endTime: '2026-03-22T10:02:00Z', durationSeconds: 120, notes: null },
  ],
};
```

## Stories to write

| Story name | State |
|---|---|
| `NoEntries` | `invoke('get_report')` returns `{ entries: [], grandTotalSeconds: 0, subjectLabel: 'Sprint 12' }` — shows EmptyState |
| `WithEntries` | Report loaded with fixture entries; grand total and individual rows visible |
| `AllPlansFilter` | `selectedPlan = null` (all plans), entries from multiple plans |
| `Loading` | `invoke('get_report')` is a never-resolving promise — shows loading state |

## Notes
- Story title: `PlanTracker/Views/Reports`.
- Duration values in the entries list must be formatted as `Xh Ym` (use `formatDuration` from `src/lib/utils/duration.ts`).
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 NoEntries story shows EmptyState
- [ ] #2 WithEntries story shows grand total formatted as Xh Ym and all entry rows
- [ ] #3 Loading story shows spinner / disabled controls
- [ ] #4 No invoke() errors in Storybook console
<!-- AC:END -->
