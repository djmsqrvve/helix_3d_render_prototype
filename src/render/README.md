# Render Module

## Responsibility
Controls the low-level graphics configuration, GPU driver detection, and backend initialization (Vulkan/DX12/GL).

## Key Symbols
- `RenderPlugin`: Primary plugin for graphics setup.
- `GpuInfoState`: Resource that tracks hardware acceleration vs. software fallback.
- `display_gpu_info()`: System that notifies the user if hardware acceleration is active.

## Guardrails (DON'T DO)
- **❌ DO NOT** hardcode graphics backends unless absolutely necessary for troubleshooting.
- **❌ DO NOT** ignore the software rendering warning (llvmpipe). It will lead to extremely low performance (<10 FPS).
- **❌ DO NOT** assume the `wgpu` backend will always be Vulkan on Linux (it might be GL or Wayland/X11).

## Questions
- How to implement a robust Vulkan-headless mode for CI testing?
- Should we provide a "performance profile" for lower-end hardware?
