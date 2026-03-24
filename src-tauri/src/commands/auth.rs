use std::sync::Arc;

use tauri::State;

use crate::auth::{manager::AuthManager, oauth::start_login};

/// Authentication state returned to the frontend.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    /// Whether the user has a valid session.
    pub is_authenticated: bool,
    /// Display name of the signed-in user, or `None` if not authenticated.
    pub user_display_name: Option<String>,
}

/// Initiates the OAuth login flow and returns the resulting auth status.
///
/// # Errors
/// Returns a string error if the login flow fails or tokens cannot be saved.
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

/// Clears tokens and logs the user out.
///
/// # Errors
/// Returns a string error if clearing the keychain fails.
#[tauri::command]
pub async fn logout(auth: State<'_, Arc<AuthManager>>) -> Result<(), String> {
    auth.clear().await.map_err(|e| e.to_string())?;
    tracing::info!("User logged out");
    Ok(())
}

/// Returns the current authentication status without performing any network requests.
///
/// # Errors
/// This command is infallible in practice; the `Err` variant is never returned.
#[tauri::command]
pub async fn get_auth_status(auth: State<'_, Arc<AuthManager>>) -> Result<AuthStatus, String> {
    Ok(AuthStatus {
        is_authenticated: auth.is_authenticated().await,
        user_display_name: auth.user_display_name().await,
    })
}
