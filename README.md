# Helix 3D Renderer

🌐 **Translations**: [English](docs/en/README.md) | [Русский](docs/ru/README.md) | [Português (BR)](docs/pt-BR/README.md) | [Français](docs/fr/README.md) | [Contribute!](docs/TRANSLATIONS.md)

---

A hardware-accelerated 3D renderer for Dota 2 hero models built with **Rust** and **Bevy 0.18**.

This project serves as the 3D rendering engine for the [DJ Engine](https://github.com/djmsqrvve/DJ-Engine) project.

---

## ⚡ ACTIVE MISSION
**[Mission Control: Lesson 3 Launch](docs/MISSION_CONTROL_LESSON_3.md)**
*Check here for all links and steps for today's stream!*

---

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

## Builds & Distribution

Helix supports building for both Linux (native) and Windows (cross-compilation).

### 🐧 Linux (Native Ubuntu)
The default build for development and production.
```bash
make build           # Standard debug build
make build-release   # Optimized production build
```
Binary located at: `target/gltf_test/release/bevy-3d-renderer`

### 🪟 Windows (Cross-Compilation)
Build a Windows executable directly from Ubuntu.
```bash
make build-windows   # Compiles .exe using mingw-w64
make dist-windows    # Packages .exe + assets into ./dist/windows/
```
Binary located at: `target/gltf_test/x86_64-pc-windows-gnu/release/bevy-3d-renderer.exe`

---

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

See [docs/DEV_TROUBLESHOOTING.md](docs/DEV_TROUBLESHOOTING.md) for detailed solutions and platform-specific guides (WSL/Windows).

## Documentation Hub

### 🏰 Team Guides
- [**TEAM V: START HERE!**](docs/TEAMV_START_HERE.md) - For our younger builders 🇧🇷
- [**Lesson 1: Explorer's Signature**](docs/TEAMV_LESSON_1_SIGNING_OFF.md) - Sign the Hall of Heroes
- [**Lesson 2: Cloud Laboratory**](docs/TEAMV_LESSON_2_CLOUD_LAB.md) - Launch your computer in the sky
- [**Lesson 3: Talking to the Cloud**](docs/TEAMV_LESSON_3_TALKING_TO_CLOUD.md) - Run your first command
- [**Lesson 4: Your First 3D Picture**](docs/TEAMV_LESSON_4_FIRST_3D_PICTURE.md) - Capture the Drow Ranger
- [**Lesson 5: Color Alchemist**](docs/TEAMV_LESSON_5_COLOR_ALCHEMIST.md) - Change the world's color
- [**Lesson 6: Rotation Master**](docs/TEAMV_LESSON_6_ROTATION_MASTER.md) - Make the hero move
- [**TEAM S: START HERE!**](docs/TEAMS_START_HERE.md) - For our technical teens 🇫🇷
- [**Team Matrix**](docs/TEAM_MATRIX.md) - **Progress tracking for everyone**
- [Lesson Creation Workflow](docs/TEAM_LESSON_CREATION_WORKFLOW.md) - How Team Dragon makes lessons
- [Team Communication](docs/TEAM_COMMUNICATION.md) - How we talk to each other


### 🎓 Apprentice Learning
- [AI Helper Guide](docs/APPRENTICE_AI_GUIDE.md) - Using AI to build Helix
- [Learning Transforms](docs/APPRENTICE_TRANSFORMS.md) - 3D math 101

### 🛠️ Developer Resources
- [Native Setup Guide](docs/DEV_NATIVE_SETUP.md) - Running without WSL
- [Stream Review Guide](docs/DEV_STREAM_REVIEW_GUIDE.md) - How to shoutout students live
- [Dry Run Guide](docs/DEV_DRY_RUN_GUIDE.md) - Testing lessons as a student
- [CLI Usage](docs/DEV_CLI_USAGE.md) - Command reference

- [Troubleshooting](docs/DEV_DEV_TROUBLESHOOTING.md) - Common fixes
- [Decision Log](docs/DEV_DEV_DECISIONS.md) - Why we built it this way
- [Skeletal Analysis](docs/DEV_SKELETON_DEEP_DIVE.md) - Bone details
- [Contributing](CONTRIBUTING.md) - Standard workflows
- [AGENTS.md](AGENTS.md) - Technical AI rules

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
