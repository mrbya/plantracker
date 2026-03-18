# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## About

PlanTracker is a cross-platform desktop app for tracking time spent on Microsoft Planner tasks. Built with Tauri 2 (Rust backend) + Svelte 5 + TypeScript (frontend), with SQLite for local storage and Microsoft Graph API integration.

## Project Overview

| Dimension | Detail |
|---|---|
| App name | PlanTracker |
| Purpose | Track time spent on Microsoft Planner tasks, generate reports, export CSV |
| Platforms | Linux (x86_64) + Windows (x86_64) |
| Backend | Tauri 2 + Rust |
| Frontend | Svelte 5 + TypeScript + Vite |
| Database | SQLite via sqlx (with compile-time checked queries) |
| Auth | OAuth 2.0 PKCE → Microsoft Identity Platform (system browser, no embedded webview) |
| API | Microsoft Graph Tasks API |
| Theme | Catppuccin Mocha (CSS custom properties) |
| Font | JetBrains Mono Nerd Font |
| Data dirs | Linux: `~/.local/share/PlanTracker` / Windows: `~/Documents/PlanTracker` |

---

## Behaviour & Response Style

- Be **precise and concise**. Avoid lengthy preambles. Lead with the answer.
- When generating code, produce **complete, runnable code**. Do not use `// ... rest of the code` or similar placeholders.
- When proposing changes to existing code, show only the **changed sections** with enough context to locate them — not the entire file, unless the file is small.
- **Always explain the "why"** for non-obvious decisions (e.g., why a particular sqlx pattern, why a specific OAuth flow step).
- If a question has a clearly better answer, give that answer directly. Do not present 3 equivalent options when one is best.
- Assume proficiency in both Rust and TypeScript/Svelte. Skip beginner-level explanation unless asked.

---

## Rust Conventions

- Use `anyhow::Result` for error propagation inside library code
- Use `String` errors (`.map_err(|e| e.to_string())`) at the Tauri command boundary for serialization
- All async code uses Tokio
- Use `tracing::info!`, `tracing::error!` etc. for logging — never `println!`
- Prefer `sqlx::query_as!` macro for compile-time checked queries
- Model IDs: use `uuid::Uuid::new_v4().to_string()` for local DB IDs; Graph IDs stored separately in `graph_id` columns
- DateTime handling: store as ISO 8601 strings in SQLite (`TEXT` column); parse to `chrono::DateTime<Utc>` in Rust
- Manage shared mutable state via `tokio::sync::Mutex<T>` wrapped in Tauri managed state
- Struct naming: PascalCase; command function naming: snake_case matching the frontend invoke name exactly

---

## Svelte / TypeScript Conventions

- Svelte 5 runes are preferred over legacy `$:` reactive syntax
- All `invoke()` calls must go through `src/lib/api/index.ts` — views and stores must not call `invoke` directly
- Types live in `src/lib/types.ts` and must mirror Rust structs (using camelCase)
- Stores live in `src/lib/stores/` — one file per domain (auth, timer, planner, notifications)
- Components live in `src/lib/components/` (reusable) or `src/views/` (full-page views)
- No external CSS frameworks — all styling uses Catppuccin Mocha CSS variables defined in `src/lib/theme/mocha.css`
- No inline styles except for truly dynamic values (e.g., elapsed timer width)
- Always handle loading and error states explicitly — never leave a UI in an ambiguous state

---

## Database Rules

- Never use raw string queries — always use `sqlx::query!` or `sqlx::query_as!` macros
- After adding or changing any query, remind the user to run `cargo sqlx prepare` to regenerate the offline query cache
- Migrations go in `src-tauri/migrations/` numbered `0001_`, `0002_`, etc.
- Never perform migrations manually — always use `sqlx::migrate!()` in `init_db()`
- Foreign key constraints must be enabled at connection time: `PRAGMA foreign_keys = ON;`

---

## Authentication Rules

- Tokens are stored in the OS keychain via the `keyring` crate — never in `tauri-plugin-store`, environment variables, or plain files
- The OAuth redirect must use `http://localhost:52721/callback` consistently across Azure AD config and Rust code
- The PKCE code verifier must be at least 43 characters (43–128 characters per RFC 7636)
- Always validate the `state` parameter on the redirect callback to prevent CSRF
- Access token refresh must be transparent — commands call `auth_manager.get_valid_token()` and never handle expiry themselves
- Never log access tokens or refresh tokens at any log level

---

## Microsoft Graph API Rules

- Base URL: `https://graph.microsoft.com/v1.0`
- Always handle OData pagination (`@odata.nextLink`) in list endpoints
- Minimum required scopes: `Tasks.Read offline_access User.Read`
- Graph API errors must be surfaced to the user with the Graph `message` field, not just the HTTP status code
- Cache plans and tasks locally in SQLite after each sync — the app must be usable while offline (read-only)

