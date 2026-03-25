# Rust Conventions

## Error Handling

Use `anyhow::Result<T>` for all internal library and helper code.

At Tauri command boundaries, convert to `String` for serialisation:
```rust
#[tauri::command]
pub async fn my_command(...) -> Result<MyType, String> {
    do_work().await.map_err(|e| e.to_string())
}
```

Never use `unwrap()` or `expect()` in production paths. Reserve `expect()` only for invariants that cannot be violated at runtime (e.g., compiling a static regex).

## Async Runtime

All async code uses Tokio exclusively. No `async-std` or `smol`.

Tauri 2 provides the Tokio runtime — do not spawn a second one.

## Logging

Always use the `tracing` crate. Never use `println!` or `eprintln!`.

```rust
tracing::info!("Sync completed: {} plans, {} tasks", plans, tasks);
tracing::error!(error = %e, "Token refresh failed");
tracing::debug!(entry_id = %id, "Timer stopped");
```

Initialise `tracing_subscriber` with `EnvFilter` in `main.rs` so log levels are runtime-configurable via `RUST_LOG`.

## Shared Mutable State

Use `tokio::sync::Mutex<T>` (not `std::sync::Mutex`) for state shared across async tasks.

Register all shared state with Tauri's managed state system:
```rust
app.manage(db_pool);
app.manage(auth_manager);
app.manage(tokio::sync::Mutex::new(None::<ActiveTimer>));
```

Access in commands via `tauri::State<'_, T>`.

**Never hold a Tokio `Mutex` lock across an `.await` point.** Clone or extract the value first:
```rust
// CORRECT — lock released before await
let value = {
    let guard = state.lock().await;
    guard.clone()
};
do_async_work(value).await?;

// WRONG — lock held across await, deadlock risk
let guard = state.lock().await;
do_async_work(&*guard).await?;
```

## Identifiers

- Local DB primary keys: `uuid::Uuid::new_v4().to_string()` — stored as `TEXT` in SQLite
- Graph API IDs: stored in separate `graph_id` columns — never used as primary keys

## DateTime Handling

Store datetimes as ISO 8601 strings in SQLite `TEXT` columns:
```
2024-03-15T14:30:00Z
```

Parse to `chrono::DateTime<Utc>` in Rust when arithmetic is needed. Serialise back with `.to_rfc3339()` before writing to SQLite.

ISO 8601 strings are required for correct date ordering — SQLite compares `TEXT` lexicographically.

## Naming

| Thing | Convention |
|---|---|
| Structs, enums, traits | `PascalCase` |
| Tauri command functions | `snake_case`, matching the `invoke('name')` string on the frontend exactly |
| Modules and files | `snake_case` |
| Constants | `SCREAMING_SNAKE_CASE` |

## Serialisation

Derive `serde::Serialize` + `serde::Deserialize` on all types crossing the Tauri command boundary.

Use `#[serde(rename_all = "camelCase")]` on types returned to the frontend:
```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    pub id: String,
    pub plan_id: String,
    pub task_id: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}
```

## Command Registration

All commands must be registered in `main.rs` via `tauri::generate_handler![]`. An unregistered command silently fails on the frontend with an "unknown command" error.

