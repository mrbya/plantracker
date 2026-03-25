<script lang="ts">
  /**
   * Raised content container with an optional header.
   *
   * Renders a rounded box with a `--bg-input` background and a `--border`
   * outline.  When `title` is provided, a header row is rendered above the
   * body with the title displayed in uppercase small-caps style.
   *
   * Props:
   * - `title`    — optional header text shown above the body content.
   * - `children` — slot content rendered inside the card body.
   */
  let {
    title,
    children,
  }: {
    title?: string;
    children?: import("svelte").Snippet;
  } = $props();
</script>

<div class="card">
  {#if title}
    <div class="card-header">
      <span class="card-title">{title}</span>
    </div>
  {/if}
  <div class="card-body">
    {#if children}
      {@render children()}
    {/if}
  </div>
</div>

<style>
  .card {
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .card-header {
    padding: 0.6rem 1rem;
    border-bottom: 1px solid var(--border);
  }

  .card-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .card-body {
    padding: 1rem;
  }
</style>
