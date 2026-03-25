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
