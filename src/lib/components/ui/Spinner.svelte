<script lang="ts">
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
