/**
 * Single invoke boundary — all Tauri command calls live here.
 * Views and stores must import from this file, never call invoke() directly.
 */
import { invoke } from '@tauri-apps/api/core';

import type { ActiveTimerInfo, AuthStatus, Plan, ReportResult, SyncResult, Task, TimeEntry } from '$lib/types';

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

// ---------------------------------------------------------------------------
// Manual entries
// ---------------------------------------------------------------------------

export async function createManualEntry(params: {
  taskId: string;
  startTime: string;
  endTime: string;
  notes?: string;
}): Promise<TimeEntry> {
  return invoke<TimeEntry>('create_manual_entry', {
    taskId: params.taskId,
    startTime: params.startTime,
    endTime: params.endTime,
    notes: params.notes ?? null,
  });
}

export async function updateEntry(params: {
  id: string;
  startTime: string;
  endTime: string;
  notes?: string;
}): Promise<TimeEntry> {
  return invoke<TimeEntry>('update_entry', {
    id: params.id,
    startTime: params.startTime,
    endTime: params.endTime,
    notes: params.notes ?? null,
  });
}

export async function deleteEntry(id: string): Promise<void> {
  return invoke('delete_entry', { id });
}

// ---------------------------------------------------------------------------
// Reports
// ---------------------------------------------------------------------------

export async function exportReportCsv(report: ReportResult): Promise<string> {
  return invoke<string>('export_report_csv', { report });
}

export async function generateReport(params: {
  planId?: string;
  taskId?: string;
  fromYear: number;
  fromMonth: number;
  toYear: number;
  toMonth: number;
}): Promise<ReportResult> {
  return invoke<ReportResult>('generate_report', {
    planId: params.planId ?? null,
    taskId: params.taskId ?? null,
    fromYear: params.fromYear,
    fromMonth: params.fromMonth,
    toYear: params.toYear,
    toMonth: params.toMonth,
  });
}
