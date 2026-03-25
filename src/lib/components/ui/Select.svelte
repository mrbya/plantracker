<script lang="ts">
  /**
   * Styled native `<select>` wrapper.
   *
   * Renders a plain HTML `<select>` element with Catppuccin Mocha theming and
   * a custom chevron icon injected via a CSS background image.  Use this
   * component for short, fixed option lists.  For long or searchable lists use
   * `SearchableSelect.svelte` instead.
   *
   * Props:
   * - `options`     — array of `{ value, label }` pairs rendered as `<option>`
   *                   elements.
   * - `value`       — bindable currently selected value string.
   * - `placeholder` — optional label for a disabled, hidden "Select…" option
   *                   shown when no value is selected.
   * - `id`          — forwarded to the underlying `<select>` for label
   *                   association.
   * - `onchange`    — callback fired after the user selects a new option.
   */
  let {
    options,
    value = $bindable(""),
    placeholder,
    id,
    onchange,
  }: {
    options: { value: string; label: string }[];
    value?: string;
    placeholder?: string;
    id?: string;
    onchange?: () => void;
  } = $props();
</script>

<select {id} bind:value {onchange}>
  {#if placeholder}
    <option value="" disabled hidden>{placeholder}</option>
  {/if}
  {#each options as opt (opt.value)}
    <option value={opt.value}>{opt.label}</option>
  {/each}
</select>

<style>
  select {
    appearance: none;
    -webkit-appearance: none;
    width: 100%;
    padding: 0.45rem 2rem 0.45rem 0.75rem;
    background: var(--bg-input)
      url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='8' viewBox='0 0 12 8'%3E%3Cpath fill='%23a6adc8' d='M1 1l5 5 5-5'/%3E%3C/svg%3E")
      no-repeat right 0.6rem center;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text);
    font-family: var(--font);
    font-size: var(--font-size-base);
    cursor: pointer;
    transition: border-color 0.15s;
  }

  select:hover {
    border-color: var(--text-muted);
  }

  select:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
</style>