---

## UI / UX Rules

- Every Catppuccin Mocha color must be referenced as its CSS variable (e.g., `var(--ctp-mauve)`), never hardcoded
- Interactive elements must have visible focus styles (accent-colored outline)
- Every async action must show a loading state and disable the triggering control while pending
- Destructive actions (delete entry, sign out) must have a confirmation step
- The task dropdown must always auto-select the parent plan when a task is chosen
- Duration display format: `2h 34m` in UI; `2:34:00` in CSV exports
- Toast notifications for all errors and confirmations — no native `alert()` or `confirm()` dialogs

---

## File Structure Reference

```
plantracker/
├── src/
│   ├── lib/
│   │   ├── api/index.ts           ← all invoke() wrappers
│   │   ├── components/ui/         ← Button, Select, Input, Card, Badge, Spinner, EmptyState
│   │   ├── stores/                ← auth.ts, timer.ts, planner.ts, notifications.ts
│   │   ├── theme/                 ← mocha.css, global.css
│   │   └── types.ts               ← TypeScript types mirroring Rust structs
│   ├── views/                     ← TimeTracking, ManualEntry, Reports, Settings, Login
│   └── App.svelte
├── src-tauri/
│   ├── migrations/
│   ├── src/
│   │   ├── auth/                  ← pkce.rs, oauth.rs, keychain.rs, manager.rs
│   │   ├── commands/              ← auth.rs, sync.rs, timer.rs, entries.rs, reports.rs
│   │   ├── db/                    ← mod.rs, plans.rs, tasks.rs, entries.rs
│   │   ├── graph/                 ← client.rs, models.rs, planner.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
└── .env.example
```

---

## Common Pitfalls to Avoid

- **sqlx without offline cache**: `cargo sqlx prepare` must be run before committing query changes; CI builds will fail otherwise
- **tauri-plugin-oauth port conflicts**: Port 52721 is hardcoded — ensure it matches the Azure AD redirect URI exactly
- **Mutex deadlocks**: Never hold a Tokio `Mutex` lock across an `.await` point; use a block or clone the value out first
- **Window close with active timer**: Handle the `window-close` event in Tauri to warn the user if a timer is running
- **Graph rate limits**: Microsoft Graph enforces per-user throttling; add exponential backoff on 429 responses
- **SQLite datetime comparisons**: SQLite compares TEXT lexicographically — ISO 8601 format (`YYYY-MM-DDTHH:MM:SSZ`) is required for correct ordering and range queries
- **PKCE verifier entropy**: Do not use `rand::random::<u64>()` directly — use `rand::RngCore::fill_bytes` on a `[u8; 64]` buffer

---

## Development Commands Reference

```bash
# Dev server
cargo tauri dev

# Type-check frontend only
pnpm tsc --noEmit

# Regenerate sqlx query cache (run after any query change)
cargo sqlx prepare --workspace

# Lint Rust
cargo clippy -- -D warnings

# Build release
cargo tauri build

# Generate app icons from source PNG
cargo tauri icon assets/icon.png
```

---

## Out of Scope

Do not suggest or implement these unless explicitly asked:
- Any cloud sync or remote backend beyond Microsoft Graph
- Web-based deployment of this app
- React, Vue, or any frontend framework other than Svelte
- Diesel or any ORM — sqlx only
- Tauri v1 APIs (this project uses Tauri v2)
- `localStorage` or `sessionStorage` (not available in Tauri WebView in a reliable cross-platform way)
- Custom window decorations or frameless window (keep standard OS chrome)


<!-- BACKLOG.MD MCP GUIDELINES START -->

<CRITICAL_INSTRUCTION>

## BACKLOG WORKFLOW INSTRUCTIONS

This project uses Backlog.md MCP for all task and project management activities.

**CRITICAL GUIDANCE**

- If your client supports MCP resources, read `backlog://workflow/overview` to understand when and how to use Backlog for this project.
- If your client only supports tools or the above request fails, call `backlog.get_workflow_overview()` tool to load the tool-oriented overview (it lists the matching guide tools).

- **First time working here?** Read the overview resource IMMEDIATELY to learn the workflow
- **Already familiar?** You should have the overview cached ("## Backlog.md Overview (MCP)")
- **When to read it**: BEFORE creating tasks, or when you're unsure whether to track work

These guides cover:
- Decision framework for when to create tasks
- Search-first workflow to avoid duplicates
- Links to detailed guides for task creation, execution, and finalization
- MCP tools reference

You MUST read the overview resource to understand the complete workflow. The information is NOT summarized here.

</CRITICAL_INSTRUCTION>

<!-- BACKLOG.MD MCP GUIDELINES END -->
