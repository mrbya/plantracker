# Appendices

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

`src/lib/api/index.ts` is a singular boundary for all `invoke()`s exporting typed invoke
wrappers for the rest of the frontend. Keep all `invoke` calls in this file — views and stores
import from here, never call `invoke` directly.


```typescript
import { invoke } from '@tauri-apps/api/core';

// Typed wrapper example
export async function startTimer(planId: string, taskId?: string): Promise<TimeEntry> {
    return invoke<TimeEntry>('start_timer', { planId, taskId: taskId ?? null });
}
```

---

## Appendix C — Type Definitions

Create `src/lib/types.ts` mirroring Rust structs:

```typescript
export interface Plan {
    id: string;
    graphId: string;
    title: string;
    syncedAt: string;
}

export interface Task {
    id: string;
    graphId: string;
    planId: string;
    title: string;
    syncedAt: string;
}

export interface TimeEntry {
    id: string;
    planId: string;
    taskId: string | null;
    startTime: string;
    endTime: string | null;
    notes: string | null;
    createdAt: string;
}

export interface AuthStatus {
    isAuthenticated: boolean;
    userDisplayName: string | null;
}

export interface ReportEntry {
    taskTitle: string;
    planTitle: string;
    startTime: string;
    endTime: string;
    durationSeconds: number;
    notes: string | null;
}

export interface ReportResult {
    entries: ReportEntry[];
    grandTotalSeconds: number;
    subjectLabel: string;
}
```
