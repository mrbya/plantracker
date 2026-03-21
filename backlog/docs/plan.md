# PlanTracker — Implementation Phase Overview

High-level summary of implementation phases. For full detail, see `docs/IMPLEMENTATION_PLAN.md`.

## Phases

| Phase | Title | Status |
|---|---|---|
| 0 | Project Scaffold | Done |
| 1 | Theme & Design System | Done |
| 2 | Database Layer | Done |
| 3 | Authentication | Done |
| 4 | Microsoft Graph Integration | Done |
| 5 | Time Tracking View | Done |
| 6 | Manual Entry View | Done |
| 7 | Reports View | Done |
| 8 | Settings & Polish | Done |
| 9 | Build & Distribution | In Progress |

## Phase 9 Notes

### 9.4 — CI Pipeline (GitLab CI/CD)

`.gitlab-ci.yml` uses `spec.inputs` with a `run_image` boolean to toggle between normal build jobs and image-build jobs.

- `linux-build`: builds Linux artifacts (`.deb`, `.AppImage`) using `registry.gitlab.com/family-treasure/plantracker/linux-build:latest`
- `windows-build`: builds Windows artifacts (`.msi`, `.exe`) on tag pushes using `registry.gitlab.com/family-treasure/plantracker/windows-build:latest`
- `upload-release`: uploads artifacts to GitLab Generic Packages Registry and creates a Release entry (runs on tags only)
- `windows-image`: builds and pushes the Windows Docker image (self-hosted `windows-docker` runner required); only runs when `run_image` input is `true`

Linux Docker build image is built via `just docker-linux`. The Windows image CI job (`windows-image`) runs inline PowerShell since bash is unavailable on Windows runners.

### 9.5 — Bundle Fonts (Done)

JetBrains Mono Nerd Font is **self-hosted**: all 16 weight/style `.ttf` files are stored in `static/fonts/` and loaded via `@font-face` declarations in `src/lib/theme/fonts.css`. This file is imported first in `src/app.css`. Users do not need the font installed system-wide on either Linux or Windows.
