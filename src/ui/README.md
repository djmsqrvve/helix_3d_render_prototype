# UI Module

## Responsibility
Manages both the native Bevy 2D UI (overlays, accessory panel) and the optional `egui` debug panel.

## Key Symbols
- `AccessoryPanel`: The right-side native UI for toggling items.
- `EguiPlugin`: Feature-gated debug window with advanced controls.
- `UiWindowState`: Shared resource for tracking panel visibility.

## Guardrails (DON'T DO)
- **❌ DO NOT** add `egui` code outside of `#[cfg(feature = "egui")]` blocks. It will break non-GUI builds.
- **❌ DO NOT** block the main renderer's events while interacting with the UI.
- **❌ DO NOT** hardcode screen coordinates for UI elements. Use Bevy's relative positioning (`Percent`, `Val::Px`).

## Questions
- Should the control panel be moved to a separate OS window (multi-window support)?
- How to implement a more "Dota-like" theme for the native Bevy UI?
