FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Install Bevy dependencies, Virtual Screen tools, fast linkers (mold/lld), and sccache
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get -y install --no-install-recommends build-essential pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev xvfb mesa-vulkan-drivers vulkan-tools libvulkan-dev mold lld clang sccache

# Set up a virtual display environment variable
ENV DISPLAY=:99
# Enable sccache
ENV RUSTC_WRAPPER=sccache
