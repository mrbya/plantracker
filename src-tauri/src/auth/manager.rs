use std::sync::Arc;

use chrono::{Duration, Utc};
use tokio::sync::Mutex;

use crate::auth::{
    keychain,
    oauth::{refresh_access_token, TokenSet},
};

/// Central authority for OAuth token lifecycle and authentication state.
///
/// `AuthManager` is the **only** component in `PlanTracker` that reads from or writes
/// to the OS keychain and that has direct access to the raw token strings. All other
/// components (command handlers, [`crate::graph::client::GraphClient`]) obtain a
/// valid access token by calling [`AuthManager::get_valid_token`] and never touch the
/// [`TokenSet`] directly.
///
/// The struct holds two `tokio::sync::Mutex`-guarded fields:
/// - `tokens`: the current [`TokenSet`], or `None` when the user is not authenticated.
/// - `display_name`: a cached copy of the user's display name fetched from Microsoft
///   Graph `/me`, or `None` before the first successful sync.
///
/// `AuthManager` is always created inside an [`Arc`] (see [`AuthManager::new`]) and
/// registered as Tauri managed state so it can be injected into any command handler.
///
/// # Mutex safety
///
/// Both mutexes use `tokio::sync::Mutex`. Locks are **always** acquired, the required
/// value is cloned, and the lock is **released before** any `.await` point to prevent
/// deadlocks. No lock is ever held across an asynchronous operation.
pub struct AuthManager {
    /// The current token set, guarded by an async mutex to prevent concurrent refresh races.
    tokens: Mutex<Option<TokenSet>>,
    /// Cached display name — populated after login, cleared on logout.
    display_name: Mutex<Option<String>>,
    /// Azure AD application (client) ID.
    client_id: String,
    /// Azure AD tenant ID (or `"common"` for multi-tenant).
    tenant_id: String,
}

impl AuthManager {
    /// Creates a new `AuthManager` and attempts to restore a previous session from the keychain.
    ///
    /// Reads `VITE_AZURE_CLIENT_ID` and `VITE_AZURE_TENANT_ID` from the environment and
    /// tries to load a [`TokenSet`] from the OS keychain via [`keychain::load_tokens`].
    /// If keychain access fails (e.g. the service is unavailable) the error is silently
    /// discarded and the manager starts in an unauthenticated state. This is intentional:
    /// a keychain failure at startup should not prevent the application from launching.
    ///
    /// # Arguments
    ///
    /// - `client_id`: Azure AD application (client) ID string.
    /// - `tenant_id`: Azure AD tenant ID string, or `"common"` for multi-tenant apps.
    ///
    /// # Returns
    ///
    /// An [`Arc<AuthManager>`]. The `Arc` wrapper is required because the manager is
    /// shared between Tauri's managed state (which holds a clone of the `Arc`) and the
    /// `setup` closure (which also holds one).
    #[must_use]
    pub fn new(client_id: String, tenant_id: String) -> Arc<Self> {
        let tokens = keychain::load_tokens().unwrap_or(None);
        Arc::new(Self {
            tokens: Mutex::new(tokens),
            display_name: Mutex::new(None),
            client_id,
            tenant_id,
        })
    }

