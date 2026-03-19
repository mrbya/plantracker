---
id: TASK-28
title: Phase 5.2 — Timer frontend store
status: Done
assignee: []
created_date: '2026-03-19 13:24'
updated_date: '2026-03-19 13:30'
labels:
  - frontend
  - typescript
  - svelte
milestone: Phase 5
dependencies:
  - TASK-27
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/lib/stores/timer.ts` with reactive timer state that polls the backend and drives the live elapsed-time display.

## API wrappers (add to `src/lib/api/index.ts`)

```typescript
export async function startTimer(taskId: string): Promise<TimeEntry>
export async function stopTimer(): Promise<TimeEntry>
export async function getActiveTimer(): Promise<ActiveTimerInfo | null>
export async function getRecentEntries(opts: { taskId?: string; planId?: string; limit: number }): Promise<TimeEntry[]>
```

## Types (add to `src/lib/types.ts`)

```typescript
export interface ActiveTimerInfo {
  entryId: string;
  taskId: string;
  startTime: string;
  elapsedSeconds: number;
}
```

## Store (`src/lib/stores/timer.ts`)

```typescript
export const activeEntry = writable<TimeEntry | null>(null);
export const elapsedSeconds = writable<number>(0);
export const isRunning = derived(activeEntry, e => e !== null);

export async function initTimer(): Promise<void>
// Call getActiveTimer on startup; if running, set activeEntry and start interval

export async function start(taskId: string): Promise<void>
// Call startTimer, set activeEntry, start 1-second interval to increment elapsedSeconds

export async function stop(): Promise<void>
// Call stopTimer, clear activeEntry, clear interval, set elapsedSeconds to 0
```

The `setInterval` ticks every 1000 ms and increments `elapsedSeconds` by 1. Always `clearInterval` before starting a new one to avoid leaks.

## Startup wiring

Call `initTimer()` from the root `+page.svelte` `onMount` alongside `initAuth()` so the timer resumes correctly after an app restart (pairs with the backend restore added in TASK-27).
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 timer.ts exports activeEntry, elapsedSeconds, isRunning, start(), stop(), initTimer()
- [x] #2 isRunning is derived from activeEntry (true when non-null)
- [x] #3 elapsedSeconds increments every second while a timer is running
- [x] #4 Interval is cleared when stop() is called — no memory leaks
- [x] #5 initTimer() restores elapsed count from backend if a timer was already running
- [x] #6 ActiveTimerInfo type added to types.ts
- [x] #7 All invoke calls go through api/index.ts
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/lib/stores/timer.ts` with `activeEntry`, `elapsedSeconds`, `isRunning` (derived), `start()`, `stop()`, `initTimer()`. Interval is managed via a module-level `intervalId` — always cleared before starting a new one. `initTimer()` calls `getActiveTimer` and restores elapsed count from backend. Added `ActiveTimerInfo` to `types.ts`, timer API wrappers to `api/index.ts`. Wired `initTimer()` into `+page.svelte` `onMount`. TypeScript passes with no errors.
<!-- SECTION:FINAL_SUMMARY:END -->
