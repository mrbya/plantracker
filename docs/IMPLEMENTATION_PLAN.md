# PlanTracker — Implementation Plan

> Structured step-by-step plan for code-generation agents.
> Each phase is self-contained. Complete and verify each phase before starting the next.

---

## Conventions

- **Frontend**: Svelte 5 + TypeScript, located in `src/`
- **Backend**: Rust + Tauri 2, located in `src-tauri/`
- **DB**: SQLite accessed via `sqlx` with compile-time checked queries
- **Commands**: Tauri `#[tauri::command]` functions bridge frontend ↔ Rust
- **Stores**: Svelte stores hold reactive UI state; backend is source of truth
- **Theme**: All colors from Catppuccin Mocha palette via CSS custom properties
- **Font**: JetBrains Mono Nerd Font throughout — bundled in `static/fonts/`, loaded via `@font-face` in `src/lib/theme/fonts.css`; no system install required

---

## Phase 0 — Project Scaffold

### 0.1 Initialize Tauri + Svelte project (Done)

```bash
pnpm create tauri-app plantracker \
  --template svelte-ts \
  --manager pnpm \
  --tauri-version 2
cd plantracker
```

### 0.2 Install frontend dependencies

```bash
pnpm add -D @types/node
# No runtime UI framework — vanilla Svelte only
```

### 0.3 Install Rust crates

In `src-tauri/Cargo.toml`, add:

```toml
[dependencies]
tauri = { version = "2", features = ["protocol-asset"] }
tauri-plugin-store = "2"
tauri-plugin-shell = "2"
tauri-plugin-oauth = "2"          # localhost redirect server
tauri-plugin-dialog = "2"         # Save file dialogs (CSV export)
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-rustls", "chrono", "macros"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
keyring = "2"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
anyhow = "1"
base64 = "0.22"
sha2 = "0.10"
rand = "0.8"
csv = "1"
```

### 0.4 Set up environment config

Create `.env.example`:
```env
VITE_AZURE_CLIENT_ID=
VITE_AZURE_TENANT_ID=common
```

Create `src-tauri/build.rs` to forward env vars to Rust at compile time:
```rust
fn main() {
    println!("cargo:rerun-if-env-changed=VITE_AZURE_CLIENT_ID");
    tauri_build::build()
}
```

### 0.5 Configure Tauri capabilities

In `src-tauri/capabilities/default.json`, enable:
- `core:default`
- `shell:allow-open` (system browser)
- `store:default`
- `dialog:default`
- `oauth:default`

### Verification checklist
- [x] `cargo tauri dev` launches without errors
- [x] Svelte HMR works (edit `App.svelte`, see instant update)
- [x] `.env` variables accessible as `import.meta.env.VITE_AZURE_CLIENT_ID`

---

## Phase 1 — Theme & Design System

### 1.1 Define Catppuccin Mocha CSS variables

Create `src/lib/theme/mocha.css`:

```css
:root {
  /* Catppuccin Mocha palette */
  --ctp-base:    #1e1e2e;
  --ctp-mantle:  #181825;
  --ctp-crust:   #11111b;
  --ctp-surface0:#313244;
  --ctp-surface1:#45475a;
  --ctp-surface2:#585b70;
  --ctp-overlay0:#6c7086;
  --ctp-overlay1:#7f849c;
  --ctp-overlay2:#9399b2;
  --ctp-subtext0:#a6adc8;
  --ctp-subtext1:#bac2de;
  --ctp-text:    #cdd6f4;
  --ctp-lavender:#b4befe;
  --ctp-blue:    #89b4fa;
  --ctp-sapphire:#74c7ec;
  --ctp-sky:     #89dceb;
  --ctp-teal:    #94e2d5;
  --ctp-green:   #a6e3a1;
  --ctp-yellow:  #f9e2af;
  --ctp-peach:   #fab387;
  --ctp-maroon:  #eba0ac;
  --ctp-red:     #f38ba8;
  --ctp-mauve:   #cba6f7;
  --ctp-pink:    #f5c2e7;
  --ctp-flamingo:#f2cdcd;
  --ctp-rosewater:#f5e0dc;

  /* Semantic aliases */
  --bg:          var(--ctp-base);
  --bg-raised:   var(--ctp-mantle);
  --bg-input:    var(--ctp-surface0);
  --border:      var(--ctp-surface1);
  --text:        var(--ctp-text);
  --text-muted:  var(--ctp-subtext0);
  --accent:      var(--ctp-mauve);
  --accent-hover:var(--ctp-lavender);
  --success:     var(--ctp-green);
  --warning:     var(--ctp-yellow);
  --danger:      var(--ctp-red);
  --timer-active:var(--ctp-green);

  /* Typography */
  --font: 'JetBrainsMono Nerd Font', 'JetBrains Mono', monospace;
  --font-size-sm: 0.75rem;
  --font-size-base: 0.875rem;
  --font-size-lg: 1rem;
  --font-size-xl: 1.25rem;

  /* Spacing */
  --radius: 6px;
  --radius-lg: 10px;
  --gap: 0.75rem;
}
```

### 1.2 Global reset and base styles

