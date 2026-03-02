FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Install Bevy dependencies, Virtual Screen tools, fast linkers, and sccache
# libxcursor-dev is required for cursor management on Linux
# libxi-dev is required for X11 input handling (bevy_winit)
# libxkbcommon-x11-dev is required for X11 keyboard handling
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get -y install --no-install-recommends build-essential pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev libxkbcommon-x11-dev xvfb mesa-vulkan-drivers vulkan-tools libvulkan-dev mold lld clang sccache libxcursor-dev libxi-dev

# Pre-install Rust 1.89.0 so the user doesn't have to (avoids permission errors)
RUN rustup install 1.89.0 && rustup default 1.89.0

# Set up a virtual display environment variable
ENV DISPLAY=:99
# Enable sccache
ENV RUSTC_WRAPPER=sccache
