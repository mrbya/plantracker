use std::fmt;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tauri_plugin_opener::OpenerExt;

use crate::auth::pkce;

/// OAuth redirect URI — must match the Azure AD app registration exactly.
///
/// `tauri-plugin-oauth` always starts its callback server on port 52721. This constant
/// must match the redirect URI registered in the Azure AD portal under
/// *Authentication → Redirect URIs (Public client / mobile & desktop)*.
/// Any mismatch — including `http` vs `https`, a different port, or a missing `/callback`
/// suffix — will cause Microsoft to return an error to the browser rather than redirecting
/// to the local server.
const REDIRECT_URI: &str = "http://localhost:52721/callback";

// ---------------------------------------------------------------------------
// TokenSet
// ---------------------------------------------------------------------------

/// A set of OAuth 2.0 tokens returned by the Microsoft identity platform.
///
/// `TokenSet` is the canonical in-memory representation of the user's authentication
/// credentials. It is:
/// - stored in [`super::manager::AuthManager`] behind a `tokio::sync::Mutex`,
/// - persisted to the OS keychain by [`super::keychain::save_tokens`] after every
///   successful login or token refresh,
/// - never written to any file, environment variable, or `SQLite` database.
///
/// The [`fmt::Debug`] implementation deliberately redacts `access_token` and
/// `refresh_token` so that tokens cannot accidentally leak into log files even at
/// `TRACE` or `DEBUG` log levels.
#[derive(Serialize, Deserialize, Clone)]
pub struct TokenSet {
    /// Short-lived Bearer token for authenticating Graph API requests.
    ///
    /// Typically valid for one hour. `AuthManager::get_valid_token` refreshes it
    /// automatically when fewer than 60 seconds remain.
    pub access_token: String,
    /// Long-lived token used to obtain new access tokens without re-authentication.
    ///
    /// Must be kept secret. Never logged, even at debug level.
    pub refresh_token: String,
    /// UTC timestamp at which the access token expires.
    ///
    /// Computed at token issuance time as `Utc::now() + expires_in seconds`.
    pub expires_at: DateTime<Utc>,
}

/// Redacts sensitive token fields from debug output.
///
/// This prevents `access_token` and `refresh_token` from appearing in log files,
/// panic messages, or any other output that uses the `Debug` format, regardless of
/// the active log level.
impl fmt::Debug for TokenSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenSet")
            .field("expires_at", &self.expires_at)
            .field("access_token", &"[REDACTED]")
            .field("refresh_token", &"[REDACTED]")
            .finish()
    }
}

// ---------------------------------------------------------------------------
// Microsoft token endpoint response shapes
// ---------------------------------------------------------------------------

/// Raw JSON shape of a successful token endpoint response.
///
/// Only the fields consumed by this application are modelled. Additional fields
/// returned by Microsoft (e.g. `token_type`, `scope`) are ignored.
#[derive(Deserialize)]
struct TokenResponse {
    /// Opaque Bearer token string.
    access_token: String,
    /// Opaque refresh token string.
    refresh_token: String,
    /// Token lifetime in seconds from the time of issuance.
    expires_in: i64,
}

