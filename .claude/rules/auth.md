# Authentication

## Flow Overview

PlanTracker uses **OAuth 2.0 Authorization Code + PKCE** via the user's system browser.
No credentials are stored in the app. No Microsoft login page is embedded in the WebView.

```
User clicks "Sign In"
  → Rust generates code_verifier, code_challenge, state
  → tauri-plugin-oauth starts HTTP server on localhost:52721
  → System browser opens Microsoft login URL
  → User authenticates in browser
  → Microsoft redirects to http://localhost:52721/callback?code=...&state=...
  → Rust validates state, exchanges code for tokens
  → Tokens stored in OS keychain
  → HTTP server shuts down
  → App fetches user info + syncs plans/tasks
```

## Module Layout

```
src-tauri/src/auth/
├── pkce.rs       ← generate_code_verifier(), generate_code_challenge(), generate_state()
├── oauth.rs      ← start_login(), exchange_code_for_tokens(), refresh_access_token()
├── keychain.rs   ← save_tokens(), load_tokens(), clear_tokens()
└── manager.rs    ← AuthManager struct, get_valid_token()
```

## PKCE Requirements

The code verifier must be generated from a cryptographically secure source. Use `rand::RngCore`:

```rust
use rand::RngCore;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

pub fn generate_code_verifier() -> String {
    let mut bytes = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut bytes);  // 64 bytes = 512 bits of entropy
    URL_SAFE_NO_PAD.encode(bytes)               // produces 86-char string (within RFC 7636's 43–128 range)
}

pub fn generate_code_challenge(verifier: &str) -> String {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}
```

**Do not use `rand::random::<u64>()` or any fixed-size integer.** Insufficient entropy will be rejected by some Microsoft tenants.

## Redirect URI

The redirect URI is **`http://localhost:52721/callback`** — hardcoded in both:
1. Azure AD app registration (Public client / mobile & desktop redirect type)
2. Rust source (OAuth request builder and token exchange)

These must match exactly. Any mismatch causes a silent redirect failure.

## State Parameter (CSRF)

Generate a random `state` string for every login attempt. On callback, validate that the returned `state` matches. Reject and abort if it does not.

```rust
pub fn generate_state() -> String {
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}
```

## Token Storage

Tokens are stored in the OS keychain using the `keyring` crate. Service name: `"PlanTracker"`. Never store tokens in:
- `tauri-plugin-store` (plaintext JSON file)
- Environment variables
- Any file in the data directory
- In-memory state that persists after app exit

```rust
use keyring::Entry;

pub fn save_tokens(tokens: &TokenSet) -> anyhow::Result<()> {
    let entry = Entry::new("PlanTracker", "token_set")?;
    entry.set_password(&serde_json::to_string(tokens)?)?;
    Ok(())
}
```

## AuthManager

`AuthManager` is registered as Tauri managed state and is the only place that handles token lifecycle:

```rust
impl AuthManager {
    /// Returns a valid access token, refreshing silently if expired.
    /// Commands call this — they never check expiry themselves.
    pub async fn get_valid_token(&self) -> anyhow::Result<String> {
        let mut guard = self.tokens.lock().await;
        if let Some(ref tokens) = *guard {
            if tokens.expires_at > Utc::now() + Duration::seconds(60) {
                return Ok(tokens.access_token.clone());
            }
            // Token expired or near expiry — refresh
            let refreshed = refresh_access_token(&tokens.refresh_token, ...).await?;
            save_tokens(&refreshed)?;
            *guard = Some(refreshed.clone());
            return Ok(refreshed.access_token);
        }
        anyhow::bail!("Not authenticated")
    }
}
```

## Logging Restriction

**Never log access tokens or refresh tokens at any log level, including `tracing::debug!`.** Log token expiry timestamps and auth state transitions, not the token values.

## Required Azure AD Configuration

- App type: **Public client / mobile & desktop** (not web)
- Redirect URI: `http://localhost:52721/callback`
- Scopes: `Tasks.Read offline_access User.Read`
- Supported account types: *Accounts in any organizational directory and personal Microsoft accounts* (or tenant-specific if restricting to one org)
- No client secret required (public client PKCE flow)

