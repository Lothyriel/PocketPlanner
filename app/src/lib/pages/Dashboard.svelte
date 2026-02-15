<script lang="ts">
  import { appStore } from '$lib/stores/app.svelte';
  import * as Card from '$lib/components/ui/card/index.js';
  import { Button } from '$lib/components/ui/button/index.js';
  import { goto } from '@mateothegreat/svelte5-router';

  const totalBalance = $derived(appStore.cards.reduce((sum, card) => sum + card.currentBalance, 0));

  const totalCreditLimit = $derived(
    appStore.cards
      .filter((c) => c.type === 'credit')
      .reduce((sum, card) => sum + (card.creditLimit ?? 0), 0),
  );

  const totalCreditUsed = $derived(
    appStore.cards
      .filter((c) => c.type === 'credit')
      .reduce((sum, card) => sum + card.currentBalance, 0),
  );

  const recentTransactions = $derived(
    [...appStore.transactions].sort((a, b) => b.date.getTime() - a.date.getTime()).slice(0, 5),
  );

  function formatCurrency(amount: number) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
    }).format(amount);
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-3xl font-bold">Dashboard</h1>
    <span class="text-muted-foreground">Welcome, {appStore.user?.name}</span>
  </div>

  <div class="grid gap-4 md:grid-cols-3">
    <Card.Root>
      <Card.Header class="pb-2">
        <Card.Description>Total Balance</Card.Description>
        <Card.Title class="text-2xl">{formatCurrency(totalBalance)}</Card.Title>
      </Card.Header>
    </Card.Root>

    <Card.Root>
      <Card.Header class="pb-2">
        <Card.Description>Credit Used</Card.Description>
        <Card.Title class="text-2xl">
          {formatCurrency(totalCreditUsed)} / {formatCurrency(totalCreditLimit)}
        </Card.Title>
      </Card.Header>
    </Card.Root>

    <Card.Root>
      <Card.Header class="pb-2">
        <Card.Description>Cards</Card.Description>
        <Card.Title class="text-2xl">{appStore.cards.length}</Card.Title>
      </Card.Header>
    </Card.Root>
  </div>

  <div class="grid gap-4 md:grid-cols-2">
    <Card.Root>
      <Card.Header>
        <Card.Title>Your Cards</Card.Title>
      </Card.Header>
      <Card.Content>
        {#if appStore.cards.length === 0}
          <p class="text-muted-foreground text-sm">No cards yet.</p>
          <Button class="mt-2" onclick={() => goto('/cards')}>Add a card</Button>
        {:else}
          <div class="space-y-2">
            {#each appStore.cards as card}
              <div class="flex items-center justify-between rounded-lg border p-3">
                <div>
                  <p class="font-medium">{card.name}</p>
                  <p class="text-muted-foreground text-sm">
                    {card.type === 'credit' ? 'Credit Card' : 'Debit Card'}
                  </p>
                </div>
                <div class="text-right">
                  <p class="font-medium">{formatCurrency(card.currentBalance)}</p>
                  {#if card.type === 'credit' && card.creditLimit}
                    <p class="text-muted-foreground text-sm">
                      of {formatCurrency(card.creditLimit)}
                    </p>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header>
        <Card.Title>Recent Transactions</Card.Title>
      </Card.Header>
      <Card.Content>
        {#if recentTransactions.length === 0}
          <p class="text-muted-foreground text-sm">No transactions yet.</p>
          <Button class="mt-2" onclick={() => goto('/transactions')}>Add a transaction</Button>
        {:else}
          <div class="space-y-2">
            {#each recentTransactions as transaction}
              {@const category = appStore.getCategory(transaction.categoryId)}
              {@const card = appStore.getCard(transaction.cardId)}
              <div class="flex items-center justify-between rounded-lg border p-3">
                <div>
                  <p class="font-medium">{transaction.description}</p>
                  <p class="text-muted-foreground text-sm">
                    {category?.name} • {card?.name}
                  </p>
                </div>
                <p
                  class="font-medium"
                  class:text-red-500={transaction.amount >= 0}
                  class:text-green-500={transaction.amount < 0}
                >
                  {formatCurrency(transaction.amount)}
                </p>
              </div>
            {/each}
          </div>
        {/if}
      </Card.Content>
    </Card.Root>
  </div>
</div>
