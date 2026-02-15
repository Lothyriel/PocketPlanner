<script lang="ts">
  import { appStore } from "$lib/stores/app.svelte";
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
    if (appStore.isAuthenticated) {
      return;
    }
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
  <main
    class="flex flex-1 flex-col items-center justify-center gap-6 px-6 text-center"
  >
    <div class="space-y-3">
      <h1 class="text-4xl font-bold">Finance, simplified.</h1>
      <p class="text-muted-foreground max-w-xl">
        Landing page coming soon. Sign in to access your dashboard.
      </p>
    </div>
    {#if !appStore.isAuthenticated}
      <div
        class="flex justify-center"
        bind:this={buttonContainer}
        aria-label="Sign in with Google"
      ></div>
    {/if}
    {#if errorMessage}
      <p class="text-sm text-red-500">{errorMessage}</p>
    {/if}
  </main>
</div>
