/**
 * Storybook mock for `@tauri-apps/api/core`.
 *
 * Provides sensible default responses for all commands used by the app so
 * components render without errors in Storybook.  Individual stories can
 * override a command's response for the duration of that story by calling
 * `setInvokeHandler` before mounting, and restore defaults with
 * `clearInvokeHandlers`.
 */

type InvokeHandler = (args?: Record<string, unknown>) => unknown;

const overrides = new Map<string, InvokeHandler>();

export function setInvokeHandler(
  command: string,
  handler: InvokeHandler,
): void {
  overrides.set(command, handler);
}

export function clearInvokeHandlers(): void {
  overrides.clear();
}

const DEFAULTS: Record<string, InvokeHandler> = {
  get_auth_status: () => ({ isAuthenticated: false, userDisplayName: null }),
  login: () => ({ isAuthenticated: true, userDisplayName: "Ada Lovelace" }),
  logout: () => null,

  list_plans: () => [],
  list_tasks_for_plan: () => [],
  sync_plans_and_tasks: () => ({ plansCount: 0, tasksCount: 0 }),

  get_active_timer: () => null,
  start_timer: (args) => ({
    id: "mock-entry-id",
    planId: (args as Record<string, string>)?.planId ?? "p1",
    taskId: (args as Record<string, string | null>)?.taskId ?? null,
    startTime: new Date().toISOString(),
    endTime: null,
    notes: null,
    createdAt: new Date().toISOString(),
  }),
  stop_timer: () => ({
    id: "mock-entry-id",
    planId: "p1",
    taskId: null,
    startTime: new Date(Date.now() - 3_600_000).toISOString(),
    endTime: new Date().toISOString(),
    notes: null,
    createdAt: new Date(Date.now() - 3_600_000).toISOString(),
  }),

  get_recent_entries: () => [],
  create_manual_entry: () => ({
    id: "mock-entry-id",
    planId: "p1",
    taskId: null,
    startTime: new Date().toISOString(),
    endTime: new Date().toISOString(),
    notes: null,
    createdAt: new Date().toISOString(),
  }),
  update_entry: () => null,
  delete_entry: () => null,

  generate_report: () => ({
    entries: [],
    grandTotalSeconds: 0,
    subjectLabel: "",
  }),
  export_report_csv: () => "/tmp/report.csv",

  get_data_dir: () => "/home/user/.local/share/PlanTracker",
};

export async function invoke<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  const handler = overrides.get(command) ?? DEFAULTS[command];
  if (handler) {
    return Promise.resolve(handler(args) as T);
  }
  console.warn(`[Storybook] Unhandled invoke: "${command}"`);
  return Promise.resolve(null as T);
}
