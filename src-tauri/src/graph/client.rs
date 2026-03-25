use std::{sync::Arc, time::Duration};

use serde::de::DeserializeOwned;

use crate::auth::manager::AuthManager;

/// Base URL for all Microsoft Graph v1.0 API requests.
///
/// All relative paths passed to [`GraphClient::get`] are prepended with this string.
/// Absolute URLs (e.g. `@odata.nextLink` values) are used directly via
/// [`GraphClient::get_url`].
const GRAPH_BASE: &str = "https://graph.microsoft.com/v1.0";

/// Authenticated HTTP client for the Microsoft Graph API.
///
/// `GraphClient` wraps `reqwest::Client` with automatic token management and resilience
/// handling. It is responsible for:
///
/// - Attaching an `Authorization: Bearer {token}` header to every request, using a
///   valid token obtained from [`AuthManager::get_valid_token`].
/// - Retrying once on `401 Unauthorized` after forcing a token refresh via another call
///   to [`AuthManager::get_valid_token`] (which internally calls
///   [`crate::auth::oauth::refresh_access_token`]).
/// - Honouring `429 Too Many Requests` rate-limit responses by reading the `Retry-After`
///   header (defaulting to 10 seconds if absent) and sleeping before retrying once.
/// - Surfacing human-readable Graph API error messages from the JSON error body rather
///   than exposing only the HTTP status code.
///
/// `GraphClient` is not registered as Tauri managed state; instead it is created on
/// demand inside command handlers that need it, holding a clone of the [`Arc<AuthManager>`].
pub struct GraphClient {
    /// Underlying HTTP client. Reused across requests for connection pooling.
    http: reqwest::Client,
    /// Auth manager used to obtain and refresh bearer tokens on every request.
    auth: Arc<AuthManager>,
}

impl GraphClient {
    /// Creates a new `GraphClient` backed by the given [`AuthManager`].
    ///
    /// A fresh `reqwest::Client` is constructed for each `GraphClient` instance.
    ///
    /// # Arguments
    ///
    /// - `auth`: An [`Arc`]-wrapped [`AuthManager`] shared with the rest of the application.
    ///
    /// # Returns
    ///
    /// A new `GraphClient` ready to issue authenticated Graph API requests.
    #[must_use]
    pub fn new(auth: Arc<AuthManager>) -> Self {
        Self {
            http: reqwest::Client::new(),
            auth,
        }
    }

    /// Issues a GET request to `{GRAPH_BASE}{path}` with a valid bearer token.
    ///
    /// This is the primary entry point for Graph API requests that use relative paths.
    /// The `path` argument is appended directly to `GRAPH_BASE`, so it must begin
    /// with a `/` (e.g. `"/me/planner/plans"`).
    ///
    /// Internally delegates to [`GraphClient::get_url`] with the fully-formed URL, which
    /// handles `401` retry and `429` backoff.
    ///
    /// # Arguments
    ///
    /// - `path`: A Graph API path starting with `/` (e.g. `"/me/planner/plans"`).
    ///
    /// # Returns
    ///
    /// `Ok(T)` — the deserialised response body on success.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - a valid bearer token cannot be obtained from [`AuthManager`],
    /// - the HTTP request fails (network error, DNS, etc.),
    /// - the Graph API returns a non-2xx status (see `parse_response`), or
    /// - the response body cannot be deserialised as `T`.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = format!("{GRAPH_BASE}{path}");
        self.get_url(&url).await
    }

    /// Issues a GET request to an absolute URL with a valid bearer token.
    ///
    /// This method is used for two cases:
    /// 1. **Pagination** — `@odata.nextLink` values are absolute URLs that must be
    ///    used verbatim without the `GRAPH_BASE` prefix.
    /// 2. **Internal delegation** from [`GraphClient::get`] after URL construction.
    ///
    /// ## Error handling
    ///
    /// - **`401 Unauthorized`**: The token may have been externally revoked. The method
    ///   calls [`AuthManager::get_valid_token`] again (which triggers a refresh) and
    ///   retries the request once. If the retry also returns `401`, the error is returned
    ///   to the caller.
    /// - **`429 Too Many Requests`**: The `Retry-After` header is read (defaulting to 10 s
    ///   if absent or unparseable) and the thread sleeps for that duration before retrying
    ///   the request once with a fresh token.
    /// - **Other non-2xx**: Forwarded to `parse_response`, which extracts the Graph
    ///   error message and returns it as an `anyhow` error.
    ///
    /// # Arguments
    ///
    /// - `url`: The absolute URL to request (e.g. a `@odata.nextLink` continuation URL).
    ///
    /// # Returns
    ///
    /// `Ok(T)` — the deserialised response body on success.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - a valid bearer token cannot be obtained,
    /// - the HTTP request fails,
    /// - the Graph API returns a non-2xx status after retries, or
    /// - the response body cannot be deserialised as `T`.
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

/// Parses a Graph API HTTP response into `T` on success, or returns a descriptive error.
///
/// On a 2xx response the body is deserialised directly as `T`.
///
/// On a non-2xx response the body is attempted to deserialise as a [`GraphErrorBody`]
/// to extract the human-readable error code and message from the Graph error envelope.
/// If that deserialisation fails (e.g. the server returned HTML or a plain-text error),
/// a fallback error using the raw HTTP status code is returned instead.
///
/// # Arguments
///
/// - `resp`: The raw `reqwest::Response` from a Graph API request.
///
/// # Returns
///
/// `Ok(T)` — the deserialised response body on success.
///
/// # Errors
///
/// Returns an error if the response status is not 2xx, or if the success body cannot
/// be deserialised as `T`.
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
