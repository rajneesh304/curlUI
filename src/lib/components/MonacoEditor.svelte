<script lang="ts">
	import * as monaco from 'monaco-editor';
	import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
	import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';

	let { value = $bindable(''), language = 'json', readOnly = false } = $props();

	// The Svelte Action handles the complete lifecycle imperatively
	function editorAction(node: HTMLElement, initialValue: string) {
		// 1. Setup Web Workers (Vite requirement)
		if (typeof self !== 'undefined' && !self.MonacoEnvironment) {
			self.MonacoEnvironment = {
				getWorker: function (_moduleId: any, label: string) {
					if (label === 'json') return new jsonWorker();
					return new editorWorker();
				}
			};
		}

		// 2. Create the Editor instance
		const editor = monaco.editor.create(node, {
			value: initialValue,
			language,
			readOnly,
			theme: 'vs-dark',
			minimap: { enabled: false },
			automaticLayout: true,
			scrollBeyondLastLine: false,
			padding: { top: 16, bottom: 16 }
		});

		// 3. Sync Editor -> Svelte (When the user types)
		const listener = editor.onDidChangeModelContent(() => {
			const currentValue = editor.getValue();
			if (value !== currentValue) {
				value = currentValue;
			}
		});

		// 4. Svelte Action Lifecycle Hooks
		return {
			// Sync Svelte -> Editor (When the Rust network response arrives)
			update(newValue: string) {
				if (editor && editor.getValue() !== newValue) {
					editor.setValue(newValue);
				}
			},
			// Cleanup to prevent memory leaks when switching tabs
			destroy() {
				listener.dispose();
				editor.dispose();
			}
		};
	}
</script>

<div class="relative h-full min-h-[200px] w-full">
	<div use:editorAction={value} class="absolute inset-0"></div>
</div>
