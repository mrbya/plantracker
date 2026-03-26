---
id: TASK-78.3
title: Create locale store and wire startup loading (§12.2.4–12.2.5)
status: Done
assignee: []
created_date: '2026-03-26 07:16'
updated_date: '2026-03-26 07:30'
labels:
  - frontend
  - i18n
  - phase-12
dependencies: []
parent_task_id: TASK-78
priority: medium
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/lib/stores/locale.ts` and update `src/routes/+page.svelte` to load the saved locale before any strings are rendered.

## Locale store (`src/lib/stores/locale.ts`)

Wraps Paraglide's `setLocale`/`getLocale` with persistence to `config.json` via `tauri-plugin-store`. The store has no Svelte writable — `setLocale` is already reactive. Only handles the persistence layer.

```typescript
import { Store } from '@tauri-apps/plugin-store';
import { setLocale, getLocale } from '$lib/paraglide/runtime';

export type AppLocale = 'en' | 'sk' | 'de';

export const LOCALE_LABELS: Record<AppLocale, string> = {
    en: 'English',
    sk: 'Slovenčina',
    de: 'Deutsch',
};

let _store: Store | null = null;

async function getStore(): Promise<Store> {
    if (!_store) _store = await Store.load('config.json');
    return _store;
}

export async function loadLocale(): Promise<void> {
    const store = await getStore();
    const saved = await store.get<AppLocale>('locale');
    if (saved && ['en', 'sk', 'de'].includes(saved)) {
        setLocale(saved);
    }
    // If no saved value, Paraglide uses the baseLocale ('en') — no action needed.
}

export async function saveLocale(value: AppLocale): Promise<void> {
    setLocale(value);
    const store = await getStore();
    await store.set('locale', value);
    await store.save();
}
```

## Startup wiring (`src/routes/+page.svelte`)

Add `loadLocale()` as the **first** call in `onMount`, before `loadTheme()`:

```typescript
import { loadLocale } from '$lib/stores/locale';

onMount(async () => {
    await loadLocale();   // ← first
    await loadTheme();
    await loadSettings();
    await initAuth();
    // ... rest unchanged
});
```

Locale must be applied before any strings are rendered to avoid a flash of English text on non-English locales.

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.2.4 and §12.2.5
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 src/lib/stores/locale.ts exports AppLocale type, LOCALE_LABELS, loadLocale(), and saveLocale()
- [x] #2 loadLocale() is called first in onMount in src/routes/+page.svelte, before loadTheme()
- [x] #3 saveLocale() calls setLocale() synchronously and persists to config.json under key 'locale'
- [x] #4 loadLocale() is a no-op (does not call setLocale) when no locale is saved — Paraglide defaults to 'en'
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `src/lib/stores/locale.ts` and updated `src/routes/+page.svelte`.

**locale.ts:** Exports `AppLocale` type, `LOCALE_LABELS` (native language names), `loadLocale()`, and `saveLocale()`. Follows the same `_store` singleton pattern as `theme.ts`. `loadLocale()` is a no-op when no locale is saved (Paraglide defaults to `baseLocale: "en"`). `saveLocale()` calls `setLocale()` synchronously before the async persist.

**+page.svelte:** Added `import { loadLocale } from "$lib/stores/locale"` and `loadLocale()` as the first call in `onMount`, before `loadTheme()`. Kept the existing non-async `onMount` pattern to preserve the cleanup return for the media query listener. Updated the startup sequence JSDoc comment to document the new step 1.

**Verified:** `pnpm check:svelte` — 0 errors, 0 warnings across 4608 files.
<!-- SECTION:FINAL_SUMMARY:END -->
