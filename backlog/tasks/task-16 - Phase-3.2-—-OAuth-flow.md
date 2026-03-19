---
id: TASK-16
title: Phase 3.2 — OAuth flow
status: Done
assignee: []
created_date: '2026-03-19 11:04'
updated_date: '2026-03-19 11:14'
labels:
  - backend
  - auth
  - phase-3
dependencies:
  - TASK-15
priority: high
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/auth/oauth.rs` implementing the full Authorization Code + PKCE flow.

**`TokenSet` struct** (define here, used across auth module):
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

impl fmt::Debug for TokenSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenSet")
            .field("expires_at", &self.expires_at)
            .field("access_token", &"[REDACTED]")
            .field("refresh_token", &"[REDACTED]")
            .finish()
    }
}
```

**`start_login(app_handle, client_id, tenant_id) -> anyhow::Result<TokenSet>`**
1. Call `pkce::generate_code_verifier()`, `generate_code_challenge()`, `generate_state()`
2. Start local callback server on port 52721 via `tauri_plugin_oauth::start()`
3. Build the Microsoft authorization URL and open it via `tauri_plugin_shell::open()`
4. Await the redirect callback, extract `code` and `state` query params
5. Validate `state` matches — return error if not (CSRF protection)
6. Call `exchange_code_for_tokens()` with the code + verifier
7. Return `TokenSet`

**`exchange_code_for_tokens(code, verifier, client_id, tenant_id) -> anyhow::Result<TokenSet>`**
- POST to `https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token`
- Body: `grant_type=authorization_code`, `code`, `code_verifier`, `redirect_uri=http://localhost:52721/callback`, `client_id`
- Parse response into `TokenSet`

**`refresh_access_token(refresh_token, client_id, tenant_id) -> anyhow::Result<TokenSet>`**
- POST with `grant_type=refresh_token`

**Critical constraints:**
- Redirect URI must be exactly `http://localhost:52721/callback` — matches Azure AD registration (see `pitfalls.md`)
- Never log token strings — only log expiry timestamps and state transitions
- `client_id` and `tenant_id` read from env vars `VITE_AZURE_CLIENT_ID` / `VITE_AZURE_TENANT_ID` (forwarded at compile time via `build.rs`)

Expose from `auth/mod.rs`: `pub mod oauth;`
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 TokenSet Debug impl redacts access_token and refresh_token fields
- [x] #2 Redirect URI is hardcoded as http://localhost:52721/callback in both auth URL and token exchange
- [x] #3 state parameter is validated on callback — mismatch returns an error
- [x] #4 exchange_code_for_tokens correctly POSTs to the Microsoft token endpoint
- [x] #5 refresh_access_token correctly uses grant_type=refresh_token
- [x] #6 No token strings appear in tracing logs at any level
<!-- AC:END -->
