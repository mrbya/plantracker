use tauri::Manager;

/// Returns the platform-specific data directory path used by this application.
///
/// # Errors
/// Returns a string error if the platform data directory cannot be resolved.
#[tauri::command]
pub async fn get_data_dir(app: tauri::AppHandle) -> Result<String, String> {
    #[cfg(target_os = "windows")]
    let base = app.path().document_dir().map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    let base = app.path().app_local_data_dir().map_err(|e| e.to_string())?;

    Ok(base.display().to_string())
}
