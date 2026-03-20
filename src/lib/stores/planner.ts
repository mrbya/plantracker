import { get, writable } from "svelte/store";

import { listPlans, listTasksForPlan, syncPlansAndTasks } from "$lib/api";
import { addError, addSuccess } from "$lib/stores/notifications";
import { saveLastSyncedAt } from "$lib/stores/settings";
import type { Plan, Task } from "$lib/types";

export const plans = writable<Plan[]>([]);
export const tasksByPlan = writable<Record<string, Task[]>>({});
export const selectedPlan = writable<Plan | null>(null);
export const selectedTask = writable<Task | null>(null);

/** Selects a task and auto-updates selectedPlan to its parent. */
export function selectTask(task: Task): void {
  selectedTask.set(task);
  const plan = get(plans).find((p) => p.id === task.planId) ?? null;
  selectedPlan.set(plan);
}

/** Syncs plans/tasks from Microsoft Graph, then reloads stores from SQLite. */
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
