pub mod auth;
pub mod commands;
pub mod db;
pub mod graph;
pub mod models;

use std::sync::Arc;

use tauri::Manager;
use tokio::sync::Mutex;

use commands::timer::ActiveTimer;

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
            app.manage(pool.clone());

            // Auth manager — loads any cached tokens from the OS keychain on startup
            let client_id = std::env::var("VITE_AZURE_CLIENT_ID").unwrap_or_default();
            let tenant_id = std::env::var("VITE_AZURE_TENANT_ID").unwrap_or_default();
            eprintln!("[plantracker] client_id loaded: {}", if client_id.is_empty() { "EMPTY" } else { "OK" });
            eprintln!("[plantracker] tenant_id loaded: {}", if tenant_id.is_empty() { "EMPTY" } else { "OK" });
            let auth_manager = auth::manager::AuthManager::new(client_id, tenant_id);
            app.manage(Arc::clone(&auth_manager));

            // Active timer state — restore from DB if a timer was running before shutdown.
            let timer_state: Mutex<Option<ActiveTimer>> =
                tauri::async_runtime::block_on(async {
                    let active = db::entries::find_active_entry(&pool).await;
                    match active {
                        Ok(Some(entry)) => {
                            match entry.start_time.parse::<chrono::DateTime<chrono::Utc>>() {
                                Ok(start_time) => {
                                    tracing::info!(
                                        entry_id = %entry.id,
                                        "Restored active timer from DB"
                                    );
                                    Mutex::new(Some(ActiveTimer {
                                        entry_id: entry.id,
                                        task_id: entry.task_id,
                                        start_time,
                                    }))
                                }
                                Err(e) => {
                                    tracing::warn!("Could not parse active entry start_time: {e}");
                                    Mutex::new(None)
                                }
                            }
                        }
                        Ok(None) => Mutex::new(None),
                        Err(e) => {
                            tracing::warn!("Could not query for active timer on startup: {e}");
                            Mutex::new(None)
                        }
                    }
                });
            app.manage(timer_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::login,
            commands::auth::logout,
            commands::auth::get_auth_status,
            commands::sync::sync_plans_and_tasks,
            commands::sync::list_plans,
            commands::sync::list_tasks_for_plan,
            commands::timer::start_timer,
            commands::timer::stop_timer,
            commands::timer::get_active_timer,
            commands::timer::get_recent_entries,
            commands::entries::create_manual_entry,
            commands::entries::update_entry,
            commands::entries::delete_entry,
            commands::reports::generate_report,
            commands::reports::export_report_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
