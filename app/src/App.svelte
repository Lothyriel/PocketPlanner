<script lang="ts">
  import './app.css';
  import { Router, goto } from '@mateothegreat/svelte5-router';
  import { appStore } from '$lib/stores/app.svelte';
  import Landing from '$lib/pages/Landing.svelte';
  import Dashboard from '$lib/pages/Dashboard.svelte';
  import Cards from '$lib/pages/Cards.svelte';
  import Transactions from '$lib/pages/Transactions.svelte';
  import Layout from '$lib/components/Layout.svelte';
  import { onMount } from 'svelte';

  const requireAuth = () => {
    if (!appStore.isAuthenticated) {
      goto('/');
      return false;
    }
    return true;
  };

  const routes = [
    { path: '/', component: Landing },
    { path: '/dashboard', component: Dashboard, hooks: { pre: requireAuth } },
    { path: '/cards', component: Cards, hooks: { pre: requireAuth } },
    {
      path: '/transactions',
      component: Transactions,
      hooks: { pre: requireAuth },
    },
  ];

  onMount(() => {
    appStore.bootstrap().catch((err) => {
      console.error('Failed to restore session', err);
    });
  });
</script>

{#if appStore.isBootstrapping}
  <div class="flex h-screen items-center justify-center text-sm text-muted-foreground">
    Loading...
  </div>
{:else if appStore.isAuthenticated}
  <Layout>
    <Router {routes} />
  </Layout>
{:else}
  <Router {routes} />
{/if}
