/**
 * Single invoke boundary — all Tauri command calls live here.
 *
 * Views and stores must import from this file, never call `invoke()` directly.
 * Grouping all IPC calls in one place makes the contract between the frontend
 * and the Rust backend visible at a glance and simplifies mocking in tests.
 */
import { invoke } from "@tauri-apps/api/core";

import type {
  ActiveTimerInfo,
  AuthStatus,
  Plan,
  ReportResult,
  SyncResult,
  Task,
  TimeEntry,
} from "$lib/types";

// ---------------------------------------------------------------------------
// Auth
// ---------------------------------------------------------------------------

/**
 * Opens the Microsoft OAuth 2.0 PKCE login flow in the system browser.
 *
 * Resolves once the user has authenticated and the backend has stored the
 * resulting tokens in the OS keychain.  The returned `AuthStatus` reflects
 * the freshly authenticated state (including the display name fetched from
 * `GET /me`).
 *
 * @returns The updated authentication status.
 * @throws If the login flow is cancelled, times out, or the token exchange fails.
 */
export async function login(): Promise<AuthStatus> {
  return invoke<AuthStatus>("login");
}

/**
 * Signs the current user out and removes all tokens from the OS keychain.
 *
 * After this call the `auth` store resets to unauthenticated and the app
 * navigates to the `Login` view.
 *
 * @throws If the backend fails to clear tokens from the keychain.
 */
export async function logout(): Promise<void> {
  return invoke("logout");
}

/**
 * Reads the current authentication state from the backend without triggering
 * a new login flow.
 *
 * Used on startup to determine whether a valid (or silently refreshable)
 * token already exists in the OS keychain so the app can skip the login
 * screen.
 *
 * @returns The current authentication status; `isAuthenticated` is `false`
 *   if no token exists or the stored token cannot be refreshed.
 * @throws If the keychain cannot be read (rare platform error).
 */
export async function getAuthStatus(): Promise<AuthStatus> {
  return invoke<AuthStatus>("get_auth_status");
}

// ---------------------------------------------------------------------------
// Sync & Planner
// ---------------------------------------------------------------------------

/**
 * Triggers a full synchronisation with Microsoft Graph.
 *
 * Fetches all plans accessible to the signed-in user and their tasks, then
 * upserts every item into the local SQLite database.  The returned counts
 * reflect what was written, not necessarily what changed.
 *
 * @returns Summary of how many plans and tasks were upserted.
 * @throws If the Graph request fails (network error, `401`, `429`, etc.).
 */
export async function syncPlansAndTasks(): Promise<SyncResult> {
  return invoke<SyncResult>("sync_plans_and_tasks");
}

/**
 * Loads all plans currently stored in the local SQLite database.
 *
 * Returns whatever was persisted during the last successful sync — no network
 * request is made.  The array is ordered by title ascending.
 *
 * @returns All locally cached plans, or an empty array if none have been
 *   synced yet.
 * @throws If the database query fails.
 */
export async function listPlans(): Promise<Plan[]> {
  return invoke<Plan[]>("list_plans");
}

/**
 * Loads all tasks belonging to a specific plan from the local SQLite database.
 *
 * No network request is made; results reflect the last successful sync.
 *
 * @param planId - The local UUID of the parent plan (`Plan.id`).
 * @returns Tasks for the given plan, ordered by title ascending.
 * @throws If the database query fails or `planId` does not exist.
 */
export async function listTasksForPlan(planId: string): Promise<Task[]> {
  return invoke<Task[]>("list_tasks_for_plan", { planId });
}

// ---------------------------------------------------------------------------
// Timer
// ---------------------------------------------------------------------------

/**
 * Starts a new time-tracking session for the given plan, optionally scoped to
 * a specific task.
 *
 * Inserts a `time_entries` row with `end_time = NULL`.  The backend enforces
 * at most one active entry at a time — calling this while a timer is running
 * will throw an error.
 *
 * @param planId - Local UUID of the plan to track time against.
 * @param taskId - Optional local UUID of a specific task within the plan.
 *   Pass `undefined` to create a plan-level (unscoped) entry.
 * @returns The newly created `TimeEntry` with `endTime` set to `null`.
 * @throws If a timer is already running, or if the database insert fails.
 */
export async function startTimer(
  planId: string,
  taskId?: string,
): Promise<TimeEntry> {
  return invoke<TimeEntry>("start_timer", { planId, taskId: taskId ?? null });
}

/**
 * Stops the currently running timer and marks it as complete.
 *
 * Sets `end_time` on the active `time_entries` row to the current UTC
 * timestamp.
 *
 * @returns The updated `TimeEntry` with `endTime` now populated.
 * @throws If no timer is running, or if the database update fails.
 */
export async function stopTimer(): Promise<TimeEntry> {
  return invoke<TimeEntry>("stop_timer");
}

/**
 * Returns a snapshot of the currently running timer, or `null` if no timer is
 * active.
 *
 * `elapsedSeconds` is computed server-side as `now − startTime`, so it is
 * always accurate even if the polling interval has drifted.
 *
 * @returns Timer info including elapsed seconds, or `null` when idle.
 * @throws If the database query fails.
 */
export async function getActiveTimer(): Promise<ActiveTimerInfo | null> {
  return invoke<ActiveTimerInfo | null>("get_active_timer");
}

/**
 * Returns the most recent completed (and optionally in-progress) time entries,
 * filtered by task or plan.
 *
 * Scoping priority: if `taskId` is provided it takes precedence over `planId`.
 * If neither is provided, entries from all plans are returned.
 *
 * @param opts.taskId - Optional: restrict to entries for this task UUID.
 * @param opts.planId - Optional: restrict to entries for this plan UUID
 *   (ignored when `taskId` is also provided).
 * @param opts.limit - Maximum number of rows to return.
 * @returns Entries ordered by `start_time` descending, at most `limit` rows.
 * @throws If the database query fails.
 */
