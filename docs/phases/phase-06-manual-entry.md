# Phase 6 — Manual Entry View

## 6.1 Tauri commands

In `src-tauri/src/commands/entries.rs`:
```rust
#[tauri::command]
pub async fn create_manual_entry(
    plan_id: String,
    task_id: Option<String>,
    start_time: String,  // ISO 8601
    end_time: String,    // ISO 8601
    notes: Option<String>,
    pool: State<'_, SqlitePool>,
    timer: State<'_, Mutex<Option<ActiveTimer>>>,
) -> Result<TimeEntry, String>
// Validate: end_time > start_time, no active timer running

#[tauri::command]
pub async fn delete_entry(id: String, pool: State<'_, SqlitePool>) -> Result<(), String>

#[tauri::command]
pub async fn update_entry(id: String, start_time: String, end_time: String, notes: Option<String>, pool: State<'_, SqlitePool>) -> Result<TimeEntry, String>
```

## 6.2 Manual Entry view

Create `src/views/ManualEntry.svelte`:

**Top section — form**
- Plan dropdown → Task dropdown (same logic as Time Tracking; "No specific task" option)
- Start datetime input (`<Input type="datetime-local">`)
- End datetime input (must be after start)
- Notes textarea (optional)
- Submit button → calls `create_manual_entry`
- Inline validation errors (end before start, missing plan)

**Bottom section — recent entries**
- Same table as Time Tracking view
- Edit button per row: populates form with entry data, submit updates instead of creates

## Verification checklist
- [x] Submitting valid form creates entry and refreshes list
- [x] Validation prevents: missing plan, end before start
- [x] Entry can be created for a plan without selecting a specific task
- [x] Edit flow pre-fills form and updates on submit
- [x] Delete removes entry with confirmation
- [x] Cannot add a manual entry while a timer is running
