/**
 * Formats an ISO 8601 timestamp into a compact, locale-aware display string.
 *
 * The output omits the year (assumed to be the current year in most
 * contexts) and uses 24-hour time to match the monospace font aesthetic.
 * The exact format depends on the user's system locale; for `en-US` the
 * output looks like:
 *
 * ```
 * "Mar 15, 14:30"
 * ```
 *
 * Used in entry tables (Time Tracking, Manual Entry, Reports) and wherever
 * a human-readable timestamp is needed in the UI.  For CSV exports use
 * the raw ISO string instead.
 *
 * @param iso - An ISO 8601 date-time string, e.g. `"2024-03-15T14:30:00Z"`.
 * @returns A locale-formatted string with abbreviated month, day, hour, and
 *   minute; no year, no seconds.
 */
export function formatDateTime(iso: string): string {
  return new Date(iso).toLocaleString(undefined, {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  });
}
