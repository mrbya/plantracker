# Phase 12 — Quality of Life

---

## 12.1 Searchable Task / Plan Dropdowns

> Pure frontend change. No Rust commands, no database migrations, no new Tauri
> plugins. The existing `Select.svelte` is left untouched — a new
> `SearchableSelect.svelte` combobox primitive replaces the task (and plan)
> `<Select>` in the three views that contain them.

---

### Why a custom combobox instead of extending `Select.svelte`

A native `<select>` element cannot be filtered: the browser owns the dropdown
and exposes no hook to hide individual `<option>` elements in response to user
input. A combobox — a visible text input that controls a custom listbox panel —
is the only cross-platform way to add live filtering. The new component must be
built from scratch but should match the existing `Select.svelte` prop interface
exactly so that callers change only the component name, not the surrounding
logic.

---

### 12.1.1 `SearchableSelect.svelte` — new UI primitive

Create `src/lib/components/ui/SearchableSelect.svelte`.

**Prop interface** (mirrors `Select.svelte` to allow drop-in replacement):

```typescript
let {
    options,
    value = $bindable(''),
    placeholder,
    id,
    onchange,
    disabled = false,
}: {
    options: { value: string; label: string }[];
    value?: string;
    placeholder?: string;
    id?: string;
    onchange?: () => void;
    disabled?: boolean;
} = $props();
```

**Internal state:**

```typescript
let open      = $state(false);
let query     = $state('');
let activeIdx = $state(-1);    // keyboard-highlighted index into filteredOptions
let inputEl   = $state<HTMLInputElement | null>(null);
let listEl    = $state<HTMLUListElement | null>(null);
```

**Derived values:**

```typescript
// Label shown in the input when a value is selected and the dropdown is closed.
const selectedLabel = $derived(
    options.find((o) => o.value === value)?.label ?? ''
);

// Case-insensitive substring match against the current query.
const filteredOptions = $derived(
    query.trim() === ''
        ? options
        : options.filter((o) =>
              o.value === ''     // always show header/separator options (e.g. "No specific task")
              || o.label.toLowerCase().includes(query.toLowerCase())
          )
);
```

**Behaviour spec:**

| Trigger | Result |
|---|---|
| Click input (closed) | Open panel, select all input text so user can start typing immediately |
| Type in input | Filter `options` by `label`; reset `activeIdx` to `0` if results exist, `-1` otherwise |
| `ArrowDown` | Open if closed; move `activeIdx` down (wraps); scroll active item into view |
| `ArrowUp` | Move `activeIdx` up (wraps); scroll into view |
| `Enter` / `Tab` | Select `filteredOptions[activeIdx]` if valid; close; call `onchange` |
| `Escape` | Close and restore `query` to `''` without changing `value` |
| Click option | Select it; close; call `onchange` |
| Click outside | Close; restore `query` to `''` |
| Close (any reason) | Set `query = ''`; `activeIdx = -1`; input displays `selectedLabel` |

When the panel is open, the input shows `query` (what the user is typing).
When closed, the input shows `selectedLabel` (the label of the selected option,
or the placeholder string if nothing is selected). This is achieved by binding
`input.value` imperatively in a `$effect`, not via `bind:value`, because the
two states need different sources.

**Accessibility:**

```svelte
<div class="combobox-wrapper">
  <input
    {id}
    role="combobox"
    aria-expanded={open}
    aria-controls="listbox-{id}"
    aria-activedescendant={activeIdx >= 0 ? `opt-{id}-{activeIdx}` : undefined}
    aria-autocomplete="list"
    autocomplete="off"
    {disabled}
    ...
  />
  <ul
    id="listbox-{id}"
    role="listbox"
    bind:this={listEl}
    hidden={!open}
  >
    {#each filteredOptions as opt, i (opt.value)}
      <li
        id="opt-{id}-{i}"
        role="option"
        aria-selected={opt.value === value}
        class:active={i === activeIdx}
        onmousedown={(e) => { e.preventDefault(); selectOption(opt); }}
      >
        {opt.label}
      </li>
    {/each}
  </ul>
</div>
```

Use `onmousedown` + `e.preventDefault()` on the list items so the `onblur` on
the input fires _after_ the selection is registered, not before (a classic
combobox pitfall where blur closes the dropdown before the click is processed).

**Click-outside handling:**

Use a Svelte action (`use:clickOutside`) rather than a global `document`
listener attached in `$effect`, to avoid stale-closure bugs:

