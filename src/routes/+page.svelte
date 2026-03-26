<script lang="ts">
  /**
   * Application root page.
   *
   * Handles the three-phase startup sequence and conditionally renders either
   * the authenticated app shell (`Layout`) or the sign-in screen (`Login`).
   *
   * Startup sequence (all inside `onMount`):
   *   1. `loadLocale()`   — reads the persisted locale from `config.json` and
   *      calls Paraglide's `setLocale()`, minimising the flash of the default
   *      locale on non-English sessions.
   *   2. `loadTheme()`    — reads the persisted theme from `config.json` and
   *      applies the `theme-light` HTML class if needed, preventing a flash
   *      of the wrong theme.
   *   3. `loadSettings()` — hydrates `entriesLimit`, `syncFrequency`, and
   *      `lastSyncedAt` from `config.json`.
   *   4. `initAuth()`     — checks the OS keychain for a valid token; if found,
   *      also triggers `syncAndLoad()` so plans and tasks are ready
   *      immediately.
   *   5. `initTimer()`    — queries the backend for any open timer row and
   *      restores in-memory state + tick loop if one is found.
   *
   * OS colour scheme listener:
   *   A `MediaQueryList` change listener is attached to
   *   `(prefers-color-scheme: light)`.  When the OS switches appearance and
   *   the user's choice is `"system"`, the `themeChoice` store is nudged
   *   (via an identity update) to trigger the `resolvedTheme` derived store,
   *   which in turn updates the `<html>` class.  The listener is removed on
   *   component destroy via the `onMount` cleanup return value.
   */
  import { get } from "svelte/store";
  import { onMount } from "svelte";
  import { isAuthenticated, initAuth } from "$lib/stores/auth";
  import { initTimer } from "$lib/stores/timer";
  import { loadSettings } from "$lib/stores/settings";
  import { loadTheme, themeChoice } from "$lib/stores/theme";
  import { loadLocale } from "$lib/stores/locale";
  import Layout from "$lib/components/Layout.svelte";
  import Login from "../views/Login.svelte";
  import ToastContainer from "$lib/components/ToastContainer.svelte";

  onMount(() => {
    loadLocale();
    loadTheme();
    loadSettings();
    initAuth();
    initTimer();

    // Re-apply theme when OS preference changes (only when choice is 'system').
    const mq = window.matchMedia("(prefers-color-scheme: light)");
    const onSystemChange = () => {
      if (get(themeChoice) === "system") {
        themeChoice.update((v) => v);
      }
    };
    mq.addEventListener("change", onSystemChange);
    return () => mq.removeEventListener("change", onSystemChange);
  });
</script>

<ToastContainer />

{#if $isAuthenticated}
  <Layout />
{:else}
  <Login />
{/if}
