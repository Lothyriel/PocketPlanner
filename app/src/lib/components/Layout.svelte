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
        <a href="/" use:route class="text-xl font-bold">PocketPlanner</a>
        <nav class="hidden gap-6 md:flex">
          {#each navItems as item}
            <a
              href={item.path}
              use:route={{
                active: { class: 'bg-secondary text-foreground', absolute: true },
                default: { class: 'text-muted-foreground', absolute: true },
              }}
              class="hover:text-foreground rounded-md px-3 py-1.5 text-sm font-medium transition-colors"
            >
              {item.label}
            </a>
          {/each}
        </nav>
      </div>
      <div class="flex items-center gap-4">
        <span class="text-muted-foreground hidden text-sm md:inline">{appStore.user?.email}</span>
        <Button variant="ghost" size="sm" onclick={handleLogout}>Logout</Button>
      </div>
    </div>
  </header>

  <main class="container mx-auto px-4 py-8 pb-24 md:pb-8">
    {@render children()}
  </main>

  <nav
    class="bg-background/95 fixed inset-x-0 bottom-0 z-40 border-t backdrop-blur supports-[backdrop-filter]:bg-background/80 md:hidden"
  >
    <div class="mx-auto grid max-w-md grid-cols-3 gap-1 px-3 pt-2 pb-[calc(env(safe-area-inset-bottom)+0.5rem)]">
      {#each navItems as item}
        <a
          href={item.path}
          use:route={{
            active: {
              class:
                'bg-primary text-primary-foreground shadow-[0_6px_16px_-10px_color-mix(in_oklch,var(--primary)_60%,transparent)]',
              absolute: true,
            },
            default: { class: 'text-muted-foreground', absolute: true },
          }}
          class="rounded-xl px-3 py-2.5 text-center text-sm font-medium transition-all duration-200"
        >
          {item.label}
        </a>
      {/each}
    </div>
  </nav>
</div>