/// Raw JSON shape of a token endpoint error response.
#[derive(Deserialize)]
struct ErrorResponse {
    /// Machine-readable error code (e.g. `"invalid_grant"`).
    error: String,
    /// Human-readable description of the error, if provided.
    error_description: Option<String>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Starts the full OAuth 2.0 Authorization Code + PKCE login flow.
///
/// This is the top-level entry point for user authentication. The function
/// orchestrates the following six steps:
///
/// 1. **PKCE generation** — [`pkce::generate_code_verifier`], [`pkce::generate_code_challenge`],
///    and [`pkce::generate_state`] produce a fresh verifier, challenge, and CSRF state token
///    for this login attempt.
/// 2. **Local callback server** — `tauri-plugin-oauth` starts an HTTP server on port 52721.
///    A one-shot channel passes the raw callback URL from the server thread back to this task.
/// 3. **Browser open** — the Microsoft authorization URL (built by [`build_auth_url`]) is
///    opened in the user's default system browser via `tauri-plugin-opener`.
/// 4. **Callback receive** — the function awaits the one-shot channel. The channel fires
///    when Microsoft redirects the user to `http://localhost:52721/callback?code=...&state=...`
///    after successful authentication.
/// 5. **CSRF validation** — the `state` returned in the callback is compared against the
///    locally generated state. If they differ the function bails immediately with an error.
/// 6. **Token exchange** — [`exchange_code_for_tokens`] posts the authorization code and
///    PKCE verifier to the Microsoft token endpoint and returns a [`TokenSet`].
///
/// # Arguments
///
/// - `app_handle`: Tauri application handle, used to open the browser via `tauri-plugin-opener`.
/// - `client_id`: Azure AD application (client) ID from the app registration.
/// - `tenant_id`: Azure AD tenant ID, or `"common"` for multi-tenant apps.
///
/// # Returns
///
/// `Ok(TokenSet)` on success, containing both the access and refresh tokens.
///
/// # Errors
///
/// Returns an error if:
/// - the local OAuth callback server cannot be started on port 52721,
/// - the system browser cannot be opened,
/// - the callback URL contains an OAuth error field,
/// - the returned `state` parameter does not match the locally generated one,
/// - the HTTP request to the token endpoint fails, or
/// - the token endpoint returns a non-success response.
pub async fn start_login(
    app_handle: &tauri::AppHandle,
    client_id: &str,
    tenant_id: &str,
) -> anyhow::Result<TokenSet> {
    let verifier = pkce::generate_code_verifier();
    let challenge = pkce::generate_code_challenge(&verifier);
    let csrf_state = pkce::generate_state();

    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let tx = std::sync::Mutex::new(Some(tx));

    tauri_plugin_oauth::start_with_config(
        tauri_plugin_oauth::OauthConfig {
            ports: Some(vec![52721]),
            response: None,
        },
        move |callback_url| {
            if let Ok(mut guard) = tx.lock() {
                if let Some(sender) = guard.take() {
                    drop(sender.send(callback_url));
                }
            }
        },
    )?;

    let auth_url = build_auth_url(client_id, tenant_id, &challenge, &csrf_state);
    tracing::info!("Opening Microsoft login in system browser");
    app_handle.opener().open_url(auth_url, None::<&str>)?;

    let callback_url = rx.await?;
    tracing::info!("OAuth callback received");

    let (code, returned_state) = parse_callback(&callback_url)?;

    if returned_state != csrf_state {
        anyhow::bail!("OAuth state mismatch — possible CSRF attack");
    }

    let tokens = exchange_code_for_tokens(&code, &verifier, client_id, tenant_id).await?;
    tracing::info!(expires_at = %tokens.expires_at, "Login successful, tokens acquired");
    Ok(tokens)
}

/// Exchanges an OAuth 2.0 authorization code for a [`TokenSet`].
///
/// Posts an `authorization_code` grant to the Microsoft token endpoint at
/// `https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token`.
/// The PKCE verifier is included in the POST body so Microsoft can confirm that
/// the party exchanging the code is the same party that initiated the login.
///
/// The redirect URI in the POST body must exactly match the one used in the
/// authorization request and in the Azure AD app registration; see [`REDIRECT_URI`].
///
/// # Arguments
///
/// - `code`: The authorization code from the OAuth callback URL.
/// - `verifier`: The PKCE code verifier generated at the start of the login flow.
/// - `client_id`: Azure AD application (client) ID.
/// - `tenant_id`: Azure AD tenant ID or `"common"`.
///
/// # Returns
///
/// `Ok(TokenSet)` containing the access token, refresh token, and computed expiry.
///
/// # Errors
///
/// Returns an error if:
/// - the HTTP POST request fails (network error, DNS failure, etc.), or
/// - the token endpoint returns a non-success HTTP status (see [`parse_token_response`]).
pub async fn exchange_code_for_tokens(
    code: &str,
    verifier: &str,
    client_id: &str,
    tenant_id: &str,
) -> anyhow::Result<TokenSet> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!(
            "https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token"
        ))
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("code_verifier", verifier),
            ("redirect_uri", REDIRECT_URI),
            ("client_id", client_id),
        ])
        .send()
        .await?;

    parse_token_response(resp).await
}

/// Silently refreshes an expired or near-expiry access token using a refresh token.
///
/// Posts a `refresh_token` grant to the Microsoft token endpoint. On success the
/// old refresh token is superseded by the newly issued one; callers must persist the
/// returned [`TokenSet`] (the caller, [`super::manager::AuthManager::get_valid_token`],
/// forwards it to [`super::keychain::save_tokens`]).
///
/// This function is called automatically by [`super::manager::AuthManager::get_valid_token`]
/// when fewer than 60 seconds remain before expiry. It should never need to be called
/// directly by command handlers.
///
/// # Arguments
///
/// - `refresh_token`: The current refresh token from the stored [`TokenSet`].
/// - `client_id`: Azure AD application (client) ID.
/// - `tenant_id`: Azure AD tenant ID or `"common"`.
///
/// # Returns
///
/// `Ok(TokenSet)` containing the new access token, new refresh token, and updated expiry.
///
/// # Errors
///
/// Returns an error if:
/// - the HTTP POST request fails, or
/// - the token endpoint returns a non-success HTTP status (e.g. `invalid_grant` if the
///   refresh token has been revoked or expired).
pub async fn refresh_access_token(
    refresh_token: &str,
    client_id: &str,
    tenant_id: &str,
) -> anyhow::Result<TokenSet> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!(
            "https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token"
        ))
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", client_id),
        ])
        .send()
        .await?;

    let tokens = parse_token_response(resp).await?;
    tracing::info!(expires_at = %tokens.expires_at, "Token refreshed silently");
    Ok(tokens)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Builds the Microsoft authorization URL with all required PKCE and CSRF parameters.
