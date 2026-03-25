---
id: TASK-60
title: Phase 12.1.1 — SearchableSelect component and clickOutside action
status: Done
assignee: []
created_date: '2026-03-23 20:00'
updated_date: '2026-03-23 20:06'
labels:
  - frontend
  - ui-component
  - phase-12
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create two new files: `src/lib/actions/clickOutside.ts` (a Svelte action) and `src/lib/components/ui/SearchableSelect.svelte` (a combobox primitive that replaces native `<select>` where filtering is needed).

## Why this exists

Native `<select>` cannot be filtered — the browser owns the dropdown. Plans and task lists can be large (30+ items). A combobox (text input + custom listbox panel) is the only cross-platform way to add live filtering. The existing `Select.svelte` is **not modified** — it stays for settings dropdowns where filtering is unnecessary.

## File 1: `src/lib/actions/clickOutside.ts`

Create the directory `src/lib/actions/` (it does not exist yet).

```typescript
// src/lib/actions/clickOutside.ts
export function clickOutside(
  node: HTMLElement,
  handler: () => void
): { destroy(): void } {
  const onClick = (e: MouseEvent) => {
    if (!node.contains(e.target as Node)) handler();
  };
  document.addEventListener("mousedown", onClick, true);
  return { destroy: () => document.removeEventListener("mousedown", onClick, true) };
}
```

Use `mousedown` (not `click`) with the capture phase (`true`) so it fires before the input's blur event.

## File 2: `src/lib/components/ui/SearchableSelect.svelte`

### Prop interface (mirrors `Select.svelte` exactly, plus `disabled`)

```typescript
let {
  options,
  value = $bindable(""),
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

### Internal state

```typescript
let open      = $state(false);
let query     = $state("");
let activeIdx = $state(-1);
let inputEl   = $state<HTMLInputElement | null>(null);
let listEl    = $state<HTMLUListElement | null>(null);
```

### Derived values

```typescript
const selectedLabel = $derived(
  options.find((o) => o.value === value)?.label ?? ""
);

const filteredOptions = $derived(
  query.trim() === ""
    ? options
    : options.filter(
        (o) =>
          o.value === "" || // always show "No specific task" / "All tasks" header entries
          o.label.toLowerCase().includes(query.toLowerCase())
      )
);
```

### Input value management

Do **not** use `bind:value` on the `<input>`. Instead, manage it imperatively in a `$effect` so the two states (showing `query` when open, showing `selectedLabel` when closed) can come from different sources:

```typescript
$effect(() => {
  if (!inputEl) return;
  inputEl.value = open ? query : (selectedLabel || placeholder || "");
});
```

### Event handlers

```typescript
function openPanel() {
  if (disabled) return;
  open = true;
  activeIdx = filteredOptions.length > 0 ? 0 : -1;
  if (inputEl) {
    inputEl.value = query;
    inputEl.select();
  }
}

function close() {
  open = false;
  query = "";
  activeIdx = -1;
}

function selectOption(opt: { value: string; label: string }) {
  value = opt.value;
  close();
  onchange?.();
}

function onInput(e: Event) {
  query = (e.target as HTMLInputElement).value;
  activeIdx = filteredOptions.length > 0 ? 0 : -1;
}

function onKeydown(e: KeyboardEvent) {
  if (disabled) return;
  if (!open && (e.key === "ArrowDown" || e.key === "Enter")) {
    openPanel();
    e.preventDefault();
    return;
  }
  if (!open) return;
  if (e.key === "ArrowDown") {
    activeIdx = (activeIdx + 1) % filteredOptions.length;
    scrollActiveIntoView();
    e.preventDefault();
  } else if (e.key === "ArrowUp") {
    activeIdx = (activeIdx - 1 + filteredOptions.length) % filteredOptions.length;
    scrollActiveIntoView();
    e.preventDefault();
  } else if (e.key === "Enter" || e.key === "Tab") {
    if (activeIdx >= 0 && filteredOptions[activeIdx]) {
      selectOption(filteredOptions[activeIdx]);
      if (e.key === "Tab") return; // allow Tab to move focus after selection
      e.preventDefault();
    } else {
      close();
    }
  } else if (e.key === "Escape") {
    close();
    e.preventDefault();
  }
}

function scrollActiveIntoView() {
  if (!listEl || activeIdx < 0) return;
  const item = listEl.querySelectorAll("[role=option]")[activeIdx] as HTMLElement | undefined;
  item?.scrollIntoView({ block: "nearest" });
}
```

### Template

```svelte
<script lang="ts">
  import { clickOutside } from "$lib/actions/clickOutside";
  // ... (all the state and handlers above)
</script>

<div
  class="combobox-wrapper"
  class:open
  use:clickOutside={close}
