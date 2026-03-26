---
id: TASK-78.6
title: Update build tooling and CI for i18n (§12.2.8–12.2.9)
status: Done
assignee: []
created_date: '2026-03-26 07:16'
updated_date: '2026-03-26 07:56'
labels:
  - frontend
  - i18n
  - devops
  - phase-12
dependencies: []
parent_task_id: TASK-78
priority: medium
ordinal: 6000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a `just i18n` recipe to the Justfile, add the compiled output to `.gitignore`, and update CI to run the compile step before building.

## Justfile recipe

```just
# Extract and compile message files (run after adding new keys)
i18n:
    pnpx @inlang/paraglide-js compile \
        --project ./project.inlang \
        --outdir ./src/lib/paraglide
```

Add `i18n` as a dependency of `build` so compiled message modules are always up-to-date before a release build:

```just
build: i18n
    cargo tauri build
```

## `.gitignore`

The compiled output in `src/lib/paraglide/` is **generated** and must not be committed:

```
src/lib/paraglide/
```

## CI (`.gitlab-ci.yml`)

Update the `build` job script to run the compile step before `just ci-build`:

```yaml
script:
  - just i18n
  - just ci-build
```

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.2.8 and §12.2.9
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 just i18n recipe exists in justfile and compiles message files to src/lib/paraglide/
- [ ] #2 build recipe in justfile depends on i18n (i18n runs before cargo tauri build)
- [ ] #3 src/lib/paraglide/ is listed in .gitignore
- [ ] #4 .gitlab-ci.yml build job script runs 'just i18n' before 'just ci-build'
<!-- AC:END -->
