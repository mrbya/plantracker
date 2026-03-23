<script lang="ts">
  import { get } from "svelte/store";
  import { onMount } from "svelte";
  import { isAuthenticated, initAuth } from "$lib/stores/auth";
  import { initTimer } from "$lib/stores/timer";
  import { loadSettings } from "$lib/stores/settings";
  import { loadTheme, themeChoice } from "$lib/stores/theme";
  import Layout from "$lib/components/Layout.svelte";
  import Login from "../views/Login.svelte";
  import ToastContainer from "$lib/components/ToastContainer.svelte";

  onMount(() => {
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
