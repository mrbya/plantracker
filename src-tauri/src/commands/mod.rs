/// Authentication commands.
///
/// Exposes [`auth::login`], [`auth::logout`], and [`auth::get_auth_status`] to the
/// frontend. These commands drive the OAuth PKCE flow, clear credentials on logout,
/// and return the current [`auth::AuthStatus`] (used by the frontend login gate and
/// to display the signed-in user's name).
pub mod auth;

/// Time entry CRUD commands.
///
/// Exposes [`entries::create_manual_entry`], [`entries::update_entry`], and
/// [`entries::delete_entry`]. All three commands go through
/// `entries::parse_and_validate_times` to ensure `end_time` is strictly after
/// `start_time` before writing to `SQLite`.
pub mod entries;

/// Report generation and CSV export commands.
///
/// Exposes [`reports::generate_report`] (produces a [`reports::ReportResult`] for
/// display in the Reports view) and [`reports::export_report_csv`] (writes a CSV file
/// via the system save-file dialog). Cancellation of the dialog is signalled by
/// returning the sentinel error string `"Export cancelled"`.
pub mod reports;

/// Application settings commands.
///
/// Exposes [`settings::get_data_dir`], which returns the platform-specific path to the
/// `PlanTracker` data directory. This path is shown in the Settings view so users can
/// locate their database file.
pub mod settings;

/// Microsoft Graph sync and plan/task list commands.
///
/// Exposes [`sync::sync_plans_and_tasks`] (fetches from Graph and upserts into `SQLite`),
/// [`sync::list_plans`] (reads plans from local `SQLite`), and
/// [`sync::list_tasks_for_plan`] (reads tasks for a plan from local `SQLite`). The list
/// commands are lightweight cache reads used to hydrate frontend stores on startup.
pub mod sync;

/// Timer start/stop and recent-entries commands.
///
/// Exposes [`timer::start_timer`], [`timer::stop_timer`], [`timer::get_active_timer`],
/// and [`timer::get_recent_entries`]. The in-memory [`timer::ActiveTimer`] state is
/// stored in a `tokio::sync::Mutex` registered as Tauri managed state and is restored
/// from `SQLite` on application startup to survive force-quit restarts.
pub mod timer;
