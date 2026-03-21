import { derived, writable } from "svelte/store";

import { getActiveTimer, startTimer, stopTimer } from "$lib/api";
import { addError } from "$lib/stores/notifications";
import type { TimeEntry } from "$lib/types";

const activeEntry = writable<TimeEntry | null>(null);
export const elapsedSeconds = writable<number>(0);
export const isRunning = derived(activeEntry, (e) => e !== null);

let intervalId: ReturnType<typeof setInterval> | null = null;

function startInterval(): void {
  if (intervalId !== null) clearInterval(intervalId);
  intervalId = setInterval(() => {
    elapsedSeconds.update((s) => s + 1);
  }, 1000);
}

function stopInterval(): void {
  if (intervalId !== null) {
    clearInterval(intervalId);
    intervalId = null;
  }
}

/** Restores timer state on app startup. Call from root onMount. */
export async function initTimer(): Promise<void> {
  try {
    const info = await getActiveTimer();
    if (info) {
      // Reconstruct a minimal TimeEntry so isRunning becomes true.
      activeEntry.set({
        id: info.entryId,
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

/** Starts a timer for the given task. */
export async function start(taskId: string): Promise<void> {
  try {
    const entry = await startTimer(taskId);
    activeEntry.set(entry);
    elapsedSeconds.set(0);
    startInterval();
  } catch (e) {
    addError("Failed to start timer: " + String(e));
    throw e;
  }
}

/** Stops the running timer. */
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
