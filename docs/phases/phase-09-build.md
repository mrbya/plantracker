# Phase 9 — Build & Distribution

## 9.1 Production build

```bash
# Generate sqlx offline query cache (required for CI builds without a live DB)
cargo sqlx prepare --workspace

# Build
cargo tauri build
```

## 9.2 App icons

Generate icons from a source 1024×1024 PNG:
```bash
cargo tauri icon assets/icon.png
```

## 9.3 Updater (optional)

Configure `tauri-plugin-updater` pointing to a Gitlab Releases endpoint.

## 9.4 Bundle fonts (Done)

JetBrains Mono Nerd Font is self-hosted: all 16 weight/style `.ttf` files live in
`static/fonts/`. They are declared via `@font-face` in `src/lib/theme/fonts.css`
(imported first in `src/app.css`). Users do not need the font installed system-wide.

## 9.5 CI pipeline (Gitlab CI/CD)

Create `.gitlab-ci.yml` with jobs for:
- `ubuntu-latest` — produces `.deb` and `.AppImage`
- `windows-latest` — produces `.msi` and `.exe` (NSIS, cross-compiled from Linux)

Each job:
1. `cargo sqlx prepare --check` (verify query cache is up to date)
2. `pnpm install && cargo tauri build`
3. Upload artifacts

## Verification checklist
- [x] Release build launches on clean Linux and Windows machines
- [x] App icon appears in taskbar and title bar
- [x] No `.env` secrets bundled in release binary (client ID is expected, not a secret)
- [x] Installer creates correct data directory on first run
