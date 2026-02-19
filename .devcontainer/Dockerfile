FROM mcr.microsoft.com/devcontainers/base:ubuntu-24.04

# System dependencies for building Zed (from script/linux for Ubuntu 24.04)
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        gcc \
        g++ \
        libasound2-dev \
        libfontconfig-dev \
        libwayland-dev \
        libx11-xcb-dev \
        libxkbcommon-x11-dev \
        libssl-dev \
        libzstd-dev \
        libvulkan1 \
        libgit2-dev \
        make \
        cmake \
        clang \
        jq \
        curl \
        gettext-base \
        elfutils \
        libsqlite3-dev \
        musl-tools \
        musl-dev \
        build-essential \
        mold \
        libstdc++-14-dev \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain as the vscode user
USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Ensure cargo is on PATH for all shells
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

# Pre-install the wasm32-unknown-unknown target for the project's pinned toolchain.
# The project's rust-toolchain.toml pins channel 1.93 and only includes wasm32-wasip2;
# we need wasm32-unknown-unknown for the GPUI web platform.
RUN rustup target add wasm32-unknown-unknown --toolchain 1.93-x86_64-unknown-linux-gnu
