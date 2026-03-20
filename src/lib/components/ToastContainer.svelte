<script lang="ts">
  import { notifications } from "$lib/stores/notifications";
</script>

<div class="toast-container">
  {#each $notifications as toast (toast.id)}
    <div class="toast toast--{toast.type}">
      <span class="toast__dot"></span>
      <span class="toast__message">{toast.message}</span>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    bottom: 1.5rem;
    right: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    z-index: 1000;
    pointer-events: none;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
    background: var(--bg-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: var(--font-size-sm);
    color: var(--text);
    max-width: 360px;
    animation: slide-in 0.15s ease-out;
  }

  .toast__dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .toast--success .toast__dot {
    background: var(--success);
  }
  .toast--error .toast__dot {
    background: var(--danger);
  }
  .toast--warning .toast__dot {
    background: var(--warning);
  }

  @keyframes slide-in {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
