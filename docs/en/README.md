# Helix 3D Renderer

A hardware-accelerated 3D renderer for Dota 2 hero models built with **Rust** and **Bevy 0.18**.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-2021 edition-orange.svg)
![Bevy](https://img.shields.io/badge/bevy-0.18-purple.svg)

## Overview

Helix 3D Renderer is a prototype showcasing real-time rendering of Dota 2's Drow Ranger model with full skeletal animation, accessory attachment, and debug visualization tools.

### Features

- **🎮 Real-time 3D Rendering** - Hardware-accelerated GPU rendering with Bevy engine
- **🦴 Skeletal Animation** - 10 built-in animations (Idle, Run, Attack, Death, Victory, etc.)
- **🎨 Accessory System** - 8 interchangeable accessories (weapon, armor, cape, etc.)
- **🔧 Debug Visualizations** - Wireframe, skeleton, transforms, stats overlay
- **⌨️ Interactive Controls** - Keyboard shortcuts and optional egui panel
- **📸 Screenshot Capture** - CLI automation support for testing
- **🖥️ Cross-platform** - Linux (Ubuntu primary), Windows, WSL support

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+ with Cargo
- GPU with Vulkan support (standard on modern Ubuntu/Linux)
- Ubuntu 22.04+ (Recommended)

### Run

```bash
# Clone the repository
git clone https://github.com/djmsqrvve/helix_3d_render_prototype.git
cd helix_3d_render_prototype

# Setup dependencies (Ubuntu)
make setup

# Run with default settings
make dev
# or
cargo run --features egui
```

### Keyboard Controls

| Key | Action |
|-----|--------|
| `1-9` | Change animation |
| `Space` | Pause/Resume animation |
| `F1` | Toggle wireframe |
| `F2` | Toggle skeleton view |
| `F3` | Toggle performance stats |
| `F4` | Toggle transform gizmos |
| `F5` | Toggle grid |
| `F6` | Capture screenshot |
| `ESC` | Exit |

## CLI Usage

```bash
# Play specific animation
cargo run -- --animation 2  # Attack animation

# Disable specific accessories
cargo run -- --disable-accessories "drow_cape,drow_quiver"

# Capture screenshot
cargo run -- --screenshot output.png --capture-frame 60

# Enable verbose logging
cargo run -- --verbose
```

See [docs/DEV_CLI_USAGE.md](docs/DEV_CLI_USAGE.md) for full documentation.

## Project Structure

```
helix_3d_render_prototype/
├── src/
│   └── main.rs              # Main application
├── src/debug/               # Debug visualization systems
├── src/model/               # Model loading & accessory attachment
├── src/ui/                  # UI systems (Bevy native + egui)
├── assets/                  # 3D models and textures
│   └── test_models/dota_models/models/heroes/drow/
├── tests/                   # Integration and regression tests
├── docs/                    # Documentation
│   ├── DEV_CLI_USAGE.md         # Command reference
│   ├── DEV_TROUBLESHOOTING.md   # Common issues
│   └── DEV_LOGGING.md           # Logging configuration
├── Cargo.toml
└── Makefile                 # Build commands
```

## Development

### Build Commands

```bash
make dev             # Full development build with checks
make dev-fast        # Quick start (skip checks)
make guardrail       # Build + test + format check
make guardrail-strict # Full checks including clippy
make test            # Run all tests
make clean           # Clean build artifacts
```

### Feature Flags

```bash
# Run with egui panel (experimental)
cargo run --features egui
```

## Troubleshooting

Common issues and solutions:

**Graphics initialization errors:**
```bash
# Try forcing the OpenGL backend if Vulkan fails
export WGPU_BACKEND=gl
make dev
```

**Software rendering warning:**
Ensure you have the correct GPU drivers installed for your Linux distribution. For Ubuntu:
```bash
sudo apt update && sudo apt install mesa-vulkan-drivers
```

See [docs/DEV_TROUBLESHOOTING.md](../DEV_TROUBLESHOOTING.md) for detailed solutions and platform-specific guides (WSL/Windows).


## Documentation

- [CLI Usage](docs/DEV_CLI_USAGE.md) - Command-line reference
- [Troubleshooting](docs/DEV_TROUBLESHOOTING.md) - Common issues and fixes
- [Logging](docs/DEV_LOGGING.md) - Structured logging configuration
- [Animation Indices](docs/animation_indices.md) - Animation reference
- [AGENTS.md](AGENTS.md) - Development guidelines

## Testing

```bash
# Run all tests
make test

# Regression test (comprehensive)
make regression-test

# Feature flag tests
cargo test --test build_test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Bevy](https://bevyengine.org/) - A refreshingly simple data-driven game engine
- Dota 2 models are property of Valve Corporation (used for educational/research purposes)
