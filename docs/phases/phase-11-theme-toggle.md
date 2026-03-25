# Phase 11 — Dark / Light Theme Toggle

> Pure frontend change. No Rust commands, no database migrations, no new Tauri
> plugins. The theme is applied by toggling a CSS class on `<html>`, persisted
> via the existing `tauri-plugin-store` (`config.json`), and defaults to the
> OS preference via `prefers-color-scheme`.

---

## 11.1 Catppuccin Latte CSS file

Create `src/lib/theme/latte.css`. It must define the same set of variables as
`mocha.css` — palette first, then semantic aliases — so that swapping the
active theme file replaces the entire colour set without touching any component
styles.

```css
/* src/lib/theme/latte.css */
:root.theme-light {

  /* Catppuccin Latte palette */
  --ctp-base:      #eff1f5;
  --ctp-mantle:    #e6e9ef;
  --ctp-crust:     #dce0e8;
  --ctp-surface0:  #ccd0da;
  --ctp-surface1:  #bcc0cc;
  --ctp-surface2:  #acb0be;
  --ctp-overlay0:  #9ca0b0;
  --ctp-overlay1:  #8c8fa1;
  --ctp-overlay2:  #7c7f93;
  --ctp-subtext0:  #6c6f85;
  --ctp-subtext1:  #5c5f77;
  --ctp-text:      #4c4f69;
  --ctp-lavender:  #7287fd;
  --ctp-blue:      #1e66f5;
  --ctp-sapphire:  #209fb5;
  --ctp-sky:       #04a5e5;
  --ctp-teal:      #179299;
  --ctp-green:     #40a02b;
  --ctp-yellow:    #df8e1d;
  --ctp-peach:     #fe640b;
  --ctp-maroon:    #e64553;
  --ctp-red:       #d20f39;
  --ctp-mauve:     #8839ef;
  --ctp-pink:      #ea76cb;
  --ctp-flamingo:  #dd7878;
  --ctp-rosewater: #dc8a78;

  /* Semantic aliases — identical names to mocha.css */
  --bg:           var(--ctp-base);
  --bg-raised:    var(--ctp-mantle);
  --bg-input:     var(--ctp-surface0);
  --border:       var(--ctp-surface1);
  --text:         var(--ctp-text);
  --text-muted:   var(--ctp-subtext0);
  --accent:       var(--ctp-mauve);
  --accent-hover: var(--ctp-lavender);
  --success:      var(--ctp-green);
  --warning:      var(--ctp-yellow);
  --danger:       var(--ctp-red);
  --timer-active: var(--ctp-green);
}
```

The selector is `:root.theme-light`, not `:root`. Mocha remains the default
(`:root` selector in `mocha.css`) — no flash of unstyled content on first load.

---

## 11.2 Wire `latte.css` into `app.css`

Add the import after `mocha.css` and before `global.css`:

```css
/* src/app.css */
@import "$lib/theme/fonts.css";
@import "$lib/theme/mocha.css";
@import "$lib/theme/latte.css";   /* ← add this line */
@import "$lib/theme/global.css";
```

---

## 11.3 Theme store (`src/lib/stores/theme.ts`)

```typescript
import { Store } from '@tauri-apps/plugin-store';
import { writable, derived, get } from 'svelte/store';

export type ThemeChoice = 'dark' | 'light' | 'system';

export const themeChoice = writable<ThemeChoice>('system');

// Derived: the colour scheme actually in use right now ('dark' or 'light').
export const resolvedTheme = derived(themeChoice, ($choice) => {
    if ($choice !== 'system') return $choice;
    return window.matchMedia('(prefers-color-scheme: light)').matches
        ? 'light'
        : 'dark';
});

// Apply the correct class to <html> whenever the resolved theme changes.
resolvedTheme.subscribe((resolved) => {
    if (typeof document === 'undefined') return;
    document.documentElement.classList.toggle('theme-light', resolved === 'light');
});

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

---

## 11.4 Settings view — Appearance section

Add to `src/views/Settings.svelte` before the Account section:

```svelte
<!-- Appearance -->
<section class="settings-section">
  <h2 class="section-title">Appearance</h2>
  <div class="setting-row">
    <div class="setting-info">
      <span class="setting-label">Theme</span>
      <span class="setting-desc">
        Controls the colour scheme of the application.
      </span>
    </div>
    <div class="setting-control">
      <Select
        options={[
          { value: 'dark',   label: 'Dark'           },
          { value: 'light',  label: 'Light'          },
          { value: 'system', label: 'System Default' },
        ]}
        value={$themeChoice}
        onchange={(e) =>
          saveTheme((e.target as HTMLSelectElement).value as ThemeChoice)
        }
      />
    </div>
  </div>
</section>
```

---

## 11.5 Startup wiring and system-preference reactivity

In `src/routes/+page.svelte` (or `App.svelte`) `onMount`:

```typescript
import { loadTheme, themeChoice } from '$lib/stores/theme';
import { get } from 'svelte/store';

onMount(async () => {
    await loadTheme();    // must run before initAuth for correct first-frame class
    await loadSettings();
    await initAuth();
    // ...

    // Re-apply theme when OS changes (e.g. system switches to dark mode at sunset)
    const mq = window.matchMedia('(prefers-color-scheme: light)');
    const onSystemChange = () => {
        if (get(themeChoice) === 'system') {
            themeChoice.update((v) => v);
        }
    };
    mq.addEventListener('change', onSystemChange);
    return () => mq.removeEventListener('change', onSystemChange);
});
```

---

## 11.6 `global.css` scrollbar colours in light mode

The scrollbar thumb uses `--border` and `--text-muted`, both already defined
in `latte.css` as semantic aliases — scrollbar automatically adopts correct
light-mode colours. No change needed.

---

## Verification checklist

- [x] Selecting **Dark** applies Catppuccin Mocha immediately; no page reload
- [x] Selecting **Light** applies Catppuccin Latte immediately; no page reload
- [x] Selecting **System Default** follows the OS preference in real time
- [x] Theme choice persists across app restarts (stored in `config.json` under key `"theme"`)
- [x] First launch with no saved preference defaults to **System Default** (resolves to OS setting)
- [?] OS switches from light → dark (or vice versa) while app is open and **System Default** is active — app follows immediately
- [x] All semantic colour variables render correctly in Latte
- [x] Focus rings, scrollbars, toasts, and badges all look correct in both themes
- [x] `pnpm tsc --noEmit` passes with no type errors
- [x] `cargo tauri dev` starts cleanly (no Rust changes required)