Create `src/lib/theme/global.css`. Include:
- `* { box-sizing: border-box; margin: 0; padding: 0; }`
- `body { background: var(--bg); color: var(--text); font-family: var(--font); font-size: var(--font-size-base); }`
- Scrollbar styling (thin, Catppuccin-colored)
- Focus ring styles (accent color outline)
- Import both files in `src/app.css`

### 1.3 Base UI components

Create the following in `src/lib/components/ui/`:

| Component | Props | Notes |
|---|---|---|
| `Button.svelte` | `variant: 'primary'\|'ghost'\|'danger'`, `disabled`, `loading` | Accent-colored primary |
| `Select.svelte` | `options: {value, label}[]`, `value`, `placeholder` | Styled `<select>` wrapper |
| `Input.svelte` | `type`, `value`, `label`, `error` | Text/datetime input |
| `Card.svelte` | `title?` | Surface0 background, rounded |
| `Badge.svelte` | `color: 'green'\|'red'\|'yellow'` | Small status indicator |
| `Spinner.svelte` | `size: 'sm'\|'md'` | CSS-only loading animation |
| `EmptyState.svelte` | `message` | Centered muted text |

### 1.4 Layout shell

Create `src/lib/components/Layout.svelte`:
- Left sidebar (64px wide, icon-only navigation)
- Main content area (fills remaining space)
- Nav items: Time Tracking, Manual Entry, Reports, Settings (icons via Nerd Font glyphs)
- Active state highlighted with accent color
- User avatar / sign-out at the bottom of sidebar

### Verification checklist
- [x] App renders with dark Catppuccin Mocha background
- [x] All base components render correctly in isolation
- [x] Font is JetBrains Mono throughout
- [x] Layout shell navigates between placeholder views

---

## Phase 2 — Database Layer

### 2.1 Configure sqlx and data directory

In `src-tauri/src/db/mod.rs`:
- Determine data directory using `tauri::path::app_data_dir()` on Linux, `documents_dir()` on Windows
- Create directory if it doesn't exist
- Return path to `plantracker.db`

### 2.2 Write migrations

Create `src-tauri/migrations/` with numbered files:

**`0001_initial.sql`**
```sql
CREATE TABLE IF NOT EXISTS plans (
    id        TEXT PRIMARY KEY,
    graph_id  TEXT NOT NULL UNIQUE,
    title     TEXT NOT NULL,
    synced_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks (
    id        TEXT PRIMARY KEY,
    graph_id  TEXT NOT NULL UNIQUE,
    plan_id   TEXT NOT NULL REFERENCES plans(id) ON DELETE CASCADE,
    title     TEXT NOT NULL,
    synced_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS time_entries (
    id         TEXT PRIMARY KEY,
    task_id    TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    start_time TEXT NOT NULL,
    end_time   TEXT,
    notes      TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_time_entries_task_id ON time_entries(task_id);
CREATE INDEX IF NOT EXISTS idx_time_entries_start_time ON time_entries(start_time);
CREATE INDEX IF NOT EXISTS idx_tasks_plan_id ON tasks(plan_id);
```

### 2.3 Database initialization

In `src-tauri/src/db/mod.rs`:
```rust
pub async fn init_db(app_handle: &tauri::AppHandle) -> anyhow::Result<SqlitePool> {
    let db_path = resolve_db_path(app_handle)?;
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path.display())).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
```

### 2.4 Repository functions

Create `src-tauri/src/db/plans.rs`, `tasks.rs`, `entries.rs` with these functions:

**plans.rs**
- `upsert_plan(pool, plan: &Plan) -> Result<()>`
- `list_plans(pool) -> Result<Vec<Plan>>`
- `get_plan(pool, id: &str) -> Result<Option<Plan>>`

**tasks.rs**
- `upsert_task(pool, task: &Task) -> Result<()>`
- `list_tasks_for_plan(pool, plan_id: &str) -> Result<Vec<Task>>`
- `get_task_by_graph_id(pool, graph_id: &str) -> Result<Option<Task>>`

**entries.rs**
- `insert_entry(pool, entry: &TimeEntry) -> Result<()>`
- `update_entry_end_time(pool, id: &str, end_time: DateTime<Utc>) -> Result<()>`
- `list_entries_for_task(pool, task_id: &str, limit: u32) -> Result<Vec<TimeEntry>>`
- `list_entries_for_plan(pool, plan_id: &str, limit: u32) -> Result<Vec<TimeEntry>>`
- `list_entries_in_range(pool, plan_id: Option<&str>, task_id: Option<&str>, from: NaiveDate, to: NaiveDate) -> Result<Vec<TimeEntry>>`
- `delete_entry(pool, id: &str) -> Result<()>`

### 2.5 Shared state

In `src-tauri/src/main.rs`, add `SqlitePool` to Tauri's managed state:
```rust
app.manage(db_pool);
```
Pass `State<SqlitePool>` into all command functions.

### Verification checklist
- [x] App starts and creates `plantracker.db` in correct platform directory
- [x] `sqlx migrate run` applies all migrations cleanly
- [x] `cargo sqlx prepare` generates `.sqlx/` query cache without errors

---

## Phase 3 — Authentication

### 3.1 PKCE helper functions

Create `src-tauri/src/auth/pkce.rs`:
- `generate_code_verifier() -> String` — 64 random bytes, base64url-encoded
- `generate_code_challenge(verifier: &str) -> String` — SHA-256 of verifier, base64url-encoded
- `generate_state() -> String` — 16 random bytes, hex-encoded

