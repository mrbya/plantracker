<script lang="ts">
  let {
    type = 'text',
    value = $bindable(''),
    label,
    error,
  }: {
    type?: string;
    value?: string;
    label?: string;
    error?: string;
  } = $props();
</script>

<div class="field">
  {#if label}
    <label class="label">{label}</label>
  {/if}
  <input
    {type}
    bind:value
    class:has-error={!!error}
  />
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
