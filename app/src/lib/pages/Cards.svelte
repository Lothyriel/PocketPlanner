<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import type { CardType } from '$lib/types';

	let showForm = $state(false);
	let name = $state('');
	let cardType = $state<CardType>('debit');
	let creditLimit = $state('');

	function formatCurrency(amount: number) {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
		}).format(amount);
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		try {
			await appStore.addCard({
				name,
				type: cardType,
				creditLimit: cardType === 'credit' ? Number(creditLimit) : undefined,
				currentBalance: 0,
			});
			resetForm();
		} catch (err) {
			console.error(err);
		}
	}

	function resetForm() {
		showForm = false;
		name = '';
		cardType = 'debit';
		creditLimit = '';
	}
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<h1 class="text-3xl font-bold">Cards</h1>
		<Button onclick={() => (showForm = !showForm)}>
			{showForm ? 'Cancel' : 'Add Card'}
		</Button>
	</div>

	{#if showForm}
		<Card.Root>
			<Card.Header>
				<Card.Title>Add New Card</Card.Title>
			</Card.Header>
			<Card.Content>
				<form onsubmit={handleSubmit} class="space-y-4">
					<div class="grid gap-2">
						<Label for="card-name">Card Name</Label>
						<Input
							id="card-name"
							bind:value={name}
							placeholder="e.g., Chase Freedom"
							required
						/>
					</div>

					<div class="grid gap-2">
						<Label>Card Type</Label>
						<div class="flex gap-4">
							<label class="flex items-center gap-2">
								<input
									type="radio"
									name="cardType"
									value="debit"
									bind:group={cardType}
									class="accent-primary"
								/>
								Debit
							</label>
							<label class="flex items-center gap-2">
								<input
									type="radio"
									name="cardType"
									value="credit"
									bind:group={cardType}
									class="accent-primary"
								/>
								Credit
							</label>
						</div>
					</div>

					{#if cardType === 'credit'}
						<div class="grid gap-2">
							<Label for="credit-limit">Credit Limit</Label>
							<Input
								id="credit-limit"
								type="number"
								bind:value={creditLimit}
								placeholder="5000"
								min="0"
								step="1"
								required
							/>
						</div>
					{/if}

					<Button type="submit" class="w-full">Add Card</Button>
				</form>
			</Card.Content>
		</Card.Root>
	{/if}

	{#if appStore.cards.length === 0 && !showForm}
		<Card.Root>
			<Card.Content class="py-8 text-center">
				<p class="text-muted-foreground">No cards yet. Add your first card to get started.</p>
			</Card.Content>
		</Card.Root>
	{:else}
		<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each appStore.cards as card}
				<Card.Root>
					<Card.Header>
						<div class="flex items-start justify-between">
							<div>
								<Card.Title>{card.name}</Card.Title>
								<Card.Description>
									{card.type === 'credit' ? 'Credit Card' : 'Debit Card'}
								</Card.Description>
							</div>
							<Button
								variant="ghost"
								size="sm"
								onclick={() => appStore.deleteCard(card.id)}
							>
								Delete
							</Button>
						</div>
					</Card.Header>
					<Card.Content>
						<div class="space-y-2">
							<div class="flex justify-between">
								<span class="text-muted-foreground">Balance</span>
								<span class="font-medium">{formatCurrency(card.currentBalance)}</span>
							</div>
							{#if card.type === 'credit' && card.creditLimit}
								<div class="flex justify-between">
									<span class="text-muted-foreground">Credit Limit</span>
									<span class="font-medium">{formatCurrency(card.creditLimit)}</span>
								</div>
								<div class="flex justify-between">
									<span class="text-muted-foreground">Available</span>
									<span class="font-medium">
										{formatCurrency(card.creditLimit - card.currentBalance)}
									</span>
								</div>
								<div class="bg-muted mt-2 h-2 rounded-full">
									<div
										class="bg-primary h-2 rounded-full transition-all"
										style="width: {Math.min((card.currentBalance / card.creditLimit) * 100, 100)}%"
									></div>
								</div>
							{/if}
						</div>
					</Card.Content>
				</Card.Root>
			{/each}
		</div>
	{/if}
</div>
