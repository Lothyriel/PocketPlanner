<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';

	let showForm = $state(false);
	let description = $state('');
	let amount = $state('');
	let cardId = $state('');
	let categoryId = $state('');
	let date = $state(new Date().toISOString().split('T')[0]);

	const sortedTransactions = $derived(
		[...appStore.transactions].sort((a, b) => b.date.getTime() - a.date.getTime())
	);

	function formatCurrency(amount: number) {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD',
		}).format(amount);
	}

	function formatDate(date: Date) {
		return new Intl.DateTimeFormat('en-US', {
			month: 'short',
			day: 'numeric',
			year: 'numeric',
		}).format(date);
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		try {
			await appStore.addTransaction({
				description,
				amount: Number(amount),
				cardId,
				categoryId,
				type: 'expense',
				date: new Date(date),
			});
			resetForm();
		} catch (err) {
			console.error(err);
		}
	}

	function resetForm() {
		showForm = false;
		description = '';
		amount = '';
		cardId = '';
		categoryId = '';
		date = new Date().toISOString().split('T')[0];
	}
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<h1 class="text-3xl font-bold">Transactions</h1>
		<Button onclick={() => (showForm = !showForm)} disabled={appStore.cards.length === 0}>
			{showForm ? 'Cancel' : 'Add Transaction'}
		</Button>
	</div>

	{#if appStore.cards.length === 0}
		<Card.Root>
			<Card.Content class="py-8 text-center">
				<p class="text-muted-foreground">Add a card first before creating transactions.</p>
			</Card.Content>
		</Card.Root>
	{:else if showForm}
		<Card.Root>
			<Card.Header>
				<Card.Title>Add Transaction</Card.Title>
			</Card.Header>
			<Card.Content>
				<form onsubmit={handleSubmit} class="space-y-4">
					<div class="grid gap-2">
						<Label for="description">Description</Label>
						<Input
							id="description"
							bind:value={description}
							placeholder="e.g., Grocery shopping"
							required
						/>
					</div>

					<div class="grid gap-2">
						<Label for="amount">Amount</Label>
						<Input
							id="amount"
							type="number"
							bind:value={amount}
							placeholder="0.00"
							step="0.01"
							required
						/>
					</div>

					<div class="grid gap-2">
						<Label for="card">Card</Label>
						<select
							id="card"
							bind:value={cardId}
							required
							class="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none"
						>
							<option value="">Select a card</option>
							{#each appStore.cards as card}
								<option value={card.id}>
									{card.name}
								</option>
							{/each}
						</select>
					</div>

					<div class="grid gap-2">
						<Label for="category">Category</Label>
						<select
							id="category"
							bind:value={categoryId}
							required
							class="border-input bg-background ring-offset-background placeholder:text-muted-foreground focus-visible:ring-ring flex h-10 w-full rounded-md border px-3 py-2 text-sm focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:outline-none"
						>
							<option value="">Select a category</option>
							{#each appStore.categories as category}
								<option value={category.id}>{category.name}</option>
							{/each}
						</select>
					</div>

					<div class="grid gap-2">
						<Label for="date">Date</Label>
						<Input id="date" type="date" bind:value={date} required />
					</div>

					<Button type="submit" class="w-full">Add Transaction</Button>
				</form>
			</Card.Content>
		</Card.Root>
	{/if}

	{#if sortedTransactions.length === 0 && !showForm}
		<Card.Root>
			<Card.Content class="py-8 text-center">
				<p class="text-muted-foreground">No transactions yet.</p>
			</Card.Content>
		</Card.Root>
	{:else if sortedTransactions.length > 0}
		<Card.Root>
			<Card.Content class="p-0">
				<div class="divide-y">
					{#each sortedTransactions as transaction}
						{@const category = appStore.getCategory(transaction.categoryId)}
						{@const card = appStore.getCard(transaction.cardId)}
						<div class="flex items-center justify-between p-4">
							<div class="flex items-center gap-4">
								<div
									class="h-3 w-3 rounded-full"
									style="background-color: {category?.color ?? '#6b7280'}"
								></div>
								<div>
									<p class="font-medium">{transaction.description}</p>
									<p class="text-muted-foreground text-sm">
										{category?.name} • {card?.name} • {formatDate(transaction.date)}
									</p>
								</div>
							</div>
							<div class="flex items-center gap-4">
								<p
									class="font-medium"
									class:text-red-500={transaction.amount >= 0}
									class:text-green-500={transaction.amount < 0}
								>
									{formatCurrency(transaction.amount)}
								</p>
								<Button
									variant="ghost"
									size="sm"
									onclick={() => appStore.deleteTransaction(transaction.id)}
								>
									Delete
								</Button>
							</div>
						</div>
					{/each}
				</div>
			</Card.Content>
		</Card.Root>
	{/if}
</div>
