<script lang="ts">
  /**
   * Themed button component.
   *
   * Wraps a native `<button>` with Catppuccin Mocha styling and a built-in
   * loading state.  When `loading` is `true` a small `Spinner` is shown
   * alongside the slot content and the button is automatically disabled.
   *
   * Props:
   * - `variant`  — visual style: `"primary"` (accent fill), `"ghost"` (border
   *                only), `"danger"` (red fill), or `"success"` (green fill).
   *                Defaults to `"primary"`.
   * - `disabled` — disables the button independently of `loading`.
   * - `loading`  — shows a spinner and prevents interaction.
   * - `onclick`  — click handler forwarded to the native `<button>`.
   * - `title`    — tooltip text; forwarded to the native `<button>`.
   * - `children` — slot content rendered next to the spinner (if present).
   */
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
