use std::sync::Arc;

use tauri::State;

use crate::auth::{manager::AuthManager, oauth::start_login};

/// Authentication state returned to the frontend after every auth-related command.
///
/// The frontend uses this struct in two ways:
/// - As a **login gate**: if `is_authenticated` is `false`, the Login view is shown
///   instead of the main application.
/// - As a **display** in the Settings view: `user_display_name` is shown next to a
///   "Sign out" button when the user is authenticated.
///
/// Serialised with `camelCase` field names to match the TypeScript `AuthStatus` interface.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    /// Whether the user currently has a valid session (tokens present in memory).
    pub is_authenticated: bool,
    /// Display name of the signed-in Microsoft account, or `None` if not authenticated
    /// or if the name has not been fetched from Graph yet.
    pub user_display_name: Option<String>,
}

/// Initiates the OAuth 2.0 Authorization Code + PKCE login flow.
///
/// Delegates to [`start_login`] which opens the Microsoft authorization page in the
/// system browser, awaits the local callback, and exchanges the code for tokens. On
/// success, the tokens are stored in [`AuthManager`] and persisted to the OS keychain.
///
/// The returned [`AuthStatus`] reflects the state immediately after login; note that
/// `user_display_name` may be `None` at this point because the display name is populated
/// by a subsequent call to [`crate::commands::sync::sync_plans_and_tasks`].
///
/// # Arguments
///
/// - `app`: Tauri application handle, forwarded to [`start_login`] for browser opening.
/// - `auth`: Tauri managed state reference to the shared [`AuthManager`].
///
/// # Returns
///
/// `Ok(AuthStatus)` with `is_authenticated: true` on success.
///
/// # Errors
///
/// Returns a string error if:
/// - the OAuth flow fails (browser cannot open, callback error, CSRF mismatch), or
/// - saving the tokens to the OS keychain fails.
#[tauri::command]
pub async fn login(
    app: tauri::AppHandle,
    auth: State<'_, Arc<AuthManager>>,
) -> Result<AuthStatus, String> {
    let (client_id, tenant_id) = auth.credentials();

    let tokens = start_login(&app, &client_id, &tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    auth.set_tokens(tokens).await.map_err(|e| e.to_string())?;

    tracing::info!("User logged in successfully");
    Ok(AuthStatus {
        is_authenticated: true,
        user_display_name: auth.user_display_name().await,
    })
}

/// Clears all authentication state and removes tokens from the OS keychain.
///
/// After this command completes, [`get_auth_status`] will return
/// `AuthStatus { is_authenticated: false, user_display_name: None }`.
///
/// # Arguments
///
/// - `auth`: Tauri managed state reference to the shared [`AuthManager`].
///
/// # Returns
///
/// `Ok(())` on success.
///
/// # Errors
///
/// Returns a string error if removing the tokens from the OS keychain fails.
#[tauri::command]
pub async fn logout(auth: State<'_, Arc<AuthManager>>) -> Result<(), String> {
    auth.clear().await.map_err(|e| e.to_string())?;
    tracing::info!("User logged out");
    Ok(())
}

/// Returns the current authentication status without performing any network requests.
///
/// Reads the in-memory state of [`AuthManager`] — no keychain access or Graph API
/// call is made. Safe to call frequently from the frontend (e.g. on every route
/// navigation to decide whether to show the login gate).
///
/// # Arguments
///
/// - `auth`: Tauri managed state reference to the shared [`AuthManager`].
///
/// # Returns
///
/// `Ok(AuthStatus)` reflecting the current in-memory authentication state.
///
/// # Errors
///
/// This command is infallible in practice; the `Err` variant is never returned.
#[tauri::command]
pub async fn get_auth_status(auth: State<'_, Arc<AuthManager>>) -> Result<AuthStatus, String> {
    Ok(AuthStatus {
        is_authenticated: auth.is_authenticated().await,
        user_display_name: auth.user_display_name().await,
    })
}