    /// Returns a valid access token, refreshing silently if the token is near expiry.
    ///
    /// If the stored access token has more than 60 seconds remaining before expiry, it
    /// is returned immediately without any network request. If it has 60 seconds or fewer
    /// remaining (or has already expired), [`refresh_access_token`] is called, the new
    /// [`TokenSet`] is persisted to the keychain, and the new access token is returned.
    ///
    /// # Returns
    ///
    /// `Ok(access_token)` — the raw Bearer token string to include in `Authorization` headers.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the user is not authenticated (no [`TokenSet`] is stored), or
    /// - the silent token refresh fails (e.g. the refresh token has been revoked).
    ///
    /// # Mutex safety
    ///
    /// The `tokens` lock is acquired, the required fields are cloned, and the lock is
    /// **released before** the `refresh_access_token` `.await` point. A second short-lived
    /// lock acquisition writes the refreshed token back. No lock is ever held across an
    /// async operation, preventing deadlocks.
    pub async fn get_valid_token(&self) -> anyhow::Result<String> {
        // Extract what we need while holding the lock, then drop it before any await.
        let (access_token, refresh_token, expires_at) = {
            let guard = self.tokens.lock().await;
            match guard.as_ref() {
                None => anyhow::bail!("Not authenticated"),
                Some(t) => (
                    t.access_token.clone(),
                    t.refresh_token.clone(),
                    t.expires_at,
                ),
            }
        }; // lock released here

        if expires_at.signed_duration_since(Utc::now()) > Duration::seconds(60) {
            return Ok(access_token);
        }

        // Token expired or near expiry — refresh without holding the original lock.
        tracing::info!("Access token near expiry, refreshing silently");
        let refreshed =
            refresh_access_token(&refresh_token, &self.client_id, &self.tenant_id).await?;
        keychain::save_tokens(&refreshed)?;

        let access = refreshed.access_token.clone();
        *self.tokens.lock().await = Some(refreshed);
        Ok(access)
    }

    /// Stores a new [`TokenSet`] in memory and persists it to the OS keychain.
    ///
    /// Called after a successful login by [`crate::commands::auth::login`] to save the
    /// freshly issued tokens. Any previously stored tokens are overwritten.
    ///
    /// # Arguments
    ///
    /// - `tokens`: The new [`TokenSet`] to store and persist.
    ///
    /// # Errors
    ///
    /// Returns an error if saving the tokens to the OS keychain fails.
    pub async fn set_tokens(&self, tokens: TokenSet) -> anyhow::Result<()> {
        keychain::save_tokens(&tokens)?;
        *self.tokens.lock().await = Some(tokens);
        Ok(())
    }

    /// Clears all in-memory authentication state and removes tokens from the OS keychain.
    ///
    /// Called by [`crate::commands::auth::logout`]. After this call,
    /// [`is_authenticated`][Self::is_authenticated] returns `false` and
    /// [`get_valid_token`][Self::get_valid_token] returns an error.
    ///
    /// # Errors
    ///
    /// Returns an error if removing the tokens from the OS keychain fails.
    pub async fn clear(&self) -> anyhow::Result<()> {
        keychain::clear_tokens()?;
        *self.tokens.lock().await = None;
        *self.display_name.lock().await = None;
        Ok(())
    }

    /// Returns `true` if a [`TokenSet`] is currently stored in memory.
    ///
    /// This check is purely in-memory — no network request or keychain access is made.
    /// It does not verify whether the stored token is still valid.
    pub async fn is_authenticated(&self) -> bool {
        self.tokens.lock().await.is_some()
    }

    /// Returns the cached display name of the signed-in user, or `None`.
    ///
    /// The display name is populated by [`set_display_name`][Self::set_display_name]
    /// after a successful sync that fetches the user's profile from `/me`. It is
    /// cleared on logout. It is exposed to the frontend via [`AuthStatus`][crate::commands::auth::AuthStatus].
    pub async fn user_display_name(&self) -> Option<String> {
        self.display_name.lock().await.clone()
    }

    /// Caches the display name fetched from Microsoft Graph `/me`.
    ///
    /// Called by [`crate::commands::sync::sync_plans_and_tasks`] after a successful
    /// call to [`crate::graph::planner::fetch_user_info`].
    ///
    /// # Arguments
    ///
    /// - `name`: The `displayName` field from the Graph `/me` response.
    pub async fn set_display_name(&self, name: String) {
        *self.display_name.lock().await = Some(name);
    }

    /// Returns the Azure AD `client_id` and `tenant_id` stored in this manager.
    ///
    /// Used by [`crate::commands::auth::login`] to obtain the credentials needed to
    /// build the authorisation URL and exchange the code for tokens without exposing
    /// the private fields directly.
    ///
    /// # Returns
    ///
    /// A tuple `(client_id, tenant_id)` of owned `String` values.
    pub fn credentials(&self) -> (String, String) {
        (self.client_id.clone(), self.tenant_id.clone())
    }
}
