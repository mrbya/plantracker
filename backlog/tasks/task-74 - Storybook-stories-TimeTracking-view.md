---
id: TASK-74
title: 'Storybook stories: TimeTracking view'
status: To Do
assignee: []
created_date: '2026-03-25 14:50'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 11200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/views/TimeTracking.stories.svelte` covering `src/views/TimeTracking.svelte`.

## Approach
The view reads from the `planner` store (plans, tasks) and the `timer` store (isRunning, elapsedSeconds). Mock `invoke()` calls and pre-seed stores with fixture data for each story state.

Recommended fixture data:
```ts
const fixturePlans = [
  { id: 'p1', graphId: 'g1', title: 'Sprint 12', syncedAt: '...' },
  { id: 'p2', graphId: 'g2', title: 'Backlog', syncedAt: '...' },
];
const fixtureTasks = [
  { id: 't1', graphId: 'gt1', planId: 'p1', title: 'Implement auth flow', syncedAt: '...' },
  { id: 't2', graphId: 'gt2', planId: 'p1', title: 'Write migration', syncedAt: '...' },
];
```

## Stories to write

| Story name | Store state |
|---|---|
| `NoPlans` | `plans = []`, `isRunning = false` — shows EmptyState |
| `IdleWithPlans` | Plans and tasks loaded, no timer running, no plan/task selected |
| `PlanSelected` | `selectedPlan` set, tasks listed |
| `TimerRunning` | `isRunning = true`, `elapsedSeconds = 754`, plan and task selected |
| `TimerRunningNoTask` | `isRunning = true`, plan selected, `selectedTask = null` |

## Notes
- Story title: `PlanTracker/Views/TimeTracking`.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 NoPlans story shows EmptyState component
- [ ] #2 TimerRunning story shows elapsed time formatted as Xh Ym and a stop button
- [ ] #3 TimerRunning stop button is styled as danger variant
- [ ] #4 No invoke() errors in Storybook console
<!-- AC:END -->
