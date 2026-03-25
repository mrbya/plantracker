# Tauri bridge

## Purpose of the bridge layer

`src/lib/api/index.ts` is the single point where the frontend calls `invoke()` from `@tauri-apps/api/core`. Every Tauri command call in the application goes through a typed wrapper exported from this file.

Views and stores import the named functions they need; they never call `invoke()` directly.

This design has three practical effects:

1. **Single audit surface.** The full set of Tauri commands the frontend uses is visible in one file. Reviewing the IPC contract does not require grepping across views and stores.
2. **Type safety at the boundary.** Each wrapper specifies the exact argument types and return type. TypeScript catches mismatches at the call site.
3. **Testability.** The Tauri mock API (`@tauri-apps/api/mocks`) lets tests intercept `invoke` calls by command name. Because all real calls go through `src/lib/api/index.ts`, the tests for that file cover the entire IPC surface with no additional mocking infrastructure.

---

## Design rules

- One exported function per Tauri command.
- No state, no side effects beyond the IPC call itself. Wrappers do not import stores, do not show toasts, and do not catch errors — error handling is the caller's responsibility.
- All optional arguments that map to `Option<T>` in Rust are explicitly passed as `null` when absent, never as `undefined`. Tauri's JSON serialiser treats `undefined` values inconsistently across platforms; `null` maps cleanly to Rust `None`.
- Return types are always the concrete TypeScript interface, not a union with `null` unless the Rust command can return `None` (e.g., `getActiveTimer` returns `ActiveTimerInfo | null`).

---

## Wrapper shape and naming conventions

Function names are `camelCase` versions of the Rust command names (`snake_case`):

| Rust command | TypeScript wrapper |
|---|---|
| `start_timer` | `startTimer` |
| `stop_timer` | `stopTimer` |
| `get_active_timer` | `getActiveTimer` |
| `sync_plans_and_tasks` | `syncPlansAndTasks` |
| `create_manual_entry` | `createManualEntry` |
| `generate_report` | `generateReport` |
| `export_report_csv` | `exportReportCsv` |
| `get_data_dir` | `getDataDir` |

The `invoke` call always passes the exact `snake_case` string as the command name — it must match what is registered in `main.rs` via `tauri::generate_handler![]`.

For commands with multiple optional parameters, the wrapper accepts a single `params` object rather than positional arguments:

```typescript
export async function createManualEntry(params: {
  planId: string;
  taskId?: string;
  startTime: string;
  endTime: string;
  notes?: string;
}): Promise<TimeEntry> {
  return invoke<TimeEntry>("create_manual_entry", {
    planId: params.planId,
    taskId: params.taskId ?? null,
    ...
  });
}
```

This makes call sites readable and avoids argument-order mistakes. The object spread in the `invoke` call does the `undefined → null` mapping explicitly.

---

## Type contracts

All TypeScript types for IPC values live in `src/lib/types.ts`. They mirror the Rust structs in `src-tauri/src/models.rs` and `src-tauri/src/commands/`. The Rust structs derive `#[serde(rename_all = "camelCase")]`, so `plan_id` in Rust becomes `planId` in TypeScript.

The mapping is maintained manually — there is no code generation. When a Rust struct gains or loses a field, the corresponding TypeScript interface in `src/lib/types.ts` must be updated in the same change.

Current types and their Rust counterparts:

| TypeScript interface | Rust struct / location |
|---|---|
| `Plan` | `Plan` in `src-tauri/src/models.rs` |
| `Task` | `Task` in `src-tauri/src/models.rs` |
| `TimeEntry` | `TimeEntry` in `src-tauri/src/models.rs` |
| `AuthStatus` | `AuthStatus` in `src-tauri/src/commands/auth.rs` |
| `SyncResult` | `SyncResult` in `src-tauri/src/commands/sync.rs` |
| `ActiveTimerInfo` | `ActiveTimerInfo` in `src-tauri/src/commands/timer.rs` |
| `ReportEntry` | `ReportEntry` in `src-tauri/src/commands/reports.rs` |
| `ReportResult` | `ReportResult` in `src-tauri/src/commands/reports.rs` |

All datetime fields are ISO 8601 strings in both Rust (`String` stored as RFC 3339 via `.to_rfc3339()`) and TypeScript (`string`). There are no `Date` objects at the boundary; conversion to display strings happens in `src/lib/utils/datetime.ts`.

---

## Error handling

Tauri commands return `Result<T, String>` on the Rust side. When the command returns `Err(msg)`, Tauri rejects the `invoke` promise with the error message string as the rejection value.

Wrappers in `src/lib/api/index.ts` do not catch these rejections. The error propagates to the caller — typically a store action — which catches it, calls `addError(String(e))` to show a toast, and re-throws if the view needs to know about the failure (e.g., to reset a loading flag).

```
invoke("start_timer", ...)
  -> Rust: Err("A timer is already running")
  -> Promise rejects with "A timer is already running"
  -> timer.start() catches, calls addError(), re-throws
  -> TimeTracking.handleStart() catches, sets timerBusy = false
```

The string cast `String(e)` in store catch blocks handles both the common case (a plain string) and any unusual rejection shapes without throwing a secondary error.

---

## Testing

`src/lib/api/index.test.ts` uses `@tauri-apps/api/mocks`'s `mockIPC` to intercept `invoke` calls by command name and return fixture data. The test file covers:

- Correct serialisation of optional arguments (`undefined → null`).
- Return-type shape validation (e.g., that `stopTimer` returns an entry with a non-null `endTime`).
- Null-propagation for scope fields (e.g., that `generateReport` sends `planId: null` when not provided).

Tests use Vitest and run in jsdom. The setup file at `src/tests/setup.ts` initialises the Tauri mock environment.

To run the tests: `pnpm test` (or `just test` via the justfile).

No tests currently cover the store layer directly. Store behaviour is tested indirectly via the view-level integration tests in `src/lib/api/index.test.ts`.

---

## Adding a new command end-to-end

1. **Rust side:** implement the command function in the appropriate file under `src-tauri/src/commands/`, derive `serde::Serialize` + `#[serde(rename_all = "camelCase")]` on any new return struct, and register the function in `tauri::generate_handler![]` in `main.rs`.

2. **Types:** if the command introduces a new return type or parameter struct, add the corresponding TypeScript interface to `src/lib/types.ts`. Match field names and nullability exactly.

3. **Wrapper:** add an exported async function to `src/lib/api/index.ts` following the naming convention above. Map any optional parameters to `null`. Import and re-export the new type if needed.

4. **Store or view:** if the result is shared state, add the call to the appropriate store action. If it is a one-off query specific to one view (like `generateReport`), call the wrapper directly from the view.

5. **Test:** add a test case to `src/lib/api/index.test.ts` using `mockIPC`. At minimum verify that optional arguments are correctly serialised to `null` and that the return type matches the TypeScript interface.

6. **sqlx cache:** if the command touches the database with a new or changed query, run `just precache` and commit the updated `.sqlx/` cache files. CI builds use offline mode and will fail without this step.
