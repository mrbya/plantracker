#!/usr/bin/env just --justfile
set dotenv-load := true

# Output this list
list:
    @just --list

# Installs node deps
deps *FLAGS:
    pnpm install {{FLAGS}}

# Installs node deps with --frozen-lockfile
deps-ci:
    @just deps --frozen-lockfile

# Pre caches db queries
precache *FLAGS:
    cargo sqlx prepare --workspace {{FLAGS}}

# Chechs pre-cached db queries
precache-check:
    @just precache --check

# Builds app release
build:
    cargo tauri build

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

# Full app build used by ci
ci-build:
    @just precache-check
    @just deps-ci
    @just build

