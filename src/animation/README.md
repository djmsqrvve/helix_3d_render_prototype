# Animation Module

## Responsibility
Manages the loading, playback, and synchronization of skeletal animations for the 3D models.

## Key Symbols
- `AnimationHandler`: Component that handles individual animation state.
- `AnimationGraph`: Bevy's core structure for blending and playback.

## Guardrails (DON'T DO)
- **❌ DO NOT** use additive/pose animations (indices < 246 channels) for primary playback. They will distort the model.
- **❌ DO NOT** call `play()` every frame. Only call it when the intended animation state changes.
- **❌ DO NOT** hardcode animation indices in the core logic. Use the `ANIMATIONS` constant in `src/constants.rs`.

## Questions
- How can we implement cross-fading between animations smoothly?
- Should we support custom GLTF animation files at runtime?
