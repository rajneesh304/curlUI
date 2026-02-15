# Project: Cross-Platform API Client (Curl GUI)
# Tech Stack: Tauri v2, Svelte 5, Rust, Tailwind CSS, shadcn-svelte, Monaco Editor

## Core Architectural Rules
* **Separation of Concerns:** The Svelte frontend is strictly a presentation and state management layer. It MUST NOT make direct external HTTP requests. All external networking MUST be routed through Tauri IPC commands to the Rust backend.
* **Svelte 5 Strict Mode:** Always use Svelte 5 Runes (`$state`, `$derived`, `$effect`, `$props`). Do not use Svelte 4 syntax (`export let`, `$:`, or `svelte/store`). 
* **Static Site Generation:** The frontend is a Single Page Application (SPA). SvelteKit must be configured with `@sveltejs/adapter-static`. Server-side rendering (SSR) must be disabled (`ssr = false`, `prerender = true` in `+layout.ts`).
* **Backend Networking:** The Rust backend uses `reqwest` and `tokio`. It must handle CORS, headers, and payloads, returning serialized JSON (`ResponsePayload`) to the frontend.
* **Security:** Tauri CSP must allow `style-src 'unsafe-inline'` exclusively to support the Monaco Editor rendering.
* **Styling:** Use Tailwind CSS and shadcn-svelte components.

## Data Structures
* `RequestPayload`: { url: string, method: string, headers: Record<string, string>, body: string }
* `ResponsePayload`: { status: number, time_ms: number, headers: Record<string, string>, body: string, error?: string }