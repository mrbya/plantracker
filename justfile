#!/usr/bin/env just --justfile
set dotenv-load := true

# Output this list.
list:
    @just --list

# Installs node deps.
deps *FLAGS:
    pnpm install {{FLAGS}}

# Installs node deps with --frozen-lockfile.
deps-ci:
    @just deps --frozen-lockfile

# Apply strict formatting to js/ts/svelte sources.
fmt-js:
    pnpm format

# Checks formatting of js/ts/svelte sources.
fmt-js-check:
    pnpm format-check

# Apply strict formatting to rust sources.
[working-directory: 'src-tauri']
fmt-rs *FLAGS:
    cargo +nightly fmt --all {{FLAGS}}

# Apply strict formatting to all sources.
fmt:
    @just fmt-js
    @just fmt-rs

# Runs svelte-check on js/ts/svelte sources.
check-js *FLAGS:
    pnpm check {{FLAGS}}

# Runs clippy on rs sources, tests, examples, while testing all features.
[working-directory: 'src-tauri']
check-rs *FLAGS:
    cargo clippy --tests --examples --all-targets --all-features --workspace {{FLAGS}}

# Runs linter checks on sources.
check:
    @just check-js
    @just check-rs

# Checks for unused dependencies in js/ts/svelte sources.
unused-js:
    pnpm unused

# Checks for unused dependencies in rust sources.
[working-directory: 'src-tauri']
unused-rs:
    cargo +nightly udeps --all-targets

# Checks for unused dependencies in all sources.
unused:
    @just unused-js
    @just unused-rs

# Audits for js source vulnerabilities.
audit-js *FLAGS:
    pnpm audit --prod {{FLAGS}}

# Audits for vulnerabilities in rust sources.
[working-directory: 'src-tauri']
audit-rs *FLAGS:
    cargo audit {{FLAGS}}

# Audits sources for vulnerabilities and unused deps.
audit:
    @just audit-js
    @just audit-rs

# Runs rust unit tests.
[working-directory: 'src-tauri']
test-rs *FLAGS:
    SQLX_OFFLINE=true cargo test --workspace --doc
    SQLX_OFFLINE=true cargo test {{FLAGS}}

# Runs frontend unit tests.
test-js *FLAGS:
    pnpm test {{FLAGS}}

# Runs all unit tests.
test:
    @just test-rs
    @just test-js

# Runs tests with a coverage report for js/ts/svelte sources.
test-cov-js:
    pnpm test:coverage

# Runs tests with a coverage report for rs sources.
[working-directory: 'src-tauri']
test-cov-rs *FLAGS:
    cargo llvm-cov nextest --all-features --workspace {{FLAGS}}

# Runs tests with a coverage report for rs sources.
test-cov:
    @just test-cov-js
    @just test-cov-rs

# Pre caches db queries.
[working-directory: 'src-tauri']
precache *FLAGS:
    #!/usr/bin/env bash
    set -euo pipefail
    SCRATCH=$(mktemp -d)
    DB="sqlite:${SCRATCH}/prepare.db"
    cargo sqlx database create -D "$DB"
    cargo sqlx migrate run -D "$DB"
    cargo sqlx prepare --workspace -D "$DB" {{FLAGS}}

# Chechs pre-cached db queries.
precache-check:
    @just precache --check

# Generates icon pack from the icon.svg provided in src-tauri/icons.
icons:
    cargo tauri icon static/icon.svg

# Run app in development.
dev *FLAGS:
    cargo tauri dev {{FLAGS}}

# Builds app release.
build:
    cargo tauri build

# Build app release setup for windows.
build-windows:
    cargo tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc

# A thorough codebase check ran before running ci-builds.
thorough-check:
    @just fmt-js-check
    @just fmt-rs --check
    @just check-js
    @just check-rs -- -D warnings

# Indexes README.
index:
    pnpm index README.md

