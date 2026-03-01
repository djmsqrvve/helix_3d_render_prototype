# Input Module

## Responsibility
Handles keyboard and mouse input for controlling the camera, selecting animations, and capturing screenshots.

## Key Symbols
- `KeyboardInputPlugin`: Main plugin for input handling.
- `keyboard_input()`: The core system mapping keys to actions.
- `ScreenshotEvent`: Event triggered for capturing frames.

## Guardrails (DON'T DO)
- **❌ DO NOT** hardcode keys in multiple places. Use the `keyboard_input` system in `src/input/keyboard.rs`.
- **❌ DO NOT** block input handling with heavy computations.
- **❌ DO NOT** assume the user has a specific keyboard layout. Stick to standard alphanumeric and function keys.

## Questions
- Should we support custom keybindings via a configuration file?
- How to handle touch input for mobile platforms in the future?
