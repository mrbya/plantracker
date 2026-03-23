---
id: TASK-54
title: 'Testing: justfile recipes and CI pipeline integration'
status: Done
assignee: []
created_date: '2026-03-23 07:52'
updated_date: '2026-03-23 10:57'
labels:
  - testing
  - ci
  - devops
dependencies:
  - TASK-48
  - TASK-49
  - TASK-50
  - TASK-51
  - TASK-52
  - TASK-53
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Wire tests into the dev workflow and CI pipeline. Covers sections 10.10 and 10.11 of Phase 10.

**10.10 — justfile**

Add the following recipes:

```just
# Runs rust unit tests.
[working-directory: 'src-tauri']
test-rs *FLAGS:
    SQLX_OFFLINE=true cargo test {{FLAGS}}

# Runs frontend unit tests.
test-js *FLAGS:
    pnpm test {{FLAGS}}

# Runs all unit tests.
test:
    @just test-rs
    @just test-js

# Runs frontend unit tests with coverage report.
test-js-coverage:
    pnpm test:coverage
```

Update the `pre-commit` recipe to run tests before the build step:

```just
pre-commit:
    @just thorough-check
    @just test          # ← add before build
    @just unused
    @just audit
    @just precache-check
    @just build
    @just index
```

**10.11 — `.gitlab-ci.yml`**

Add a `test` stage before `build`. Tests run on the same `linux-build` image:

```yaml
stages:
  - test
  - build
  - release

test:
  stage: test
  image: registry.gitlab.com/family-treasure/plantracker/linux-build:latest
  variables:
    SQLX_OFFLINE: "true"
    VITE_AZURE_CLIENT_ID: "test-client-id"
    VITE_AZURE_TENANT_ID: "test-tenant-id"
  script:
    - just deps-ci
    - just test-rs
    - just test-js
  coverage: '/^TOTAL\s+\S+\s+\S+\s+(\d+\.?\d+%)$/'
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: coverage/cobertura-coverage.xml
    expire_in: 1 day
```

The dummy `VITE_AZURE_*` values are required because `dotenvy` attempts to load `.env` at startup even during `cargo test`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 justfile has test-rs, test-js, test, and test-js-coverage recipes
- [x] #2 test-rs sets SQLX_OFFLINE=true and runs from src-tauri working directory
- [x] #3 pre-commit recipe calls just test before just build
- [x] #4 just test runs both Rust and frontend tests end-to-end cleanly
- [x] #5 .gitlab-ci.yml has a test stage defined before build
- [x] #6 CI test job sets SQLX_OFFLINE=true and dummy Azure vars
- [x] #7 CI test job runs just test-rs and just test-js
- [x] #8 Coverage artifact configured with cobertura format
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added test-rs (SQLX_OFFLINE=true cargo test from src-tauri dir), test-js (pnpm test), test (calls both), and test-js-coverage recipes to justfile. Added @just test after @just thorough-check in pre-commit. Updated .gitlab-ci.yml: added test stage before build, added test job with SQLX_OFFLINE=true + dummy Azure vars, running just deps-ci + just test-rs + just test-js, with cobertura coverage artifact. just test runs cleanly: 30 Rust + 10 frontend tests all green.
<!-- SECTION:FINAL_SUMMARY:END -->
