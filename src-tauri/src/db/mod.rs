use sqlx::SqlitePool;

/// Time-entry CRUD operations.
pub mod entries;
/// Plan CRUD operations.
pub mod plans;
/// Task CRUD operations.
pub mod tasks;

#[cfg(test)]
/// In-memory pool helper for integration tests.
pub mod test_helpers;

/// Resolves the platform-specific path to the `SQLite` database file,
/// creating the parent directory if it does not already exist.
///
/// The resolved path depends on the target operating system:
///
/// | Platform | Base directory | Full path |
/// |---|---|---|
/// | Windows | `%USERPROFILE%\Documents\` | `%USERPROFILE%\Documents\plantracker.db` |
/// | Linux / macOS | `~/.local/share/PlanTracker/` | `~/.local/share/PlanTracker/plantracker.db` |
///
/// `create_dir_all` is called on the base directory to ensure it exists before
/// the database file is opened. The `SQLite` connection string uses `?mode=rwc`
/// which creates the file if it does not exist.
///
/// # Arguments
///
/// - `app`: Tauri application handle, used to resolve the platform data directory
///   via [`tauri::path::PathResolver`].
///
/// # Returns
///
/// `Ok(path)` — the absolute path to `plantracker.db` on this system.
///
/// # Errors
///
/// Returns an error if:
/// - the platform data directory cannot be resolved (e.g. `tauri::path` returns an error), or
/// - [`std::fs::create_dir_all`] fails (e.g. insufficient permissions).
fn resolve_db_path(app: &tauri::AppHandle) -> anyhow::Result<std::path::PathBuf> {
    use tauri::Manager;

    #[cfg(target_os = "windows")]
    let base = app.path().document_dir()?;

    #[cfg(not(target_os = "windows"))]
    let base = app.path().app_local_data_dir()?;

    std::fs::create_dir_all(&base)?;
    Ok(base.join("plantracker.db"))
}

/// Initialises the `SQLite` connection pool, enables foreign-key enforcement,
/// and runs all pending schema migrations.
///
/// This function is called once during Tauri's `setup` closure in [`crate::run`].
/// The returned pool is immediately registered as Tauri managed state and reused
/// for the lifetime of the application.
///
/// The connection string uses `?mode=rwc`, which causes `SQLite` to create the
/// database file if it does not yet exist. After the pool is created,
/// `PRAGMA foreign_keys = ON` is executed to enable referential integrity
/// enforcement (disabled by default in `SQLite`). Finally, `sqlx::migrate!()` runs
/// all migration files from `src-tauri/migrations/` that have not yet been applied,
/// in numerical order.
///
/// # Arguments
///
/// - `app_handle`: Tauri application handle forwarded to `resolve_db_path`.
///
/// # Returns
///
/// `Ok(pool)` — a ready-to-use [`SqlitePool`] with foreign keys enabled and all
/// migrations applied.
///
/// # Errors
///
/// Returns an error if:
/// - the database path cannot be resolved (see `resolve_db_path`),
/// - `SQLite` cannot open or create the file at the resolved path,
/// - the `PRAGMA foreign_keys = ON` statement fails, or
/// - any migration file fails to apply.
pub async fn init_db(app_handle: &tauri::AppHandle) -> anyhow::Result<SqlitePool> {
    let db_path = resolve_db_path(app_handle)?;
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display())).await?;
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
