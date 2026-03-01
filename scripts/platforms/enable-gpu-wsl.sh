#!/bin/bash

# Helix 3D Renderer - Enable WSL2 Custom Mesa GPU Acceleration
# Activates a custom Mesa 24.0.7 build with D3D12 support (specifically for WSL2)

echo "🎮 Enabling custom Mesa GPU acceleration for WSL2..."

# NOTE: This is primarily for WSL2 users who need D3D12/Dozen support.
# Standard Ubuntu users should use system-provided drivers.

# Custom Mesa installation path
export MESA_ROOT="$HOME/.local/mesa"
export MESA_LIB="$MESA_ROOT/lib/x86_64-linux-gnu"
export MESA_DRI="$MESA_LIB/dri"

# Set driver paths
export LIBGL_DRIVERS_PATH="$MESA_DRI"
export LIBEGL_DRIVERS_PATH="$MESA_DRI"

# Vulkan ICD for D3D12 (Dozen)
if [ -f "$MESA_LIB/libvulkan_dzn.so" ]; then
    export VK_ICD_FILENAMES="$MESA_ROOT/share/vulkan/icd.d/dzn_icd.x86_64.json"
    echo "✅ Vulkan D3D12 driver configured"
fi

# Library path for runtime linking
export LD_LIBRARY_PATH="$MESA_LIB:$LD_LIBRARY_PATH"

# Performance optimizations for WSL2
export MESA_D3D12_DEFAULT_SHADER_MODEL=6_6
export MESA_NO_ERROR=1  # Disable GL error checking for performance

# Enable D3D12 video acceleration
export LIBVA_DRIVERS_PATH="$MESA_DRI"
export LIBVA_DRIVER_NAME=d3d12

# CRITICAL: Force wgpu to use the non-compliant D3D12 adapter
# Without this, wgpu hides the GPU because D3D12 reports as "not Vulkan compliant"
export WGPU_ALLOW_UNDERLYING_NONCOMPLIANT_ADAPTER=1

# CRITICAL: Force X11 instead of Wayland for WSL2 compatibility
# D3D12 Vulkan driver doesn't support VK_KHR_wayland_surface
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY  # Unset to prevent Wayland detection

# Additional debug options (uncomment if needed)
# export DZN_DEBUG=always
# export WGPU_DEBUG=1

echo "✅ Custom Mesa 24.0.7 activated"
echo "   D3D12 (Dozen) driver: enabled"
echo "   Vulkan: enabled (non-compliant adapter allowed)"
echo "   OpenGL: enabled"
echo "   Backend: X11 (for WSL2 compatibility)"
echo ""
echo "Run your application with GPU acceleration:"
echo "  source scripts/enable-gpu.sh && ./run.sh dev"
echo ""
echo "Or use the gpu-enabled wrapper:"
echo "  ./run.sh dev:gpu"
