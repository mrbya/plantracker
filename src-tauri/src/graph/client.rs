use std::{sync::Arc, time::Duration};

use serde::de::DeserializeOwned;

use crate::auth::manager::AuthManager;

/// Base URL for all Microsoft Graph v1.0 API requests.
const GRAPH_BASE: &str = "https://graph.microsoft.com/v1.0";

/// HTTP client for the Microsoft Graph API, with built-in token management and retry logic.
pub struct GraphClient {
    /// Underlying HTTP client.
    http: reqwest::Client,
    /// Auth manager used to obtain and refresh bearer tokens.
    auth: Arc<AuthManager>,
}

impl GraphClient {
    /// Creates a new `GraphClient` backed by the given `AuthManager`.
    #[must_use]
    pub fn new(auth: Arc<AuthManager>) -> Self {
        Self {
            http: reqwest::Client::new(),
            auth,
        }
    }

    /// GET `{GRAPH_BASE}{path}` with a valid bearer token.
    /// On 401, refreshes the token silently and retries once.
    ///
    /// # Errors
    /// Returns an error if the token cannot be obtained, the request fails, or the response is an error.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = format!("{GRAPH_BASE}{path}");
        self.get_url(&url).await
    }

    /// GET an absolute URL with a valid bearer token.
    /// Handles 401 (silent token refresh + one retry) and 429 (Retry-After backoff + one retry).
    ///
    /// # Errors
    /// Returns an error if the token cannot be obtained, the request fails, or the response is an error.
    pub async fn get_url<T: DeserializeOwned>(&self, url: &str) -> anyhow::Result<T> {
        let token = self.auth.get_valid_token().await?;
        let resp = self.http.get(url).bearer_auth(&token).send().await?;

        match resp.status() {
            s if s == reqwest::StatusCode::UNAUTHORIZED => {
                tracing::info!("Graph returned 401 — refreshing token and retrying");
                let retry_token = self.auth.get_valid_token().await?;
                let retry_resp = self.http.get(url).bearer_auth(&retry_token).send().await?;
                parse_response(retry_resp).await
            }
            s if s == reqwest::StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = resp
                    .headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|wait_str| wait_str.parse::<u64>().ok())
                    .unwrap_or(10);
                tracing::warn!("Graph rate-limited (429), waiting {retry_after}s before retry");
                tokio::time::sleep(Duration::from_secs(retry_after)).await;
                let retry_token = self.auth.get_valid_token().await?;
                let retry_resp = self.http.get(url).bearer_auth(&retry_token).send().await?;
                parse_response(retry_resp).await
            }
            _ => parse_response(resp).await,
        }
    }
}

// ---------- internal helpers ----------

/// Top-level wrapper for Graph API error JSON responses.
#[derive(serde::Deserialize)]
struct GraphErrorBody {
    /// Nested error detail object.
    error: GraphErrorDetail,
}

/// Inner error detail from a Graph API error response.
#[derive(serde::Deserialize)]
struct GraphErrorDetail {
    /// Machine-readable error code (e.g. `"Forbidden"`).
    code: String,
    /// Human-readable error description.
    message: String,
}

/// Parses a Graph API HTTP response, returning the deserialised body on success or a rich error.
///
/// # Errors
/// Returns an error if the response status is not 2xx, or if deserialisation fails.
async fn parse_response<T: DeserializeOwned>(resp: reqwest::Response) -> anyhow::Result<T> {
    if resp.status().is_success() {
        return Ok(resp.json::<T>().await?);
    }

    let status = resp.status();
    let body: GraphErrorBody = resp.json().await.unwrap_or_else(|_| GraphErrorBody {
        error: GraphErrorDetail {
            code: status.as_str().to_owned(),
            message: "No error detail returned by Graph".to_owned(),
        },
    });
    anyhow::bail!(
        "Graph API error ({}) — {}: {}",
        status,
        body.error.code,
        body.error.message
    )
}
