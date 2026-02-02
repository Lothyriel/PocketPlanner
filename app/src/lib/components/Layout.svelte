<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { appStore } from '$lib/stores/app.svelte';
	import { goto, route } from '@mateothegreat/svelte5-router';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	const navItems = [
		{ path: '/dashboard', label: 'Dashboard' },
		{ path: '/cards', label: 'Cards' },
		{ path: '/transactions', label: 'Transactions' },
	];

	function handleLogout() {
		appStore.logout();
		goto('/');
	}
</script>

<div class="bg-background min-h-screen">
	<header class="border-b">
		<div class="container mx-auto flex h-16 items-center justify-between px-4">
			<div class="flex items-center gap-8">
				<a href="/dashboard" use:route class="text-xl font-bold">PocketPlanner</a>
				<nav class="hidden gap-6 md:flex">
					{#each navItems as item}
						<a
							href={item.path}
							use:route
							class="text-muted-foreground hover:text-foreground text-sm font-medium transition-colors"
						>
							{item.label}
						</a>
					{/each}
				</nav>
			</div>
			<div class="flex items-center gap-4">
				<span class="text-muted-foreground text-sm">{appStore.user?.email}</span>
				<Button variant="ghost" size="sm" onclick={handleLogout}>Logout</Button>
			</div>
		</div>
	</header>

	<main class="container mx-auto px-4 py-8">
		{@render children()}
	</main>
</div>
