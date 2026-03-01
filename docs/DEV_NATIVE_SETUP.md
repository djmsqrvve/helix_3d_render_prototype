# Native Environment Setup Guide

This guide explains how to build and run the Helix 3D Renderer natively on your operating system (Linux, Windows, or macOS) without using WSL or virtualization.

## 🛠️ Recommended IDE: VS Code

To keep our development environment consistent across the team, we recommend using **Visual Studio Code**.

1.  **Install VS Code**: [Download here](https://code.visualstudio.com/).
2.  **Open the Project**: Open the `helix_3d_render_prototype` folder in VS Code.
3.  **Install Recommended Extensions**: When you open the folder, VS Code will suggest installing the project's recommended extensions. Click **"Install All"** in the popup.
    - If you don't see the popup, open the Extensions view (`Ctrl+Shift+X`), type `@recommended`, and install the list.
    - Key extensions include: `rust-analyzer`, `Even Better TOML`, and `CodeLLDB`.

---

## 🐧 Linux (Standard Ubuntu/Debian/Arch)

Standard Linux environments usually provide the best performance for Bevy.

### 1. Install System Dependencies
You need the development headers for X11, Wayland, and Alsa.

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
```

**Arch:**
```bash
sudo pacman -S clang pkg-config alsa-lib
```

### 2. Graphics Drivers
Ensure you have Vulkan drivers installed:
```bash
sudo apt install mesa-vulkan-drivers vulkan-tools
```

### 3. Build & Run
```bash
cargo run --features egui
```

---

## 🪟 Native Windows (PowerShell/CMD)

For the best experience on Windows, run natively rather than through WSL.

### 1. Build Tools
- Install [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/) (Community edition is fine).
- Select the **"Desktop development with C++"** workload during installation.

### 2. Build & Run
Open PowerShell in the project root:
```powershell
cargo run --features egui
```
*Note: The first build may take several minutes as it compiles Bevy's shaders and engine core.*

---

## 🍎 macOS (Apple Silicon/Intel)

Bevy uses the **Metal** backend on macOS.

### 1. Xcode Tools
Ensure you have the command line tools installed:
```bash
xcode-select --install
```

### 2. Build & Run
```bash
cargo run --features egui
```

---

## 🛠️ Build Options Reference

| Command | Result |
|---------|--------|
| `cargo run` | Lean CLI mode (no control panel) |
| `cargo run --features egui` | Standard dev mode with UI |
| `cargo build --release` | Optimized binary in `target/release/` |

## ❓ Troubleshooting

If the window fails to open:
1. **Linux**: Try `export WINIT_UNIX_BACKEND=x11` before running.
2. **Windows**: Ensure your GPU drivers are up to date from NVIDIA/AMD/Intel.
3. **General**: See [docs/DEV_TROUBLESHOOTING.md](DEV_TROUBLESHOOTING.md) for more details.
