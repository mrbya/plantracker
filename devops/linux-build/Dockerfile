FROM ubuntu:24.04

RUN apt-get update -qq && \
    apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    pkg-config \
    libwebkit2gtk-4.1-dev \
    librsvg2-dev \
    patchelf \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash && \
    apt-get install -y nodejs && \
    npm install -g pnpm@next-11 && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable && \
    . "$HOME/.cargo/env" && \
    cargo install tauri-cli --locked && \
    cargo install just --locked

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install sqlx-cli

RUN apt-get install -y xdg-utils

RUN apt-get install -y fuse libfuse2 && \
    modprobe fuse || true

RUN apt-get install -y nsis clang lld llvm

RUN rustup target add x86_64-pc-windows-msvc

RUN cargo install --locked cargo-xwin

RUN cargo xwin cache xwin

RUN cargo install cargo-sbom && \
    cargo install cargo-udeps && \
    cargo install cargo-audit --locked --features=fix

RUN rustup component add llvm-tools-preview && \
    cargo install cargo-llvm-cov && \
    cargo install cargo-nextest --locked

ARG NODE_VERSION=v22.13.0

RUN apt-get update && apt-get install -y curl ca-certificates xz-utils --no-install-recommends \
    && curl -fsSL https://nodejs.org/dist/${NODE_VERSION}/node-${NODE_VERSION}-linux-x64.tar.xz \
       -o /tmp/node.tar.xz \
    && tar -xJf /tmp/node.tar.xz -C /usr/local --strip-components=1 \
    && rm /tmp/node.tar.xz \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

ENV PATH="/root/.local/share/pnpm/bin:${PATH}"

RUN npm install -g playwright && \
    npm exec playwright install-deps && \
    npm exec playwright install

CMD ["bash"]