///
/// The URL targets the v2.0 endpoint, which supports both personal Microsoft accounts
/// and organisational (Azure AD) accounts. The requested scopes are `Tasks.Read`,
/// `offline_access` (required to receive a refresh token), and `User.Read`.
///
/// # Arguments
///
/// - `client_id`: Azure AD application (client) ID.
/// - `tenant_id`: Azure AD tenant ID or `"common"`.
/// - `challenge`: The PKCE code challenge (base64url-encoded SHA-256 of the verifier).
/// - `state`: The CSRF state token to embed in the URL and validate on callback.
///
/// # Returns
///
/// A fully-formed URL string ready to be opened in the system browser.
fn build_auth_url(client_id: &str, tenant_id: &str, challenge: &str, state: &str) -> String {
    format!(
        "https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/authorize\
         ?client_id={client_id}\
         &response_type=code\
         &redirect_uri={REDIRECT_URI}\
         &scope=Tasks.Read%20offline_access%20User.Read\
         &code_challenge={challenge}\
         &code_challenge_method=S256\
         &state={state}"
    )
}

/// Extracts the `code` and `state` query parameters from an OAuth callback URL.
///
/// The callback URL is produced by `tauri-plugin-oauth`'s local HTTP server when
/// Microsoft redirects the browser after authentication. The URL has the form:
/// `http://localhost:52721/callback?code=...&state=...`
///
/// URL-encoded characters in parameter values are decoded before being returned.
///
/// # Arguments
///
/// - `url`: The full callback URL string received from the local HTTP server.
///
/// # Returns
///
/// `Ok((code, state))` on success.
///
/// # Errors
///
/// Returns an error if:
/// - the URL contains an `error` query parameter (Microsoft reported an auth failure),
/// - the `code` parameter is absent, or
/// - the `state` parameter is absent.
fn parse_callback(url: &str) -> anyhow::Result<(String, String)> {
    // The callback URL may use the http: scheme with localhost — parse it manually
    // by splitting on '?' to extract query parameters.
    let query = url.split_once('?').map_or("", |(_, q)| q);

    let mut code = None;
    let mut state = None;
    let mut error_val = None;
    let mut error_desc = None;

    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            let v = urlencoding::decode(v).unwrap_or(std::borrow::Cow::Borrowed(v));
            match k {
                "code" => code = Some(v.into_owned()),
                "state" => state = Some(v.into_owned()),
                "error" => error_val = Some(v.into_owned()),
                "error_description" => error_desc = Some(v.into_owned()),
                _ => {}
            }
        }
    }

    if let Some(err) = error_val {
        anyhow::bail!(
            "Authorization error: {} — {}",
            err,
            error_desc.unwrap_or_default()
        );
    }

    let code = code.ok_or_else(|| anyhow::anyhow!("No authorization code in callback URL"))?;
    let state = state.ok_or_else(|| anyhow::anyhow!("No state in callback URL"))?;
    Ok((code, state))
}

/// Parses a Microsoft token endpoint HTTP response into a [`TokenSet`].
///
/// On a 2xx response the body is deserialised as [`TokenResponse`] and converted to a
/// [`TokenSet`] with `expires_at` computed by adding `expires_in` seconds to the current
/// UTC time.
///
/// On a non-2xx response the body is deserialised as [`ErrorResponse`] and the error
/// information is surfaced in a human-readable message. If the error body cannot be
/// deserialised (e.g. the server returned HTML), a fallback error with code `"unknown"`
/// is used instead.
///
/// # Arguments
///
/// - `resp`: The raw `reqwest` response from the token endpoint.
///
/// # Returns
///
/// `Ok(TokenSet)` on success.
///
/// # Errors
///
/// Returns an error if the HTTP status is not 2xx, or if the success body cannot be
/// deserialised as [`TokenResponse`].
async fn parse_token_response(resp: reqwest::Response) -> anyhow::Result<TokenSet> {
    if !resp.status().is_success() {
        let err: ErrorResponse = resp.json().await.unwrap_or_else(|_| ErrorResponse {
            error: "unknown".into(),
            error_description: None,
        });
        anyhow::bail!(
            "Token endpoint error ({}): {}",
            err.error,
            err.error_description.unwrap_or_default()
        );
    }

    let body: TokenResponse = resp.json().await?;
    Ok(TokenSet {
        access_token: body.access_token,
        refresh_token: body.refresh_token,
        expires_at: Utc::now()
            .checked_add_signed(Duration::seconds(body.expires_in))
            .unwrap_or_else(Utc::now),
    })
}