```typescript
// src/lib/actions/clickOutside.ts
export function clickOutside(
    node: HTMLElement,
    handler: () => void
): { destroy(): void } {
    const onClick = (e: MouseEvent) => {
        if (!node.contains(e.target as Node)) handler();
    };
    document.addEventListener('mousedown', onClick, true);
    return { destroy: () => document.removeEventListener('mousedown', onClick, true) };
}
```

Apply it on the wrapper `<div>`:

```svelte
<div class="combobox-wrapper" use:clickOutside={close}>
```

**Styling rules:**

- Input: identical padding, border, border-radius, font, and focus ring as `Select.svelte`'s `<select>` — users should not notice a visual difference in the resting state
- The chevron icon on the right is the same inline SVG data URI as `Select.svelte`; rotate it `180deg` when `open`
- Panel (`<ul>`): `position: absolute`, full width of the wrapper, `z-index: 50`, `background: var(--bg-raised)`, `border: 1px solid var(--border)`, `border-radius: var(--radius)`, `max-height: 220px`, `overflow-y: auto`, thin scrollbar styles matching `global.css`
- Each `<li>`: `padding: 0.45rem 0.75rem`, `cursor: pointer`, `color: var(--text)`, `list-style: none`
- `:hover` and `.active` on `<li>`: `background: var(--bg-input)`
- `aria-selected="true"` on `<li>`: accent-colored left border (`3px solid var(--accent)`) + `padding-left: calc(0.75rem - 3px)`
- Empty state inside panel (no matches): single `<li class="no-results">` with `color: var(--text-muted)` and `pointer-events: none`
- The wrapper `<div>` must be `position: relative` so the panel positions correctly

**Files to create:**
- `src/lib/components/ui/SearchableSelect.svelte`
- `src/lib/actions/clickOutside.ts` (new `actions/` directory)

---

### 12.1.2 Wire `SearchableSelect` into `TimeTracking.svelte`

Replace the plan and task `<Select>` components with `<SearchableSelect>`.
All props (`id`, `options`, `bind:value`, `placeholder`, `onchange`) remain
identical; only the component name changes.

---

### 12.1.3 Wire `SearchableSelect` into `ManualEntry.svelte`

Same substitution as 12.1.2. Replace both `<Select>` components (plan and task)
with `<SearchableSelect>`. Props unchanged.

---

### 12.1.4 Wire `SearchableSelect` into `Reports.svelte`

Same substitution for the plan and task filter dropdowns. The Reports view uses
prepended "All plans" / "All tasks" options as selectable entries;
`SearchableSelect` handles these correctly because the filter matches against
`label`, and an empty query shows all options unchanged.

---

### 12.1.5 Update barrel export (if it exists)

If `src/lib/components/ui/index.ts` exists, add:

```typescript
export { default as SearchableSelect } from './SearchableSelect.svelte';
```

---

### 12.1 Verification checklist

**Functional — plan dropdown**
- [x] Clicking the plan dropdown opens the panel immediately
- [x] Typing a partial plan name filters the list to matching plans only
- [x] Clearing the query restores the full plan list
- [x] Selecting a plan closes the panel and shows the plan name in the input
- [x] After selection, re-opening the panel shows all plans (query is cleared)

**Functional — task dropdown**
- [x] Typing a partial task name filters within the current plan's tasks
- [x] "No specific task" entry always appears in the task list (even when query is active) — it is never filtered out
- [x] Selecting a task closes the panel, shows the task label, and triggers `onTaskChange`
- [x] With no plan selected, the task dropdown remains disabled / shows placeholder

**Keyboard navigation**
- [x] `ArrowDown` on the closed input opens the panel and highlights the first option
- [x] `ArrowDown` / `ArrowUp` cycle through `filteredOptions`; the highlighted item scrolls into view
- [x] `Enter` selects the highlighted option and closes the panel
- [x] `Tab` selects the highlighted option (if any) and closes the panel
- [x] `Escape` closes the panel without changing the selected value

**Edge cases**
- [x] Query that matches nothing shows "No results" text inside the panel
- [?] A plan with 50+ tasks is still performant — filtering is synchronous and derived, no debounce required at this data scale
- [x] Clicking outside the open panel closes it without selecting anything

