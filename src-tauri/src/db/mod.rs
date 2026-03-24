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
/// # Errors
/// Returns an error if the platform data directory cannot be resolved
/// or if `create_dir_all` fails.
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
/// and runs all pending migrations.
///
/// # Errors
/// Returns an error if the database path cannot be resolved, the pool cannot
/// be created, or a migration fails.
pub async fn init_db(app_handle: &tauri::AppHandle) -> anyhow::Result<SqlitePool> {
    let db_path = resolve_db_path(app_handle)?;
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display())).await?;
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
