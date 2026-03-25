<script lang="ts">
  /**
   * Animated loading indicator.
   *
   * Renders a CSS-animated circular spinner using `--border` as the track
   * colour and `--accent` as the rotating arc.  Used inside `Button` when
   * `loading={true}` and as a standalone indicator in content areas while
   * data is being fetched.
   *
   * Props:
   * - `size` — `"sm"` (12 × 12 px, used inside buttons) or `"md"` (18 × 18 px,
   *            used in content areas).  Defaults to `"md"`.
   */
  let { size = "md" }: { size?: "sm" | "md" } = $props();
</script>

<span
  class="spinner"
  class:sm={size === "sm"}
  class:md={size === "md"}
  aria-label="Loading"
></span>

<style>
  .spinner {
    display: inline-block;
    border: 2px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
    flex-shrink: 0;
  }

  .sm {
    width: 12px;
    height: 12px;
  }

  .md {
    width: 18px;
    height: 18px;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
