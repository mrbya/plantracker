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
    npm install -g pnpm && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable && \
    . "$HOME/.cargo/env" && \
    cargo install tauri-cli --locked && \
    cargo install just --locked

ENV PATH="/root/.cargo/bin:${PATH}"

CMD ["bash"]