# Generates documentation for rs sources.
[working-directory: 'src-tauri']
docs-rs *FLAGS:
    RUSTDOCFLAGS="--default-theme ayu" cargo doc --no-deps --all-features --document-private-items --workspace {{FLAGS}}

# Generates documentation for frontend API.
docs-api:
    pnpm docs:api

# Generates documentation for frontend UI.
docs-ui:
    pnpm docs:ui

# Generates full project documentation.
docs:
    pnpm docs:build

# Runs and opens generated project docs in a http server.
docs-show:
    pnpm docs:show

# Runs formating, tests and checks necessary before a commit.
pre-commit:
    @just fmt
    @just thorough-check
    @just unused
    @just audit
    @just precache
    @just test
    @just build
    @just index

# Runs checks and tests run by ci.
test-ci:
    @just deps-ci
    @just thorough-check
    @just precache-check
    @just unused
    @just audit
    @just test-cov

# Full app build used by ci.
ci-build:
    @just deps-ci
    @just build
    @just build-windows

# Generate SBOM for rs sources.
[working-directory: 'src-tauri']
sbom-rs:
    mkdir -p ../sbom
    cargo sbom > ../sbom/sbom-backend.json

# Generate SBOM for js/ts/svelte sources.
sbom-js:
    mkdir -p sbom
    pnpm sbom --sbom-format spdx --prod > sbom/sbom-frontend.json

# Generates SBOM for all sources.
sbom:
    @just sbom-rs
    @just sbom-js

# Build linux-build-image.
docker-linux:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ -z "${GITLAB_IMAGE_REGISTRY}" ]; then
        exit 1 # GITLAB_IMAGE_REGISTRY variable has to be set
    fi
    IMAGE_TAG="$(git rev-parse --short HEAD)"
    IMAGE_BASE="${GITLAB_IMAGE_REGISTRY}/linux-build"
    IMAGE="${IMAGE_BASE}:${IMAGE_TAG}"
    IMAGE_LATEST="${IMAGE_BASE}:latest"
    sudo docker buildx build -f "devops/linux-build/Dockerfile" -t "${IMAGE}" --load \
        --label "org.opencontainers.image.revision=$(git rev-parse HEAD)" \
        --label "org.opencontainers.image.created=$(date)" \
        --label "org.opencontainers.image.version=${IMAGE_TAG}" \
        .
    sudo docker tag "${IMAGE}" "${IMAGE_LATEST}"
    sudo docker push "${IMAGE}"
    sudo docker push "${IMAGE_LATEST}"

# Initializes the project by installing all necessary tooling. Should be run once before beginning of development.
init:
    echo # installing nightly, windows-msvc target and xwin
    rustup install nightly
    rustup target add x86_64-pc-windows-msvc
    cargo install --locked cargo-xwin

    echo # Chaching windows SDK
    cargo xwin cache xwin

    echo # Installing cargo-binstall for faster setup time
    cargo binstall -V || cargo install cargo-binstall

    echo # Installing tauri cli
    cargo tauri -V || cargo binstall tauri-cli --no-confirm

    echo # Installing sqlx cli for db migrations and pre-caching 
    cargo sqlx -V || cargo binstall sqlx-cli --no-confirm

    echo # Installing test, coverage, lint, audit and other utilities
    rustup component add llvm-tools-preview
    cargo binstall cargo-llvm-cov --no-confirm
    cargo nextest -V || cargo binstall nextest --no-confirm
    cargo udeps -V || cargo binstall cargo-udeps --no-confirm
    cargo audit fix -V || cargo install cargo-audit --locked --features=fix
    cargo sbom -V || cargo binstall cargo-sbom --no-confirm

    echo # Installing pnpm
    pnpm_major=$(pnpm --version 2>/dev/null | cut -d. -f1)
    [[ "${pnpm_major:-0}" -lt 11 ]] && npm install -g pnpm@next-11 || true

    echo # Synch node_modules
    pnpm install

    echo # Creating local .env file from .env.example
    cp .env.example .env

