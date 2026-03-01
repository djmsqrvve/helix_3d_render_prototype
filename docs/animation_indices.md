# Animation Indices Reference

## Overview

Drow Ranger's model contains **72 animations** in the GLTF file. However, not all are suitable for direct playback. This guide explains the difference and which indices to use.

## Animation Types in Source 2 Exports

### 1. Full Animations ✅ (Use These)

These animate **all 82 bones** in the skeleton and are the primary animations.

| Index | Name | Description |
|-------|------|-------------|
| 48 | `idle` | Standing idle |
| 47 | `run` | Running loop |
| 46 | `attack01` | Basic bow attack |
| 49 | `attack_frost_arrow` | Frost arrow attack |
| 25 | `attack_fast` | Quick attack |
| 31 | `death` | Death animation |
| 39 | `victory` | Victory pose |
| 50 | `stun` | Stunned/dazed |
| 55 | `teleport` | Teleport animation |
| 52 | `spawn` | Spawn/respawn |

### 2. Additive/Pose Animations ❌ (Don't Use Directly)

These are **partial animations** designed to be layered on top of base poses. They have prefixes like `@` or suffixes like `_anim`.

| Index | Name | Type | Problem |
|-------|------|------|---------|
| 19 | `idle_anim` | Pose variant | Only affects subset of bones |
| 12 | `@run` | Additive | Relies on base pose, incomplete alone |
| 26 | `attack01_anim` | Pose variant | Partial animation |
| 17 | `idle_alt_injured` | Variant | Incomplete for standalone use |

**Why they fail:** These only animate 20-40 bones instead of all 82. When played alone, parts of the model don't move or deform incorrectly.

### 3. Channel Count Check

Full animations have **246 channels** (3 translation + 4 rotation per bone × 82 bones).

```rust
// How to check in Python:
import json
with open('drow_base.gltf') as f:
    data = json.load(f)
    
for anim in data['animations']:
    name = anim.get('name', '')
    channels = len(anim.get('channels', []))
    print(f"{name}: {channels} channels")
```

- **246 channels** = Full animation ✅
- **< 246 channels** = Partial/additive ⚠️

## Recommended Animation Indices

```rust
const ANIMATIONS: &[(&str, usize)] = &[
    ("Idle", 48),        // Full idle
    ("Run", 47),         // Full run  
    ("Attack", 46),      // Full attack
    ("Attack Fast", 25), // Full fast attack
    ("Death", 31),       // Full death
    ("Victory", 39),     // Full victory
    ("Stun", 50),        // Full stun
    ("Teleport", 55),    // Full teleport
    ("Spawn", 52),       // Full spawn
    ("Frost Arrow", 49), // Full frost arrow
];
```

## How to Find Animations

List all animations with their channel counts:

```bash
python3 << 'EOF'
import json
with open('assets/test_models/dota_models/models/heroes/drow/drow_base.gltf') as f:
    data = json.load(f)

print("=== Full Animations (246 channels) ===")
for i, anim in enumerate(data['animations']):
    name = anim.get('name', '')
    channels = len(anim.get('channels', []))
    if channels == 246 and not name.startswith('@') and not name.endswith('_anim'):
        print(f"  {i}: {name}")
EOF
```

## Animation System Implementation

The key to proper playback:

1. **Use `AnimationGraph`** - Manages animation nodes
2. **Call `stop_all()` before playing** - Prevents mixing animations
3. **Only control one `AnimationPlayer`** - The base model's

```rust
// Play animation (stops previous automatically)
if let Some(mut player) = animation_player.iter_mut().next() {
    AnimationPlayer::stop_all(&mut player);  // Clear previous
    AnimationPlayer::play(&mut player, node_index).repeat();
}
```

## Source 2 Naming Conventions

| Prefix/Suffix | Meaning | Use Case |
|---------------|---------|----------|
| `@` | Additive layer | Blending on top of base |
| `_anim` | Pose variant | Alternative to main animation |
| `_injured` | State variant | Damaged state animation |
| `_rare` | Rare cosmetic | Special version |
| `loadout_` | Menu pose | Hero selection screen |
| `portrait_` | Portrait pose | Static display |
| `portrait_idle_` | Portrait idle | Subtle movement |

## Debugging Animation Issues

If animations look wrong:

1. **Check channel count**: Should be 246 for full animations
2. **Check name**: Avoid `@` prefix and `_anim` suffix for standalone playback
3. **Check bone count**: Model should have 82 joints
4. **Enable debug logging**: `RUST_LOG=info cargo run`

## See Also

- `docs/DEV_SKELETON_DEEP_DIVE.md` - Bone hierarchy details
- `docs/DEV_TROUBLESHOOTING.md` - Common runtime issues
