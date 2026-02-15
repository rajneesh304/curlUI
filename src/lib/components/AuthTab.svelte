<script lang="ts">
  import * as Select from "$lib/components/ui/select/index.js";
  import { Input } from "$lib/components/ui/input/index.js";

  // Define props as bindable so the parent (+page.svelte) can read the values
  let {
    authType = $bindable("none"),
    bearerToken = $bindable(""),
    basicUsername = $bindable(""),
    basicPassword = $bindable(""),
  } = $props();
</script>

<div class="flex gap-6 h-full p-2">
  <div class="w-48 border-r pr-6 pt-2">
    <label
      class="text-xs font-bold text-muted-foreground uppercase tracking-wider mb-3 block"
      >Auth Type</label
    >
    <Select.Root type="single" bind:value={authType}>
      <Select.Trigger class="w-full">
        {#if authType === "none"}
          No Auth
        {:else if authType === "bearer"}
          Bearer Token
        {:else if authType === "basic"}
          Basic Auth
        {/if}
      </Select.Trigger>
      <Select.Content>
        <Select.Item value="none">No Auth</Select.Item>
        <Select.Item value="bearer">Bearer Token</Select.Item>
        <Select.Item value="basic">Basic Auth</Select.Item>
      </Select.Content>
    </Select.Root>
  </div>

  <div class="flex-1 pt-2 pl-2">
    {#if authType === "none"}
      <div
        class="flex h-full items-center justify-center text-sm text-muted-foreground"
      >
        This request does not use any authorization.
      </div>
    {/if}

    {#if authType === "bearer"}
      <div class="flex flex-col gap-4 max-w-md">
        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium">Token</label>
          <Input
            type="text"
            placeholder="Enter your Bearer token"
            bind:value={bearerToken}
            class="font-mono"
          />
        </div>
        <p class="text-xs text-muted-foreground">
          The app will automatically format this as <code
            class="bg-muted px-1 rounded"
            >Authorization: Bearer &lt;token&gt;</code
          >
        </p>
      </div>
    {/if}

    {#if authType === "basic"}
      <div class="flex flex-col gap-4 max-w-md">
        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium">Username</label>
          <Input
            type="text"
            placeholder="Username"
            bind:value={basicUsername}
            class="font-mono"
          />
        </div>
        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium">Password</label>
          <Input
            type="password"
            placeholder="Password"
            bind:value={basicPassword}
            class="font-mono"
          />
        </div>
        <p class="text-xs text-muted-foreground">
          The app will automatically Base64 encode these credentials.
        </p>
      </div>
    {/if}
  </div>
</div>
