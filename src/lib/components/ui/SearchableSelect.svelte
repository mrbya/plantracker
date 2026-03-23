<script lang="ts">
  import { clickOutside } from "$lib/actions/clickOutside";

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

  let open = $state(false);
  let query = $state("");
  let activeIdx = $state(-1);
  let inputEl = $state<HTMLInputElement | null>(null);
  let listEl = $state<HTMLUListElement | null>(null);

  const selectedLabel = $derived(
    options.find((o) => o.value === value)?.label ?? "",
  );

  const filteredOptions = $derived(
    query.trim() === ""
      ? options
      : options.filter(
          (o) =>
            o.value === "" || // never filter out "No specific task" / "All tasks"
            o.label.toLowerCase().includes(query.toLowerCase()),
        ),
  );

  // When closed, keep the input showing the selected label (or placeholder).
  $effect(() => {
    if (!inputEl || open) return;
    inputEl.value = selectedLabel || placeholder || "";
  });

  function openPanel() {
    if (disabled || open) return;
    open = true;
    activeIdx = filteredOptions.length > 0 ? 0 : -1;
    if (inputEl) {
      inputEl.value = "";
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
      activeIdx =
        (activeIdx - 1 + filteredOptions.length) % filteredOptions.length;
      scrollActiveIntoView();
      e.preventDefault();
    } else if (e.key === "Enter" || e.key === "Tab") {
      if (activeIdx >= 0 && filteredOptions[activeIdx]) {
        selectOption(filteredOptions[activeIdx]);
        if (e.key === "Tab") return; // let Tab move focus naturally
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
    const item = listEl.querySelectorAll("[role=option]")[activeIdx] as
      | HTMLElement
      | undefined;
    item?.scrollIntoView({ block: "nearest" });
  }
</script>

<div class="combobox-wrapper" class:open use:clickOutside={close}>
  <input
    bind:this={inputEl}
    {id}
    type="text"
    role="combobox"
    aria-expanded={open}
    aria-controls={id ? `listbox-${id}` : undefined}
    aria-activedescendant={activeIdx >= 0 && id
      ? `opt-${id}-${activeIdx}`
      : undefined}
    aria-autocomplete="list"
    autocomplete="off"
    {disabled}
    onfocus={openPanel}
    onclick={openPanel}
    oninput={onInput}
    onkeydown={onKeydown}
    class:placeholder-text={!value && !open}
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
        onmousedown={(e) => {
          e.preventDefault();
          selectOption(opt);
        }}
      >
        {opt.label}
      </li>
    {:else}
      <li
        class="no-results"
        role="option"
        aria-selected={false}
        aria-disabled="true"
      >
        No results
      </li>
    {/each}
  </ul>
</div>

<style>
  .combobox-wrapper {
    position: relative;
    width: 100%;
  }

  /* Chevron icon — rotates when panel is open */
  .combobox-wrapper::after {
    content: "";
    position: absolute;
    right: 0.6rem;
    top: 50%;
    transform: translateY(-50%);
    width: 12px;
    height: 8px;
    background: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'%3E%3Cpath fill='%23a6adc8' d='M1 1l5 5 5-5'/%3E%3C/svg%3E")
      no-repeat center;
    pointer-events: none;
    transition: transform 0.15s;
  }

  .combobox-wrapper.open::after {
    transform: translateY(-50%) rotate(180deg);
  }

  input {
    appearance: none;
    -webkit-appearance: none;
    width: 100%;
    padding: 0.45rem 2rem 0.45rem 0.75rem;
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text);
    font-family: var(--font);
    font-size: var(--font-size-base);
    cursor: pointer;
    transition: border-color 0.15s;
  }

  input:hover {
    border-color: var(--text-muted);
  }

  input:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  input.placeholder-text {
    color: var(--text-muted);
  }

  ul {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    z-index: 50;
    background: var(--bg-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    max-height: 220px;
    overflow-y: auto;
    margin-top: 2px;
    padding: 0;
    list-style: none;
    scrollbar-width: thin;
    scrollbar-color: var(--border) var(--bg-raised);
  }

  ul::-webkit-scrollbar {
    width: 6px;
  }

  ul::-webkit-scrollbar-track {
    background: var(--bg-raised);
  }

  ul::-webkit-scrollbar-thumb {
    background: var(--border);
    border-radius: var(--radius);
  }

  ul::-webkit-scrollbar-thumb:hover {
    background: var(--text-muted);
  }

  li {
    padding: 0.45rem 0.75rem;
    cursor: pointer;
    color: var(--text);
    list-style: none;
  }

  li:hover,
  li.active {
    background: var(--bg-input);
  }

  li[aria-selected="true"] {
    border-left: 3px solid var(--accent);
    padding-left: calc(0.75rem - 3px);
  }

  li.no-results {
    color: var(--text-muted);
    pointer-events: none;
    cursor: default;
  }
</style>
