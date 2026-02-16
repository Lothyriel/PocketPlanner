<script lang="ts">
  import { goto } from '@mateothegreat/svelte5-router';
  import { appStore } from '$lib/stores/app.svelte';

  function goBackHome() {
    goto(appStore.isAuthenticated ? '/dashboard' : '/');
  }
</script>

<div class="flex min-h-[60vh] items-center justify-center px-6">
  <div class="mx-auto max-w-md space-y-3 text-center">
    <p class="text-muted-foreground text-7xl leading-none font-bold">404</p>
    <h1 class="text-3xl font-bold">Page not found</h1>
    <p class="text-muted-foreground text-sm">
      The page you requested does not exist or has moved.
    </p>
    <button
      type="button"
      class="bg-primary text-primary-foreground hover:bg-primary/90 mt-2 inline-flex h-10 items-center justify-center rounded-md px-4 text-sm font-medium transition-colors"
      onclick={goBackHome}
    >
      {appStore.isAuthenticated ? 'Back to dashboard' : 'Back to home'}
    </button>
  </div>
</div>
