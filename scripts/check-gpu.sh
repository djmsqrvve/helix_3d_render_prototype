#!/bin/bash

# Helix 3D Renderer - Check GPU Configuration

echo "🎮 Checking GPU configuration..."
echo ""

# Check for system graphics drivers
if command -v vulkaninfo &> /dev/null; then
    echo -e "\033[0;32m✅ Vulkan is installed\033[0m"
else
    echo -e "\033[0;33m⚠️  vulkaninfo not found. Install with: sudo apt install vulkan-tools\033[0m"
fi

# Check for custom WSL Mesa (secondary)
if [ -d "$HOME/.local/mesa" ]; then
    echo -e "\033[0;32m✅ Custom WSL Mesa found at ~/.local/mesa\033[0m"
fi

echo ""
echo "🔍 Current Environment:"
echo "  DISPLAY: ${DISPLAY:-"(not set)"}"
echo "  WAYLAND_DISPLAY: ${WAYLAND_DISPLAY:-"(not set)"}"
echo "  WGPU_BACKEND: ${WGPU_BACKEND:-"(not set)"}"

echo ""
echo "🚀 To run with hardware acceleration:"
echo "  make dev"
echo ""
echo "💡 If you encounter graphics issues:"
echo "  export WGPU_BACKEND=gl && make dev      # Try OpenGL"
echo "  ./scripts/run-x11.sh                   # Force X11 backend"
echo ""
echo "🪟 For WSL2 specific GPU acceleration:"
echo "  make dev-wsl-gpu"
