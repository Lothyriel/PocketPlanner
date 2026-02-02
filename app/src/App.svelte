<script lang="ts">
	import './app.css';
	import { Router } from '@mateothegreat/svelte5-router';
	import { appStore } from '$lib/stores/app.svelte';
	import Login from '$lib/pages/Login.svelte';
	import Dashboard from '$lib/pages/Dashboard.svelte';
	import Cards from '$lib/pages/Cards.svelte';
	import Transactions from '$lib/pages/Transactions.svelte';
	import Layout from '$lib/components/Layout.svelte';
	import { onMount } from 'svelte';

	const publicRoutes = [
		{ path: '/', component: Login },
	];

	const protectedRoutes = [
		{ path: '/', component: Dashboard },
		{ path: '/dashboard', component: Dashboard },
		{ path: '/cards', component: Cards },
		{ path: '/transactions', component: Transactions },
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
		<Router routes={protectedRoutes} />
	</Layout>
{:else}
	<Router routes={publicRoutes} />
{/if}
