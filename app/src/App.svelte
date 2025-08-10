<script lang="ts">
  import "./app.css";
  import Counter from "$lib/Counter.svelte";
  import LoginForm from "$lib/components/login-form.svelte";
</script>

<main>
  <Counter />
  <div class="flex h-screen w-full items-center justify-center px-4">
    <LoginForm />
  </div>
</main>
