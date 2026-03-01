# Physics Module

## Responsibility
Manages collision detection, ragdoll effects, and physics-based movement for the hero and accessories.

## Key Symbols
- `PhysicsPlugin`: Configures the physics engine (e.g., Rapier).
- `Collider`: Component for collision shapes.
- `RigidBody`: Component for physics-driven entities.

## Guardrails (DON'T DO)
- **❌ DO NOT** update physics properties directly via `Transform`. Use `Velocity` or `ExternalImpulse` instead.
- **❌ DO NOT** create overly complex colliders for high-poly models. Use simplified primitive shapes or convex hulls.
- **❌ DO NOT** run physics updates on every frame if it's not needed (e.g., static UI).

## Questions
- Should we use Bevy's native physics or a dedicated crate like `bevy_rapier`?
- How to implement cloth physics for the Drow Ranger's cape?
