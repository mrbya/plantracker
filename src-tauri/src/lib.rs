//! `PlanTracker` — Tauri application backend.
//!
//! Initialises the `SQLite` database, OAuth auth manager, and active-timer state,
//! then wires all Tauri commands and runs the event loop.

#![allow(clippy::module_name_repetitions)]
// clippy WARN level lints
#![warn(
    missing_docs,
    //clippy::cargo,
    clippy::pedantic,
    clippy::nursery,
    clippy::dbg_macro,
    clippy::unwrap_used,
    clippy::integer_division,
    clippy::large_include_file,
    clippy::map_err_ignore,
    clippy::missing_docs_in_private_items,
    clippy::panic,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
)]
// clippy WARN level lints, that can be upgraded to DENY if preferred
#![warn(
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
    clippy::modulo_arithmetic,
    clippy::as_conversions,
    clippy::assertions_on_result_states,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::default_union_representation,
    clippy::deref_by_slicing,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::lossy_float_literal,
    clippy::pattern_type_mismatch,
    clippy::string_slice,
    clippy::try_err
)]
// clippy DENY level lints, they always have a quick fix that should be preferred
#![deny(
    clippy::wildcard_imports,
    clippy::multiple_inherent_impl,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::separated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::implicit_clone,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::verbose_file_reads
)]

/// App auth management.
pub mod auth;
/// App tauri command definitions.
pub mod commands;
/// App DB client.
pub mod db;
/// MS Graph client.
pub mod graph;
/// App data models.
pub mod models;

use std::sync::Arc;

use tauri::Manager;
use tokio::sync::Mutex;

use commands::timer::ActiveTimer;

/// Builds and runs the Tauri application.
///
/// # Panics
/// Panics if the database cannot be initialised or the Tauri runtime encounters
/// an unrecoverable error during startup.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load .env file if present (dev convenience). Uses override so values from
    // .env take precedence over any empty vars already present in the environment.
    match dotenvy::dotenv_override() {
        Ok(path) => eprintln!("[plantracker] loaded .env from: {}", path.display()),
        Err(e) => eprintln!(
            "[plantracker] .env not loaded: {e} (cwd: {:?})",
            std::env::current_dir()
        ),
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
            eprintln!(
                "[plantracker] client_id loaded: {}",
                if client_id.is_empty() { "EMPTY" } else { "OK" }
            );
            eprintln!(
                "[plantracker] tenant_id loaded: {}",
                if tenant_id.is_empty() { "EMPTY" } else { "OK" }
            );
            let auth_manager = auth::manager::AuthManager::new(client_id, tenant_id);
            app.manage(Arc::clone(&auth_manager));

            // Active timer state — restore from DB if a timer was running before shutdown.
            let timer_state: Mutex<Option<ActiveTimer>> = tauri::async_runtime::block_on(async {
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
                                    plan_id: entry.plan_id,
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
            commands::settings::get_data_dir,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
