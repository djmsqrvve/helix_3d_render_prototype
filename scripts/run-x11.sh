#!/bin/bash
# Helix 3D Renderer - Run with X11 Backend (Fallback for Wayland issues)

# Force X11 backend - disable Wayland
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY

# Ensure DISPLAY is set (default to :0)
export DISPLAY=${DISPLAY:-:0}

echo "🚀 Running with X11 backend..."
echo "DISPLAY=$DISPLAY"
echo "WINIT_UNIX_BACKEND=$WINIT_UNIX_BACKEND"

cargo run --features egui "$@"