>
  <input
    bind:this={inputEl}
    {id}
    type="text"
    role="combobox"
    aria-expanded={open}
    aria-controls={id ? `listbox-${id}` : undefined}
    aria-activedescendant={activeIdx >= 0 && id ? `opt-${id}-${activeIdx}` : undefined}
    aria-autocomplete="list"
    autocomplete="off"
    {disabled}
    onfocus={openPanel}
    oninput={onInput}
    onkeydown={onKeydown}
    class:placeholder={!value && !open}
  />
  <ul
    id={id ? `listbox-${id}` : undefined}
    role="listbox"
    bind:this={listEl}
    hidden={!open}
  >
    {#each filteredOptions as opt, i (opt.value)}
      <li
        id={id ? `opt-${id}-${i}` : undefined}
        role="option"
        aria-selected={opt.value === value}
        class:active={i === activeIdx}
        onmousedown={(e) => { e.preventDefault(); selectOption(opt); }}
      >
        {opt.label}
      </li>
    {:else}
      <li class="no-results" role="option" aria-selected={false}>No results</li>
    {/each}
  </ul>
</div>
```

**Critical:** use `onmousedown` + `e.preventDefault()` on list items — this prevents the input's `onblur` from firing before the click is processed, which would otherwise close the panel before the selection is registered.

### Styling

Match `Select.svelte`'s resting appearance exactly (same padding, border, border-radius, font, colors). Use CSS custom properties only — no hardcoded hex values.

Key styles:
- `.combobox-wrapper`: `position: relative; width: 100%`
- `input`: identical to `select` in `Select.svelte` — `appearance: none`, same padding (`0.45rem 2rem 0.45rem 0.75rem`), same background with chevron SVG data URI (use `--ctp-subtext0` color, same as `Select.svelte`), border, border-radius, font, color, cursor
- `input.open` (or `.combobox-wrapper.open input`): rotate chevron 180deg
- `input.placeholder`: `color: var(--text-muted)`
- `input:focus-visible`: `outline: 2px solid var(--accent); outline-offset: 2px`
- `input:disabled`: `opacity: 0.5; cursor: not-allowed`
- `ul`: `position: absolute; top: 100%; left: 0; right: 0; z-index: 50; background: var(--bg-raised); border: 1px solid var(--border); border-radius: var(--radius); max-height: 220px; overflow-y: auto; margin-top: 2px; padding: 0`
- `li`: `padding: 0.45rem 0.75rem; cursor: pointer; color: var(--text); list-style: none`
- `li:hover`, `li.active`: `background: var(--bg-input)`
- `li[aria-selected="true"]`: `border-left: 3px solid var(--accent); padding-left: calc(0.75rem - 3px)`
- `li.no-results`: `color: var(--text-muted); pointer-events: none; cursor: default`
- Thin scrollbar on `ul`: same pattern as `global.css` (use `:global` or duplicate the rules scoped to `ul`)

## Checklist before marking done

- `pnpm tsc --noEmit` passes with no type errors
- `pnpm svelte-check` passes with no errors or warnings
- Component is usable in isolation (can be tested by temporarily dropping it into any view)
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 File `src/lib/actions/clickOutside.ts` is created; it exports a `clickOutside` Svelte action that attaches a `mousedown` listener in the capture phase and removes it on `destroy()`
- [x] #2 File `src/lib/components/ui/SearchableSelect.svelte` is created with the prop interface: `options`, `value` ($bindable, default `""`), `placeholder`, `id`, `onchange`, `disabled` (default `false`)
- [x] #3 Clicking the input opens the panel and selects all input text
- [x] #4 Typing in the open input filters `filteredOptions` by case-insensitive substring match on `label`
- [x] #5 Options with `value === ""` (header entries like "No specific task" / "All tasks") are never filtered out regardless of query
- [x] #6 ArrowDown/ArrowUp navigate the highlighted item and scroll it into view; wrapping at the list boundaries
- [x] #7 Enter and Tab select the highlighted option (if any) and close the panel; Tab also moves focus normally
- [x] #8 Escape closes the panel and restores the input to `selectedLabel` without changing `value`
- [x] #9 Clicking a list item selects it — `onmousedown + e.preventDefault()` is used so blur does not close the panel before the click is registered
- [x] #10 Clicking outside the open panel closes it without changing `value`
- [x] #11 When closed, input displays `selectedLabel`; when open, displays the current `query`
- [x] #12 Empty query shows all options (no filtering)
- [x] #13 Zero-match query shows a "No results" item (non-interactive, muted colour)
- [x] #14 The `disabled` prop disables the input and prevents opening (visual opacity change)
- [x] #15 `$effect` manages `inputEl.value` imperatively (no `bind:value` on the `<input>`)
- [x] #16 Resting state (closed, value selected) is visually indistinguishable from `Select.svelte`
- [x] #17 Chevron rotates 180deg when the panel is open
- [x] #18 Panel uses correct theme variables: `--bg-raised`, `--border`, `--radius`, `--bg-input`, `--accent`, `--text`, `--text-muted`
- [x] #19 Selected option (`aria-selected=true`) has an accent-coloured left border
- [x] #20 Active (keyboard-highlighted) option has `--bg-input` background
- [x] #21 `pnpm tsc --noEmit` passes; `pnpm svelte-check` passes
<!-- AC:END -->