**Visual / theme**
- [x] Resting state (closed, value selected) is visually identical to the existing `Select.svelte`
- [x] Resting state (closed, no value) shows placeholder in muted colour
- [x] Chevron rotates when the panel is open
- [x] Focused input shows the standard `2px solid var(--accent)` focus ring
- [x] Active (keyboard-highlighted) and selected options are visually distinct
- [x] Panel and items render correctly in both Mocha (dark) and Latte (light) themes

**Regression — unmodified components**
- [x] Settings view dropdowns still use `Select.svelte` — unaffected
- [x] `Select.svelte` itself is unmodified
- [x] `pnpm tsc --noEmit` passes with no type errors
- [x] `pnpm svelte-check` passes with no errors or warnings

---

---

### 12.2 Multi-language locales (EN / SK / DE)

> Pure frontend change. No Rust commands, no database migrations. The locale
> preference is persisted via the existing `tauri-plugin-store` (`config.json`)
> and applied at startup before the first render. Three locales are shipped:
> **English** (default), **Slovak**, and **German**.

---

#### Why Paraglide JS

Paraglide 2.0 is SvelteKit's officially recommended i18n library. It is a
compiler that turns message files into **tree-shakable TypeScript functions**,
so unused locale strings are never bundled. It has no async waterfalls, is
fully type-safe with IDE autocomplete on message keys, and requires zero
runtime overhead. For a desktop app that ships all three locales inside the
binary, this is the right tradeoff: bundle size is small and lookups are
synchronous function calls.

The URL-based routing strategy used by Paraglide for web apps is irrelevant
here — PlanTracker is a single-window SPA with no URL routing. The locale
resolution strategy is **`cookie` + `baseLocale`**: a cookie named `locale` is
read on startup; if absent the base locale (`en`) is used. The locale is
switched by writing the cookie and calling `setLocale()` — no page reload is
required because Paraglide 2.0 supports reactive locale switching.

---

#### 12.2.1 Install and initialise

```bash
pnpm add @inlang/paraglide-js
```

Run the interactive init:

```bash
pnpx @inlang/paraglide-js@latest init
```

When prompted:
- Languages: `en`, `sk`, `de`
- Framework: SvelteKit (Vite plugin)
- Output directory: `./src/lib/paraglide` (default)

This creates:
- `project.inlang/` — inlang project config
- `messages/en.json`, `messages/sk.json`, `messages/de.json` — translation files
- Adds the compile step to `package.json` build scripts

---

#### 12.2.2 Wire the Vite plugin

Update `vite.config.ts`:

```typescript
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { paraglideVitePlugin } from '@inlang/paraglide-js';

export default defineConfig({
    plugins: [
        paraglideVitePlugin({
            project: './project.inlang',
            outdir:  './src/lib/paraglide',
            strategy: ['cookie', 'baseLocale'],
        }),
        sveltekit(),
    ],
});
```

`paraglideVitePlugin` must come **before** `sveltekit()` so the compiled
message modules are available when SvelteKit processes imports.

The `strategy` array `['cookie', 'baseLocale']` tells Paraglide to:
1. Read the locale from a cookie named `locale` on startup
2. Fall back to `en` (the base locale) if the cookie is absent or invalid

There is no `url` strategy because PlanTracker has no URL routing.

---

#### 12.2.3 Message files

All user-visible strings in the app are extracted into these three files.
Keys use `snake_case` namespaced by feature area. Interpolation uses the
`{variable}` syntax.

**`messages/en.json`** (base locale — authoritative):

