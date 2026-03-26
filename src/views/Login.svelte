<script lang="ts">
  /**
   * Login view — the unauthenticated entry point.
   *
   * Displayed by `+page.svelte` whenever `$isAuthenticated` is `false`.
   * Renders a centred card with the app name, tagline, and a single
   * "Sign in with Microsoft" button.
   *
   * OAuth flow:
   *   Clicking the button calls `login()` from the `auth` store, which
   *   invokes the `login` Tauri command.  The backend opens the system browser
   *   at the Microsoft OAuth 2.0 PKCE authorisation URL, starts a local HTTP
   *   server on `localhost:52721` to receive the callback, exchanges the
   *   authorisation code for tokens, stores them in the OS keychain, and
   *   returns the updated `AuthStatus`.  The auth store then sets
   *   `isAuthenticated = true`, which causes `+page.svelte` to render
   *   `Layout` instead of this view.
   *
   * Loading state:
   *   The button is disabled and shows a spinner while `login()` is awaited.
   *   `loading` is reset in the `finally` block whether the login succeeds,
   *   fails, or the user cancels the browser flow.
   */
  import Button from "$lib/components/ui/Button.svelte";
  import { login } from "$lib/stores/auth";
  import * as m from "$lib/paraglide/messages";

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
    <h1 class="app-name">{m.login_title()}</h1>
    <p class="tagline">{m.login_tagline()}</p>

    <Button
      variant="primary"
      {loading}
      disabled={loading}
      onclick={handleLogin}
    >
      {m.login_sign_in_btn()}
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
