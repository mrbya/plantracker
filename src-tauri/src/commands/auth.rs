use std::sync::Arc;

use tauri::State;

use crate::auth::{manager::AuthManager, oauth::start_login};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    pub is_authenticated: bool,
    pub user_display_name: Option<String>,
}

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

#[tauri::command]
pub async fn logout(auth: State<'_, Arc<AuthManager>>) -> Result<(), String> {
    auth.clear().await.map_err(|e| e.to_string())?;
    tracing::info!("User logged out");
    Ok(())
}

#[tauri::command]
pub async fn get_auth_status(auth: State<'_, Arc<AuthManager>>) -> Result<AuthStatus, String> {
    Ok(AuthStatus {
        is_authenticated: auth.is_authenticated().await,
        user_display_name: auth.user_display_name().await,
    })
}