### 3.2 OAuth flow

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

### 3.3 Token storage

Create `src-tauri/src/auth/keychain.rs`:
- `save_tokens(tokens: &TokenSet) -> Result<()>` — store via `keyring` crate, service name `"PlanTracker"`
- `load_tokens() -> Result<Option<TokenSet>>`
- `clear_tokens() -> Result<()>`

### 3.4 Token manager

Create `src-tauri/src/auth/manager.rs` with `AuthManager` struct:
- Holds current `TokenSet` in memory (wrapped in `Mutex`)
- `get_valid_token(&self) -> Result<String>` — checks expiry, refreshes if needed, returns access token
- Exposed as Tauri managed state

### 3.5 Tauri commands for auth

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

### 3.6 Frontend auth store

Create `src/lib/stores/auth.ts`:
```typescript
import { writable, derived } from 'svelte/store';
export const authStatus = writable<AuthStatus | null>(null);
export const isAuthenticated = derived(authStatus, s => s?.is_authenticated ?? false);
export async function login() { /* invoke 'login' command */ }
export async function logout() { /* invoke 'logout' command */ }
```

### 3.7 Login screen

Create `src/views/Login.svelte`:
- Centered layout, app title, short description
- "Sign in with Microsoft" button → calls `login()`
- Loading state while auth completes
- Error display if login fails

Conditionally render `Login.svelte` vs `Layout.svelte` in `App.svelte` based on `isAuthenticated`.

### Verification checklist
- [x] Clicking "Sign in" opens system browser to Microsoft login
- [x] After login, app shows main layout
- [x] Refreshing the app re-uses stored tokens without re-login
- [x] "Sign out" clears tokens and returns to login screen
- [x] Expired tokens auto-refresh transparently

---

## Phase 4 — Microsoft Graph Integration

### 4.1 Graph API client (Rust)

Create `src-tauri/src/graph/client.rs`:
- `GraphClient` struct wrapping `reqwest::Client` + `AuthManager`
- `async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T>` — GET with bearer token, handles 401 by refreshing and retrying once

### 4.2 Graph models

Create `src-tauri/src/graph/models.rs` with structs matching Graph API responses:
```rust
pub struct GraphPlan { pub id: String, pub title: String, ... }
pub struct GraphTask { pub id: String, pub title: String, pub plan_id: String, ... }
pub struct GraphPagedResponse<T> { pub value: Vec<T>, #[serde(rename = "@odata.nextLink")] pub next_link: Option<String> }
```

### 4.3 Graph queries

Create `src-tauri/src/graph/planner.rs`:
- `fetch_my_plans(client) -> Result<Vec<GraphPlan>>` — GET `/me/planner/plans` (paginated)
- `fetch_tasks_for_plan(client, plan_id) -> Result<Vec<GraphTask>>` — GET `/planner/plans/{id}/tasks`
- `fetch_my_tasks(client) -> Result<Vec<GraphTask>>` — GET `/me/planner/tasks`
- `fetch_user_info(client) -> Result<UserInfo>` — GET `/me`

Handle OData pagination (`@odata.nextLink`) in all list endpoints.

### 4.4 Sync command

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

### 4.5 Frontend data stores

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

### Verification checklist
- [x] After login, plans and tasks populate from Graph and are saved to SQLite
- [x] Subsequent launches use cached SQLite data immediately, sync in background
- [x] Selecting a task in any dropdown auto-selects its parent plan

---

## Phase 5 — Time Tracking View

### 5.1 Timer logic (Rust)

Create `src-tauri/src/commands/timer.rs`:
```rust
// In-memory active timer state
pub struct ActiveTimer {
    pub entry_id: String,
    pub task_id: String,
    pub start_time: DateTime<Utc>,
}

#[tauri::command]
pub async fn start_timer(task_id: String, pool: State<'_, SqlitePool>, timer: State<'_, Mutex<Option<ActiveTimer>>>) -> Result<TimeEntry, String>
// Inserts entry with end_time = NULL, stores in ActiveTimer state

#[tauri::command]
pub async fn stop_timer(pool: State<'_, SqlitePool>, timer: State<'_, Mutex<Option<ActiveTimer>>>) -> Result<TimeEntry, String>
// Sets end_time = now on the in-progress entry, clears ActiveTimer

#[tauri::command]
pub async fn get_active_timer(timer: State<'_, Mutex<Option<ActiveTimer>>>) -> Result<Option<ActiveTimerInfo>, String>
// Returns active timer info including elapsed seconds

#[tauri::command]
pub async fn get_recent_entries(task_id: Option<String>, plan_id: Option<String>, limit: u32, pool: State<'_, SqlitePool>) -> Result<Vec<TimeEntry>, String>
```

### 5.2 Timer store (frontend)

Create `src/lib/stores/timer.ts`:
- `isRunning: Readable<boolean>`
- `elapsedSeconds: Readable<number>` — updated every second via `setInterval` when running
- `activeEntry: Writable<TimeEntry | null>`
- `start(taskId: string)` and `stop()` — invoke backend commands

### 5.3 Time Tracking view

Create `src/views/TimeTracking.svelte`:

**Top section — controls**
- Plan dropdown (`<Select>` component, options from `plans` store)
- Task dropdown (`<Select>` component, filtered by selected plan)
- Start/Stop button — green when stopped, red when running; shows elapsed time when running
- Selected task label displayed below dropdowns

