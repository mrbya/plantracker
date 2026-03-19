---
id: TASK-38
title: Phase 9.4 — GitLab CI/CD pipeline
status: Done
assignee: []
created_date: '2026-03-19 16:15'
updated_date: '2026-03-19 16:51'
labels:
  - ci
  - devops
  - phase-9
milestone: Phase 9
dependencies:
  - TASK-37
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `.gitlab-ci.yml` with build jobs for Linux and Windows targets.

## Jobs

### linux-build
- Runner: `ubuntu-latest` (or a GitLab shared runner with Docker)
- Install system deps: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`, Node.js, pnpm, Rust stable
- Steps:
  1. `cargo sqlx prepare --check --workspace` (validates offline cache matches schema)
  2. `pnpm install`
  3. `cargo tauri build`
  4. Upload artifacts: `src-tauri/target/release/bundle/deb/*.deb` and `src-tauri/target/release/bundle/appimage/*.AppImage`

### windows-build
- Runner: `windows-latest` (Windows shared runner or self-hosted)
- Install Node.js, pnpm, Rust stable (via rustup)
- Steps:
  1. `cargo sqlx prepare --check --workspace`
  2. `pnpm install`
  3. `cargo tauri build`
  4. Upload artifacts: `src-tauri/target/release/bundle/msi/*.msi` and `src-tauri/target/release/bundle/nsis/*.exe`

## Environment Variables (CI/CD settings)
- `VITE_AZURE_CLIENT_ID` — Azure app client ID
- `VITE_AZURE_TENANT_ID` — Azure tenant ID
- `SQLX_OFFLINE=true` — must be set so sqlx uses the committed `.sqlx/` cache

## Notes
- Pipeline should only run on `master` branch and merge requests
- Cache `~/.cargo` and `node_modules` between runs to speed up builds
- TASK-37 (sqlx cache) must be completed first — `.sqlx/` must be committed before this pipeline can succeed
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `.gitlab-ci.yml` exists at repo root
- [x] #2 linux-build job installs all required system dependencies for Tauri on Ubuntu
- [x] #3 windows-build job builds successfully on Windows runner
- [x] #4 Both jobs run `cargo sqlx prepare --check` to validate the offline cache
- [x] #5 Artifacts are uploaded: .deb, .AppImage (Linux) and .msi, .exe (Windows)
- [x] #6 SQLX_OFFLINE=true is set in the pipeline environment
- [x] #7 VITE_AZURE_CLIENT_ID and VITE_AZURE_TENANT_ID are injected from CI/CD variables
- [x] #8 Pipeline runs on master branch and merge requests
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Created `.gitlab-ci.yml` at repo root with:
- `workflow` rules limiting pipeline to `master` branch and merge requests
- `SQLX_OFFLINE=true` set globally
- Shared cache for `~/.cargo/registry`, `~/.cargo/git`, and `node_modules` keyed on lockfiles
- `linux-build` job on `ubuntu:24.04` image: installs all Tauri system deps (libwebkit2gtk-4.1-dev, libappindicator3-dev, librsvg2-dev, patchelf, etc.), Node.js 20, pnpm, Rust stable, and tauri-cli; runs `cargo sqlx prepare --check`, `pnpm install`, `cargo tauri build`; uploads .deb and .AppImage artifacts
- `windows-build` job on Windows runner (tagged `windows`): installs Node.js, pnpm via Chocolatey, Rust stable via rustup; same build steps; uploads .msi and .exe artifacts
- `VITE_AZURE_CLIENT_ID` and `VITE_AZURE_TENANT_ID` injected from CI/CD variables in both jobs
- Artifacts expire after 7 days
<!-- SECTION:FINAL_SUMMARY:END -->
