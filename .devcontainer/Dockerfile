FROM mcr.microsoft.com/devcontainers/typescript-node:1-22-bookworm

USER root

# Install the xz-utils package
RUN apt-get update && apt-get install -y \
    lintian \
    build-essential \
    curl

USER node

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN npm install -g pnpm