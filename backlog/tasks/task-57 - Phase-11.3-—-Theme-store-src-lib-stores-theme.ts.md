---
id: TASK-57
title: Phase 11.3 — Theme store (src/lib/stores/theme.ts)
status: Done
assignee: []
created_date: '2026-03-23 18:47'
updated_date: '2026-03-23 18:49'
labels:
  - frontend
  - theme
  - phase-11
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/lib/stores/theme.ts` — the single source of truth for the user's theme choice.

The store handles three concerns:
1. **State**: a `writable` for the user's choice (`'dark' | 'light' | 'system'`).
2. **Resolution**: a `derived` store that resolves `'system'` to the actual OS preference via `window.matchMedia`.
3. **DOM application**: a `subscribe` side-effect that adds or removes the `theme-light` class on `document.documentElement` whenever the resolved theme changes.
4. **Persistence**: `loadTheme()` and `saveTheme()` backed by `tauri-plugin-store` (`config.json`, key `"theme"`).

## Context

- The app uses `tauri-plugin-store` for settings persistence (already wired in Phase 8). The store file is `config.json`.
- Existing stores live in `src/lib/stores/`. New store follows the same pattern.
- The frontend uses Svelte 5 runes. However, this store uses the classic `writable`/`derived` API (not runes) to match the existing store files.
- `document` may be undefined during SSR-like contexts (Vite SSR or Vitest). Guard with `if (typeof document === 'undefined') return;`.

## Implementation

```typescript
// src/lib/stores/theme.ts
import { Store } from '@tauri-apps/plugin-store';
import { writable, derived, get } from 'svelte/store';

export type ThemeChoice = 'dark' | 'light' | 'system';

export const themeChoice = writable<ThemeChoice>('system');

// Resolves 'system' to the actual OS preference.
export const resolvedTheme = derived(themeChoice, ($choice) => {
    if ($choice !== 'system') return $choice;
    return window.matchMedia('(prefers-color-scheme: light)').matches
        ? 'light'
        : 'dark';
});

// Side-effect: keep <html> class in sync with resolved theme.
resolvedTheme.subscribe((resolved) => {
    if (typeof document === 'undefined') return;
    document.documentElement.classList.toggle('theme-light', resolved === 'light');
});

// ── Persistence ───────────────────────────────────────────────────────────────

let _store: Store | null = null;

async function getStore(): Promise<Store> {
    if (!_store) _store = await Store.load('config.json');
    return _store;
}

export async function loadTheme(): Promise<void> {
    const store = await getStore();
    const saved = await store.get<ThemeChoice>('theme');
    if (saved) themeChoice.set(saved);
}

export async function saveTheme(value: ThemeChoice): Promise<void> {
    themeChoice.set(value);
    const store = await getStore();
    await store.set('theme', value);
    await store.save();
}
```

## Exports

The file must export: `ThemeChoice` (type), `themeChoice` (writable store), `resolvedTheme` (derived store), `loadTheme()`, `saveTheme()`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 File `src/lib/stores/theme.ts` is created and exports `ThemeChoice`, `themeChoice`, `resolvedTheme`, `loadTheme`, and `saveTheme`
- [x] #2 `themeChoice` is a `writable<ThemeChoice>` with default value `'system'`
- [x] #3 `resolvedTheme` is a `derived` store: returns the choice directly for `'dark'`/`'light'`, and checks `window.matchMedia('(prefers-color-scheme: light)')` for `'system'`
- [x] #4 A `resolvedTheme.subscribe` side-effect toggles `theme-light` class on `document.documentElement` (guards against undefined `document`)
- [x] #5 `loadTheme()` reads key `"theme"` from `config.json` and sets `themeChoice` if a value is found; does nothing if key is absent
- [x] #6 `saveTheme(value)` sets `themeChoice`, writes key `"theme"` to `config.json`, and calls `store.save()`
- [ ] #7 `pnpm tsc --noEmit` passes with no type errors in this file
<!-- AC:END -->
