#!/bin/bash

# Helix 3D Renderer - Port Cleanup Utility
# Note: This is a native app, not a server, so port cleanup is minimal

echo "🧹 Cleaning up processes..."

# Kill any previous instances of the binary (if running in background)
pkill -f "bevy-3d-renderer" 2>/dev/null || true
pkill -f "bevy_3d_renderer" 2>/dev/null || true

# Kill any cargo run processes for this project
cargo_pid=$(pgrep -f "cargo run" | head -1)
if [ -n "$cargo_pid" ]; then
    echo "Killing cargo run process: $cargo_pid"
    kill $cargo_pid 2>/dev/null || true
fi

echo "✅ Cleanup complete"