```json
{
    "$schema": "https://inlang.com/schema/inlang-message-format",

    "nav_time_tracking":  "Time Tracking",
    "nav_manual_entry":   "Manual Entry",
    "nav_reports":        "Reports",
    "nav_settings":       "Settings",
    "nav_sign_out":       "Sign out",

    "login_title":        "PlanTracker",
    "login_tagline":      "Track time on Microsoft Planner tasks",
    "login_sign_in_btn":  "Sign in with Microsoft",

    "timer_start":        "Start Timer",
    "timer_stop":         "Stop — {elapsed}",
    "timer_running":      "Running…",

    "label_plan":         "Plan",
    "label_task":         "Task",
    "label_start":        "Start",
    "label_end":          "End",
    "label_duration":     "Duration",
    "label_notes":        "Notes",
    "label_no_task":      "No specific task",

    "placeholder_select_plan":  "Select a plan…",
    "placeholder_select_task":  "Select a task…",
    "placeholder_select_plan_first": "Select a plan first",

    "entries_title":      "Recent Entries",
    "entries_empty":      "No entries yet. Select a plan and start a timer.",
    "entries_loading":    "Loading…",
    "entries_delete_confirm": "Sure?",
    "entries_delete_cancel":  "Cancel",

    "manual_new_entry":   "New Entry",
    "manual_edit_entry":  "Edit Entry",
    "manual_save":        "Save Entry",
    "manual_cancel":      "Cancel",
    "manual_update":      "Update Entry",

    "reports_title":         "Reports",
    "reports_from":          "From",
    "reports_to":            "To",
    "reports_generate":      "Generate",
    "reports_export_csv":    "Export CSV",
    "reports_col_month":     "Month",
    "reports_col_hours":     "Hours",
    "reports_col_minutes":   "Minutes",
    "reports_grand_total":   "Grand Total",
    "reports_empty":         "No entries found for the selected range.",
    "reports_all_plans":     "All plans",
    "reports_all_tasks":     "All tasks",

    "settings_title":              "Settings",
    "settings_section_entries":    "Entries",
    "settings_section_sync":       "Sync",
    "settings_section_storage":    "Storage",
    "settings_section_account":    "Account",
    "settings_section_appearance": "Appearance",
    "settings_section_language":   "Language",

    "settings_entries_limit_label": "Recent Entries Limit",
    "settings_entries_limit_desc":  "Maximum number of entries shown in the tracking views.",

    "settings_sync_frequency_label": "Sync Frequency",
    "settings_sync_frequency_desc":  "How often to automatically sync plans and tasks from Microsoft.",
    "settings_sync_now":             "Sync Now",
    "settings_sync_last":            "Last synced: {time}",
    "settings_sync_never":           "Never synced",
    "settings_sync_manual":          "Manual only",
    "settings_sync_30min":           "Every 30 minutes",
    "settings_sync_1hour":           "Every hour",

    "settings_storage_label": "Data Directory",
    "settings_storage_desc":  "Location of the local database and configuration files.",
    "settings_open_folder":   "Open Folder",

    "settings_account_signed_in_as": "Signed in as",
    "settings_sign_out":             "Sign Out",

    "settings_theme_label":   "Theme",
    "settings_theme_desc":    "Controls the colour scheme of the application.",
    "settings_theme_dark":    "Dark",
    "settings_theme_light":   "Light",
    "settings_theme_system":  "System Default",

    "settings_language_label": "Language",
    "settings_language_desc":  "Language used throughout the application.",

    "toast_entry_saved":      "Entry saved",
    "toast_entry_updated":    "Entry updated",
    "toast_entry_deleted":    "Entry deleted",
    "toast_sync_success":     "Sync complete",
    "toast_export_saved":     "Report exported to {path}",
    "toast_sign_out_failed":  "Sign out failed: {error}",
    "toast_sign_in_failed":   "Sign in failed: {error}",
    "toast_sync_failed":      "Sync failed: {error}",
    "toast_load_failed":      "Failed to load entries: {error}",
    "toast_delete_failed":    "Failed to delete entry: {error}",
    "toast_timer_restored":   "Timer restored from previous session"
}
```

**`messages/sk.json`**:

