---
id: TASK-46.3
title: 'Frontend: plan-level tracking in TimeTracking, ManualEntry, and entries tables'
status: Done
assignee: []
created_date: '2026-03-23 02:43'
updated_date: '2026-03-23 03:02'
labels:
  - frontend
dependencies:
  - TASK-46.2
parent_task_id: TASK-46
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Update the frontend so that selecting a plan (without a task) is sufficient to start a timer or save a manual entry. Update entry table displays in both views to show "No specific task" for taskless entries. Depends on TASK-46.2 (backend command changes).

**`src/lib/types.ts`**

```typescript
export interface TimeEntry {
    id: string;
    planId: string;            // NEW
    taskId: string | null;     // CHANGED — null for plan-level entries
    startTime: string;
    endTime: string | null;
    notes: string | null;
    createdAt: string;
}

export interface ActiveTimerInfo {
    entryId: string;
    planId: string;            // NEW
    taskId: string | null;     // CHANGED
    startTime: string;
    elapsedSeconds: number;
}
```

**`src/lib/api/index.ts`**

```typescript
export async function startTimer(planId: string, taskId?: string): Promise<TimeEntry> {
    return invoke<TimeEntry>('start_timer', { planId, taskId: taskId ?? null });
}

export async function createManualEntry(params: {
    planId: string;
    taskId?: string;
    startTime: string;
    endTime: string;
    notes?: string;
}): Promise<TimeEntry> {
    return invoke<TimeEntry>('create_manual_entry', {
        planId: params.planId,
        taskId: params.taskId ?? null,
        startTime: params.startTime,
        endTime: params.endTime,
        notes: params.notes ?? null,
    });
}
```

**`src/lib/stores/timer.ts`**

```typescript
export async function start(planId: string, taskId?: string): Promise<void> { ... }
```
In `initTimer`, reconstruct the minimal `TimeEntry` using `info.planId` and `info.taskId` (now nullable).

**`src/views/TimeTracking.svelte`**

- Start button: `disabled={timerBusy || !selectedPlanId}` (was `|| !selectedTaskId`).
- `handleStart()`: `await start(selectedPlanId, selectedTaskId || undefined)`.
- Entries table Task column: `entry.taskId ? (taskById[entry.taskId]?.title ?? entry.taskId) : "No specific task"`.
- Entries table Plan column: derive from `entry.planId` directly (available on all entries now) instead of going through the task — `planById[entry.planId]`.

**`src/views/ManualEntry.svelte`**

- `validate()`: require `selectedPlanId` (not `selectedTaskId`). Replace the task error with a plan error shown next to the Plan field. Remove the "Please select a task" message.
- `handleSubmit()`: pass `planId: selectedPlanId`, `taskId: selectedTaskId || undefined` to `createManualEntry`.
- Entries table Task and Plan columns: same logic as TimeTracking above.
- Rename `taskError` state variable to `planError` since validation now guards the plan.

**No changes needed**
- `Reports.svelte` — already renders `entry.taskTitle` from the backend; "No specific task" will come through automatically.
- `update_entry` flow in ManualEntry — the edit command does not touch `plan_id` or `task_id`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 TimeEntry type has planId: string and taskId: string | null.
- [ ] #2 ActiveTimerInfo type has planId: string and taskId: string | null.
- [ ] #3 startTimer API wrapper passes planId and optional taskId.
- [ ] #4 createManualEntry API wrapper passes planId and optional taskId.
- [ ] #5 timer store start() accepts planId and optional taskId.
- [ ] #6 In TimeTracking, Start Timer button is enabled when a plan is selected even if no task is selected.
- [ ] #7 In ManualEntry, Save Entry submits successfully with a plan selected but no task selected.
- [ ] #8 Both views show 'No specific task' in the Task column when entry.taskId is null.
- [ ] #9 Both views correctly resolve the Plan name for task-level and plan-level entries using entry.planId.
- [ ] #10 svelte-check passes with no errors.
<!-- AC:END -->
