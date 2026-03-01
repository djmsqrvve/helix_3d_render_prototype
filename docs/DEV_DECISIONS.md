# Architectural Decisions & Questions

This document tracks major design choices, the reasoning behind them, and any open questions for the Helix project.

## 🟢 Decisions (Stable)

### 1. Ubuntu-First Development Focus
- **Date**: 2026-02-27
- **Context**: WSL2 graphics (WSLg) can be unstable. Many students/developers now use pure Ubuntu or full Linux environments.
- **Decision**: Transition the primary documentation, scripts, and testing target to **Pure Ubuntu Linux**.
- **Impact**: WSL2 is now a secondary target. Commands like `make dev` work natively on Ubuntu.

### 2. Bevy 0.18 Engine Choice
- **Date**: 2026-02-15
- **Context**: Need a modern, data-driven Rust engine.
- **Decision**: Use Bevy 0.18 as the core engine.
- **Impact**: Leverage Bevy's cross-platform `wgpu` backend, modern ECS, and animation graphs.

### 3. Feature Flag for Egui
- **Date**: 2026-02-20
- **Context**: Want to keep the binary small for CLI usage but have a rich UI for local dev.
- **Decision**: Use an `egui` feature flag.
- **Impact**: `cargo run` is a lean CLI; `cargo run --features egui` (or `make dev-fast`) provides the GUI.

## 🟡 Decisions (Pending/Proposed)

### 4. Separate Control Window vs. Embedded UI
- **Question**: Should the control panel be in a separate window or part of the main 3D scene?
- **Current State**: We have a secondary window prototype in `feature/separate-window-ui`.
- **Discussion**: Separate windows avoid cluttering the 3D scene but introduce multi-window event handling complexity.

## 🔴 Questions (Help Wanted)

- **Q: How to implement dynamic GLTF file loading at runtime?**
  - Current: The `egui_panel.rs` file picker has a TODO for loading models dynamically.
  - Needed: A strategy for hot-swapping the `HeroModel` base entity and its animations at runtime.

- **Q: How to handle high-fidelity physics for accessories (e.g., cape)?**
  - Current: Bone-attached static mesh.
  - Needed: Suggestion for lightweight cloth/physics simulation.
  
- **Q: Best way to handle headless Vulkan on CI without GPUs?**
  - Current: Using software rendering in tests.
  - Needed: Robust Xvfb/Vulkan-headless configuration.

---
*Add new decisions at the top of the list.*
