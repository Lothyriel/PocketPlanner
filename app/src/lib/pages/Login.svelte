<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { appStore } from '$lib/stores/app.svelte';
	import { goto } from '@mateothegreat/svelte5-router';
	import { onMount } from 'svelte';

	type GoogleCredentialResponse = {
		credential: string;
	};

	let errorMessage = $state('');
	let buttonContainer: HTMLDivElement | null = $state(null);

	async function handleGoogleLogin(response: GoogleCredentialResponse) {
		if (!response.credential) {
			errorMessage = 'Google sign-in failed. Please try again.';
			return;
		}
		try {
			await appStore.loginWithToken(response.credential);
			goto('/dashboard');
		} catch (err) {
			console.error(err);
			errorMessage = 'Unable to sign in. Please try again.';
		}
	}

	onMount(() => {
		const clientId = import.meta.env.VITE_GOOGLE_CLIENT_ID;
		if (!clientId) {
			errorMessage = 'Missing Google client ID. Set VITE_GOOGLE_CLIENT_ID.';
			return;
		}
		let tries = 0;
		const maxTries = 50;
		const initGoogle = () => {
			if (!window.google?.accounts?.id) {
				tries += 1;
				if (tries >= maxTries) {
					errorMessage = 'Google login script did not load.';
					return;
				}
				setTimeout(initGoogle, 100);
				return;
			}

			window.google.accounts.id.initialize({
				client_id: clientId,
				callback: handleGoogleLogin,
			});

			if (buttonContainer) {
				window.google.accounts.id.renderButton(buttonContainer, {
					theme: 'outline',
					size: 'large',
					width: 320,
				});
			}

			window.google.accounts.id.prompt();
		};

		initGoogle();
	});
</script>

<div class="flex h-screen w-full items-center justify-center px-4">
	<Card.Root class="mx-auto w-full max-w-sm">
		<Card.Header class="text-center">
			<Card.Title class="text-2xl">PocketPlanner</Card.Title>
			<Card.Description>Sign in to manage your finances</Card.Description>
		</Card.Header>
		<Card.Content>
			<div class="flex flex-col items-center gap-3">
				<div bind:this={buttonContainer}></div>
				{#if errorMessage}
					<p class="text-sm text-red-500">{errorMessage}</p>
				{/if}
				{#if !buttonContainer}
					<Button variant="outline" class="w-full" disabled>Loading Google sign-in…</Button>
				{/if}
			</div>
		</Card.Content>
	</Card.Root>
</div>