```json
{
    "$schema": "https://inlang.com/schema/inlang-message-format",

    "nav_time_tracking":  "Sledovanie času",
    "nav_manual_entry":   "Manuálny záznam",
    "nav_reports":        "Reporty",
    "nav_settings":       "Nastavenia",
    "nav_sign_out":       "Odhlásiť sa",

    "login_title":        "PlanTracker",
    "login_tagline":      "Sledujte čas na úlohách Microsoft Planner",
    "login_sign_in_btn":  "Prihlásiť sa cez Microsoft",

    "timer_start":        "Spustiť časovač",
    "timer_stop":         "Zastaviť — {elapsed}",
    "timer_running":      "Beží…",

    "label_plan":         "Plán",
    "label_task":         "Úloha",
    "label_start":        "Začiatok",
    "label_end":          "Koniec",
    "label_duration":     "Trvanie",
    "label_notes":        "Poznámky",
    "label_no_task":      "Bez konkrétnej úlohy",

    "placeholder_select_plan":       "Vyberte plán…",
    "placeholder_select_task":       "Vyberte úlohu…",
    "placeholder_select_plan_first": "Najprv vyberte plán",

    "entries_title":          "Posledné záznamy",
    "entries_empty":          "Zatiaľ žiadne záznamy. Vyberte plán a spustite časovač.",
    "entries_loading":        "Načítavanie…",
    "entries_delete_confirm": "Naozaj?",
    "entries_delete_cancel":  "Zrušiť",

    "manual_new_entry":  "Nový záznam",
    "manual_edit_entry": "Upraviť záznam",
    "manual_save":       "Uložiť záznam",
    "manual_cancel":     "Zrušiť",
    "manual_update":     "Aktualizovať záznam",

    "reports_title":       "Reporty",
    "reports_from":        "Od",
    "reports_to":          "Do",
    "reports_generate":    "Generovať",
    "reports_export_csv":  "Exportovať CSV",
    "reports_col_month":   "Mesiac",
    "reports_col_hours":   "Hodiny",
    "reports_col_minutes": "Minúty",
    "reports_grand_total": "Celkový súčet",
    "reports_empty":       "Pre zvolený rozsah neboli nájdené žiadne záznamy.",
    "reports_all_plans":   "Všetky plány",
    "reports_all_tasks":   "Všetky úlohy",

    "settings_title":              "Nastavenia",
    "settings_section_entries":    "Záznamy",
    "settings_section_sync":       "Synchronizácia",
    "settings_section_storage":    "Úložisko",
    "settings_section_account":    "Účet",
    "settings_section_appearance": "Vzhľad",
    "settings_section_language":   "Jazyk",

    "settings_entries_limit_label": "Limit posledných záznamov",
    "settings_entries_limit_desc":  "Maximálny počet záznamov zobrazených v sledovacích pohľadoch.",

    "settings_sync_frequency_label": "Frekvencia synchronizácie",
    "settings_sync_frequency_desc":  "Ako často automaticky synchronizovať plány a úlohy z Microsoftu.",
    "settings_sync_now":             "Synchronizovať",
    "settings_sync_last":            "Posledná synchronizácia: {time}",
    "settings_sync_never":           "Ešte nesynchronizované",
    "settings_sync_manual":          "Len manuálne",
    "settings_sync_30min":           "Každých 30 minút",
    "settings_sync_1hour":           "Každú hodinu",

    "settings_storage_label": "Dátový adresár",
    "settings_storage_desc":  "Umiestnenie lokálnej databázy a konfiguračných súborov.",
    "settings_open_folder":   "Otvoriť priečinok",

    "settings_account_signed_in_as": "Prihlásený ako",
    "settings_sign_out":             "Odhlásiť sa",

    "settings_theme_label":  "Motív",
    "settings_theme_desc":   "Farebná schéma aplikácie.",
    "settings_theme_dark":   "Tmavý",
    "settings_theme_light":  "Svetlý",
    "settings_theme_system": "Podľa systému",

    "settings_language_label": "Jazyk",
    "settings_language_desc":  "Jazyk používaný v celej aplikácii.",

    "toast_entry_saved":     "Záznam uložený",
    "toast_entry_updated":   "Záznam aktualizovaný",
    "toast_entry_deleted":   "Záznam vymazaný",
    "toast_sync_success":    "Synchronizácia dokončená",
    "toast_export_saved":    "Report exportovaný do {path}",
    "toast_sign_out_failed": "Odhlásenie zlyhalo: {error}",
    "toast_sign_in_failed":  "Prihlásenie zlyhalo: {error}",
    "toast_sync_failed":     "Synchronizácia zlyhala: {error}",
    "toast_load_failed":     "Načítanie záznamov zlyhalo: {error}",
    "toast_delete_failed":   "Vymazanie záznamu zlyhalo: {error}",
    "toast_timer_restored":  "Časovač obnovený z predchádzajúcej relácie"
}
```

**`messages/de.json`**:

