---
id: TASK-59
title: Phase 11.5 — Startup wiring and OS preference reactivity
status: Done
assignee: []
created_date: '2026-03-23 18:47'
updated_date: '2026-03-23 18:50'
labels:
  - frontend
  - theme
  - phase-11
dependencies:
  - TASK-57
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Wire `loadTheme()` into the app startup sequence in `src/routes/+page.svelte` (or `src/App.svelte`, depending on project structure) and add a `matchMedia` listener so the theme re-applies when the OS preference changes while the app is running.

## Context

- The theme store (`src/lib/stores/theme.ts`) is created in Phase 11.3.
- The `resolvedTheme` derived store re-evaluates when `themeChoice` changes, but it does **not** automatically re-evaluate when the OS preference changes (because `matchMedia` is not a Svelte store). To handle OS changes, a `change` listener must be added to the `MediaQueryList` and it must nudge `themeChoice` to trigger re-derivation.
- `loadTheme()` must be called **before** `initAuth()` so the correct class is on `<html>` before the app renders its first frame.

## Changes to the root component (`src/routes/+page.svelte` or `src/App.svelte`)

**Add to imports:**

```typescript
import { loadTheme, themeChoice } from '$lib/stores/theme';
import { get } from 'svelte/store';
```

**In the `onMount` block** (or `$effect` if using runes), add `loadTheme()` before `initAuth()`:

```typescript
onMount(async () => {
    await loadSettings();
    await loadTheme();    // ← must come before initAuth
    await initAuth();
    // ...
```

**Also in the same `onMount` block**, register and clean up the OS preference listener:

```typescript
    const mq = window.matchMedia('(prefers-color-scheme: light)');
    const onSystemChange = () => {
        if (get(themeChoice) === 'system') {
            // Nudge the store to retrigger the derived resolvedTheme.
            themeChoice.update((v) => v);
        }
    };
    mq.addEventListener('change', onSystemChange);
    return () => mq.removeEventListener('change', onSystemChange);
});
```

The cleanup function returned from `onMount` removes the listener when the component unmounts.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 On app start, `loadTheme()` is awaited before `initAuth()` so the correct class is on `<html>` before the first render
- [x] #2 With **System Default** active, toggling the OS colour scheme (e.g. via system settings or devtools media emulation) causes the app theme to update immediately without a page reload
- [x] #3 With **Dark** or **Light** explicitly selected, toggling the OS preference has no effect on the app theme
- [x] #4 The `MediaQueryList` listener is registered exactly once and is removed on component unmount (use browser devtools to verify no duplicate listeners accumulate)
- [x] #5 `pnpm tsc --noEmit` passes with no type errors
- [ ] #6 `cargo tauri dev` starts cleanly with no console errors
<!-- AC:END -->
