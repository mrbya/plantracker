pub mod auth;
pub mod commands;
pub mod db;
pub mod graph;
pub mod models;

use std::sync::Arc;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load .env file if present (dev convenience). Uses override so values from
    // .env take precedence over any empty vars already present in the environment.
    match dotenvy::dotenv_override() {
        Ok(path) => eprintln!("[plantracker] loaded .env from: {}", path.display()),
        Err(e) => eprintln!("[plantracker] .env not loaded: {e} (cwd: {:?})", std::env::current_dir()),
    }

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
            eprintln!("[plantracker] client_id loaded: {}", if client_id.is_empty() { "EMPTY" } else { "OK" });
            eprintln!("[plantracker] tenant_id loaded: {}", if tenant_id.is_empty() { "EMPTY" } else { "OK" });
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
            commands::sync::sync_plans_and_tasks,
            commands::sync::list_plans,
            commands::sync::list_tasks_for_plan,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
