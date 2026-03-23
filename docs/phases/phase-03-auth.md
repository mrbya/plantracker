# Phase 3 — Authentication

## 3.1 PKCE helper functions

Create `src-tauri/src/auth/pkce.rs`:
- `generate_code_verifier() -> String` — 64 random bytes, base64url-encoded
- `generate_code_challenge(verifier: &str) -> String` — SHA-256 of verifier, base64url-encoded
- `generate_state() -> String` — 16 random bytes, hex-encoded

## 3.2 OAuth flow

Create `src-tauri/src/auth/oauth.rs`:

**`start_login(app_handle, client_id, tenant_id) -> Result<TokenSet>`**
1. Generate `code_verifier`, `code_challenge`, `state`
2. Use `tauri-plugin-oauth` to start a localhost server on port `52721`
3. Build the authorization URL:
   ```
   https://login.microsoftonline.com/{tenant}/oauth2/v2.0/authorize
     ?client_id=...
     &response_type=code
     &redirect_uri=http://localhost:52721/callback
     &scope=Tasks.Read+offline_access+User.Read
     &code_challenge=...
     &code_challenge_method=S256
     &state=...
   ```
4. Open URL in system browser via `tauri-plugin-shell`
5. Await the redirect (callback received by the local server)
6. Validate `state` parameter
7. Call `exchange_code_for_tokens()` with the code + verifier
8. Return `TokenSet { access_token, refresh_token, expires_at }`

**`exchange_code_for_tokens(code, verifier, client_id, tenant_id) -> Result<TokenSet>`**
- POST to `https://login.microsoftonline.com/{tenant}/oauth2/v2.0/token`
- Body: `grant_type=authorization_code`, code, verifier, redirect_uri, client_id

**`refresh_access_token(refresh_token, client_id, tenant_id) -> Result<TokenSet>`**
- POST with `grant_type=refresh_token`

## 3.3 Token storage

Create `src-tauri/src/auth/keychain.rs`:
- `save_tokens(tokens: &TokenSet) -> Result<()>` — store via `keyring` crate, service name `"PlanTracker"`
- `load_tokens() -> Result<Option<TokenSet>>`
- `clear_tokens() -> Result<()>`

## 3.4 Token manager

Create `src-tauri/src/auth/manager.rs` with `AuthManager` struct:
- Holds current `TokenSet` in memory (wrapped in `Mutex`)
- `get_valid_token(&self) -> Result<String>` — checks expiry, refreshes if needed, returns access token
- Exposed as Tauri managed state

## 3.5 Tauri commands for auth

In `src-tauri/src/commands/auth.rs`:
```rust
#[tauri::command]
pub async fn login(app: tauri::AppHandle, auth: State<'_, AuthManager>) -> Result<UserInfo, String>

#[tauri::command]
pub async fn logout(auth: State<'_, AuthManager>) -> Result<(), String>

#[tauri::command]
pub async fn get_auth_status(auth: State<'_, AuthManager>) -> Result<AuthStatus, String>
// AuthStatus: { is_authenticated: bool, user_display_name: Option<String> }
```

## 3.6 Frontend auth store

Create `src/lib/stores/auth.ts`:
```typescript
import { writable, derived } from 'svelte/store';
export const authStatus = writable<AuthStatus | null>(null);
export const isAuthenticated = derived(authStatus, s => s?.is_authenticated ?? false);
export async function login() { /* invoke 'login' command */ }
export async function logout() { /* invoke 'logout' command */ }
```

## 3.7 Login screen

Create `src/views/Login.svelte`:
- Centered layout, app title, short description
- "Sign in with Microsoft" button → calls `login()`
- Loading state while auth completes
- Error display if login fails

Conditionally render `Login.svelte` vs `Layout.svelte` in `App.svelte` based on `isAuthenticated`.

## Verification checklist
- [x] Clicking "Sign in" opens system browser to Microsoft login
- [x] After login, app shows main layout
- [x] Refreshing the app re-uses stored tokens without re-login
- [x] "Sign out" clears tokens and returns to login screen
- [x] Expired tokens auto-refresh transparently
