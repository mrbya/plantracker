<script lang="ts">
  import { isAuthenticated } from '$lib/stores/auth';
  import Layout from '$lib/components/Layout.svelte';
  import Login from '../../views/Login.svelte';
</script>

{#if $isAuthenticated}
  <Layout />
{:else}
  <Login />
{/if}