**Bottom section — recent entries**
- Table: Task | Plan | Start | End | Duration
- Last 20 entries for selected plan (or task if selected)
- Delete icon per row (with confirmation)
- `EmptyState` when no entries

### Verification checklist
- [x] Start timer creates DB entry with no end_time
- [x] Elapsed time counts up in real time
- [x] Stop timer saves end_time and refreshes entry list
- [x] Cannot start a second timer if one is already running (button disabled, tooltip shown)
- [x] App restart correctly restores active timer if `end_time` is NULL

---

## Phase 6 — Manual Entry View

### 6.1 Tauri command

Create `src-tauri/src/commands/entries.rs`:
```rust
#[tauri::command]
pub async fn create_manual_entry(
    task_id: String,
    start_time: String,  // ISO 8601
    end_time: String,    // ISO 8601
    notes: Option<String>,
    pool: State<'_, SqlitePool>,
) -> Result<TimeEntry, String>
// Validate: end_time > start_time, no overlap with active timer

#[tauri::command]
pub async fn delete_entry(id: String, pool: State<'_, SqlitePool>) -> Result<(), String>

#[tauri::command]
pub async fn update_entry(id: String, start_time: String, end_time: String, notes: Option<String>, pool: State<'_, SqlitePool>) -> Result<TimeEntry, String>
```

### 6.2 Manual Entry view

Create `src/views/ManualEntry.svelte`:

**Top section — form**
- Plan dropdown → Task dropdown (same logic as Time Tracking)
- Start datetime input (`<Input type="datetime-local">`)
- End datetime input (must be after start)
- Notes textarea (optional)
- Submit button → calls `create_manual_entry`
- Inline validation errors (end before start, missing task)

**Bottom section — recent entries**
- Same table as Time Tracking view
- Edit button per row: populates form with entry data, submit updates instead of creates

### Verification checklist
- [x] Submitting valid form creates entry and refreshes list
- [x] Validation prevents: missing task, end before start
- [x] Edit flow pre-fills form and updates on submit
- [x] Delete removes entry with confirmation

---

## Phase 7 — Reports View

### 7.1 Report query (Rust)

Create `src-tauri/src/commands/reports.rs`:

```rust
#[derive(Serialize)]
pub struct MonthlyTotal {
    pub year: i32,
    pub month: u32,
    pub total_seconds: i64,
}

#[derive(Serialize)]
pub struct ReportResult {
    pub monthly_totals: Vec<MonthlyTotal>,
    pub grand_total_seconds: i64,
    pub subject_label: String,  // plan name or task name
}

#[tauri::command]
pub async fn generate_report(
    plan_id: Option<String>,
    task_id: Option<String>,
    from_year: i32,
    from_month: u32,
    to_year: i32,
    to_month: u32,
    pool: State<'_, SqlitePool>,
) -> Result<ReportResult, String>
```

SQL approach: group `time_entries` by `strftime('%Y-%m', start_time)`, filter by plan/task join, compute `SUM(unixepoch(end_time) - unixepoch(start_time))`.

### 7.2 CSV export (Rust)

```rust
#[tauri::command]
pub async fn export_report_csv(
    report: ReportResult,
    app: tauri::AppHandle,
) -> Result<String, String>
```
- Use `tauri-plugin-dialog` to show save file dialog (default: `~/Downloads/plantracker-report-{date}.csv`)
- Write CSV using the `csv` crate
- Return saved file path for confirmation toast

### 7.3 Reports view

Create `src/views/Reports.svelte`:

**Controls row**
- From: month `<Select>` + year `<Input type="number">`
- To: month `<Select>` + year `<Input type="number">`
- Plan dropdown → Task dropdown (task optional)
- Generate button
- Export CSV button (disabled until report is generated)

**Report table**
- Columns: Month | Total Hours | Total Minutes
- Footer row: Grand Total
- Subject header: "Plan: {name}" or "Task: {name}"
- `EmptyState` if no entries in range

**Duration formatting utility** (`src/lib/utils/duration.ts`):
- `formatDuration(seconds: number): string` → `"2h 34m"`
- `formatDurationCSV(seconds: number): string` → `"2:34:00"`

### Verification checklist
- [x] Report generates correct totals (manually verify with known entries)
- [x] Selecting task only shows entries for that task
- [x] Selecting plan with no task shows all task entries for that plan
- [x] CSV export produces correct file and opens save dialog
- [x] Date range spanning year boundary works correctly

---

## Phase 8 — Settings & Polish

### 8.1 Settings view

Create `src/views/Settings.svelte`:
- Recent entries limit: number input (default: 20)
- Sync frequency: dropdown (manual only / every 30 min / every hour)
- Data directory: read-only path display + "Open folder" button
- Account section: logged-in user name + avatar, Sign out button
- "Sync Now" button with last sync timestamp

Store settings via `tauri-plugin-store` in `config.json`.

### 8.2 App initialization sequence

In `App.svelte` `onMount`:
1. Check auth status → show Login or main app
2. If authenticated: load cached plans/tasks from SQLite
3. Trigger background sync (if due)
4. Check for active timer (in-progress entry with no `end_time`)
5. Load user settings

### 8.3 Error handling

