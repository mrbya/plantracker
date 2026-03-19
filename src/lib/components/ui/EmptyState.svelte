<script lang="ts">
  let { message }: { message: string } = $props();
</script>

<div class="empty-state">
  <span class="icon">󰟢</span>
  <p>{message}</p>
</div>

<style>
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 3rem 1rem;
    color: var(--text-muted);
    text-align: center;
  }

  .icon {
    font-size: 2rem;
    opacity: 0.5;
  }

  p {
    font-size: var(--font-size-sm);
  }
</style>
