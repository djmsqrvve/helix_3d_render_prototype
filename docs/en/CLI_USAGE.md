# Helix 3D Renderer - CLI Usage

The renderer now supports command-line arguments for automated testing, screenshot capture, and headless rendering.

## Basic Usage

```bash
# Run with default settings (Ubuntu/Linux)
make dev

# Run with egui panel (standard)
make dev-fast

# Or directly with cargo
cargo run --features egui
```

## CLI Arguments

### Screenshot Capture

```bash
# Capture screenshot to file (waits 60 frames then captures)
cargo run -- --screenshot output.png

# Capture with custom frame delay
cargo run -- --screenshot output.png --capture-frame 120
```

### Headless Rendering

```bash
# Render without window (output to file)
cargo run -- --render output.png --headless
```

### Animation Selection

```bash
# Play specific animation (0-9)
cargo run -- --animation 2  # Attack animation
cargo run -- --animation 5  # Victory animation

# Animation indices:
# 0 = Idle (default)
# 1 = Run
# 2 = Attack
# 3 = Attack Fast
# 4 = Death
# 5 = Victory
# 6 = Stun
# 7 = Teleport
# 8 = Spawn
# 9 = Frost Arrow
```

### Accessory Control

```bash
# Disable specific accessories
cargo run -- --disable-accessories "drow_cape,drow_quiver"

# Disable all accessories except weapon
cargo run -- --disable-accessories "drow_armor,drow_bracer,drow_cape,drow_haircowl,drow_legs,drow_quiver"
```

### Automated Testing

```bash
# Output test results to directory
cargo run -- --test-output ./test-results

# Full test configuration
cargo run -- --animation 0 --capture-frame 60 --test-output ./test-results --screenshot base.png
```

### Verbose Logging

```bash
# Enable debug logging
cargo run -- --verbose
```

## Keyboard Shortcuts

While running:

- `1-9` - Change animation
- `Space` - Pause/Resume animation
- `F1` - Toggle wireframe
- `F2` - Toggle skeleton
- `F3` - Toggle stats
- `F4` - Toggle transforms
- `F5` - Toggle grid
- `F6` - Capture screenshot
- `ESC` - Exit

## Logging Format

The renderer uses structured logging with prefixes:

- `[LOAD]` - Asset loading
- `[ATTACH]` - Accessory attachment to skeleton
- `[TOGGLE]` - Accessory visibility toggles
- `[ANIMATION]` - Animation changes
- `[DEBUG]` - Debug mode changes
- `[SCREENSHOT]` - Screenshot capture
- `[TEST]` - Automated testing
- `[PERF]` - Performance diagnostics
- `[UI]` - UI system events

## Examples

### Compare All Animations

```bash
for i in {0..9}; do
    cargo run -- --animation $i --screenshot anim_$i.png --capture-frame 30
done
```

### Test Each Accessory Individually

```bash
for acc in drow_armor drow_bracer drow_cape drow_haircowl drow_legs drow_quiver drow_weapon drow_marksmanship_arrow; do
    # Disable all except one
    disabled=$(echo "drow_armor,drow_bracer,drow_cape,drow_haircowl,drow_legs,drow_quiver,drow_weapon,drow_marksmanship_arrow" | sed "s/$acc//g")
    cargo run -- --disable-accessories "$disabled" --screenshot ${acc}_test.png
done
```

### CI/CD Integration

```bash
# Run automated test suite
cargo run --release -- \
    --animation 0 \
    --capture-frame 60 \
    --test-output ./ci-results \
    --screenshot ci-test.png

# Check if screenshot was created (indicates success)
if [ -f ci-test.png ]; then
    echo "✅ Render test passed"
else
    echo "❌ Render test failed"
    exit 1
fi
```

## Known Limitations

1. **Screenshot implementation**: Full PNG screenshot via CLI is planned but currently logs the request. Use OS screenshot tools or RenderDoc for now.

2. **Headless mode**: Requires display server even with `--headless` flag due to Bevy 0.18 limitations. Use Xvfb on Linux for truly headless operation.

3. **Exit behavior**: After screenshot capture in CLI mode, the app logs the exit request but may require manual close (Ctrl+C or window close).
