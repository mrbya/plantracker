---
id: TASK-18
title: Phase 3.4 — AuthManager (token lifecycle)
status: Done
assignee: []
created_date: '2026-03-19 11:04'
updated_date: '2026-03-19 11:17'
labels:
  - backend
  - auth
  - phase-3
dependencies:
  - TASK-17
priority: high
ordinal: 4000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/auth/manager.rs` with the `AuthManager` struct that manages the in-memory token state and handles silent refresh. Register it as Tauri managed state.

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{Utc, Duration};
use crate::auth::{keychain, oauth::{TokenSet, refresh_access_token}};

pub struct AuthManager {
    tokens: Mutex<Option<TokenSet>>,
    client_id: String,
    tenant_id: String,
}

impl AuthManager {
    pub fn new(client_id: String, tenant_id: String) -> Arc<Self> {
        let tokens = keychain::load_tokens().unwrap_or(None);
        Arc::new(Self {
            tokens: Mutex::new(tokens),
            client_id,
            tenant_id,
        })
    }

    /// Returns a valid access token, refreshing silently if expired or near expiry.
    pub async fn get_valid_token(&self) -> anyhow::Result<String> {
        let mut guard = self.tokens.lock().await;
        if let Some(ref tokens) = *guard {
            if tokens.expires_at > Utc::now() + Duration::seconds(60) {
                return Ok(tokens.access_token.clone());
            }
            let refreshed = refresh_access_token(&tokens.refresh_token, &self.client_id, &self.tenant_id).await?;
            keychain::save_tokens(&refreshed)?;
            *guard = Some(refreshed.clone());
            return Ok(refreshed.access_token);
        }
        anyhow::bail!("Not authenticated")
    }

    pub async fn set_tokens(&self, tokens: TokenSet) -> anyhow::Result<()> {
        keychain::save_tokens(&tokens)?;
        *self.tokens.lock().await = Some(tokens);
        Ok(())
    }

    pub async fn clear(&self) -> anyhow::Result<()> {
        keychain::clear_tokens()?;
        *self.tokens.lock().await = None;
        Ok(())
    }

    pub async fn is_authenticated(&self) -> bool {
        self.tokens.lock().await.is_some()
    }

    pub async fn user_display_name(&self) -> Option<String> {
        // Populated after login via set_display_name(); stored in memory only.
        // (Display name is not persisted — fetched fresh from /me on each login.)
        None  // placeholder; extend in TASK-19 after Graph /me call
    }
}
```

**Important:** Never hold the `Mutex` lock across an `.await` point (see `pitfalls.md`). Always clone/extract before awaiting.

Construct `AuthManager` in `lib.rs` setup, reading `client_id` and `tenant_id` from compile-time env vars, and register with `app.manage(Arc::clone(&auth_manager))`.

Expose from `auth/mod.rs`: `pub mod manager;`
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 AuthManager::new() loads any existing tokens from the keychain on startup
- [x] #2 get_valid_token() returns a valid access token without re-login if token is fresh
- [x] #3 get_valid_token() silently refreshes and persists the new token when near expiry (< 60s)
- [x] #4 get_valid_token() returns an error (not panic) when not authenticated
- [x] #5 Mutex lock is never held across an .await point
- [x] #6 AuthManager is registered as Arc<AuthManager> in Tauri managed state in lib.rs
<!-- AC:END -->