export async function getRecentEntries(opts: {
  taskId?: string;
  planId?: string;
  limit: number;
}): Promise<TimeEntry[]> {
  return invoke<TimeEntry[]>("get_recent_entries", {
    taskId: opts.taskId ?? null,
    planId: opts.planId ?? null,
    limit: opts.limit,
  });
}

// ---------------------------------------------------------------------------
// Manual entries
// ---------------------------------------------------------------------------

/**
 * Inserts a manually recorded time entry into the database.
 *
 * Both `startTime` and `endTime` must be provided and `endTime` must be after
 * `startTime`.  The backend validates this and returns an error otherwise.
 *
 * @param params.planId - Local UUID of the parent plan.
 * @param params.taskId - Optional local UUID of the task; omit for plan-level.
 * @param params.startTime - ISO 8601 UTC start timestamp.
 * @param params.endTime - ISO 8601 UTC end timestamp.
 * @param params.notes - Optional free-text annotation for the entry.
 * @returns The newly created `TimeEntry`.
 * @throws If the timestamps are invalid, or if the database insert fails.
 */
export async function createManualEntry(params: {
  planId: string;
  taskId?: string;
  startTime: string;
  endTime: string;
  notes?: string;
}): Promise<TimeEntry> {
  return invoke<TimeEntry>("create_manual_entry", {
    planId: params.planId,
    taskId: params.taskId ?? null,
    startTime: params.startTime,
    endTime: params.endTime,
    notes: params.notes ?? null,
  });
}

/**
 * Updates the timestamps and notes of an existing time entry.
 *
 * Only completed entries (those with a non-null `endTime`) should be edited
 * via this command.  The plan and task association cannot be changed.
 *
 * @param params.id - UUID of the entry to update.
 * @param params.startTime - New ISO 8601 UTC start timestamp.
 * @param params.endTime - New ISO 8601 UTC end timestamp.
 * @param params.notes - Replacement notes text; `undefined` clears the notes.
 * @returns The updated `TimeEntry`.
 * @throws If the entry is not found, timestamps are invalid, or the update fails.
 */
export async function updateEntry(params: {
  id: string;
  startTime: string;
  endTime: string;
  notes?: string;
}): Promise<TimeEntry> {
  return invoke<TimeEntry>("update_entry", {
    id: params.id,
    startTime: params.startTime,
    endTime: params.endTime,
    notes: params.notes ?? null,
  });
}

/**
 * Permanently deletes a time entry from the database.
 *
 * This operation is irreversible.  The UI should present a confirmation step
 * before calling this function.
 *
 * @param id - UUID of the entry to delete.
 * @throws If the entry is not found or the database delete fails.
 */
export async function deleteEntry(id: string): Promise<void> {
  return invoke("delete_entry", { id });
}

// ---------------------------------------------------------------------------
// Reports
// ---------------------------------------------------------------------------

/**
 * Serialises a `ReportResult` to a CSV file chosen by the user via the
 * platform save-file dialog.
 *
 * The dialog is opened by the backend (via `tauri-plugin-dialog`), so the
 * frontend only provides the data.  If the user cancels the dialog the backend
 * returns an error containing the string `"Export cancelled"`, which the
 * caller should detect and silently ignore.
 *
 * @param report - The report data previously returned by `generateReport`.
 * @returns The absolute file path where the CSV was written.
 * @throws `"Export cancelled"` if the user dismissed the dialog, or any other
 *   string on write failure.
 */
export async function exportReportCsv(report: ReportResult): Promise<string> {
  return invoke<string>("export_report_csv", { report });
}

/**
 * Queries completed time entries matching the given scope and date range, then
 * aggregates them into a `ReportResult`.
 *
 * Scope priority: `taskId` → `planId` → all plans.  The date range is
 * inclusive: entries whose `start_time` falls within
 * `[fromYear-fromMonth-01, toYear-toMonth-last-day]` are included.
 *
 * @param params.planId - Optional: restrict report to this plan UUID.
 * @param params.taskId - Optional: restrict report to this task UUID
 *   (takes precedence over `planId`).
 * @param params.fromYear - Start year of the date range (e.g. `2024`).
 * @param params.fromMonth - Start month of the date range, 1-indexed (1–12).
 * @param params.toYear - End year of the date range (inclusive).
 * @param params.toMonth - End month of the date range, 1-indexed (inclusive).
 * @returns Aggregated report with all matching entries and a grand total.
 * @throws If the database query fails.
 */
export async function generateReport(params: {
  planId?: string;
  taskId?: string;
  fromYear: number;
  fromMonth: number;
  toYear: number;
  toMonth: number;
}): Promise<ReportResult> {
  return invoke<ReportResult>("generate_report", {
    planId: params.planId ?? null,
    taskId: params.taskId ?? null,
    fromYear: params.fromYear,
    fromMonth: params.fromMonth,
    toYear: params.toYear,
    toMonth: params.toMonth,
  });
}

// ---------------------------------------------------------------------------
// Settings
// ---------------------------------------------------------------------------

/**
 * Returns the absolute path of the directory where PlanTracker stores its
 * database and configuration files.
 *
 * On Linux this resolves to `~/.local/share/PlanTracker/`; on Windows to
 * `%USERPROFILE%\Documents\PlanTracker\`.  Used by the Settings view to
 * display the path and offer an "Open Folder" shortcut.
 *
 * @returns Absolute path string for the application data directory.
 * @throws If the platform data directory cannot be resolved.
 */
export async function getDataDir(): Promise<string> {
  return invoke<string>("get_data_dir");
}
