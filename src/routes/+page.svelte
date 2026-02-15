<script lang="ts">
	import { Input } from '$lib/components/ui/input/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import * as Tabs from '$lib/components/ui/tabs/index.js';
	import MonacoEditor from '$lib/components/MonacoEditor.svelte';
	import { invoke } from '@tauri-apps/api/core';

	// Svelte 5 Runes for highly reactive state management
	let url = $state('https://jsonplaceholder.typicode.com/todos/1');
	let method = $state('GET');
	let activeTab = $state('headers');

	// Monaco config
	let requestBody = $state('{\n  "key": "value"\n}');
	let responseBody = $state('// Waiting for response...');

	let responseStatus = $state<number | null>(null);
	let responseTime = $state<number | null>(null);
	let isRequesting = $state(false);

	async function handleSend() {
		if (!url) return;
		isRequesting = true;
		responseBody = '// Loading...';

		try {
			// Bundle the data exactly as the Rust RequestPayload struct expects
			const payload = {
				url,
				method,
				headers: {}, // We will wire up the Headers table later
				body: activeTab === 'body' ? requestBody : ''
			};

			// Invoke the Rust function
			const res: any = await invoke('perform_request', { request: payload });

			responseStatus = res.status;
			responseTime = res.time_ms;

			// Attempt to format the response if it's JSON, otherwise dump raw text
			try {
				responseBody = JSON.stringify(JSON.parse(res.body), null, 2);
			} catch {
				responseBody = res.body;
			}
		} catch (err) {
			responseStatus = null;
			responseBody = `// Error executing request:\n${err}`;
		} finally {
			isRequesting = false;
		}
	}
</script>

<div class="bg-background text-foreground flex h-screen w-full flex-col overflow-hidden">
	<header class="flex items-center gap-3 border-b p-4 shadow-sm">
		<Select.Root type="single" bind:value={method}>
			<Select.Trigger class="w-[120px] font-bold tracking-wide">
				{method}
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="GET">GET</Select.Item>
				<Select.Item value="POST">POST</Select.Item>
				<Select.Item value="PUT">PUT</Select.Item>
				<Select.Item value="PATCH">PATCH</Select.Item>
				<Select.Item value="DELETE">DELETE</Select.Item>
			</Select.Content>
		</Select.Root>

		<Input
			type="text"
			placeholder="Enter API URL..."
			bind:value={url}
			class="bg-muted/20 flex-1 font-mono"
		/>

		<Button
			variant="default"
			class="w-24 font-bold tracking-wider"
			onclick={handleSend}
			disabled={isRequesting}
		>
			{isRequesting ? 'Sending...' : 'Send'}
		</Button>
	</header>

	<main class="flex-1 overflow-hidden">
		<Resizable.PaneGroup direction="vertical" class="h-full w-full">
			<Resizable.Pane defaultSize={50} minSize={20}>
				<div class="flex h-full flex-col p-4">
					<Tabs.Root bind:value={activeTab} class="flex h-full flex-col">
						<Tabs.List class="w-fit">
							<Tabs.Trigger value="params">Params</Tabs.Trigger>
							<Tabs.Trigger value="headers">Headers</Tabs.Trigger>
							<Tabs.Trigger value="auth">Auth</Tabs.Trigger>
							<Tabs.Trigger value="body">Body</Tabs.Trigger>
						</Tabs.List>

						<div class="bg-muted/10 mt-4 flex-1 overflow-y-auto rounded-md border p-4">
							<Tabs.Content value="params" class="m-0 h-full">
								<p class="text-muted-foreground text-sm">
									URL Query parameters editor will go here.
								</p>
							</Tabs.Content>
							<Tabs.Content value="headers" class="m-0 h-full">
								<p class="text-muted-foreground text-sm">Header key-value table will go here.</p>
							</Tabs.Content>
							<Tabs.Content value="auth" class="m-0 h-full">
								<p class="text-muted-foreground text-sm">Authentication settings will go here.</p>
							</Tabs.Content>
							<Tabs.Content value="body" class="m-0 h-full">
								<MonacoEditor bind:value={requestBody} language="json" />
							</Tabs.Content>
						</div>
					</Tabs.Root>
				</div>
			</Resizable.Pane>

			<Resizable.Handle withHandle />

			<Resizable.Pane defaultSize={50} minSize={20}>
				<div class="bg-muted/5 flex h-full flex-col p-4">
					<div class="mb-2 flex items-center justify-between">
						<h2 class="text-muted-foreground text-sm font-semibold tracking-wider uppercase">
							Response
						</h2>

						<div class="flex gap-4 font-mono text-xs">
							{#if responseStatus !== null}
								<span class="text-muted-foreground">
									Status: <span
										class="{responseStatus >= 200 && responseStatus < 300
											? 'text-emerald-500'
											: 'text-red-500'} font-bold">{responseStatus}</span
									>
								</span>
								<span class="text-muted-foreground">
									Time: <span class="font-bold text-sky-500">{responseTime}ms</span>
								</span>
							{/if}
						</div>
					</div>

					<div
						class="bg-background flex-1 overflow-y-auto rounded-md border p-4 font-mono text-sm shadow-inner"
					>
						<div
							class="bg-background flex-1 overflow-hidden rounded-md border font-mono text-sm shadow-inner"
						>
							<MonacoEditor bind:value={responseBody} language="json" readOnly={true} />
						</div>
					</div>
				</div>
			</Resizable.Pane>
		</Resizable.PaneGroup>
	</main>
</div>
