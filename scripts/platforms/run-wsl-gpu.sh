#!/bin/bash
# Helix 3D Renderer — WSL2 GPU Accelerated Launch
# Sources enable-gpu.sh for Mesa/Dozen D3D12 passthrough, then runs cargo

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source the GPU enablement script
if [ -f "$SCRIPT_DIR/enable-gpu-wsl.sh" ]; then
    source "$SCRIPT_DIR/enable-gpu-wsl.sh"
else
    echo "⚠️  scripts/platforms/enable-gpu-wsl.sh not found, using system defaults"
    # Minimal WSL2 GPU vars as fallback
    export MESA_D3D12_DEFAULT_ADAPTER_NAME=NVIDIA
    export GALLIUM_DRIVER=d3d12
    export WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER=1
    export WINIT_UNIX_BACKEND=x11
    unset WAYLAND_DISPLAY
fi

echo "🎮 Launching Helix 3D Renderer with GPU acceleration..."
exec cargo run --features egui "$@"
