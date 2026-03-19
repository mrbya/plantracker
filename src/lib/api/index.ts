/**
 * Single invoke boundary — all Tauri command calls live here.
 * Views and stores must import from this file, never call invoke() directly.
 */
import { invoke } from '@tauri-apps/api/core';

import type { ActiveTimerInfo, AuthStatus, Plan, SyncResult, Task, TimeEntry } from '$lib/types';

// ---------------------------------------------------------------------------
// Auth
// ---------------------------------------------------------------------------

export async function login(): Promise<AuthStatus> {
  return invoke<AuthStatus>('login');
}

export async function logout(): Promise<void> {
  return invoke('logout');
}

export async function getAuthStatus(): Promise<AuthStatus> {
  return invoke<AuthStatus>('get_auth_status');
}

// ---------------------------------------------------------------------------
// Sync & Planner
// ---------------------------------------------------------------------------

export async function syncPlansAndTasks(): Promise<SyncResult> {
  return invoke<SyncResult>('sync_plans_and_tasks');
}

export async function listPlans(): Promise<Plan[]> {
  return invoke<Plan[]>('list_plans');
}

export async function listTasksForPlan(planId: string): Promise<Task[]> {
  return invoke<Task[]>('list_tasks_for_plan', { planId });
}

// ---------------------------------------------------------------------------
// Timer
// ---------------------------------------------------------------------------

export async function startTimer(taskId: string): Promise<TimeEntry> {
  return invoke<TimeEntry>('start_timer', { taskId });
}

export async function stopTimer(): Promise<TimeEntry> {
  return invoke<TimeEntry>('stop_timer');
}

export async function getActiveTimer(): Promise<ActiveTimerInfo | null> {
  return invoke<ActiveTimerInfo | null>('get_active_timer');
}

export async function getRecentEntries(opts: {
  taskId?: string;
  planId?: string;
  limit: number;
}): Promise<TimeEntry[]> {
  return invoke<TimeEntry[]>('get_recent_entries', {
    taskId: opts.taskId ?? null,
    planId: opts.planId ?? null,
    limit: opts.limit,
  });
}
