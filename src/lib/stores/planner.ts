/**
 * Planner store.
 *
 * Owns all plan and task data that has been synced from Microsoft Graph and
 * cached in the local SQLite database, as well as the currently selected plan
 * and task (shared across the Time Tracking, Manual Entry, and Reports views).
 *
 * Selection invariant: whenever a task is selected its parent plan is also
 * selected.  Use `selectTask()` (never `selectedTask.set()` directly) to
 * maintain this invariant.
 *
 * Two-phase sync design:
 *   1. `syncPlansAndTasks()` pushes to/from Graph and updates SQLite.
 *   2. `listPlans()` / `listTasksForPlan()` read from SQLite into memory.
 *   Separating these phases means the UI still loads from cache even when the
 *   Graph request fails.
 */
import { get, writable } from "svelte/store";

import { listPlans, listTasksForPlan, syncPlansAndTasks } from "$lib/api";
import { addError, addSuccess } from "$lib/stores/notifications";
import { saveLastSyncedAt } from "$lib/stores/settings";
import type { Plan, Task } from "$lib/types";

/** All plans currently cached in the local SQLite database. */
export const plans = writable<Plan[]>([]);

/**
 * Task lists keyed by local plan UUID.
 * Access as `$tasksByPlan[planId]`; may be an empty array for plans with no
 * tasks, or `undefined` if the plan has not been loaded yet.
 */
export const tasksByPlan = writable<Record<string, Task[]>>({});

/**
 * The plan currently selected in any view, or `null` if none is selected.
 * Updated by dropdown change handlers and automatically by `selectTask()`.
 */
export const selectedPlan = writable<Plan | null>(null);

/**
 * The task currently selected in any view, or `null` if none is selected.
 * Always update via `selectTask()` rather than directly to keep `selectedPlan`
 * in sync.
 */
export const selectedTask = writable<Task | null>(null);

/**
 * Selects a task and automatically updates `selectedPlan` to the task's parent.
 *
 * Use this function everywhere a task selection needs to be persisted to the
 * global store — it enforces the invariant that `selectedPlan` always matches
 * the selected task's `planId`.
 *
 * @param task - The task to select.
 */
export function selectTask(task: Task): void {
  selectedTask.set(task);
  const plan = get(plans).find((p) => p.id === task.planId) ?? null;
  selectedPlan.set(plan);
}

/**
 * Syncs plans and tasks from Microsoft Graph, then reloads stores from the
 * local SQLite database.
 *
 * Shows a success toast with the upserted counts on a successful Graph sync,
 * or an error toast if the network request fails.  Either way the function
 * continues to the local-load phase so the UI always displays the most
 * recently cached data.
 *
 * Also persists the sync timestamp via `saveLastSyncedAt` so the Settings
 * view can display "Last synced: …".
 */
export async function syncAndLoad(): Promise<void> {
  try {
    const result = await syncPlansAndTasks();
    addSuccess(
      `Synced ${result.plansCount} plans and ${result.tasksCount} tasks`,
    );
    saveLastSyncedAt(new Date().toISOString());
  } catch (e) {
    addError("Sync failed: " + String(e));
    // Fall through to still load whatever is cached in SQLite.
  }

  try {
    const loadedPlans = await listPlans();
    plans.set(loadedPlans);

    const byPlan: Record<string, Task[]> = {};
    for (const plan of loadedPlans) {
      byPlan[plan.id] = await listTasksForPlan(plan.id);
    }
    tasksByPlan.set(byPlan);
  } catch (e) {
    addError("Failed to load plans from local storage: " + String(e));
  }
}
