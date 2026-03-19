<script lang="ts">
  import { onMount } from 'svelte';
  import { isAuthenticated, initAuth } from '$lib/stores/auth';
  import { initTimer } from '$lib/stores/timer';
  import Layout from '$lib/components/Layout.svelte';
  import Login from '../views/Login.svelte';
  import ToastContainer from '$lib/components/ToastContainer.svelte';

  onMount(() => {
    initAuth();
    initTimer();
  });
</script>

<ToastContainer />

{#if $isAuthenticated}
  <Layout />
{:else}
  <Login />
{/if}
