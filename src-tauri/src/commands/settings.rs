use tauri::Manager;

/// Returns the application version string as declared in `Cargo.toml`.
///
/// Tauri reads the version from `Cargo.toml` at build time and exposes it on the
/// [`tauri::AppHandle`]. This command surfaces it to the frontend so the Settings view can
/// display it without any frontend hardcoding — `Cargo.toml` remains the single source of truth.
///
/// # Returns
///
/// `Ok(version)` — the version string in `SemVer` format, e.g. `"0.1.2"`.
///
/// # Errors
///
/// This command is infallible in practice; the `Result` wrapper exists only to satisfy the
/// uniform Tauri command boundary convention used across this codebase.
#[tauri::command]
pub async fn get_app_version(app: tauri::AppHandle) -> Result<String, String> {
    Ok(app.package_info().version.to_string())
}

/// Returns the platform-specific data directory path used by `PlanTracker`.
///
/// This path is displayed in the Settings view so users can locate their `SQLite`
/// database file (`plantracker.db`) and any other application data. The path depends
/// on the target operating system:
///
/// | Platform | Path |
/// |---|---|
/// | Windows | `%USERPROFILE%\Documents\` |
/// | Linux / macOS | `~/.local/share/PlanTracker/` |
///
/// These are the same paths used by [`crate::db::init_db`] to create and open the
/// database file, so the directory returned by this command always contains
/// `plantracker.db` (created on first launch).
///
/// # Arguments
///
/// - `app`: Tauri application handle, used to resolve the platform data directory
///   via [`tauri::path::PathResolver`].
///
/// # Returns
///
/// `Ok(path)` — the absolute path to the data directory as a `String`.
///
/// # Errors
///
/// Returns a string error if the platform data directory cannot be resolved
/// (e.g. Tauri's path resolver returns an error).
#[tauri::command]
pub async fn get_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    let base = app.path().document_dir().map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    let base = app.path().app_local_data_dir().map_err(|e| e.to_string())?;

    Ok(base.display().to_string())
}
