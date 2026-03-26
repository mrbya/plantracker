---
id: TASK-79.2
title: API wrapper getAppVersion and Storybook mock (§12.3.2–12.3.3)
status: Done
assignee: []
created_date: '2026-03-26 12:54'
updated_date: '2026-03-26 13:08'
labels:
  - frontend
  - phase-12
dependencies:
  - TASK-79.1
references:
  - docs/phases/phase-12-quality-of-life.md
parent_task_id: TASK-79
priority: medium
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add the `getAppVersion` typed wrapper to `src/lib/api/index.ts` and register a default mock handler in the Storybook mock file.

## API wrapper

Add to the Settings section of `src/lib/api/index.ts`:

```typescript
/**
 * Returns the application version string as declared in `Cargo.toml`.
 *
 * Example return value: `"0.1.2"`.
 * The version is read from the Tauri `AppHandle` at runtime, so it always
 * reflects the built binary's version without any frontend hardcoding.
 */
export async function getAppVersion(): Promise<string> {
    return invoke<string>('get_app_version');
}
```

## Storybook mock

Add a default handler to `src/stories/__mocks__/tauri-api-core.ts` so Settings stories render without a running Tauri process:

```typescript
const DEFAULTS: Record<string, InvokeHandler> = {
    // ... existing handlers ...
    get_app_version: () => '0.0.0-storybook',
};
```

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.3.2 and §12.3.3
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 getAppVersion() is exported from src/lib/api/index.ts with full JSDoc
- [x] #2 get_app_version is present in the DEFAULTS map in tauri-api-core.ts returning '0.0.0-storybook'
- [x] #3 pnpm tsc --noEmit passes with no type errors
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `getAppVersion()` to the Settings section of `src/lib/api/index.ts` with full JSDoc. Added `get_app_version: () => '0.0.0-storybook'` to the DEFAULTS map in `src/stories/__mocks__/tauri-api-core.ts`. `pnpm tsc --noEmit` passes clean.
<!-- SECTION:FINAL_SUMMARY:END -->
