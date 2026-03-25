/**
 * Notifications store.
 *
 * Provides a lightweight toast notification system used throughout the app.
 * Toasts are rendered by `ToastContainer.svelte` and auto-dismiss after
 * 4 seconds.
 *
 * Usage:
 * ```typescript
 * import { addSuccess, addError, addWarning } from '$lib/stores/notifications';
 *
 * addSuccess('Entry saved');
 * addError('Sync failed: ' + message);
 * addWarning('No plans found — run a sync first.');
 * ```
 *
 * Colour coding (defined in `ToastContainer.svelte`):
 * - `success` → green (`--success`)
 * - `error`   → red (`--danger`)
 * - `warning` → yellow (`--warning`)
 */
import { writable } from "svelte/store";

/** The three severity levels supported by the toast system. */
type NotificationType = "success" | "error" | "warning";

/**
 * A single toast notification managed by the `notifications` store.
 *
 * The `id` is an auto-incrementing integer used to identify the entry in
 * the store array so the `setTimeout` dismiss callback can target the
 * correct item.
 */
interface Notification {
  /** Monotonically increasing identifier, unique within the current session. */
  id: number;
  /** Visual styling and semantic category of the notification. */
  type: NotificationType;
  /** Human-readable message to display in the toast. */
  message: string;
}

/**
 * Reactive list of active toast notifications.
 *
 * Consumed by `ToastContainer.svelte` to render the visible toasts.
 * Each entry is removed automatically 4 seconds after it is added.
 */
export const notifications = writable<Notification[]>([]);

/** Monotonically increasing counter used to generate unique toast IDs. */
let nextId = 0;

/**
 * Internal helper that appends a new notification and schedules its removal.
 *
 * @param type - Severity level of the notification.
 * @param message - Text to display in the toast.
 */
function add(type: NotificationType, message: string): void {
  const id = ++nextId;
  notifications.update((n) => [...n, { id, type, message }]);
  setTimeout(
    () => notifications.update((n) => n.filter((x) => x.id !== id)),
    4000,
  );
}

/**
 * Shows a green success toast.
 *
 * @param msg - The success message to display.
 */
export const addSuccess = (msg: string): void => add("success", msg);

/**
 * Shows a red error toast.
 *
 * @param msg - The error message to display.
 */
export const addError = (msg: string): void => add("error", msg);

/**
 * Shows a yellow warning toast.
 *
 * @param msg - The warning message to display.
 */
export const addWarning = (msg: string): void => add("warning", msg);
