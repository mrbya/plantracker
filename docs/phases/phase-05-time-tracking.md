# Phase 5 — Time Tracking View

## 5.1 Timer logic (Rust)

Create `src-tauri/src/commands/timer.rs`:
```rust
// In-memory active timer state
pub struct ActiveTimer {
    pub entry_id: String,
    pub plan_id: String,
    pub task_id: Option<String>,
    pub start_time: DateTime<Utc>,
}

#[tauri::command]
pub async fn start_timer(
    plan_id: String,
    task_id: Option<String>,
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String>
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

## 5.2 Timer store (frontend)

Create `src/lib/stores/timer.ts`:
- `isRunning: Readable<boolean>`
- `elapsedSeconds: Readable<number>` — updated every second via `setInterval` when running
- `activeEntry: Writable<TimeEntry | null>`
- `start(planId: string, taskId?: string)` and `stop()` — invoke backend commands

## 5.3 Time Tracking view

Create `src/views/TimeTracking.svelte`:

**Top section — controls**
- Plan dropdown (`<SearchableSelect>` component, options from `plans` store)
- Task dropdown (`<SearchableSelect>` component, filtered by selected plan; prepends "No specific task" option)
- Start/Stop button — green when stopped, red when running; shows elapsed time when running
- Selected task label displayed below dropdowns

**Bottom section — recent entries**
- Table: Task | Plan | Start | End | Duration
- Last 20 entries for selected plan (or task if selected)
- Delete icon per row (with confirmation)
- `EmptyState` when no entries

## Verification checklist
- [x] Start timer creates DB entry with no end_time
- [x] Elapsed time counts up in real time
- [x] Stop timer saves end_time and refreshes entry list
- [x] Cannot start a second timer if one is already running (button disabled, tooltip shown)
- [x] App restart correctly restores active timer if `end_time` is NULL
- [x] Timer can be started for a plan without selecting a specific task
