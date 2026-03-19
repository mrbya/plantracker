use std::sync::Arc;

use chrono::{Duration, Utc};
use tokio::sync::Mutex;

use crate::auth::{
    keychain,
    oauth::{refresh_access_token, TokenSet},
};

pub struct AuthManager {
    tokens: Mutex<Option<TokenSet>>,
    /// Cached display name — populated after login, cleared on logout.
    display_name: Mutex<Option<String>>,
    client_id: String,
    tenant_id: String,
}

impl AuthManager {
    /// Creates a new `AuthManager`, pre-loading any existing tokens from the OS keychain.
    pub fn new(client_id: String, tenant_id: String) -> Arc<Self> {
        let tokens = keychain::load_tokens().unwrap_or(None);
        Arc::new(Self {
            tokens: Mutex::new(tokens),
            display_name: Mutex::new(None),
            client_id,
            tenant_id,
        })
    }

    /// Returns a valid access token, refreshing silently if near expiry (< 60 s remaining).
    ///
    /// # Mutex safety
    /// The lock is always released before any `.await` point to prevent deadlocks.
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

        if expires_at > Utc::now() + Duration::seconds(60) {
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

    /// Stores a new `TokenSet` in memory and persists it to the keychain.
    pub async fn set_tokens(&self, tokens: TokenSet) -> anyhow::Result<()> {
        keychain::save_tokens(&tokens)?;
        *self.tokens.lock().await = Some(tokens);
        Ok(())
    }

    /// Clears in-memory state and removes tokens from the keychain.
    pub async fn clear(&self) -> anyhow::Result<()> {
        keychain::clear_tokens()?;
        *self.tokens.lock().await = None;
        *self.display_name.lock().await = None;
        Ok(())
    }

    pub async fn is_authenticated(&self) -> bool {
        self.tokens.lock().await.is_some()
    }

    pub async fn user_display_name(&self) -> Option<String> {
        self.display_name.lock().await.clone()
    }

    /// Sets the cached display name after fetching it from Microsoft Graph `/me`.
    pub async fn set_display_name(&self, name: String) {
        *self.display_name.lock().await = Some(name);
    }

    /// Returns the Azure AD client_id and tenant_id stored in this manager.
    pub fn credentials(&self) -> (String, String) {
        (self.client_id.clone(), self.tenant_id.clone())
    }
}
