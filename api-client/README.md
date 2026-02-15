# API Client

A desktop application for making API requests, built with SvelteKit and Tauri.

## Features

- A modern and reactive user interface with SvelteKit.
- A robust backend powered by Rust and Tauri.
- An integrated Monaco Editor for request/response bodies.

## Tech Stack

- **Frontend:** SvelteKit, TypeScript
- **Backend:** Rust, Tauri
- **Styling:** Tailwind CSS
- **Bundler:** Vite

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) and npm
- [Rust](https://www.rust-lang.org/tools/install) and Cargo

### Installation and Development

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd api-client
    ```

2.  **Install the dependencies:**
    ```bash
    npm install
    ```

3.  **Run the development server:**
    This will start the SvelteKit frontend and the Tauri backend in development mode with hot-reloading.
    ```bash
    npm run tauri dev
    ```

## Building for Production

You can build the application for your target platform using the Tauri CLI.

1.  **Build the application:**
    ```bash
    npm run tauri build
    ```
    This command will bundle the SvelteKit frontend and the Rust backend into a standalone executable for your operating system. The output will be located in `src-tauri/target/release/`.

## Scripts

- `npm run dev`: Starts the SvelteKit development server.
- `npm run build`: Builds the SvelteKit frontend.
- `npm run preview`: Previews the built SvelteKit site.
- `npm run check`: Runs Svelte check to validate the code.
- `npm run lint`: Lints the codebase using ESLint and Prettier.
- `npm run format`: Formats the codebase with Prettier.

## Project Status

This project is in the early stages of development. The basic functionality is in place, but many features are still pending to make it a full-fledged API client.

### Done

- **Core Request/Response:** Can send HTTP requests with a URL, method, and body, and receive a response.
- **HTTP Methods:** Supports GET, POST, PUT, PATCH, and DELETE.
- **Request Body Editor:** Includes a Monaco editor for writing JSON request bodies.
- **Response Viewer:** Displays the response body (with JSON formatting), HTTP status code, and response time.
- **Backend Engine:** Uses a capable Rust backend with the `reqwest` library to handle the actual HTTP logic.
- **UI Structure:** A modern, resizable, tabbed interface is in place for organizing request and response views.

### Pending

- **Request Headers:** The UI for adding/editing request headers needs to be built. The backend is already capable of processing them.
- **URL Query Parameters:** The UI for adding/editing query parameters is a placeholder.
- **Authentication:** The UI for managing authentication (e.g., Bearer Token, Basic Auth) is a placeholder.
- **Response Headers:** The backend sends response headers, but the frontend does not display them.
- **Request History:** The ability to view, search, and re-run past requests.
- **Collections:** The ability to save and organize requests into folders or collections.
- **Environments & Variables:** Support for defining environments (e.g., staging, production) and using variables like `{{base_url}}` in requests.
- **Improved Error Handling:** Displaying errors from the backend in a more structured and user-friendly way.
