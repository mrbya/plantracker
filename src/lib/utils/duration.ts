/**
 * Formats a duration in whole seconds as a compact `Xh Ym` string.
 *
 * The hours component is omitted when the duration is less than one hour,
 * giving cleaner output for short sessions:
 *
 * ```
 * formatDuration(9000)  // → "2h 30m"
 * formatDuration(300)   // → "5m"
 * formatDuration(3600)  // → "1h 0m"
 * ```
 *
 * Used in UI entry tables and the active timer button label.
 *
 * @param seconds - Total duration in whole seconds (non-negative integer).
 * @returns Human-readable duration string in `Xh Ym` or `Ym` format.
 */
export function formatDuration(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  return h > 0 ? `${h}h ${m}m` : `${m}m`;
}

/**
 * Formats a duration in whole seconds as a zero-padded `H:MM:SS` string
 * suitable for CSV export.
 *
 * The hours component is not zero-padded so that totals beyond 9 hours
 * render cleanly in spreadsheet applications:
 *
 * ```
 * formatDurationCSV(9000)  // → "2:30:00"
 * formatDurationCSV(300)   // → "0:05:00"
 * formatDurationCSV(36610) // → "10:10:10"
 * ```
 *
 * Used by the `export_report_csv` Tauri command (via the `ReportResult`
 * payload) to populate the Duration column in exported CSV files.
 *
 * @param seconds - Total duration in whole seconds (non-negative integer).
 * @returns Zero-padded duration string in `H:MM:SS` format.
 */
export function formatDurationCSV(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  return `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}
