FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Explicitly install and set Rust 1.89.0 (required for Bevy 0.18)
RUN rustup install 1.89.0 && rustup default 1.89.0

# Install Bevy dependencies and Virtual Screen tools
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get -y install --no-install-recommends build-essential pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev xvfb mesa-vulkan-drivers vulkan-tools libvulkan-dev

# Set up a virtual display environment variable
ENV DISPLAY=:99
