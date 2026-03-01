# Model & Accessory Module

## Responsibility
Handles the loading of the primary hero model and the dynamic attachment of accessories to the underlying skeleton.

## Key Symbols
- `HeroModel`: Primary component for the hero entity.
- `Accessory`: Marker component for attached items (armor, capes, weapons).
- `attach_accessories_to_skeleton()`: The core system that maps accessory bones to base bones by name.

## Guardrails (DON'T DO)
- **❌ DO NOT** assume all models share the same bone hierarchy. Always check bone names dynamically.
- **❌ DO NOT** manually scale individual bones without accounting for the global model scale (0.1x for Dota 2 models).
- **❌ DO NOT** delete the base skeleton while accessories are still attached.

## Questions
- How to efficiently handle material/texture overrides on individual accessories?
- Should we bake accessories into a single mesh for performance (LODs)?
