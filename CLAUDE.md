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
├── devops/
│   ├── linux-build/Dockerfile     ← CI image for linux-build job
│   └── windows-build/Dockerfile   ← CI image for windows-build job
├── justfile                       ← all dev workflow recipes
└── .env.example                   ← VITE_AZURE_CLIENT_ID, VITE_AZURE_TENANT_ID
```

## Development Commands

All common tasks are managed through [just](https://just.systems/) recipes. Run `just` or `just list` to see all available recipes.

### First-time setup

```bash
just init        # install all required tooling (run once)
just deps        # install node dependencies
```

### Daily development

```bash
just dev         # start dev server (Vite HMR + Rust hot-recompile)
just fmt         # format all sources (JS/TS/Svelte + Rust)
just fmt-js      # format JS/TS/Svelte sources only
just fmt-rs      # format Rust sources only (requires nightly)
just check       # run all linter checks (svelte-check + clippy)
just check-js    # svelte-check only
just check-rs    # clippy only
just precache    # regenerate sqlx offline query cache — run after ANY query change
just build       # build release binaries
```

### Before committing

```bash
just pre-commit  # format check + lint + audit + sqlx cache check + build + index README
```

### Auditing

```bash
just audit       # check for vulnerabilities and unused dependencies
just audit-js    # JS/TS vulnerability audit only
just audit-rs    # Rust unused deps + vulnerability audit only
```

### CI

```bash
just ci-build    # thorough checks + sqlx cache validation + install deps + build
```

### Docker CI images

```bash
just docker-linux   # build and push linux-build image (requires GITLAB_IMAGE_REGISTRY in .env)
```

### Miscellaneous

```bash
just index                         # update README table of contents
cargo tauri icon assets/icon.png   # generate all icon sizes from a 1024×1024 PNG
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
