---
id: TASK-26
title: Phase 4.5 — Frontend planner store and sync integration
status: Done
assignee: []
created_date: '2026-03-19 13:08'
updated_date: '2026-03-19 13:20'
labels:
  - frontend
  - typescript
  - svelte
milestone: Phase 4
dependencies:
  - TASK-25
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/lib/stores/planner.ts` with reactive stores for plans and tasks, wire up sync on login/startup, and add the `syncPlansAndTasks` invoke wrapper to the API layer.

## Requirements

**`src/lib/api/index.ts`** — add:
```typescript
export async function syncPlansAndTasks(): Promise<SyncResult> { return invoke('sync_plans_and_tasks'); }
export async function getPlans(): Promise<Plan[]> { return invoke('list_plans'); }  // if needed
```

**`src/lib/stores/planner.ts`**:
```typescript
export const plans = writable<Plan[]>([]);
export const tasksByPlan = writable<Record<string, Task[]>>({});
export const selectedPlan = writable<Plan | null>(null);
export const selectedTask = writable<Task | null>(null);

export function selectTask(task: Task): void {
    selectedTask.set(task);
    const plan = get(plans).find(p => p.id === task.planId) ?? null;
    selectedPlan.set(plan);
}

export async function syncAndLoad(): Promise<void> {
    // 1. Call syncPlansAndTasks invoke
    // 2. Refresh plans store from returned data or a follow-up list query
    // 3. Show success/error toast
}
```

**`src/lib/types.ts`** — add if missing:
```typescript
export interface SyncResult { plansCount: number; tasksCount: number; syncedAt: string; }
```

**Startup wiring**: call `syncAndLoad()` from the auth store's `login()` function (after tokens are set) and from `initAuth()` when already authenticated on app load.

- Follow all rules in `frontend.md`: stores call API, views bind to stores, no direct `invoke()` in views.
- Show a success toast with plan/task counts after sync, error toast on failure.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 planner.ts exports plans, tasksByPlan, selectedPlan, selectedTask stores
- [x] #2 selectTask() auto-updates selectedPlan to the task's parent plan
- [x] #3 syncAndLoad() is called after login and on startup when already authenticated
- [x] #4 Success toast shows synced plan and task counts
- [x] #5 Error toast shown if sync fails (app remains functional with cached data)
- [x] #6 SyncResult type added to types.ts
- [x] #7 All invoke calls go through src/lib/api/index.ts
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/lib/stores/planner.ts` with `plans`, `tasksByPlan`, `selectedPlan`, `selectedTask` stores and `selectTask()` / `syncAndLoad()` functions. Added `SyncResult` to `types.ts`. Added `syncPlansAndTasks`, `listPlans`, `listTasksForPlan` wrappers to `api/index.ts`. Added `list_plans` and `list_tasks_for_plan` Tauri commands to `commands/sync.rs` and registered them in `lib.rs`. Wired `syncAndLoad()` into `auth.ts` — called after login and when already authenticated on startup. TypeScript and Rust both pass type-checks with no errors.
<!-- SECTION:FINAL_SUMMARY:END -->
