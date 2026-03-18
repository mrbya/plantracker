# PlanTracker

> A cross-platform desktop app for tracking time spent on Microsoft Planner tasks.

Built with **Tauri + Rust** on the backend and **Svelte + TypeScript** on the frontend. Stores all data locally in SQLite and integrates with the Microsoft Graph Tasks API.

---

## Features

- **Time Tracking** — Start/stop timers against any Planner task
- **Manual Entry** — Log time with explicit start/end times
- **Reports** — Monthly breakdowns, plan/task totals, CSV export
- **Offline-first** — All data stored locally; Microsoft Graph syncs task metadata
- **Secure auth** — OAuth 2.0 PKCE via system browser (no credentials stored in app)

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop shell | [Tauri 2](https://tauri.app/) |
| Backend language | Rust |
| Database | SQLite via [`sqlx`](https://github.com/launchbadge/sqlx) |
| Frontend framework | [Svelte 5](https://svelte.dev/) + TypeScript |
| Build tool | Vite |
| Authentication | OAuth 2.0 PKCE → Microsoft Identity Platform |
| API | [Microsoft Graph Tasks API](https://learn.microsoft.com/en-us/graph/api/resources/plannertask) |
| Theming | Catppuccin Mocha |
| Font | JetBrains Mono Nerd Font |

---

## Prerequisites

- [Rust toolchain](https://rustup.rs/) (stable, 1.77+)
- [Node.js](https://nodejs.org/) 20+ and `pnpm`
- [Tauri CLI v2](https://tauri.app/start/): `cargo install tauri-cli --version "^2"`
- A registered **Azure AD application** (see [Authentication Setup](#authentication-setup))
- JetBrains Mono Nerd Font installed on your system

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

## Getting Started

```bash
# Clone
git clone https://github.com/yourname/plantracker
cd plantracker

# Install frontend dependencies
pnpm install

# Copy environment config
cp .env.example .env
# → Edit .env and add your Azure App Client ID

# Run in development
cargo tauri dev

# Build for release
cargo tauri build
```

---

## Data Storage

All data is stored locally — no cloud sync, no telemetry.

| Platform | Path |
|---|---|
| Linux | `~/.local/share/PlanTracker/` |
| Windows | `%USERPROFILE%\Documents\PlanTracker\` |

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
VITE_AZURE_TENANT_ID=common
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

## Views

### Time Tracking

- Select a **Plan** then a **Task** (or pick a task directly — plan auto-fills)
- Hit **Start** to begin timing; **Stop** to save the entry
- Last *N* entries shown below (configurable in settings)

### Manual Entry

- Same plan/task dropdowns
- Pick **start** and **end** datetime
- Optional **notes** field
- Submit saves directly to SQLite

### Reports

- Select a **date range** (month + year, from/to)
- Select a **plan** (and optionally a **task**)
- Click **Generate** to build the report table
- Table shows: monthly totals, grand total, plan or task total
- **Export CSV** saves the report to your Downloads folder

---

## Project Structure

```
plantracker/
├── src/                        # Svelte frontend
│   ├── lib/
│   │   ├── components/         # Reusable UI components
│   │   ├── stores/             # Svelte stores (auth, plans, tasks, timer)
│   │   ├── api/                # Graph API client (TypeScript)
│   │   └── theme/              # Catppuccin Mocha CSS variables
│   ├── views/
│   │   ├── TimeTracking.svelte
│   │   ├── ManualEntry.svelte
│   │   └── Reports.svelte
│   └── App.svelte
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/           # Tauri commands (time entries, reports)
│   │   ├── db/                 # sqlx migrations + query functions
│   │   ├── auth/               # OAuth PKCE implementation
│   │   └── graph/              # Graph API Rust client
│   ├── migrations/             # SQLite migration files
│   └── Cargo.toml
├── .env.example
├── package.json
└── vite.config.ts
```

---

## Development Notes

- **sqlx compile-time checks**: Run `cargo sqlx prepare` after changing queries to regenerate `.sqlx/` offline query cache
- **Hot reload**: `cargo tauri dev` supports Vite HMR for the frontend; Rust recompiles on backend changes
- **Logging**: Backend uses `tracing` crate; logs viewable in the terminal running `tauri dev`
- **Font**: The app CSS references `'JetBrainsMono Nerd Font'` — ensure it is installed system-wide or bundled

---

## Roadmap

- [ ] Task search / filter in dropdowns
- [ ] Idle detection (pause timer when system is idle)
- [ ] System tray with quick start/stop
- [ ] Sync time entries back to Planner task comments
- [ ] Dark/light theme toggle (Catppuccin Latte)

---

## License

MIT

