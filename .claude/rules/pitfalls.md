# Common Pitfalls

Project-specific footguns. Check this list before implementing any of the affected areas.

---

## sqlx Offline Cache Not Updated

**Problem:** CI builds without a live database fail to compile after query changes.

**Rule:** Run `just precache` after every query addition or change and commit the updated `.sqlx/` directory. Never merge a query change without this step.

**Detect:** `cargo build` in an environment with `SQLX_OFFLINE=true` set will fail immediately if the cache is stale.

---

## Tokio Mutex Held Across `.await`

**Problem:** Holding a `tokio::sync::Mutex` lock across an `.await` point can cause deadlocks when two tasks each wait for the other's lock.

**Rule:** Always extract or clone the needed value out of the lock guard before any `.await`.

```rust
// CORRECT
let snapshot = {
    let guard = self.timer.lock().await;
    guard.clone()  // clone before dropping lock
};
do_something_async(snapshot).await?;

// WRONG — lock held across await
let guard = self.timer.lock().await;
do_something_async(&*guard).await?;
```

---

## OAuth Redirect URI Mismatch

**Problem:** `tauri-plugin-oauth` starts the callback server on `localhost:52721`. If the Azure AD app registration uses a different port or path, the login redirect silently fails — the user is redirected to an error page in the browser.

**Rule:** The redirect URI `http://localhost:52721/callback` must match exactly across:
1. Azure AD app registration
2. The authorization URL built in `oauth.rs`
3. The token exchange POST body in `oauth.rs`

All three must be identical, including `http` vs `https` and the trailing path.

---

## PKCE Verifier Insufficient Entropy

**Problem:** Using `rand::random::<u64>()` or similar small integer sources produces a short or low-entropy verifier. Some Microsoft tenants reject this.

**Rule:** Generate the verifier from a 64-byte buffer:
```rust
let mut bytes = [0u8; 64];
rand::thread_rng().fill_bytes(&mut bytes);
```
The resulting base64url string is 86 characters — well within RFC 7636's 43–128 character range.

---

## Active Timer Lost on App Restart

**Problem:** The in-memory `ActiveTimer` state is lost if the app crashes or is force-quit. The entry row in SQLite remains with `end_time = NULL`.

**Rule:** On every app startup, query for any `time_entries` row where `end_time IS NULL`. If found, restore the `ActiveTimer` state from that row so the timer resumes correctly.

```rust
// In init sequence, after init_db():
if let Some(entry) = db::entries::find_active_entry(&pool).await? {
    *timer.lock().await = Some(ActiveTimer {
        entry_id:   entry.id.clone(),
        plan_id:    entry.plan_id.clone(),
        task_id:    entry.task_id.clone(),  // Option<String>
        start_time: entry.start_time.parse()?,
    });
}
```

---

## Window Close With Active Timer

**Problem:** If the user closes the window while a timer is running, the entry is saved to SQLite with `end_time = NULL`. On restart, it restores — but the elapsed time during the closed period is silently included, producing an inflated duration.

**Rule:** Listen for the Tauri window close event and warn the user if a timer is active:

```rust
window.on_window_event(|event| {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        if timer_is_active() {
            api.prevent_close();
            // emit event to frontend to show confirmation dialog
        }
    }
});
```

---

## Microsoft Graph Rate Limiting (429)

**Problem:** Fetching many plans or tasks in rapid succession can trigger per-user Graph API throttling. Requests return `429 Too Many Requests` with a `Retry-After` header.

**Rule:** The `GraphClient` must handle `429` responses by reading the `Retry-After` header and waiting before retrying. Do not retry immediately — this will worsen the throttling.

---

## SQLite Datetime Range Queries With Non-ISO Format

**Problem:** SQLite compares `TEXT` columns lexicographically. A datetime string that is not in `YYYY-MM-DDTHH:MM:SSZ` format (e.g., RFC 2822 or locale-formatted strings) will produce incorrect `WHERE start_time >= ?` results.

**Rule:** Always store datetimes using `.to_rfc3339()` in Rust, which produces ISO 8601 / RFC 3339 format. Never store locale-formatted or human-readable datetime strings.

---

## Duplicate Active Timers

**Problem:** If `start_timer` is called while a timer is already running (e.g., a race condition or double-click), two entries with `end_time = NULL` will exist in the database.

**Rule:** The `start_timer` command must check for an existing active timer before inserting a new row. Return an error if one is already running:

```rust
let active = db::entries::find_active_entry(&pool).await?;
if active.is_some() {
    return Err("A timer is already running".to_string());
}
```

---

## Tokens Logged at Debug Level

**Problem:** Access tokens and refresh tokens must never appear in logs, even at `tracing::debug!` level. Log files may be shared in bug reports.

**Rule:** Log token expiry timestamps, auth state transitions, and scope values. Never log the token string itself. If you log a `TokenSet` struct, ensure the struct's `Debug` implementation redacts sensitive fields:

```rust
#[derive(Serialize, Deserialize)]
pub struct TokenSet {
    #[serde(skip_serializing)]
    pub access_token: String,
    #[serde(skip_serializing)]
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

