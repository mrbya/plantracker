pub mod auth;
pub mod commands;
pub mod db;
pub mod models;

use std::sync::Arc;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load .env file if present (dev convenience). Silently ignored if missing.
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_oauth::init())
        .setup(|app| {
            // Database
            let pool = tauri::async_runtime::block_on(db::init_db(app.handle()))
                .expect("Failed to initialise database");
            app.manage(pool);

            // Auth manager — loads any cached tokens from the OS keychain on startup
            let client_id = std::env::var("VITE_AZURE_CLIENT_ID").unwrap_or_default();
            let tenant_id = std::env::var("VITE_AZURE_TENANT_ID").unwrap_or_default();
            let auth_manager = auth::manager::AuthManager::new(client_id, tenant_id);
            app.manage(Arc::clone(&auth_manager));

            // TODO: restore active timer — query for time_entries WHERE end_time IS NULL
            // and re-populate ActiveTimer managed state (added in Phase 5).

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::login,
            commands::auth::logout,
            commands::auth::get_auth_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
