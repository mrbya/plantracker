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

# Audits for js source vulnerabilities.
audit-js:
    pnpm audit --prod

# RS sources:
# Check for unused dependencies, audit for vulnerabilities,
# and check if newer version of depenedencies is available.
[working-directory: 'src-tauri']
audit-rs:
    cargo +nightly udeps --all-targets
    cargo audit

# Audits sources for vulnerabilities and unused deps.
audit:
    @just audit-js
    @just audit-rs

# Pre caches db queries.
[working-directory: 'src-tauri']
precache *FLAGS:
    cargo sqlx prepare --workspace {{FLAGS}}

# Chechs pre-cached db queries.
precache-check:
    @just precache --check

# Builds app release.
build:
    cargo tauri build

thorough-check:
    @just fmt-js-check
    @just fmt-rs --check
    @just check-js
    @just check-rs -- -D warnings

# Runs all checks neccesary before a commit.
# Checks formatting, code quality, and more
pre-commit:
    @just thorough-check
    @just audit
    @just precache-check
    @just build

# Full app build used by ci.
ci-build:
    @just thorough-check
    @just precache-check
    @just deps-ci
    @just build

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

# Initializes the project, installing all necessary tooling. Should be run once before beginning of development.
init:
    echo # installing nightly used by `just fmt` and `cargo udeps`
    rustup install nightly

    echo # installing cargo-binstall for faster setup time
    cargo binstall -V || cargo install cargo-binstall

    echo # Installing tauri cli
    cargo tauri -V || cargo binstall tauri-cli --no-confirm

    echo # Installing sqlx cli for db migrations and pre-caching 
    cargo sqlx -V || cargo binstall sqlx-cli --no-confirm

    echo # Installing things required by `just pre-commit`
    cargo udeps -V || cargo binstall cargo-udeps --no-confirm
    cargo audit -V || cargo binstall cargo-audit --no-confirm

    echo # Installing pnpm
    pnpm -v || npm install -g pnpm

    echo # Install markdown-toc
    npm list -g markdown-toc || npm install -g markdown-toc

    echo # Synch node_modules
    pnpm install

    echo # Creating local .env file from .env.example
    cp .env.example .env