```json
{
    "$schema": "https://inlang.com/schema/inlang-message-format",

    "nav_time_tracking":  "Zeiterfassung",
    "nav_manual_entry":   "Manueller Eintrag",
    "nav_reports":        "Berichte",
    "nav_settings":       "Einstellungen",
    "nav_sign_out":       "Abmelden",

    "login_title":        "PlanTracker",
    "login_tagline":      "Zeit für Microsoft Planner-Aufgaben erfassen",
    "login_sign_in_btn":  "Mit Microsoft anmelden",

    "timer_start":        "Timer starten",
    "timer_stop":         "Stopp — {elapsed}",
    "timer_running":      "Läuft…",

    "label_plan":         "Plan",
    "label_task":         "Aufgabe",
    "label_start":        "Start",
    "label_end":          "Ende",
    "label_duration":     "Dauer",
    "label_notes":        "Notizen",
    "label_no_task":      "Keine spezifische Aufgabe",

    "placeholder_select_plan":       "Plan auswählen…",
    "placeholder_select_task":       "Aufgabe auswählen…",
    "placeholder_select_plan_first": "Zuerst Plan auswählen",

    "entries_title":          "Letzte Einträge",
    "entries_empty":          "Noch keine Einträge. Plan auswählen und Timer starten.",
    "entries_loading":        "Wird geladen…",
    "entries_delete_confirm": "Sicher?",
    "entries_delete_cancel":  "Abbrechen",

    "manual_new_entry":  "Neuer Eintrag",
    "manual_edit_entry": "Eintrag bearbeiten",
    "manual_save":       "Eintrag speichern",
    "manual_cancel":     "Abbrechen",
    "manual_update":     "Eintrag aktualisieren",

    "reports_title":       "Berichte",
    "reports_from":        "Von",
    "reports_to":          "Bis",
    "reports_generate":    "Generieren",
    "reports_export_csv":  "CSV exportieren",
    "reports_col_month":   "Monat",
    "reports_col_hours":   "Stunden",
    "reports_col_minutes": "Minuten",
    "reports_grand_total": "Gesamtsumme",
    "reports_empty":       "Für den gewählten Zeitraum wurden keine Einträge gefunden.",
    "reports_all_plans":   "Alle Pläne",
    "reports_all_tasks":   "Alle Aufgaben",

    "settings_title":              "Einstellungen",
    "settings_section_entries":    "Einträge",
    "settings_section_sync":       "Synchronisierung",
    "settings_section_storage":    "Speicher",
    "settings_section_account":    "Konto",
    "settings_section_appearance": "Erscheinungsbild",
    "settings_section_language":   "Sprache",

    "settings_entries_limit_label": "Limit für letzte Einträge",
    "settings_entries_limit_desc":  "Maximale Anzahl der in den Tracking-Ansichten angezeigten Einträge.",

    "settings_sync_frequency_label": "Synchronisierungsintervall",
    "settings_sync_frequency_desc":  "Wie oft Pläne und Aufgaben automatisch aus Microsoft synchronisiert werden.",
    "settings_sync_now":             "Jetzt synchronisieren",
    "settings_sync_last":            "Zuletzt synchronisiert: {time}",
    "settings_sync_never":           "Noch nicht synchronisiert",
    "settings_sync_manual":          "Nur manuell",
    "settings_sync_30min":           "Alle 30 Minuten",
    "settings_sync_1hour":           "Jede Stunde",

    "settings_storage_label": "Datenverzeichnis",
    "settings_storage_desc":  "Speicherort der lokalen Datenbank und Konfigurationsdateien.",
    "settings_open_folder":   "Ordner öffnen",

    "settings_account_signed_in_as": "Angemeldet als",
    "settings_sign_out":             "Abmelden",

    "settings_theme_label":  "Design",
    "settings_theme_desc":   "Farbschema der Anwendung.",
    "settings_theme_dark":   "Dunkel",
    "settings_theme_light":  "Hell",
    "settings_theme_system": "Systemstandard",

    "settings_language_label": "Sprache",
    "settings_language_desc":  "In der gesamten Anwendung verwendete Sprache.",

    "toast_entry_saved":     "Eintrag gespeichert",
    "toast_entry_updated":   "Eintrag aktualisiert",
    "toast_entry_deleted":   "Eintrag gelöscht",
    "toast_sync_success":    "Synchronisierung abgeschlossen",
    "toast_export_saved":    "Bericht exportiert nach {path}",
    "toast_sign_out_failed": "Abmeldung fehlgeschlagen: {error}",
    "toast_sign_in_failed":  "Anmeldung fehlgeschlagen: {error}",
    "toast_sync_failed":     "Synchronisierung fehlgeschlagen: {error}",
    "toast_load_failed":     "Einträge konnten nicht geladen werden: {error}",
    "toast_delete_failed":   "Eintrag konnte nicht gelöscht werden: {error}",
    "toast_timer_restored":  "Timer aus vorheriger Sitzung wiederhergestellt"
}
```

---

#### 12.2.4 Locale store (`src/lib/stores/locale.ts`)

Wraps Paraglide's `setLocale` / `getLocale` with persistence to `config.json`.
This store has no Svelte writable — `setLocale` is already reactive. The store
only handles the persistence layer.

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

`setLocale` from Paraglide is synchronous and immediately reactive — all `m.*`
calls in components re-evaluate on the next render tick. No page reload is
needed.

