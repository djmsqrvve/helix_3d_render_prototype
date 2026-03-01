#!/bin/bash

# Helix 3D Renderer - Run with WSL2 Custom Mesa GPU Acceleration

set -e

# Source the GPU enable script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/enable-gpu-wsl.sh"

echo "🚀 Starting with WSL2 GPU acceleration..."

# Pass all arguments to cargo
cargo run "$@"
