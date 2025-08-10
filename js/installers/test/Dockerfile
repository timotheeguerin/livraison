ARG UBUNTU_VERSION=22.04
ARG SHELL_NAME=bash

FROM ubuntu:${UBUNTU_VERSION}

# Install required packages
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    ${SHELL_NAME} \
    && rm -rf /var/lib/apt/lists/*

# Create test user
RUN useradd -m -s /bin/${SHELL_NAME} testuser

# Set working directory
WORKDIR /test

# Copy installer and test scripts
COPY . /test/

# Make scripts executable
RUN chmod +x /test/*.sh

# Switch to test user
USER testuser

# Set HOME explicitly
ENV HOME=/home/testuser
