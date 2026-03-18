# Microsoft Graph API

## Base URL

All requests target:
```
https://graph.microsoft.com/v1.0
```

## Module Layout

```
src-tauri/src/graph/
├── client.rs     ← GraphClient struct, get<T>(), authenticated request helpers
├── models.rs     ← GraphPlan, GraphTask, GraphUser, GraphPagedResponse<T>
└── planner.rs    ← fetch_my_plans(), fetch_tasks_for_plan(), fetch_my_tasks(), fetch_user_info()
```

## GraphClient

`GraphClient` wraps `reqwest::Client` and holds a reference to `AuthManager`. It is responsible for:
- Attaching `Authorization: Bearer {token}` headers (token obtained via `auth_manager.get_valid_token()`)
- Retrying once on `401 Unauthorized` after a token refresh
- Surfacing Graph error messages (see Error Handling below)

```rust
pub struct GraphClient {
    http: reqwest::Client,
    auth: Arc<AuthManager>,
}

impl GraphClient {
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let token = self.auth.get_valid_token().await?;
        let resp = self.http
            .get(format!("https://graph.microsoft.com/v1.0{path}"))
            .bearer_auth(&token)
            .send()
            .await?;

        if resp.status() == 401 {
            // Token may have been revoked externally — force refresh and retry once
            let token = self.auth.force_refresh().await?;
            return self.get_with_token(path, &token).await;
        }

        self.parse_response(resp).await
    }
}
```

## Pagination

All list endpoints in Microsoft Graph may return paginated results via `@odata.nextLink`. Always consume all pages:

```rust
pub async fn fetch_all_pages<T: DeserializeOwned>(
    client: &GraphClient,
    initial_path: &str,
) -> anyhow::Result<Vec<T>> {
    let mut results = Vec::new();
    let mut url: Option<String> = Some(format!("https://graph.microsoft.com/v1.0{initial_path}"));

    while let Some(next_url) = url {
        let page: GraphPagedResponse<T> = client.get_url(&next_url).await?;
        results.extend(page.value);
        url = page.next_link;
    }
    Ok(results)
}

#[derive(Deserialize)]
pub struct GraphPagedResponse<T> {
    pub value: Vec<T>,
    #[serde(rename = "@odata.nextLink")]
    pub next_link: Option<String>,
}
```

## Endpoints Used

| Function | Endpoint | Notes |
|---|---|---|
| `fetch_my_plans` | `GET /me/planner/plans` | Paginated |
| `fetch_tasks_for_plan` | `GET /planner/plans/{id}/tasks` | Paginated |
| `fetch_my_tasks` | `GET /me/planner/tasks` | Paginated |
| `fetch_user_info` | `GET /me` | Single object |

## Error Handling

Graph API errors return a JSON body with an `error.message` field. Surface this message to the user — do not expose only the HTTP status code.

```rust
#[derive(Deserialize)]
struct GraphErrorBody {
    error: GraphError,
}
#[derive(Deserialize)]
struct GraphError {
    code: String,
    message: String,
}

async fn parse_response<T: DeserializeOwned>(resp: reqwest::Response) -> anyhow::Result<T> {
    if !resp.status().is_success() {
        let body: GraphErrorBody = resp.json().await
            .unwrap_or_else(|_| GraphErrorBody { error: GraphError { code: "Unknown".into(), message: "No error detail".into() }});
        anyhow::bail!("Graph API error ({}): {}", body.error.code, body.error.message);
    }
    Ok(resp.json::<T>().await?)
}
```

## Rate Limiting (429)

Microsoft Graph enforces per-user throttling. Handle `429 Too Many Requests` with exponential backoff:
```rust
if resp.status() == 429 {
    let retry_after = resp.headers()
        .get("Retry-After")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(10);
    tokio::time::sleep(Duration::from_secs(retry_after)).await;
    // retry request
}
```

## Required Scopes

| Scope | Purpose |
|---|---|
| `Tasks.Read` | Read Planner plans and tasks |
| `offline_access` | Receive refresh tokens (required for silent token renewal) |
| `User.Read` | Display signed-in user name and avatar |

`Tasks.ReadWrite` is out of scope unless writing back to Planner is explicitly implemented.

## Caching Strategy

After every successful sync, upsert all plans and tasks into SQLite. On startup, load from SQLite immediately so the UI is populated without waiting for a network request. Run Graph sync in the background.

The app must be fully usable in read-only mode when offline (no network access to Graph).

