---
id: TASK-53
title: 'Testing: Frontend unit tests — duration utilities and API wrappers'
status: Done
assignee: []
created_date: '2026-03-23 07:52'
updated_date: '2026-03-23 10:55'
labels:
  - testing
  - frontend
dependencies:
  - TASK-52
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Write frontend unit tests for the duration utility and API invoke wrappers. Covers sections 10.8 and 10.9 of Phase 10.

**10.8 — `src/lib/utils/duration.test.ts`**

First, export `formatDurationCSV` from `duration.ts` (currently unexported):
```ts
// change `function` → `export function`
export function formatDurationCSV(seconds: number): string { ... }
```

Then create `src/lib/utils/duration.test.ts`:

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
    it('formats as H:MM:SS with zero-padded minutes and seconds', () => {
        expect(formatDurationCSV(0)).toBe('0:00:00');
        expect(formatDurationCSV(3661)).toBe('1:01:01');
        expect(formatDurationCSV(9240)).toBe('2:34:00');
    });
});
```

**10.9 — `src/lib/api/index.test.ts`**

Use `mockIPC` to verify correct command names and argument shapes for wrappers that have argument transformation or optional fields:

```ts
import { describe, it, expect, vi } from 'vitest';
import { mockIPC } from '@tauri-apps/api/mocks';
import { startTimer, stopTimer, createManualEntry, generateReport } from './index';
```

Test cases:
| Test | What it asserts |
|---|---|
| `startTimer — passes planId and null taskId` | `invoke` called with `{ planId, taskId: null }` when no taskId provided |
| `startTimer — passes taskId when provided` | `invoke` called with `{ planId, taskId: 'tid' }` |
| `stopTimer — returns completed entry` | `endTime` is not null in the returned entry |
| `createManualEntry — taskId undefined becomes null` | `args.taskId` is `null` when `taskId` is omitted |
| `createManualEntry — taskId passed through` | `args.taskId` is set when `taskId` is provided |
| `generateReport — null planId/taskId when not provided` | Optional fields passed as `null` to invoke |
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 formatDurationCSV exported from duration.ts
- [x] #2 duration.test.ts exists with all 4 describe/it cases passing
- [x] #3 api/index.test.ts exists with all 6 test cases passing
- [x] #4 mockIPC used for all API tests — no real Tauri IPC calls
- [x] #5 pnpm test passes with all tests green
- [x] #6 svelte-check passes with no errors
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Exported formatDurationCSV from duration.ts. Created src/lib/utils/duration.test.ts with 4 test cases covering formatDuration (sub-hour, hour+minutes, zero) and formatDurationCSV (H:MM:SS formatting). Created src/lib/api/index.test.ts with 6 test cases using mockIPC to verify startTimer null/set taskId, stopTimer non-null endTime, createManualEntry undefined→null taskId and passthrough, and generateReport null planId/taskId. All 10 tests pass; svelte-check has 0 errors.
<!-- SECTION:FINAL_SUMMARY:END -->
