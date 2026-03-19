use std::{sync::Arc, time::Duration};

use serde::de::DeserializeOwned;

use crate::auth::manager::AuthManager;

const GRAPH_BASE: &str = "https://graph.microsoft.com/v1.0";

pub struct GraphClient {
    http: reqwest::Client,
    auth: Arc<AuthManager>,
}

impl GraphClient {
    pub fn new(auth: Arc<AuthManager>) -> Self {
        Self {
            http: reqwest::Client::new(),
            auth,
        }
    }

    /// GET `{GRAPH_BASE}{path}` with a valid bearer token.
    /// On 401, refreshes the token silently and retries once.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = format!("{GRAPH_BASE}{path}");
        self.get_url(&url).await
    }

    /// GET an absolute URL with a valid bearer token.
    /// Handles 401 (silent token refresh + one retry) and 429 (Retry-After backoff + one retry).
    pub async fn get_url<T: DeserializeOwned>(&self, url: &str) -> anyhow::Result<T> {
        let token = self.auth.get_valid_token().await?;
        let resp = self.http.get(url).bearer_auth(&token).send().await?;

        match resp.status() {
            s if s == reqwest::StatusCode::UNAUTHORIZED => {
                tracing::info!("Graph returned 401 — refreshing token and retrying");
                let token = self.auth.get_valid_token().await?;
                let resp = self.http.get(url).bearer_auth(&token).send().await?;
                parse_response(resp).await
            }
            s if s == reqwest::StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = resp
                    .headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(10);
                tracing::warn!("Graph rate-limited (429), waiting {retry_after}s before retry");
                tokio::time::sleep(Duration::from_secs(retry_after)).await;
                let token = self.auth.get_valid_token().await?;
                let resp = self.http.get(url).bearer_auth(&token).send().await?;
                parse_response(resp).await
            }
            _ => parse_response(resp).await,
        }
    }
}

// ---------- internal helpers ----------

#[derive(serde::Deserialize)]
struct GraphErrorBody {
    error: GraphErrorDetail,
}

#[derive(serde::Deserialize)]
struct GraphErrorDetail {
    code: String,
    message: String,
}

async fn parse_response<T: DeserializeOwned>(resp: reqwest::Response) -> anyhow::Result<T> {
    if resp.status().is_success() {
        return Ok(resp.json::<T>().await?);
    }

    let status = resp.status();
    let body: GraphErrorBody = resp.json().await.unwrap_or(GraphErrorBody {
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
