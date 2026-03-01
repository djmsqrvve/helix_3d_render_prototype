#!/bin/bash
# Run in WSL with proper X11 configuration (based on DJ Engine working setup)

# Force X11 backend (disable Wayland)
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY

# Standard X11 display
export DISPLAY=:0

# Auto-detect GPU backend (usually vulkan with llvmpipe fallback)
export WGPU_BACKEND=${WGPU_BACKEND:-vulkan}

echo "Running with X11/WGPU configuration..."
echo "WINIT_UNIX_BACKEND=$WINIT_UNIX_BACKEND"
echo "DISPLAY=$DISPLAY"
echo "WAYLAND_DISPLAY=$WAYLAND_DISPLAY"
echo "WGPU_BACKEND=$WGPU_BACKEND"

cargo run "$@"
