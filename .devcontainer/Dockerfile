FROM mcr.microsoft.com/devcontainers/typescript-node:1-22-bookworm

# Install the xz-utils package
RUN apt-get update && apt-get install -y lintian

RUN npm install -g pnpm