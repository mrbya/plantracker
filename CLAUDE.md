# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## About

PlanTracker is a cross-platform desktop app for tracking time spent on Microsoft Planner tasks. Built with Tauri 2 (Rust backend) + Svelte 5 + TypeScript (frontend), with SQLite for local storage and Microsoft Graph API integration.

Cross-platform Tauri desktop app for tracking time spent on Microsoft Planner tasks.

## Stack

| Layer | Technology |
|---|---|
| Desktop shell | Tauri 2 |
| Backend | Rust |
| Database | SQLite via `sqlx` (compile-time checked queries) |
| Frontend | Svelte 5 + TypeScript + Vite |
| Auth | OAuth 2.0 PKCE → Microsoft Identity Platform |
| API | Microsoft Graph Tasks API |
| Theme | Catppuccin Mocha (CSS custom properties) |
| Font | JetBrains Mono Nerd Font |
| Platforms | Linux x86_64, Windows x86_64 |

Data directories:
- Linux: `~/.local/share/PlanTracker/`
- Windows: `%USERPROFILE%\Documents\PlanTracker\`

## File Structure

```
plantracker/
├── src/
│   ├── lib/
│   │   ├── api/index.ts           ← ALL invoke() wrappers live here
│   │   ├── components/ui/         ← Button, Select, Input, Card, Badge, Spinner, EmptyState
│   │   ├── stores/                ← auth.ts, timer.ts, planner.ts, notifications.ts
│   │   ├── theme/                 ← mocha.css, global.css
│   │   └── types.ts               ← TS types mirroring Rust structs (camelCase)
│   ├── views/                     ← TimeTracking, ManualEntry, Reports, Settings, Login
│   └── App.svelte
├── src-tauri/
│   ├── migrations/                ← 0001_initial.sql, 0002_..., etc.
│   ├── src/
│   │   ├── auth/                  ← pkce.rs, oauth.rs, keychain.rs, manager.rs
│   │   ├── commands/              ← auth.rs, sync.rs, timer.rs, entries.rs, reports.rs
│   │   ├── db/                    ← mod.rs, plans.rs, tasks.rs, entries.rs
│   │   ├── graph/                 ← client.rs, models.rs, planner.rs
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── .claude/
│   └── rules/
│       ├── rust.md
│       ├── frontend.md
│       ├── database.md
│       ├── auth.md
│       ├── graph.md
│       ├── ui.md
│       └── pitfalls.md
└── .env.example                   ← VITE_AZURE_CLIENT_ID, VITE_AZURE_TENANT_ID
```

## Development Commands

```bash
# Start dev server (Vite HMR + Rust hot-recompile)
cargo tauri dev

# Type-check frontend without building
pnpm tsc --noEmit

# Lint Rust (warnings are errors)
cargo clippy -- -D warnings

# Regenerate sqlx offline query cache — run after ANY query change
cargo sqlx prepare --workspace

# Build release binaries
cargo tauri build

# Generate all icon sizes from a 1024×1024 PNG
cargo tauri icon assets/icon.png
```

## Rules Index

Detailed rules are in `.claude/rules/`. Claude Code loads all of them automatically.

| File | Covers |
|---|---|
| `rust.md` | Rust idioms, error handling, async, struct conventions |
| `frontend.md` | Svelte 5, TypeScript, stores, component layout |
| `database.md` | sqlx usage, migrations, schema, query patterns |
| `auth.md` | OAuth PKCE flow, token storage, PKCE entropy |
| `graph.md` | Microsoft Graph client, pagination, error handling, scopes |
| `ui.md` | Catppuccin theme, focus styles, loading states, UX patterns |
| `pitfalls.md` | Known footguns specific to this project |

## Out of Scope

Do not suggest or implement unless explicitly instructed:

- Cloud sync or any remote backend beyond Microsoft Graph
- Web-based deployment
- React, Vue, or any framework other than Svelte
- Diesel or any ORM — `sqlx` macros only
- Tauri v1 APIs
- `localStorage` / `sessionStorage` — not reliable across platforms in Tauri WebView
- Frameless or custom-decorated window chrome

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
