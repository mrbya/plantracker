/**
 * Timer store.
 *
 * Manages the in-progress timer state that is shared across the Time Tracking
 * view and the sidebar.  The store owns:
 *
 * - `activeEntry` (private) — the `TimeEntry` row that was inserted with
 *   `end_time = NULL`; `null` when no timer is running.
 * - `elapsedSeconds` (public) — updated every second by a `setInterval` tick
 *   loop while a timer is running; reset to `0` when stopped.
 * - `isRunning` (public, derived) — `true` iff `activeEntry !== null`.
 *
 * Startup sequence:
 *   `initTimer()` is called by `+page.svelte` inside `onMount`.  It queries
 *   the backend for any open `time_entries` row (one with `end_time = NULL`)
 *   and, if found, restores the in-memory state and starts the tick loop.
 *   This ensures a timer that survived an app restart is displayed correctly.
 */
import { derived, writable } from "svelte/store";

import { getActiveTimer, startTimer, stopTimer } from "$lib/api";
import { addError } from "$lib/stores/notifications";
import type { TimeEntry } from "$lib/types";

/** Internal store — not exported; use `isRunning` to observe timer state. */
const activeEntry = writable<TimeEntry | null>(null);

/**
 * Whole seconds elapsed since the active timer started, ticked up by 1 every
 * second via `setInterval`.  The initial value on restore is seeded from the
 * backend (`ActiveTimerInfo.elapsedSeconds`) so it is accurate even after an
 * app restart.
 */
export const elapsedSeconds = writable<number>(0);

/**
 * Reactive flag that is `true` while a timer is running.  Use this to toggle
 * the Start / Stop button and to show the elapsed duration in the sidebar.
 */
export const isRunning = derived(activeEntry, (e) => e !== null);

/** Handle for the `setInterval` tick loop; `null` when no timer is active. */
let intervalId: ReturnType<typeof setInterval> | null = null;

/**
 * Starts the 1-second tick loop that increments `elapsedSeconds`.
 * Clears any existing interval first to avoid double-ticking.
 */
function startInterval(): void {
  if (intervalId !== null) clearInterval(intervalId);
  intervalId = setInterval(() => {
    elapsedSeconds.update((s) => s + 1);
  }, 1000);
}

/**
 * Stops the tick loop and clears the interval handle.
 * Safe to call even when no interval is running.
 */
function stopInterval(): void {
  if (intervalId !== null) {
    clearInterval(intervalId);
    intervalId = null;
  }
}

/**
 * Restores timer state on app startup.
 *
 * Queries the backend for an active (open-ended) timer.  If one exists the
 * store is hydrated with a reconstructed `TimeEntry`, `elapsedSeconds` is set
 * to the backend-computed elapsed time, and the tick loop is started.
 *
 * Must be called exactly once, inside `onMount` in `+page.svelte`.
 */
export async function initTimer(): Promise<void> {
  try {
    const info = await getActiveTimer();
    if (info) {
      // Reconstruct a minimal TimeEntry so isRunning becomes true.
      activeEntry.set({
        id: info.entryId,
        planId: info.planId,
        taskId: info.taskId,
        startTime: info.startTime,
        endTime: null,
        notes: null,
        createdAt: info.startTime,
      });
      elapsedSeconds.set(info.elapsedSeconds);
      startInterval();
    }
  } catch (e) {
    addError("Could not restore timer state: " + String(e));
  }
}

/**
 * Starts a timer for the given plan, optionally scoped to a task.
 *
 * Calls `start_timer` on the backend (which inserts a new open-ended
 * `time_entries` row), then sets `activeEntry`, resets `elapsedSeconds` to 0,
 * and starts the tick loop.
 *
 * Re-throws any backend error after surfacing it as a toast so callers can
 * decide how to handle the rejected promise (e.g., keep the button in a
 * non-loading state).
 *
 * @param planId - Local UUID of the plan to track time against.
 * @param taskId - Optional local UUID of a specific task within the plan.
 */
export async function start(planId: string, taskId?: string): Promise<void> {
  try {
    const entry = await startTimer(planId, taskId);
    activeEntry.set(entry);
    elapsedSeconds.set(0);
    startInterval();
  } catch (e) {
    addError("Failed to start timer: " + String(e));
    throw e;
  }
}

/**
 * Stops the running timer and marks the entry as complete.
 *
 * Calls `stop_timer` on the backend (which sets `end_time` on the active row),
 * then stops the tick loop and resets the in-memory state.
 *
 * Re-throws any backend error after surfacing it as a toast.
 *
 * @returns The completed `TimeEntry` with `endTime` now populated.
 */
export async function stop(): Promise<TimeEntry> {
  try {
    const entry = await stopTimer();
    stopInterval();
    activeEntry.set(null);
    elapsedSeconds.set(0);
    return entry;
  } catch (e) {
    addError("Failed to stop timer: " + String(e));
    throw e;
  }
}
