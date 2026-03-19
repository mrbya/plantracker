<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  import { login } from '$lib/stores/auth';

  let loading = $state(false);

  async function handleLogin() {
    loading = true;
    try {
      await login();
    } finally {
      loading = false;
    }
  }
</script>

<div class="login-shell">
  <div class="login-card">
    <h1 class="app-name">PlanTracker</h1>
    <p class="tagline">Track time spent on Microsoft Planner tasks.</p>

    <Button variant="primary" {loading} disabled={loading} onclick={handleLogin}>
      Sign in with Microsoft
    </Button>
  </div>
</div>

<style>
  .login-shell {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: var(--bg);
  }

  .login-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 2.5rem;
    background: var(--bg-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    min-width: 320px;
    text-align: center;
  }

  .app-name {
    font-size: var(--font-size-xl);
    color: var(--accent);
  }

  .tagline {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }
</style>
