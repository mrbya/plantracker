# Phase 10 — Testing

> Tests are split into three independent layers that can be run separately.
> Each layer has its own `just` recipe. Never use `cargo test` with `SQLX_OFFLINE=true`
> unset when running Rust tests locally — the in-memory pool bypasses offline checking,
> but the compile step still requires it for the macros. Set `SQLX_OFFLINE=true` or
> run via `just test-rs`.

---

## 10.1 Rust dev-dependencies

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

## 10.2 Rust test helper — in-memory pool

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

## 10.3 DB layer tests — `db::entries`

Add inline `#[cfg(test)] mod tests { ... }` at the bottom of `entries.rs`.

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

## 10.4 DB layer tests — `db::plans` and `db::tasks`

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

## 10.5 Command logic tests — `commands::entries`

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

## 10.6 PKCE helper tests — `auth::pkce`

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

## 10.7 Frontend setup — Vitest

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
            reporter: ['text', 'cobertura'],
            reportsDirectory: 'coverage',
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

Also add `@vitest/coverage-v8` for cobertura reports in CI:

```bash
pnpm add -D @vitest/coverage-v8
```

---

## 10.8 Frontend utility tests — `duration.ts`

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

Note: `formatDurationCSV` must be exported from `duration.ts`:

```ts
// src/lib/utils/duration.ts — change `function` to `export function`
export function formatDurationCSV(seconds: number): string { ... }
```

---

## 10.9 Frontend API wrapper tests — `src/lib/api/index.ts`

Create `src/lib/api/index.test.ts`. Use `mockIPC` to verify that each wrapper
invokes the correct command name and passes arguments in the expected shape.

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

## 10.10 `justfile` integration

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

## 10.11 CI integration

Add a `test` stage to `.gitlab-ci.yml` that runs before `build`:

```yaml
stages:
  - test
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

---

## Verification checklist

- [x] `just test-rs` passes — all Rust tests green, no warnings
- [x] `just test-js` passes — all Vitest tests green
- [x] `just test` runs both suites sequentially with a clear summary
- [x] `cargo clippy --tests -- -D warnings` passes (clippy covers test code too)
- [x] CI `test` stage runs before `build` and fails the pipeline if any test fails
- [x] Coverage report visible in GitLab MR sidebar
- [x] `just pre-commit` includes tests — a failing test blocks a commit
