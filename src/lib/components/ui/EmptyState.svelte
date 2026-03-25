<script lang="ts">
  /**
   * Empty-state placeholder shown in place of an empty list or table.
   *
   * Renders a centred icon and message with muted styling to signal that there
   * is no content to display — as opposed to a loading or error state.  Always
   * use this component rather than rendering nothing when a list has zero items,
   * so the user understands the absence of content is intentional.
   *
   * Props:
   * - `message` — contextual explanation of why the list is empty and what
   *               the user can do, e.g. `"No entries yet. Start a timer to
   *               track time."`.
   */
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