---

#### 12.2.5 Startup wiring

In `src/routes/+page.svelte`, add `loadLocale()` as the **first** call in
`onMount`, before `loadTheme()`. Locale must be applied before any strings are
rendered to avoid a flash of English text on non-English locales.

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

---

#### 12.2.6 Rework all views and components

Replace every hardcoded UI string with a `m.*` call. Import `m` from
`$lib/paraglide/messages` at the top of each file.

```typescript
import * as m from '$lib/paraglide/messages';
```

**`src/lib/components/Layout.svelte`**

The `navItems` array already has a `label` field used for `title` tooltip
attributes. Replace the hardcoded strings:

```typescript
// Before:
const navItems = [
    { id: 'time-tracking', icon: Timer,       label: 'Time Tracking' },
    { id: 'manual-entry',  icon: PencilLine,  label: 'Manual Entry'  },
    { id: 'reports',       icon: BarChart2,   label: 'Reports'       },
    { id: 'settings',      icon: SettingsIcon,label: 'Settings'      },
];

// After — labels become derived getters so they re-evaluate on locale change:
const navItems = $derived([
    { id: 'time-tracking', icon: Timer,        label: m.nav_time_tracking() },
    { id: 'manual-entry',  icon: PencilLine,   label: m.nav_manual_entry()  },
    { id: 'reports',       icon: BarChart2,    label: m.nav_reports()       },
    { id: 'settings',      icon: SettingsIcon, label: m.nav_settings()      },
]);
```

Sign-out button title: `title={m.nav_sign_out()}`.

**`src/views/Login.svelte`**

```svelte
<h1>{m.login_title()}</h1>
<p>{m.login_tagline()}</p>
<Button ...>{m.login_sign_in_btn()}</Button>
```

**`src/views/TimeTracking.svelte`**

| Current string | Key |
|---|---|
| `"Plan"` (label) | `m.label_plan()` |
| `"Task"` (label) | `m.label_task()` |
| `"Select a plan…"` | `m.placeholder_select_plan()` |
| `"Select a plan first"` | `m.placeholder_select_plan_first()` |
| `"Start Timer"` | `m.timer_start()` |
| `` `Stop — ${formatDuration($elapsedSeconds)}` `` | `m.timer_stop({ elapsed: formatDuration($elapsedSeconds) })` |
| `"Recent Entries"` | `m.entries_title()` |
| `"No entries yet…"` | `m.entries_empty()` |
| `"Loading…"` | `m.entries_loading()` |
| `"Running…"` | `m.timer_running()` |
| `"No specific task"` | `m.label_no_task()` |
| `"Sure?"` | `m.entries_delete_confirm()` |
| `"Cancel"` | `m.entries_delete_cancel()` |
| `"Task"` (table header) | `m.label_task()` |
| `"Plan"` (table header) | `m.label_plan()` |
| `"Start"` (table header) | `m.label_start()` |
| `"End"` (table header) | `m.label_end()` |
| `"Duration"` (table header) | `m.label_duration()` |

**`src/views/ManualEntry.svelte`** — same label/placeholder keys as above, plus:

| Current string | Key |
|---|---|
| `"New Entry"` | `m.manual_new_entry()` |
| `"Edit Entry"` | `m.manual_edit_entry()` |
| `"Save Entry"` | `m.manual_save()` |
| `"Update Entry"` | `m.manual_update()` |
| `"Cancel"` | `m.manual_cancel()` |
| `"Notes"` (label) | `m.label_notes()` |

**`src/views/Reports.svelte`**

| Current string | Key |
|---|---|
| `"From"` | `m.reports_from()` |
| `"To"` | `m.reports_to()` |
| `"Generate"` | `m.reports_generate()` |
| `"Export CSV"` | `m.reports_export_csv()` |
| `"All plans"` | `m.reports_all_plans()` |
| `"All tasks"` | `m.reports_all_tasks()` |
| `"Month"` (col) | `m.reports_col_month()` |
| `"Hours"` (col) | `m.reports_col_hours()` |
| `"Minutes"` (col) | `m.reports_col_minutes()` |
| `"Grand Total"` | `m.reports_grand_total()` |
| EmptyState message | `m.reports_empty()` |

**`src/views/Settings.svelte`**

Replace all section titles, labels, descriptions, button text, and the
`<Select>` option labels for sync frequency and theme. The full mapping follows
the key names defined in section 12.2.3 and is one-to-one with the existing
hardcoded strings.