Create `src/lib/stores/notifications.ts`:
- `notifications: Writable<Notification[]>`
- `addError(message: string)`, `addSuccess(message: string)`

Create `src/lib/components/ToastContainer.svelte`:
- Fixed bottom-right position
- Auto-dismiss after 4 seconds
- Color-coded: green (success), red (error), yellow (warning)

### 8.4 Loading states

Wrap each view's data fetching in:
- Skeleton loaders for tables (pulsing surface0 rectangles)
- `Spinner` inside buttons while async actions are pending
- Disable interactive controls during pending operations

### 8.5 Window configuration

In `src-tauri/tauri.conf.json`:
```json
{
  "app": {
    "windows": [{
      "title": "PlanTracker",
      "width": 1024,
      "height": 700,
      "minWidth": 800,
      "minHeight": 600,
      "decorations": true,
      "transparent": false
    }]
  }
}
```

### Verification checklist
- [x] Settings persist across app restarts
- [x] Toast notifications appear for errors and successes
- [x] Loading skeletons visible during data fetches
- [x] Window respects min dimensions

---

## Phase 9 — Build & Distribution

### 9.1 Production build

```bash
# Generate sqlx offline query cache (required for CI builds without a live DB)
cargo sqlx prepare --workspace

# Build
cargo tauri build
```

### 9.2 App icons

Generate icons from a source 1024×1024 PNG:
```bash
cargo tauri icon assets/icon.png
```

### 9.3 Updater (optional)

Configure `tauri-plugin-updater` pointing to a Gitlab Releases endpoint.

### 9.5 Bundle fonts (Done)

JetBrains Mono Nerd Font is self-hosted: all 16 weight/style `.ttf` files live in `static/fonts/`. They are declared via `@font-face` in `src/lib/theme/fonts.css` (imported first in `src/app.css`). Users do not need the font installed system-wide.

### 9.4 CI pipeline (Gitlab CI/CD)

Create `.gitlab-ci.yml` with jobs for:
- `ubuntu-latest` — produces `.deb` and `.AppImage`
- `windows-latest` — produces `.msi` and `.exe` (NSIS)

Each job:
1. `cargo sqlx prepare --check` (verify query cache is up to date)
2. `pnpm install && cargo tauri build`
3. Upload artifacts

### Verification checklist
- [x] Release build launches on clean Linux and Windows machines
- [x] App icon appears in taskbar and title bar
- [x] No `.env` secrets bundled in release binary (client ID is expected, not a secret)
- [x] Installer creates correct data directory on first run

---

## Phase 10 — Testing

> Tests are split into three independent layers that can be run separately.
> Each layer has its own `just` recipe. Never use `cargo test` with `SQLX_OFFLINE=true`
> unset when running Rust tests locally — the in-memory pool bypasses offline checking,
> but the compile step still requires it for the macros. Set `SQLX_OFFLINE=true` or
> run via `just test-rs`.

---

### 10.1 Rust dev-dependencies

Add to `src-tauri/Cargo.toml`:

```toml
[dev-dependencies]
tokio = { version = "1", features = ["full", "test-util"] }
```

`tokio` is already in `[dependencies]` with `features = ["full"]`, but the
`test-util` feature must be explicitly listed under `[dev-dependencies]` for
`#[tokio::test]` to work with the `pause`/`advance` time helpers. Everything
else needed for tests (`sqlx`, `chrono`, `uuid`, `anyhow`) is already present
in the main dependencies.

---

### 10.2 Rust test helper — in-memory pool

Create `src-tauri/src/db/test_helpers.rs`:

```rust
use sqlx::SqlitePool;

/// Spin up a fully migrated, in-memory SQLite pool for use in tests.
/// Each call returns an independent pool — tests are fully isolated.
pub async fn test_pool() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("failed to open in-memory SQLite");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations failed");

    sqlx::query!("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .expect("failed to enable FK constraints");

    pool
}
```

Expose it from `src-tauri/src/db/mod.rs` behind `#[cfg(test)]`:

```rust
#[cfg(test)]
pub mod test_helpers;
```

This keeps the helper out of the release binary entirely.

---

### 10.3 DB layer tests — `db::entries`

Create `src-tauri/src/db/entries_tests.rs` (or add `#[cfg(test)] mod tests { ... }`
at the bottom of `entries.rs` — either layout works, prefer the inline module
to keep tests co-located with the code):

**Cases to cover:**

| Test | What it asserts |
|---|---|
| `insert_and_fetch_entry` | `insert_entry` succeeds; `get_entry` returns the same row |
| `find_active_entry_none` | Returns `None` when no row has `end_time IS NULL` |
| `find_active_entry_some` | Returns the in-progress entry after `insert_entry` with `end_time: None` |
| `update_entry_end_time` | Sets `end_time`; `find_active_entry` returns `None` afterwards |
| `list_entries_for_task` | Returns only entries matching `task_id`, ordered by `start_time DESC`, capped by `limit` |
| `list_entries_for_plan` | Returns entries for all tasks in a plan, plus taskless entries |
| `list_entries_in_range_by_task` | Filters correctly by task within date window |
| `list_entries_in_range_by_plan` | Filters correctly by plan within date window |
| `list_entries_in_range_excludes_active` | In-progress entries (`end_time IS NULL`) are excluded |
| `delete_entry` | Row is gone; subsequent `get_entry` returns `None` |
| `fk_constraint_fires` | Inserting an entry with a nonexistent `plan_id` returns an error |
| `duplicate_active_entry_prevention` | Logic check: if one entry has `end_time IS NULL`, inserting another should be caught at the command layer (tested here by verifying two NULL rows can coexist at DB level — the guard is in the command, not the DB) |

