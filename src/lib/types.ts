/**
 * A Microsoft Planner plan cached in the local SQLite database.
 *
 * Plans are fetched from the Graph API during a sync and stored locally so the
 * UI can display them without waiting for a network request.  The `id` field is
 * a locally generated UUID; `graphId` is the immutable identifier assigned by
 * Microsoft and is used as the upsert conflict key on sync.
 *
 * Field names are `camelCase` to match the Rust struct serialised with
 * `#[serde(rename_all = "camelCase")]`.
 */
export interface Plan {
  /** Local UUID primary key, generated at upsert time. */
  id: string;
  /** Microsoft Graph ID — unique across tenants; used as upsert conflict key. */
  graphId: string;
  /** Display title of the plan as shown in Microsoft Planner. */
  title: string;
  /** ISO 8601 timestamp of the last successful sync for this plan. */
  syncedAt: string;
}

/**
 * A Microsoft Planner task cached in the local SQLite database.
 *
 * Tasks are fetched per-plan from the Graph API during sync.  The `planId`
 * field references the *local* UUID of the parent plan (not the Graph ID), so
 * it can be used directly as a foreign key when creating `TimeEntry` records.
 *
 * If a task is deleted in Microsoft Planner and a subsequent sync removes it
 * from the local database, any time entries referencing the task will have
 * their `taskId` set to `null` (`ON DELETE SET NULL`), preserving the tracked
 * time as a plan-level entry rather than deleting it.
 */
export interface Task {
  /** Local UUID primary key, generated at upsert time. */
  id: string;
  /** Microsoft Graph ID — unique across tenants; used as upsert conflict key. */
  graphId: string;
  /** Local UUID of the parent plan (references `Plan.id`, not `Plan.graphId`). */
  planId: string;
  /** Display title of the task as shown in Microsoft Planner. */
  title: string;
  /** ISO 8601 timestamp of the last successful sync for this task. */
  syncedAt: string;
}

/**
 * A tracked time interval stored in the local SQLite database.
 *
 * Each entry represents one work session: either a completed interval (both
 * `startTime` and `endTime` are set) or an in-progress timer (`endTime` is
 * `null`).  The backend guarantees at most one active entry at any time.
 *
 * `taskId` is optional — a `null` value means the time is attributed to the
 * plan as a whole rather than a specific task.  This can happen either because
 * the user intentionally started a plan-level timer, or because the task was
 * deleted after the entry was created and the foreign key was nulled.
 */
export interface TimeEntry {
  /** Local UUID primary key. */
  id: string;
  /** Local UUID of the parent plan. */
  planId: string;
  /** Local UUID of the associated task, or `null` for plan-level entries. */
  taskId: string | null;
  /** ISO 8601 start timestamp (always present). */
  startTime: string;
  /** ISO 8601 end timestamp, or `null` if the timer is still running. */
  endTime: string | null;
  /** Optional free-text notes attached to this entry. */
  notes: string | null;
  /** ISO 8601 creation timestamp, set at insert time and never updated. */
  createdAt: string;
}

/**
 * Authentication state returned by the `get_auth_status` and `login` commands.
 *
 * Used by the `auth` store to decide whether to render the main app layout or
 * the `Login` view, and to display the signed-in user's name in the Settings
 * view.
 */
export interface AuthStatus {
  /** Whether the user has a valid (or refreshable) token in the OS keychain. */
  isAuthenticated: boolean;
  /**
   * The Microsoft Graph display name of the signed-in user, or `null` if not
   * yet fetched or not authenticated.  Populated after the first successful
   * sync that calls `GET /me`.
   */
  userDisplayName: string | null;
}

/**
 * Summary statistics returned by the `sync_plans_and_tasks` command.
 *
 * Displayed as a toast notification after a successful sync so the user can
 * see how many items were fetched from Microsoft Graph.
 */
export interface SyncResult {
  /** Number of plans upserted into the local database during this sync. */
  plansCount: number;
  /** Total number of tasks upserted across all plans during this sync. */
  tasksCount: number;
  /** ISO 8601 timestamp of when this sync completed. */
  syncedAt: string;
}

/**
 * A snapshot of the currently running timer, as returned by `get_active_timer`.
 *
 * `elapsedSeconds` is computed by the backend at the moment of the call
 * (`now - startTime`) and is therefore always accurate even if the polling
 * interval drifts.  The `timer` store updates this every second via a
 * `setInterval` loop while the timer is running.
 */
export interface ActiveTimerInfo {
  /** The `TimeEntry.id` of the active (open-ended) database row. */
  entryId: string;
  /** Local UUID of the plan being tracked. */
  planId: string;
  /** Local UUID of the task being tracked, or `null` for plan-level timers. */
  taskId: string | null;
  /** ISO 8601 start timestamp of the active timer. */
  startTime: string;
  /** Whole seconds elapsed since `startTime`, computed at call time by the backend. */
  elapsedSeconds: number;
}

/**
 * A single row in a generated time-tracking report.
 *
 * `taskTitle` and `planTitle` are resolved from the local database at report
 * generation time and included directly so the frontend does not need to join
 * data itself.  Both titles reflect the state of the local database at the
 * time the report is generated, which may differ from the Microsoft Planner
 * state if a sync has not been run recently.
 */
export interface ReportEntry {
  /** Title of the task, or `"(no task)"` for plan-level entries. */
  taskTitle: string;
  /** Title of the parent plan. */
  planTitle: string;
  /** ISO 8601 start timestamp of this time entry. */
  startTime: string;
  /** ISO 8601 end timestamp of this time entry. */
  endTime: string;
  /** Duration of this entry in whole seconds (`endTime - startTime`). */
  durationSeconds: number;
  /** Optional free-text notes attached to this entry, or `null` if none. */
  notes: string | null;
}

/**
 * The complete result of a `generate_report` command invocation.
 *
 * Passed directly to `export_report_csv` when the user requests a CSV export.
 * `grandTotalSeconds` is the sum of all `ReportEntry.durationSeconds` values
 * and is displayed as the totals row in the Reports view.
 */
export interface ReportResult {
  /** All matching time entries for the requested scope and date range. */
  entries: ReportEntry[];
  /** Sum of all entry durations in whole seconds; used for the totals row. */
  grandTotalSeconds: number;
  /**
   * Human-readable label describing the report scope, e.g. `"All plans"`,
   * `"Plan: Sprint 42"`, or `"Task: Fix login bug"`.  Displayed as the report
   * title in the UI and as the header row in the CSV export.
   */
  subjectLabel: string;
}
