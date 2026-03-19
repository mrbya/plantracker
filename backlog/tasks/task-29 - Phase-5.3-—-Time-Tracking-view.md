---
id: TASK-29
title: Phase 5.3 — Time Tracking view
status: Done
assignee: []
created_date: '2026-03-19 13:24'
updated_date: '2026-03-19 13:33'
labels:
  - frontend
  - typescript
  - svelte
milestone: Phase 5
dependencies:
  - TASK-28
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/views/TimeTracking.svelte` — the main time-tracking UI with plan/task selection, a start/stop timer button, and a recent entries table.

## Layout

**Top section — controls**
- Plan `<Select>` bound to `selectedPlan` from `planner` store; options from `plans` store
- Task `<Select>` bound to `selectedTask` from `planner` store; options filtered to `tasksByPlan[selectedPlan.id]`
- When a task is selected from the dropdown, call `selectTask(task)` to auto-sync the plan selection
- Start / Stop button:
  - When not running (`!$isRunning`): green primary button labeled "Start Timer", enabled only when a task is selected
  - When running (`$isRunning`): red danger button labeled "Stop — {formatDuration($elapsedSeconds)}"
  - Disabled + spinner while the async start/stop action is in flight

**Bottom section — recent entries table**
- Fetch entries on mount and after every start/stop via `getRecentEntries` (limit 20, filtered by selected plan or task)
- Columns: Task | Plan | Start | End | Duration
  - Duration: `formatDuration(seconds)` from `src/lib/utils/duration.ts`
  - End: "Running…" (in `--timer-active` color) for entries with `endTime = null`
- Delete button per row — show inline confirmation ("Sure?") before calling `deleteEntry` (Phase 6 command; stub the call or leave a TODO if not yet implemented)
- `<EmptyState>` when no entries

## Duration utility

Create `src/lib/utils/duration.ts` if it doesn't exist:
```typescript
export function formatDuration(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  return h > 0 ? `${h}h ${m}m` : `${m}m`;
}

export function formatDurationCSV(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  return `${h}:${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
}
```

## State handling
- Loading spinner while fetching entries
- Error toast on start/stop failure (timer button re-enabled)
- Cannot start a second timer: button disabled when `$isRunning` and different task selected (show tooltip "A timer is already running")
- `<EmptyState>` when entries list is empty

## Notes
- The `deleteEntry` Tauri command is implemented in Phase 6 (TASK-31). Add the delete button UI now but stub the handler with a TODO comment if TASK-31 isn't done yet.
- All colors via CSS variables (no hardcoded hex).
- No inline styles except dynamic values like progress/elapsed display.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Plan dropdown populates from plans store
- [x] #2 Task dropdown filters to selected plan's tasks
- [x] #3 Selecting a task auto-updates the plan dropdown via selectTask()
- [x] #4 Start button is disabled when no task is selected
- [x] #5 Clicking Start creates a DB entry and begins counting elapsed time
- [x] #6 Stop button shows live elapsed time and stops the timer on click
- [x] #7 Recent entries table shows last 20 entries with Task, Plan, Start, End, Duration columns
- [x] #8 Entries with endTime = null show 'Running...' in timer-active color
- [x] #9 EmptyState shown when no entries exist
- [x] #10 formatDuration utility created in src/lib/utils/duration.ts
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented `src/views/TimeTracking.svelte` with plan/task dropdowns, start/stop timer button (green success variant when idle, red danger when running), and recent entries table. Created `src/lib/utils/duration.ts` with `formatDuration` and `formatDurationCSV`. Added `success` variant to `Button.svelte` using `--success`/`--timer-active` CSS vars. Updated `Select.svelte` to forward `id` and `onchange` props. Delete button shows inline "Sure?/Cancel" confirmation; stubbed with TODO pending Phase 6 `deleteEntry` command. TypeScript passes with no errors.
<!-- SECTION:FINAL_SUMMARY:END -->