Each test follows this structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::test_helpers::test_pool;
    use crate::models::TimeEntry;
    use uuid::Uuid;

    fn make_plan_id() -> String { Uuid::new_v4().to_string() }
    fn make_entry(plan_id: &str, task_id: Option<&str>, end_time: Option<&str>) -> TimeEntry {
        TimeEntry {
            id: Uuid::new_v4().to_string(),
            plan_id: plan_id.to_string(),
            task_id: task_id.map(str::to_string),
            start_time: "2024-03-15T10:00:00Z".to_string(),
            end_time: end_time.map(str::to_string),
            notes: None,
            created_at: "2024-03-15T10:00:00Z".to_string(),
        }
    }

    #[tokio::test]
    async fn insert_and_fetch_entry() {
        let pool = test_pool().await;
        // Insert a plan first (FK constraint)
        sqlx::query!("INSERT INTO plans (id, graph_id, title, synced_at) VALUES (?, ?, ?, ?)",
            "p1", "g1", "Test Plan", "2024-01-01T00:00:00Z")
            .execute(&pool).await.unwrap();

        let entry = make_entry("p1", None, Some("2024-03-15T11:00:00Z"));
        insert_entry(&pool, &entry).await.unwrap();

        let fetched = get_entry(&pool, &entry.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, entry.id);
    }

    // ... remaining tests
}
```

---

### 10.4 DB layer tests — `db::plans` and `db::tasks`

Add inline `#[cfg(test)]` modules to `plans.rs` and `tasks.rs`:

**`db::plans` cases:**

| Test | What it asserts |
|---|---|
| `upsert_plan_insert` | New plan is retrievable after upsert |
| `upsert_plan_update` | Upserting with same `graph_id` updates `title` and `synced_at` |
| `list_plans_empty` | Returns empty vec when no plans exist |
| `list_plans_multiple` | Returns all inserted plans |
| `get_plan_missing` | Returns `None` for an unknown ID |

**`db::tasks` cases:**

| Test | What it asserts |
|---|---|
| `upsert_task_insert` | New task is retrievable |
| `upsert_task_update` | Updates `title` on re-upsert |
| `list_tasks_for_plan` | Only tasks belonging to the given plan are returned |
| `get_task_by_graph_id` | Lookup by external Graph ID works |
| `cascade_delete` | Deleting a plan deletes its tasks (FK `ON DELETE CASCADE`) |

---

### 10.5 Command logic tests — `commands::entries`

The `parse_and_validate_times` function in `commands/entries.rs` is a pure
function — no DB, no Tauri state. Test it directly:

```rust
#[cfg(test)]
mod tests {
    use super::parse_and_validate_times;

    #[test]
    fn rejects_end_before_start() {
        let result = parse_and_validate_times(
            "2024-03-15T11:00:00Z",
            "2024-03-15T10:00:00Z",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("after start_time"));
    }

    #[test]
    fn rejects_equal_times() {
        let result = parse_and_validate_times(
            "2024-03-15T10:00:00Z",
            "2024-03-15T10:00:00Z",
        );
        assert!(result.is_err());
    }

    #[test]
    fn accepts_valid_range() {
        let result = parse_and_validate_times(
            "2024-03-15T10:00:00Z",
            "2024-03-15T11:00:00Z",
        );
        assert!(result.is_ok());
        let (start, end) = result.unwrap();
        assert!(end > start);
    }

    #[test]
    fn rejects_unparseable_start() {
        let result = parse_and_validate_times("not-a-date", "2024-03-15T11:00:00Z");
        assert!(result.is_err());
    }
}
```

---

### 10.6 PKCE helper tests — `auth::pkce`

Add to `src-tauri/src/auth/pkce.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use sha2::{Digest, Sha256};

    #[test]
    fn verifier_length_within_rfc7636_range() {
        let v = generate_code_verifier();
        assert!(v.len() >= 43 && v.len() <= 128,
            "verifier length {} outside RFC 7636 range", v.len());
    }

    #[test]
    fn verifier_is_base64url() {
        let v = generate_code_verifier();
        // URL_SAFE_NO_PAD alphabet: A-Z a-z 0-9 - _
        assert!(v.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn challenge_is_sha256_of_verifier() {
        let verifier = generate_code_verifier();
        let challenge = generate_code_challenge(&verifier);
        let expected = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
        assert_eq!(challenge, expected);
    }

    #[test]
    fn state_is_32_hex_chars() {
        let state = generate_state();
        assert_eq!(state.len(), 32);
        assert!(state.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn successive_verifiers_differ() {
        // Verifies the RNG is not returning a constant
        let a = generate_code_verifier();
        let b = generate_code_verifier();
        assert_ne!(a, b);
    }
}
```

---

### 10.7 Frontend setup — Vitest

Install dev dependencies:

```bash
pnpm add -D vitest @testing-library/svelte @testing-library/jest-dom jsdom
```

