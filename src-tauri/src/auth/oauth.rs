use std::fmt;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tauri_plugin_opener::OpenerExt;

use crate::auth::pkce;

const REDIRECT_URI: &str = "http://localhost:52721/callback";

// ---------------------------------------------------------------------------
// TokenSet
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

/// Redact tokens from debug output so they never appear in logs.
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

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
    error_description: Option<String>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Starts the full Authorization Code + PKCE login flow:
/// 1. Generates PKCE verifier/challenge and CSRF state.
/// 2. Starts a local HTTP server on port 52721.
/// 3. Opens the Microsoft authorization URL in the system browser.
/// 4. Awaits the redirect callback, validates the CSRF state.
/// 5. Exchanges the code for tokens and returns a `TokenSet`.
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
                    let _ = sender.send(callback_url);
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

/// Exchanges an authorization code for a `TokenSet`.
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

/// Silently refreshes an expired access token using a refresh token.
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

fn parse_callback(url: &str) -> anyhow::Result<(String, String)> {
    // The callback URL may use the http: scheme with localhost — parse it manually
    // by splitting on '?' to extract query parameters.
    let query = url.split_once('?').map(|(_, q)| q).unwrap_or("");

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

async fn parse_token_response(resp: reqwest::Response) -> anyhow::Result<TokenSet> {
    if !resp.status().is_success() {
        let err: ErrorResponse = resp.json().await.unwrap_or(ErrorResponse {
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
        expires_at: Utc::now() + Duration::seconds(body.expires_in),
    })
}
