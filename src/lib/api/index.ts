/**
 * Single invoke boundary — all Tauri command calls live here.
 * Views and stores must import from this file, never call invoke() directly.
 */
import { invoke } from '@tauri-apps/api/core';

import type { AuthStatus, Plan, SyncResult, Task } from '$lib/types';

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