Create `vitest.config.ts` at the repo root (alongside `vite.config.js`):

```ts
import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
    plugins: [sveltekit()],
    test: {
        environment: 'jsdom',
        globals: true,
        setupFiles: ['src/tests/setup.ts'],
        include: ['src/**/*.test.ts'],
        coverage: {
            provider: 'v8',
            include: ['src/lib/**'],
            exclude: ['src/lib/theme/**'],
        },
    },
});
```

Create `src/tests/setup.ts`:

```ts
import { randomFillSync } from 'crypto';
import { clearMocks } from '@tauri-apps/api/mocks';
import '@testing-library/jest-dom';

// jsdom does not ship WebCrypto — polyfill it so @tauri-apps/api/mocks works
Object.defineProperty(window, 'crypto', {
    value: {
        getRandomValues: (buffer: BufferSource) =>
            randomFillSync(buffer as Buffer),
    },
    writable: true,
});

// Reset all Tauri IPC mocks between tests — prevents state leaking across files
afterEach(() => {
    clearMocks();
});
```

Add a `test` script to `package.json`:

```json
"scripts": {
    "test": "vitest run",
    "test:watch": "vitest",
    "test:coverage": "vitest run --coverage"
}
```

---

### 10.8 Frontend utility tests — `duration.ts`

Create `src/lib/utils/duration.test.ts`:

```ts
import { describe, it, expect } from 'vitest';
import { formatDuration, formatDurationCSV } from './duration';

describe('formatDuration', () => {
    it('formats sub-hour durations as minutes only', () => {
        expect(formatDuration(90)).toBe('1m');
        expect(formatDuration(3540)).toBe('59m');
    });

    it('formats hour + minutes', () => {
        expect(formatDuration(3600)).toBe('1h 0m');
        expect(formatDuration(9240)).toBe('2h 34m');
    });

    it('handles zero seconds', () => {
        expect(formatDuration(0)).toBe('0m');
    });
});

describe('formatDurationCSV', () => {
    it('formats as HH:MM:SS with zero-padded minutes and seconds', () => {
        expect(formatDurationCSV(0)).toBe('0:00:00');
        expect(formatDurationCSV(3661)).toBe('1:01:01');
        expect(formatDurationCSV(9240)).toBe('2:34:00');
    });
});
```

Note: `formatDurationCSV` is currently unexported. Export it from `duration.ts`
before writing this test:

```ts
// src/lib/utils/duration.ts — change `function` to `export function`
export function formatDurationCSV(seconds: number): string { ... }
```

---

### 10.9 Frontend API wrapper tests — `src/lib/api/index.ts`

Create `src/lib/api/index.test.ts`. Use `mockIPC` to verify that each wrapper
invokes the correct command name and passes arguments in the expected shape.
Test a representative subset — not every wrapper needs a test, focus on ones
with argument transformation or optional fields.

```ts
import { describe, it, expect, vi } from 'vitest';
import { mockIPC } from '@tauri-apps/api/mocks';
import {
    startTimer,
    stopTimer,
    createManualEntry,
    generateReport,
} from './index';
import type { TimeEntry, ReportResult } from '$lib/types';

const FAKE_ENTRY: TimeEntry = {
    id: '1',
    planId: 'p1',
    taskId: null,
    startTime: '2024-03-15T10:00:00Z',
    endTime: null,
    notes: null,
    createdAt: '2024-03-15T10:00:00Z',
};

describe('startTimer', () => {
    it('invokes start_timer with planId and null taskId', async () => {
        mockIPC((cmd, args) => {
            if (cmd === 'start_timer') return FAKE_ENTRY;
        });
        const spy = vi.spyOn(window.__TAURI_INTERNALS__, 'invoke');
        await startTimer('p1', undefined);
        expect(spy).toHaveBeenCalledWith(
            'start_timer',
            { planId: 'p1', taskId: null },
            undefined,
        );
    });
});

describe('stopTimer', () => {
    it('invokes stop_timer with no arguments', async () => {
        mockIPC((cmd) => {
            if (cmd === 'stop_timer') return { ...FAKE_ENTRY, endTime: '2024-03-15T11:00:00Z' };
        });
        const result = await stopTimer();
        expect(result.endTime).not.toBeNull();
    });
});

describe('createManualEntry', () => {
    it('passes optional taskId as null when not provided', async () => {
        mockIPC((cmd, args) => {
            if (cmd === 'create_manual_entry') {
                expect(args.taskId).toBeNull();
                return FAKE_ENTRY;
            }
        });
        await createManualEntry({
            planId: 'p1',
            taskId: undefined,
            startTime: '2024-03-15T10:00:00Z',
            endTime: '2024-03-15T11:00:00Z',
        });
    });
});

describe('generateReport', () => {
    it('invokes generate_report and returns ReportResult', async () => {
        const fakeResult: ReportResult = {
            entries: [],
            grandTotalSeconds: 0,
            subjectLabel: 'Plan: Test',
        };
        mockIPC((cmd) => {
            if (cmd === 'generate_report') return fakeResult;
        });
        const result = await generateReport({
            planId: 'p1',
            taskId: undefined,
            fromYear: 2024,
            fromMonth: 1,
            toYear: 2024,
            toMonth: 3,
        });
        expect(result.subjectLabel).toBe('Plan: Test');
    });
});
```

