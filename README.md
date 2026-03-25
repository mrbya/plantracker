# PlanTracker
[![Latest Release](https://gitlab.com/family-treasure/plantracker/-/badges/release.svg)](https://gitlab.com/family-treasure/plantracker/-/releases)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)
[![pipeline status](https://gitlab.com/byacrates/hoy/badges/master/pipeline.svg)](https://gitlab.com/byacrates/hoy/-/commits/master)

> A cross-platform desktop app for tracking time spent on Microsoft Planner tasks.

Built with **Tauri + Rust** on the backend and **Svelte + TypeScript** on the frontend. Stores all data locally in an SQLite database and integrates with the Microsoft Graph Tasks API to sync MS Planner plans and task.

## Index

<!-- toc -->

- [Features](#features)
- [Tech Stack](#tech-stack)
  * [Platform-specific dependencies](#platform-specific-dependencies)
- [Views](#views)
  * [Time Tracking](#time-tracking)
  * [Manual Entry](#manual-entry)
  * [Reports](#reports)
- [Data Storage](#data-storage)
  * [Database Schema (overview)](#database-schema-overview)
- [Development](#development)
  * [Prerequisites](#prerequisites)
  * [Getting Started](#getting-started)
  * [Documentation](#documentation)
    + [Structure](#structure)
    + [Generate Docs](#generate-docs)
- [Project Structure](#project-structure)
- [Authentication Setup](#authentication-setup)
  * [1. Register an Azure AD Application](#1-register-an-azure-ad-application)
  * [2. Configure API Permissions](#2-configure-api-permissions)
  * [3. Configure the App](#3-configure-the-app)
  * [Auth Flow (how it works at runtime)](#auth-flow-how-it-works-at-runtime)
- [Development Notes](#development-notes)
- [Roadmap](#roadmap)
- [License](#license)

<!-- tocstop -->

---

## Features

- **Time Tracking** — Start/stop timers against any Planner task
- **Manual Entry** — Log time with explicit start/end times
- **Reports** — Grand total + per-entry breakdown, plan/task filter, CSV export
- **Offline-first** — All data stored locally; Microsoft Graph syncs task metadata
- **Secure auth** — OAuth 2.0 PKCE via system browser (no credentials stored in app)

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop shell | [Tauri 2](https://tauri.app/) |
| Backend language | [Rust](https://rust-lang.org/) |
| Database | [SQLite](https://sqlite.org/) via [`sqlx`](https://github.com/launchbadge/sqlx) |
| Frontend framework | [Svelte 5](https://svelte.dev/) + [TypeScript](https://www.typescriptlang.org/) |
| Build tool | [Vite](https://vite.dev/) |
| Authentication | [OAuth 2.0 PKCE](https://www.oauth.com/oauth2-servers/pkce/) → [Microsoft Identity Platform](https://learn.microsoft.com/en-us/entra/identity-platform/) |
| API | [Microsoft Graph Tasks API](https://learn.microsoft.com/en-us/graph/api/resources/plannertask) |
| Theming | [Catppuccin Mocha](https://catppuccin.com/palette/) (Dark - `Mocha`, Light - `Latte`)|
| Font | [JetBrains Mono Nerd Font](https://www.programmingfonts.org/#jetbrainsmono) |

---

### Platform-specific dependencies

**Linux**
```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

**Windows**
- WebView2 (ships with Windows 11; install from [Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) on Windows 10)
- Visual Studio C++ build tools

---

## Views

### Time Tracking

- Select a **Plan** then a **Task**
- Hit **Start** to begin timing; **Stop** to save the entry
- Last *N* entries shown below (configurable in settings)

### Manual Entry

- Same plan/task dropdowns
- Pick **start** and **end** datetime
- Optional **notes** field
- Submit to save

### Reports

- Select a **date range** (month + year, from/to)
- Select a **plan** (and optionally a **task**)
- Click **Generate** to build the report table
- Table shows: grand total at top, then individual entries (task, plan, start, end, duration, notes)
- **Export CSV** to export report to a .csv file

---

## Data Storage

All data is stored locally — no cloud sync, no telemetry.

| Platform | Path |
|---|---|
| Linux | `~/.local/share/com.siemens.plantracker/` |
| Windows | `%USERPROFILE%\Documents\com.siemens.plantracker\` |

The directory contains:
- `plantracker.db` — SQLite database (time entries, cached task/plan metadata)
- `config.json` — App preferences (managed by `tauri-plugin-store`)

### Database Schema (overview)

```
plans          id, graph_id, title, synced_at
tasks          id, graph_id, plan_id, title, synced_at
time_entries   id, task_id, start_time, end_time, notes, created_at
```

---

## Development

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable, 1.77+)
- [Node.js](https://nodejs.org/) 20+ and `pnpm`
- [Tauri CLI v2](https://tauri.app/start/): `cargo install tauri-cli --version "^2"` or bootstrap using `just init` (see [Getting Started](#getting-started))
- A registered **Azure Entra ID application** (see [Authentication Setup](#authentication-setup))

### Getting Started

Requires `just` to bootstrap all tools and configuration

```bash
cargo install just
just init # setup repo and all required tools
```

Run in development:
```bash
just dev
```

Build for release:
```bash
just build
```

Before committing work:
```bash
just pre-commit
```

To see all available commands:
```bash
just list
```

---

### Documentation

#### Structure

Project documentation is comprised of this README and 3 sets of generated docs:

1. Rust backend docs generated using `just docs-rs`:
    - Uses `cargo doc`
    - Covers backend API commands, auth, database, Graph integration, shared models, and internal backend modules

2. Frontend API docs generated using `just docs-api`:
    - Uses `TypeDoc`
    - Covers fronted typescript API - backend command invoke wrappers, type mirrors, utils, stores, route `.ts` mdules
    - Includes docs from `docs/frontend-api`

3. Frontend UI docs generated using `just docs-ui`:
    - Uses `Storybook`
    - Covers UI elements showcases, animations, usage examples and more
    - Includes overview from `.storybook/docs`

All these are strapped together using a custom landing page from `docs/landing-page`

#### Generate Docs

To generate the full project docs suite use `just docs`
To preview generated docs page use `just docs-show`

## Project Structure

```
plantracker/
├── src/                              # Svelte frontend
│   ├── lib/
│   │   ├── api/
│   │   │   └── index.ts              # All invoke() wrappers (single boundary)
│   │   │
│   │   ├── components/
│   │   │   ├── ui/                   # Reusable UI components
│   │   │   ├── Layout.svelte         # App shell with sidebar navigation
│   │   │   └── ToastContainer.svelte # Toast notification renderer
│   │   │
│   │   ├── stores/                   # Svelte stores (auth, planner, timer, notifications, settings)
│   │   ├── theme/                    # CSS theming variables
│   │   ├── types.ts                  # TypeScript types mirroring Rust structs
│   │   │
│   │   └── utils/
│   │       ├── datetime.ts           # Utilities to format datetimes
│   │       └── duration.ts           # Utilities to format durations
│   │
│   ├── routes/                       # Frontend API routes
│   │   ├── +layout.svelte
│   │   ├── +layout.ts
│   │   └── +page.svelte
│   │
│   ├── stories/                      # Storybook stories
│   │   ├── __mocks__/                # In-browser mocks for Tauri native modules
│   │   ├── views/                    # Stories for full-page views
│   │   └── *.stories.svelte          # Stories for UI primitives
│   │
│   └── views/                        # Full-page views
│
├── src-tauri/                        # Tauri / Rust backend
│   ├── src/
│   │   ├── auth/                     # OAuth PKCE: pkce, oauth, keychain, manager
│   │   ├── commands/                 # Tauri commands: auth, sync, timer, entries, reports, settings
│   │   ├── db/                       # sqlx query functions: plans, tasks, entries
│   │   ├── graph/                    # Microsoft Graph client + models
│   │   ├── models.rs                 # Shared Rust structs (serde, camelCase)
│   │   ├── lib.rs                    # Backend crate lib definitions and setup
│   │   └── main.rs                   # Tauri backend entrypoint
│   │
│   ├── migrations/                   # SQLite migration files (0001_initial.sql, …)
│   ├── .sqlx/                        # Offline DB query cache for compile-time checks
│   └── Cargo.toml
│
├── static/                           # Bundled static artefacts (fonts, icons, etc.)
│   └── fonts/
│
├── .storybook/                       # UI docs configuration
│
├── devops/
│   ├── linux-build/Dockerfile        # CI image for Linux builds
│   └── windows-build/Dockerfile      # CI image for Windows builds
│
├── .gitlab-ci.yml                    # CI/CD pipeline (build + release upload)
├── justfile                          # Dev workflow recipes (run `just list` to list all)
├── .env.example                      # Example .env file used to generate local .env file for dev
├── ...
└── vite.config.ts
```

---

## Authentication Setup

PlanTracker uses the **OAuth 2.0 Authorization Code + PKCE** flow. Tokens are obtained via the user's default system browser and stored securely in the OS keychain via the `keyring` crate. No passwords or secrets are ever stored in plain text.

### 1. Register an Azure AD Application

1. Go to [portal.azure.com](https://portal.azure.com) → **Azure Active Directory** → **App registrations** → **New registration**
2. Name: `PlanTracker` (or anything you like)
3. Supported account types: *Accounts in any organizational directory and personal Microsoft accounts*
4. Redirect URI: choose **Public client/native (mobile & desktop)** → `http://localhost:52721/callback`
5. After creation, note your **Application (client) ID**

### 2. Configure API Permissions

In your app registration, go to **API permissions** → **Add a permission** → **Microsoft Graph** → **Delegated permissions**, and add:

| Permission | Purpose |
|---|---|
| `Tasks.Read` | Read Planner tasks and plans |
| `Tasks.ReadWrite` | (Optional) Write task updates back |
| `offline_access` | Receive refresh tokens |
| `User.Read` | Display signed-in user info |

Click **Grant admin consent** if you have admin rights, or instruct users to consent on first login.

### 3. Configure the App

In your `.env` file:

```env
VITE_AZURE_CLIENT_ID=your-client-id-here
VITE_AZURE_TENANT_ID=your-tenant-id-here
GITLAB_IMAGE_REGISTRY=registry.gitlab.com/family-treasure/plantracker
```

> `common` allows both personal and work/school Microsoft accounts. Replace with your tenant ID to restrict to a single organisation.

### Auth Flow (how it works at runtime)

```
User clicks "Sign In"
  → Tauri opens system browser to Microsoft login URL (with PKCE challenge)
  → tauri-plugin-oauth spins up localhost:52721 to catch the redirect
  → Authorization code received
  → Rust backend exchanges code for access + refresh tokens
  → Tokens stored in OS keychain (libsecret on Linux, Windows Credential Manager)
  → App fetches plans/tasks from Graph API
  → Tokens silently refreshed as needed
```

---

## Development Notes

- **sqlx compile-time checks**: Run `just precache` after changing queries to regenerate `.sqlx/` offline query cache
- **Hot reload**: `just dev` supports Vite HMR for the frontend; Rust recompiles on backend changes
- **Logging**: Backend uses `tracing` crate; logs visible in the terminal running `tauri dev`
- **Font**: JetBrainsMono Nerd Font is bundled in `static/fonts/` and loaded via `@font-face` — no system installation required

---

## Roadmap

- [x] Dark/light theme toggle (Catppuccin Latte)
- [x] Task search / filter in dropdowns
- [ ] Multi-lang localizations
- [ ] Idle detection (pause timer when system is idle)
- [ ] System tray with quick start/stop
- [ ] Sync time entries back to Planner task comments

---

## License

[MIT](LICENSE)

