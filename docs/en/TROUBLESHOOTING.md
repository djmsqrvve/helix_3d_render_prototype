# Troubleshooting Guide

## Common Issues and Solutions

### 1. WGPU Surface Creation Error (VK_KHR_wayland_surface)

**Error Message:**
```
Failed to create wgpu surface: CreateSurfaceError { inner: Hal(FailedToCreateSurfaceForAnyBackend({Vulkan: InstanceError { message: "Vulkan driver does not support VK_KHR_wayland_surface", source: None }})) }
```

**Cause:**
This occurs when:
- Wayland is not properly supported by your GPU driver
- The display server is using X11 but Bevy is trying to use Wayland
- No display server is available (headless environment)

**Solutions:**

#### Option A: Force X11 Backend
If your system uses X11 or if Wayland support is flaky:
```bash
export WINIT_UNIX_BACKEND=x11
make dev
```

#### Option B: Force OpenGL Backend
If Vulkan is not working properly:
```bash
export WGPU_BACKEND=gl
make dev
```

#### Option C: Install Graphics Drivers (Ubuntu/Linux)
Ensure you have the necessary Vulkan and Mesa drivers:
```bash
sudo apt update
sudo apt install mesa-vulkan-drivers libvulkan1 vulkan-tools
```

---

### 2. Slow Performance / Software Rendering Warning

**Warning:**
```
The selected adapter is using a driver that only supports software rendering. This is likely to be very slow.
```

**Cause:** No hardware GPU acceleration is being used.

**Solutions:**
- **Ubuntu/Linux:** Install proprietary drivers (NVIDIA/AMD) or ensure Mesa is correctly configured.
- **Verification:** Run `vulkaninfo` or `glxinfo` to verify hardware acceleration status.

---

### 3. Multiple Animations Playing Simultaneously

**Symptom:** Model looks distorted, bones moving in weird combinations.

**Cause:** Animation indices were pointing to additive/pose variants instead of full animations.

**Solution:** Use full animation indices (0-9). See `docs/animation_indices.md` for the full list.

---

### 4. Accessories Not Moving With Model

**Symptom:** Armor, weapon, cape don't follow the character.

**Cause:** Accessories have their own skeletons with different bone indices.

**Solution:** The system handles this automatically by bone-name matching. If it fails, check the logs for "Missing bone" warnings.

---

## WSL & Windows Specifics

If you are running Helix in **Windows Subsystem for Linux (WSL2)** or natively on **Windows**:

### WSL2 Graphics Issues
WSLg (WSL Graphics) can sometimes be unstable. If the window doesn't appear or performance is poor:
1. **Force X11:** `export WINIT_UNIX_BACKEND=x11`
2. **Restart WSL:** `wsl --shutdown` in PowerShell.
3. **Mesa D3D12:** See [docs/platforms/WSL_DEV_TROUBLESHOOTING.md](../platforms/WSL_DEV_TROUBLESHOOTING.md) for advanced GPU configuration.

### Native Windows
For best performance, run natively in PowerShell/CMD:
```powershell
cargo run --features egui
```

---

## Environment Variables Reference

| Variable | Purpose | Example |
|----------|---------|---------|
| `WGPU_BACKEND` | Force graphics backend | `gl`, `vulkan`, `dx12`, `metal` |
| `WINIT_UNIX_BACKEND` | Force windowing backend | `x11`, `wayland` |
| `DISPLAY` | X11 display | `:0` |
| `RUST_LOG` | Logging level | `info`, `debug`, `warn` |

---

## Getting Help

1. Check this troubleshooting guide first.
2. Run `make setup` to ensure dependencies are installed.
3. Enable backtraces: `RUST_BACKTRACE=1 make dev`
