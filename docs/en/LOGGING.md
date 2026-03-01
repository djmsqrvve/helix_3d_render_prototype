# Remote Logging Documentation

## Overview

The Helix 3D Renderer supports both local and remote logging using the `tracing` ecosystem.

## Local Logging (Default)

By default, logs are written to stdout with structured formatting.

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `RUST_LOG` | Set log level | `RUST_LOG=debug cargo run` |
| `RUST_LOG` | Per-module levels | `RUST_LOG=info,bevy_3d_renderer=debug` |

### Log Levels

- `error` - Critical failures
- `warn` - Warnings that don't stop execution
- `info` - General information (default)
- `debug` - Detailed debugging info
- `trace` - Very verbose tracing

### Examples

```bash
# Default info logging
cargo run

# Debug logging for everything
cargo run --verbose
# or
RUST_LOG=debug cargo run

# Debug for our app, warnings for Bevy
RUST_LOG=debug,bevy=warn,wgpu=error cargo run

# Show only our app's logs
RUST_LOG=bevy_3d_renderer=debug cargo run
```

## Remote Logging (Planned)

Remote logging to HTTP endpoints is planned but not yet fully implemented.

The CLI accepts `--log-endpoint` for future use:

```bash
# Currently warns that remote logging is not implemented
cargo run -- --log-endpoint https://logs.example.com/ingest
```

### Implementation Plan

To implement remote logging:

1. Add `reqwest` dependency to Cargo.toml
2. Implement async log shipping in `src/logging/mod.rs`
3. Add `remote-log` feature flag

See the logging module source for the intended architecture.

## Structured Logging

Use the provided macros for structured events:

```rust
use bevy_3d_renderer::{log_event, log_metric};

// Log an event with data
log_event!("accessory_attached", { accessory: "drow_weapon", bone: "Bow1_0" });

// Log a metric
log_metric!("fps", 60.5);
log_metric!("frame_time_ms", 16.67);
```

## Log Prefixes

We use consistent prefixes for automated parsing:

| Prefix | Meaning | Example |
|--------|---------|---------|
| `[LOAD]` | Asset loading | `[LOAD] Base model loaded` |
| `[ATTACH]` | Skeleton attachment | `[ATTACH][OK] drow_weapon -> 'Bow1_0'` |
| `[TOGGLE]` | Accessory visibility | `[TOGGLE] drow_cape: OFF` |
| `[ANIMATION]` | Animation state | `[ANIMATION] Changed to: Attack` |
| `[DEBUG]` | Debug visualization | `[DEBUG] Wireframe: ON` |
| `[SCREENSHOT]` | Screenshot events | `[SCREENSHOT] Saved to screenshot_001.png` |
| `[PERF]` | Performance metrics | `[PERF] FPS: 60.1` |
| `[UI]` | UI interactions | `[UI] Accessory toggle panel spawned` |

## Platform Specific Notes

### Ubuntu/Linux (Primary)
- Standard `tracing` levels work as expected.
- Use `RUST_LOG=info,wgpu=warn` to reduce noise from graphics drivers.

### WSL2 (Secondary)
- Vulkan verbose logs are normal in WSL2.
- Remote logging works through WSL2 network bridge.
