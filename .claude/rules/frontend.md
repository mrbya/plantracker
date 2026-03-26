# Frontend Conventions (Svelte 5 + TypeScript)

## Svelte Version

This project uses **Svelte 5**. Always use runes-based reactivity. Never use legacy Svelte 3/4 syntax.

| Legacy (forbidden) | Svelte 5 (correct) |
|---|---|
| `$: derived = value * 2` | `const derived = $derived(value * 2)` |
| `let count = 0;` (reactive) | `let count = $state(0)` |
| `export let prop` | `let { prop } = $props()` |
| `onMount(() => { ... })` | `$effect(() => { ... })` |

## Invoke Boundary

**All `invoke()` calls must go through `src/lib/api/index.ts`.**

Views and stores must never call `invoke()` directly.

```typescript
// src/lib/api/index.ts  ← CORRECT — add wrappers here
import { invoke } from '@tauri-apps/api/core';
import type { TimeEntry, Plan, Task } from '../types';

export async function startTimer(planId: string, taskId?: string): Promise<TimeEntry> {
    return invoke<TimeEntry>('start_timer', { planId, taskId: taskId ?? null });
}

// src/views/TimeTracking.svelte  ← CORRECT — import from api
import { startTimer } from '$lib/api';

// src/views/TimeTracking.svelte  ← WRONG — direct invoke
import { invoke } from '@tauri-apps/api/core';
await invoke('start_timer', { taskId });
```

## Types

All TypeScript types live in `src/lib/types.ts` and mirror Rust structs with `camelCase` field names.

```typescript
// src/lib/types.ts
export interface Plan { id: string; graphId: string; title: string; syncedAt: string; }
export interface Task { id: string; graphId: string; planId: string; title: string; syncedAt: string; }
export interface TimeEntry { id: string; planId: string; taskId: string | null; startTime: string; endTime: string | null; notes: string | null; createdAt: string; }
export interface AuthStatus { isAuthenticated: boolean; userDisplayName: string | null; }
export interface ReportEntry { taskTitle: string; planTitle: string; startTime: string; endTime: string; durationSeconds: number; notes: string | null; }
export interface ReportResult { entries: ReportEntry[]; grandTotalSeconds: number; subjectLabel: string; }
```

Do not inline type definitions in component files.

## Internationalisation (i18n)

The app uses **Paraglide JS** (via `@inlang/paraglide-sveltekit`) for compile-time i18n. Three locales are supported: `en` (base, default), `sk`, `de`.

### Message files

Translation keys live in `messages/{locale}.json` at the project root. The generated runtime is emitted to `src/lib/paraglide/` by the Paraglide Vite plugin — never edit that directory by hand.

### Consuming translations

Import the generated message module in components and stores:

```typescript
import * as m from '$lib/paraglide/messages';

// Static key
m.timer_start()               // "Start Timer"

// Key with interpolation placeholder
m.timer_stop({ elapsed: '1h 23m' })   // "Stop — 1h 23m"
```

The call signature is always `m.key_name()` or `m.key_name({ param })`. TypeScript enforces that the correct parameters are supplied.

Never hard-code user-visible strings in components or stores. Every string that appears in the UI must have a key in `messages/en.json` (and matching keys in `messages/sk.json` / `messages/de.json`).

### Locale store

`src/lib/stores/locale.ts` manages locale persistence. It is **not** a Svelte writable store — it is a plain async module that wraps `tauri-plugin-store` (`config.json`, key `"locale"`).

```typescript
import { loadLocale, saveLocale, type AppLocale, LOCALE_LABELS } from '$lib/stores/locale';

// On app startup (called in +page.svelte onMount, before other init):
await loadLocale();   // reads config.json and calls setLocale() if a saved value exists

// When the user picks a new locale in Settings:
await saveLocale('sk');  // calls setLocale() immediately, then persists to config.json
```

`LOCALE_LABELS` maps each code to its display name: `{ en: 'English', sk: 'Slovenčina', de: 'Deutsch' }`.

To read the current locale at runtime (e.g., to pre-select the dropdown):
```typescript
import { getLocale } from '$lib/paraglide/runtime';
const current = getLocale(); // 'en' | 'sk' | 'de'
```

`loadLocale()` must be called **first** in the `+page.svelte` startup sequence, before `initAuth()` and other init functions, to minimise the flash of the default locale on non-English sessions.

## Stores

Stores live in `src/lib/stores/`, one file per domain:

| File | Owns |
|---|---|
| `auth.ts` | `authStatus`, `isAuthenticated`, `login()`, `logout()` |
| `planner.ts` | `plans`, `tasksByPlan`, `selectedPlan`, `selectedTask`, `selectTask()` |
| `timer.ts` | `isRunning`, `elapsedSeconds`, `activeEntry`, `start()`, `stop()` |
| `notifications.ts` | `notifications`, `addError()`, `addSuccess()`, `addWarning()` |
| `locale.ts` | `loadLocale()`, `saveLocale()`, `AppLocale`, `LOCALE_LABELS` |

Stores are the only place allowed to call `src/lib/api/index.ts` functions. Views bind to stores — they do not fetch data themselves.

## Component Layout

```
src/lib/components/ui/    ← headless, reusable primitives
  Button.svelte           props: variant ('primary'|'ghost'|'danger'), disabled, loading
  Select.svelte           props: options ({value, label}[]), value, placeholder
  Input.svelte            props: type, value, label, error
  Card.svelte             props: title?
  Badge.svelte            props: color ('green'|'red'|'yellow')
  Spinner.svelte          props: size ('sm'|'md')
  EmptyState.svelte       props: message

src/views/                ← full-page views, one per route
  Login.svelte
  TimeTracking.svelte
  ManualEntry.svelte
  Reports.svelte
  Settings.svelte
```

## Styling

No external CSS frameworks. All styling uses Catppuccin Mocha CSS variables from `src/lib/theme/mocha.css`.

No inline styles except for truly dynamic values (e.g., a progress bar width computed at runtime).

All colours must be referenced as CSS variables — see `ui.md` for the full rule.

## State Visibility

Always handle loading and error states explicitly. The UI must never be left in an ambiguous state:
- Show a `Spinner` or skeleton while data is loading
- Show an error message (via toast) if an async action fails
- Disable the triggering control while an action is pending
- Show `EmptyState` when a list has no items

## Path Aliases

Use `$lib` alias for `src/lib/`. Configure in `vite.config.ts` and `tsconfig.json`.

```typescript
import { startTimer } from '$lib/api';
import type { Task } from '$lib/types';
import { selectedTask } from '$lib/stores/planner';
```

