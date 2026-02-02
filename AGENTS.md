# Repository Guidelines

## Project Structure & Module Organization
- `app/` is the Svelte 5 + Vite frontend (not SvelteKit). Source lives in `app/src/`, static assets in `app/public/`, and shared UI components under `app/src/lib/components/`.
- `api/` is the Rust backend workspace (Axum + Tokio + SQLite with async rusqlite). The HTTP API lives in `api/src/api/`, shared library code in `api/src/lib/`, and a WASM app in `api/src/app/`. Static files served by the API are in `api/public/`.
- The frontend is a PWA; it will run Rust code in a WASM module and use SQLite backed by IndexedDB.
- The SQLite database defaults to `api/pocketplanner.db`.
- Frontend API calls are proxied under `/api` in dev (Vite) and prod (nginx).

## Build, Test, and Development Commands
Frontend (run from `app/`):
- `pnpm install` sets up dependencies.
- `pnpm dev` runs the Vite dev server.
- `pnpm build` creates the production build.
- `pnpm preview` serves the production build locally.
- `pnpm check` runs `svelte-check` and TypeScript typechecking.

Backend (run from `api/`):
- `cargo run -p api` starts the Axum API server on `[::]:8080`.
- `cargo build` builds the workspace.
- `./build.sh` builds the WASM package and copies artifacts into `api/public/`.

## Coding Style & Naming Conventions
- TypeScript/Svelte uses 2-space indentation (follow existing `.svelte`/`.ts` files).
- Rust uses `rustfmt` defaults; run `cargo fmt` before committing Rust changes.
- Prefer descriptive names: `snake_case` for Rust items, `camelCase` for TS variables/functions, `PascalCase` for Svelte components.

## Testing Guidelines
- There are no dedicated test suites in the repo right now. Use `pnpm check` for frontend validation and `cargo check` for backend compile checks.
- If you add tests, place frontend tests alongside source in `app/src/` and Rust tests in `api/src/**` using standard `#[cfg(test)]` modules.

## Commit & Pull Request Guidelines
- Git history shows short, informal commit messages with no enforced convention. Keep messages concise and descriptive (imperative tense is fine).
- PRs should include: a brief summary, a list of key changes, and screenshots for UI updates (e.g., new Svelte pages).

## Security & Configuration Tips
- The API reads `.env` via `dotenvy`. `DATABASE_PATH` can override the default SQLite file; example: `DATABASE_PATH=./pocketplanner.db`.
- Google auth client ID is injected at build time via `app/.env.development` and `app/.env.production` (`VITE_GOOGLE_CLIENT_ID`).
- API config includes `G_CLIENT_IDS` for JWT audience validation and `COOKIE_SECURE` for HTTPS-only cookies.
- Avoid committing secrets; prefer local `.env` files.
