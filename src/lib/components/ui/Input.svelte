<script lang="ts">
  /**
   * Styled text input component.
   *
   * Wraps a native `<input>` with a `<label>` and optional inline error
   * message, all themed with Catppuccin Mocha CSS variables.  The label text
   * doubles as the `for`/`id` pair for accessibility.
   *
   * Props:
   * - `type`    — HTML input type attribute; defaults to `"text"`.
   * - `value`   — bindable current input value string.
   * - `label`   — optional label text rendered above the input; also used as
   *               the `id` for the `<input>` and the `for` on the `<label>`.
   * - `error`   — optional validation error message rendered below the input
   *               in red; also applies a red border to the input field.
   * - `onblur`  — callback forwarded to the native `<input>` `blur` event;
   *               used to trigger deferred validation (e.g., the entries-limit
   *               field in Settings saves on blur rather than on every keystroke).
   */
  let {
    type = "text",
    value = $bindable(""),
    label,
    error,
    onblur,
  }: {
    type?: string;
    value?: string;
    label?: string;
    error?: string;
    onblur?: () => void;
  } = $props();
</script>

<div class="field">
  {#if label}
    <label class="label" for={label}>{label}</label>
  {/if}
  <input id={label} {type} bind:value class:has-error={!!error} {onblur} />
  {#if error}
    <span class="error-msg">{error}</span>
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .label {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }

  input {
    width: 100%;
    padding: 0.45rem 0.75rem;
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text);
    font-family: var(--font);
    font-size: var(--font-size-base);
    transition: border-color 0.15s;
  }

  input:hover {
    border-color: var(--text-muted);
  }

  input:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  input.has-error {
    border-color: var(--danger);
  }

  .error-msg {
    font-size: var(--font-size-sm);
    color: var(--danger);
  }
</style>
