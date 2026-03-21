# Changelog

All notable changes to PlanTracker are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [0.1.0] — 2026-03-21

First release.

### Added

#### Authentication
- OAuth 2.0 Authorization Code + PKCE flow via the system browser — no credentials stored in the app
- Tokens stored securely in the OS keychain (libsecret on Linux, Windows Credential Manager)
- Silent token refresh on expiry; app stays signed in across restarts
- Sign-out clears all stored tokens

#### Microsoft Planner sync
- Fetches plans and tasks from the Microsoft Graph Tasks API on login and on demand
- Results cached locally in SQLite so the app is fully usable offline
- Background sync configurable: manual only, every 30 minutes, or every hour
- Sync status and last-synced timestamp visible in Settings

#### Time Tracking
- Start/stop timer against any Planner task with a single click
- Elapsed time shown live in the Stop button while a timer is running
- Active timer survives app restarts — automatically restored from the database on next launch
- Warning on window close when a timer is still running
- Cannot start a second timer while one is already active

#### Manual Entry
- Log time with an explicit date + 24 h time (HH:MM) for both start and end
- Date picker with separate 24 h time field; fields pre-filled with today's date and current time
- Edit existing entries: form pre-populates with the entry's saved values
- Inline per-field validation with clear error messages
- Optional notes field per entry

#### Reports
- Monthly breakdown of tracked time, scoped to a plan or an individual task
- Configurable date range (from month/year — to month/year)
- Grand total row in the report table
- Export to CSV via a native save-file dialog

#### Settings
- Recent entries limit (controls how many entries appear in Time Tracking and Manual Entry)
- Sync frequency selector
- Data directory path display with Open Folder button
- Account section: signed-in user display name and Sign Out

#### UI & theming
- Catppuccin Mocha dark theme throughout (CSS custom properties, no external CSS framework)
- JetBrains Mono Nerd Font bundled in the app — no system font installation required
- Sidebar navigation with lucide-svelte SVG icons
- Toast notifications for all user-facing feedback (success, error, warning); auto-dismiss after 4 s
- Empty states, loading spinners, and disabled controls during pending operations throughout
- All datetime displays in 24 h format regardless of system locale

#### Infrastructure
- SQLite local database via `sqlx` with compile-time checked queries and migration runner
- GitLab CI/CD pipeline: Linux build (`.deb` package) and Windows cross-compiled build (nsis `.exe` installer)
- Artifacts uploaded to GitLab Generic Packages Registry on tag pushes; release created automatically
- Docker CI image for CI/CD builds
- Full dev workflow via `just` recipes (`just dev`, `just build`, `just check`, `just pre-commit`, and more)

### Platforms

| Platform | Targets |
|---|---|
| Linux x86_64 | `.deb` |
| Windows x86_64 | `.exe` (NSIS), `.msi` (if manually built on win) |