**`src/lib/stores/notifications.ts`** and call-sites

`addSuccess`, `addError`, `addWarning` accept plain strings. All callers that
currently pass hardcoded English strings must be updated to pass `m.*()` calls
instead. The notification store itself is unchanged — it does not import `m`.

Example — in `planner.ts`:
```typescript
// Before:
addSuccess('Sync complete');
addError('Sync failed: ' + e);

// After:
addSuccess(m.toast_sync_success());
addError(m.toast_sync_failed({ error: String(e) }));
```

Apply the same pattern in `timer.ts`, `auth.ts`, and all views that call the
notification helpers directly.

---

#### 12.2.7 Language selector in Settings

Add a new **Language** section to `src/views/Settings.svelte`, positioned after
the Appearance section. Use the existing `setting-row` layout.

```typescript
import { saveLocale, LOCALE_LABELS, type AppLocale } from '$lib/stores/locale';
import { getLocale } from '$lib/paraglide/runtime';

const localeOptions = Object.entries(LOCALE_LABELS).map(([value, label]) => ({
    value,
    label,
}));

// Reactive: re-reads on locale change so the dropdown reflects the active locale.
const currentLocale = $derived(getLocale() as AppLocale);
```

Template:

```svelte
<section class="settings-section">
  <h2 class="section-title">{m.settings_section_language()}</h2>
  <div class="setting-row">
    <div class="setting-info">
      <span class="setting-label">{m.settings_language_label()}</span>
      <span class="setting-desc">{m.settings_language_desc()}</span>
    </div>
    <div class="setting-control">
      <Select
        options={localeOptions}
        value={currentLocale}
        onchange={(e) =>
          saveLocale((e.target as HTMLSelectElement).value as AppLocale)
        }
      />
    </div>
  </div>
</section>
```

---

#### 12.2.8 Justfile recipe

```just
# Extract and compile message files (run after adding new keys)
i18n:
    pnpx @inlang/paraglide-js compile \
        --project ./project.inlang \
        --outdir ./src/lib/paraglide
```

Add `i18n` as a dependency of `build` so the compiled message modules are
always up-to-date before a release build:

```just
build: i18n
    cargo tauri build
```

---

#### 12.2.9 `.gitignore` addition

The compiled output in `src/lib/paraglide/` is **generated** — it should not
be committed. Add to `.gitignore`:

```
src/lib/paraglide/
```

The `pnpm build` script (and the new `just i18n` recipe) regenerates it from
the source message files. CI must run `just i18n` before `just build`.

Update `.gitlab-ci.yml` `build` job script to include the compile step:

```yaml
script:
  - just i18n
  - just ci-build
```

---

### Verification checklist

**Locale switching**
- [ ] Changing the language in Settings → Language immediately updates all visible strings without a page reload or app restart
- [ ] The selected language persists across app restarts (stored under key `"locale"` in `config.json`)
- [ ] First launch with no saved locale defaults to English
- [ ] All three locales (EN / SK / DE) are selectable and render correctly

**String coverage**
- [ ] No hardcoded English strings remain in any `.svelte` file or store
- [ ] All toast messages respect the active locale
- [ ] All table column headers respect the active locale
- [ ] All button labels, placeholders, and section titles respect the active locale
- [ ] The interpolated strings (`timer_stop`, `settings_sync_last`, `toast_export_saved`, error toasts) render the dynamic values correctly in all three locales

**Slovak locale spot-check**
- [ ] Login screen shows "Prihlásiť sa cez Microsoft"
- [ ] Time Tracking shows "Sledovanie času" in the sidebar tooltip
- [ ] Settings sync section shows "Synchronizovať" on the button
- [ ] A delete confirmation shows "Naozaj?"

**German locale spot-check**
- [ ] Login screen shows "Mit Microsoft anmelden"
- [ ] Time Tracking shows "Zeiterfassung" in the sidebar tooltip
- [ ] Reports grand total shows "Gesamtsumme"
- [ ] A delete confirmation shows "Sicher?"

**Build**
- [ ] `just i18n` runs without errors and regenerates `src/lib/paraglide/`
- [ ] `pnpm tsc --noEmit` passes — Paraglide's generated types are present
- [ ] `pnpm svelte-check` passes with no errors or warnings
- [ ] `cargo tauri build` succeeds end-to-end (Paraglide is frontend-only; Rust is unaffected)
- [ ] CI `build` job runs `just i18n` before compiling
