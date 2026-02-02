<script lang="ts">
  import { appStore } from "$lib/stores/app.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { goto } from "@mateothegreat/svelte5-router";
  import { onMount } from "svelte";

  type GoogleCredentialResponse = {
    credential: string;
  };

  let errorMessage = $state("");
  let buttonContainer: HTMLDivElement | null = $state(null);

  async function handleGoogleLogin(response: GoogleCredentialResponse) {
    if (!response.credential) {
      errorMessage = "Google sign-in failed. Please try again.";
      return;
    }
    try {
      await appStore.loginWithToken(response.credential);
      goto("/dashboard");
    } catch (err) {
      console.error(err);
      errorMessage = "Unable to sign in. Please try again.";
    }
  }

  onMount(() => {
    const clientId = import.meta.env.VITE_GOOGLE_CLIENT_ID;
    if (!clientId) {
      errorMessage = "Missing Google client ID. Set VITE_GOOGLE_CLIENT_ID.";
      return;
    }
    let tries = 0;
    const maxTries = 50;
    const initGoogle = () => {
      if (!window.google?.accounts?.id) {
        tries += 1;
        if (tries >= maxTries) {
          errorMessage = "Google login script did not load.";
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
          theme: "outline",
          size: "large",
        });
      }

      window.google.accounts.id.prompt();
    };

    initGoogle();
  });
</script>

<div class="flex min-h-screen flex-col">
  <header
    class="border-b/40 bg-background/80 supports-[backdrop-filter]:bg-background/60 sticky top-0 flex items-center justify-between px-6 py-4 backdrop-blur"
  >
    <div class="flex flex-col">
      <span class="text-muted-foreground text-xs">Personal finance workspace</span>
    </div>
    <div class="flex flex-col items-end gap-2">
      {#if appStore.isAuthenticated}
        <Button
          class="h-11 px-5 text-sm shadow-sm"
          onclick={() => goto("/dashboard")}
        >
          Go to Dashboard
        </Button>
      {:else}
        <div bind:this={buttonContainer}></div>
        {#if !buttonContainer}
          <span class="text-muted-foreground text-xs"
            >Loading Google sign-in…</span
          >
        {/if}
      {/if}
    </div>
  </header>

  <main
    class="flex flex-1 flex-col items-center justify-center gap-6 px-6 text-center"
  >
    <div class="space-y-3">
      <h1 class="text-4xl font-bold">Finance, simplified.</h1>
      <p class="text-muted-foreground max-w-xl">
        Landing page coming soon. Sign in to access your dashboard.
      </p>
    </div>
    {#if errorMessage}
      <p class="text-sm text-red-500">{errorMessage}</p>
    {/if}
  </main>
</div>
