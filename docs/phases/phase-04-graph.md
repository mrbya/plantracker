# Phase 4 — Microsoft Graph Integration

## 4.1 Graph API client (Rust)

Create `src-tauri/src/graph/client.rs`:
- `GraphClient` struct wrapping `reqwest::Client` + `AuthManager`
- `async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T>` — GET with bearer token, handles 401 by refreshing and retrying once

## 4.2 Graph models

Create `src-tauri/src/graph/models.rs` with structs matching Graph API responses:
```rust
pub struct GraphPlan { pub id: String, pub title: String, ... }
pub struct GraphTask { pub id: String, pub title: String, pub plan_id: String, ... }
pub struct GraphPagedResponse<T> { pub value: Vec<T>, #[serde(rename = "@odata.nextLink")] pub next_link: Option<String> }
```

## 4.3 Graph queries

Create `src-tauri/src/graph/planner.rs`:
- `fetch_my_plans(client) -> Result<Vec<GraphPlan>>` — GET `/me/planner/plans` (paginated)
- `fetch_tasks_for_plan(client, plan_id) -> Result<Vec<GraphTask>>` — GET `/planner/plans/{id}/tasks`
- `fetch_my_tasks(client) -> Result<Vec<GraphTask>>` — GET `/me/planner/tasks`
- `fetch_user_info(client) -> Result<UserInfo>` — GET `/me`

Handle OData pagination (`@odata.nextLink`) in all list endpoints.

## 4.4 Sync command

Create `src-tauri/src/commands/sync.rs`:
```rust
#[tauri::command]
pub async fn sync_plans_and_tasks(
    auth: State<'_, AuthManager>,
    pool: State<'_, SqlitePool>,
) -> Result<SyncResult, String>
```
- Fetch plans and tasks from Graph
- Upsert into local SQLite
- Return `SyncResult { plans_count, tasks_count, synced_at }`

## 4.5 Frontend data stores

Create `src/lib/stores/planner.ts`:
```typescript
export const plans = writable<Plan[]>([]);
export const tasksByPlan = writable<Record<string, Task[]>>({});
export const selectedPlan = writable<Plan | null>(null);
export const selectedTask = writable<Task | null>(null);

// When selectedTask changes, auto-update selectedPlan
export function selectTask(task: Task) { ... }
```

Call `sync_plans_and_tasks` on app startup (after auth) and expose a manual refresh action.

## Verification checklist
- [x] After login, plans and tasks populate from Graph and are saved to SQLite
- [x] Subsequent launches use cached SQLite data immediately, sync in background
- [x] Selecting a task in any dropdown auto-selects its parent plan
