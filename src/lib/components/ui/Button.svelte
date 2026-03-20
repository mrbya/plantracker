<script lang="ts">
  import Spinner from "./Spinner.svelte";

  let {
    variant = "primary",
    disabled = false,
    loading = false,
    onclick,
    children,
    title,
  }: {
    variant?: "primary" | "ghost" | "danger" | "success";
    disabled?: boolean;
    loading?: boolean;
    onclick?: (e: MouseEvent) => void;
    children?: import("svelte").Snippet;
    title?: string;
  } = $props();
</script>

<button class="btn {variant}" disabled={disabled || loading} {onclick} {title}>
  {#if loading}
    <Spinner size="sm" />
  {/if}
  {#if children}
    {@render children()}
  {/if}
</button>

<style>
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0.45rem 1rem;
    border: 1px solid transparent;
    border-radius: var(--radius);
    font-family: var(--font);
    font-size: var(--font-size-base);
    font-weight: 500;
    cursor: pointer;
    transition:
      background 0.15s,
      color 0.15s,
      border-color 0.15s,
      opacity 0.15s;
    white-space: nowrap;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .primary {
    background: var(--accent);
    color: var(--bg);
    border-color: var(--accent);
  }

  .primary:hover:not(:disabled) {
    background: var(--accent-hover);
    border-color: var(--accent-hover);
  }

  .ghost {
    background: transparent;
    color: var(--text);
    border-color: var(--border);
  }

  .ghost:hover:not(:disabled) {
    background: var(--bg-input);
  }

  .danger {
    background: var(--danger);
    color: var(--bg);
    border-color: var(--danger);
  }

  .danger:hover:not(:disabled) {
    opacity: 0.85;
  }

  .success {
    background: var(--success);
    color: var(--bg);
    border-color: var(--success);
  }

  .success:hover:not(:disabled) {
    opacity: 0.85;
  }
</style>
