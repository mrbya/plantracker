<script lang="ts">
  import type { Snippet } from "svelte";
  let {
    color,
    children,
  }: { color: "green" | "red" | "yellow"; children?: Snippet } = $props();
</script>

<span class="badge {color}">
  {@render children?.()}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    padding: 0.15rem 0.5rem;
    border-radius: 999px;
    font-size: var(--font-size-sm);
    font-weight: 500;
    border: 1px solid transparent;
  }

  .green {
    color: var(--success);
    background: color-mix(in srgb, var(--success) 15%, transparent);
    border-color: color-mix(in srgb, var(--success) 30%, transparent);
  }

  .red {
    color: var(--danger);
    background: color-mix(in srgb, var(--danger) 15%, transparent);
    border-color: color-mix(in srgb, var(--danger) 30%, transparent);
  }

  .yellow {
    color: var(--warning);
    background: color-mix(in srgb, var(--warning) 15%, transparent);
    border-color: color-mix(in srgb, var(--warning) 30%, transparent);
  }
</style>