---

### 10.10 `justfile` integration

Add the following recipes:

```just
# Runs rust unit tests.
[working-directory: 'src-tauri']
test-rs *FLAGS:
    SQLX_OFFLINE=true cargo test {{FLAGS}}

# Runs frontend unit tests.
test-js *FLAGS:
    pnpm test {{FLAGS}}

# Runs all unit tests.
test:
    @just test-rs
    @just test-js

# Runs frontend unit tests with coverage report.
test-js-coverage:
    pnpm test:coverage
```

Update `pre-commit` to run tests before the build:

```just
pre-commit:
    @just thorough-check
    @just test          # ← add this line before build
    @just unused
    @just audit
    @just precache-check
    @just build
    @just index
```

---

### 10.11 CI integration

Add a `test` stage to `.gitlab-ci.yml` that runs before `build`.
Tests run on the same custom `linux-build` Docker image and share
the same cache, so no additional setup is needed.

```yaml
stages:
  - test      # ← new stage before build
  - build
  - release

test:
  stage: test
  image: registry.gitlab.com/family-treasure/plantracker/linux-build:latest
  variables:
    SQLX_OFFLINE: "true"
    VITE_AZURE_CLIENT_ID: "test-client-id"
    VITE_AZURE_TENANT_ID: "test-tenant-id"
  script:
    - just deps-ci
    - just test-rs
    - just test-js
  coverage: '/^TOTAL\s+\S+\s+\S+\s+(\d+\.?\d+%)$/'
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: coverage/cobertura-coverage.xml
    expire_in: 1 day
```

The `VITE_AZURE_CLIENT_ID` dummy value is required because `dotenvy` attempts to
load `.env` at startup even during `cargo test`. The value is never used in tests
since no command that reads it is exercised.

Add `@vitest/coverage-v8` for the cobertura report in CI:

```bash
pnpm add -D @vitest/coverage-v8
```

Update `vitest.config.ts` coverage output:

```ts
coverage: {
    provider: 'v8',
    reporter: ['text', 'cobertura'],
    reportsDirectory: 'coverage',
    include: ['src/lib/**'],
    exclude: ['src/lib/theme/**'],
},
```

---

### Verification checklist

- [x] `just test-rs` passes — all Rust tests green, no warnings
- [x] `just test-js` passes — all Vitest tests green
- [x] `just test` runs both suites sequentially with a clear summary
- [x] `cargo clippy --tests -- -D warnings` passes (clippy covers test code too)
- [x] CI `test` stage runs before `build` and fails the pipeline if any test fails
- [x] Coverage report visible in GitLab MR sidebar
- [x] `just pre-commit` includes tests — a failing test blocks a commit

---

## Appendix A — Tauri Command Registration

All commands must be registered in `src-tauri/src/main.rs`:

```rust
tauri::Builder::default()
    .manage(db_pool)
    .manage(auth_manager)
    .manage(Mutex::new(None::<ActiveTimer>))
    .invoke_handler(tauri::generate_handler![
        commands::auth::login,
        commands::auth::logout,
        commands::auth::get_auth_status,
        commands::sync::sync_plans_and_tasks,
        commands::timer::start_timer,
        commands::timer::stop_timer,
        commands::timer::get_active_timer,
        commands::timer::get_recent_entries,
        commands::entries::create_manual_entry,
        commands::entries::delete_entry,
        commands::entries::update_entry,
        commands::reports::generate_report,
        commands::reports::export_report_csv,
    ])
```

---

## Appendix B — Frontend ↔ Backend Contract

All Tauri invoke calls follow this pattern in TypeScript:

```typescript
import { invoke } from '@tauri-apps/api/core';

// Typed wrapper example
export async function startTimer(planId: string, taskId?: string): Promise<TimeEntry> {
    return invoke<TimeEntry>('start_timer', { planId, taskId: taskId ?? null });
}
```

Create `src/lib/api/index.ts` as the single file exporting all typed invoke wrappers. Keep all `invoke` calls in this file — views and stores import from here, never call `invoke` directly.

---

## Appendix C — Type Definitions

Create `src/lib/types.ts` mirroring Rust structs:

```typescript
export interface Plan { id: string; graphId: string; title: string; syncedAt: string; }
export interface Task { id: string; graphId: string; planId: string; title: string; syncedAt: string; }
export interface TimeEntry { id: string; planId: string; taskId: string | null; startTime: string; endTime: string | null; notes: string | null; createdAt: string; }
export interface AuthStatus { isAuthenticated: boolean; userDisplayName: string | null; }
export interface ReportEntry { taskTitle: string; planTitle: string; startTime: string; endTime: string; durationSeconds: number; notes: string | null; }
export interface ReportResult { entries: ReportEntry[]; grandTotalSeconds: number; subjectLabel: string; }
```

---

## Implementation Order Summary

| Phase | Deliverable | Est. Complexity |
|---|---|---|
| 0 | Project scaffold | Low |
| 1 | Theme & components | Medium |
| 2 | Database layer | Medium |
| 3 | Authentication | High |
| 4 | Graph API sync | Medium |
| 5 | Time tracking | Medium |
| 6 | Manual entry | Low |
| 7 | Reports | Medium |
| 8 | Settings & polish | Medium |
| 9 | Build & CI | Low |
| 10 | Testing |  

