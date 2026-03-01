# Debug Module

## Responsibility
Provides visual debugging tools like wireframe mode, skeleton viewing, and transform gizmos.

## Key Symbols
- `DebugPlugin`: Main plugin for debug visualizations.
- `DebugGizmos`: System for drawing bones and orientations.
- `WireframePlugin`: Bevy's built-in wireframe support.

## Guardrails (DON'T DO)
- **❌ DO NOT** run debug systems in release mode if they consume too many resources (O(N²) skeleton checks fixed).
- **❌ DO NOT** assume debug gizmos match the final render quality.
- **❌ DO NOT** leave debug features enabled by default in the final binary.

## Questions
- How to implement a toggle for individual debug categories (e.g., just bones)?
- Can we add a performance profiler directly into the debug UI?
