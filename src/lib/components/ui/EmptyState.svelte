<script lang="ts">
  import { CircleSlash2 } from "lucide-svelte";
  let { message }: { message: string } = $props();
</script>

<div class="empty-state">
  <CircleSlash2 size={28} strokeWidth={1.5} />
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
    opacity: 0.5;
  }

  p {
    font-size: var(--font-size-base);
  }
</style>
