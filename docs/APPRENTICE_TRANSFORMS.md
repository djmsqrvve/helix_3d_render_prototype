# Understanding Transforms in 3D Graphics

## What is a "Transform"?

A **transform** is a mathematical description of an object's position, rotation, and scale in 3D space. It's the fundamental building block of 3D scene graphs.

In code, a transform typically looks like:
```rust
struct Transform {
    translation: Vec3,  // Position (x, y, z)
    rotation: Quat,     // Rotation (quaternion)
    scale: Vec3,        // Scale (x, y, z)
}
```

---

## Is "Transform" a Blender Term?

**Yes and no.**

- **Blender**: Uses "Transform" in the UI (G key to grab/move, R to rotate, S to scale)
- **Maya/3ds Max**: Same concept, sometimes called "Transform" or "Local axes"
- **Game Engines**: Universal term (Unity, Unreal, Godot, Bevy all use "Transform")
- **Math/Graphics**: Called "affine transformation" or "rigid body transform"

It's a **universal graphics concept**, not specific to any one tool.

---

## The Transform Hierarchy (Scene Graph)

```
Scene (Root)
├── Camera (Transform)
├── Light (Transform)
└── Character (Transform)
    ├── Torso (Transform)
    │   ├── Head (Transform)
    │   │   └── Hat (Transform)
    │   ├── LeftArm (Transform)
    │   │   └── LeftHand (Transform)
    │   └── RightArm (Transform)
    │       └── RightHand (Transform)
    │           └── Sword (Transform)
    └── Legs (Transform)
```

**Key concept**: Child transforms are **relative to parent**.
- If you move the torso, head and arms move with it
- If you rotate the arm, the hand follows
- This is called a **scene graph** or **transform hierarchy**

---

## Types of Transforms

### 1. Local Transform
Position/rotation/scale **relative to parent**.

```rust
// Hand is 5 units from shoulder
hand.local_transform.translation = Vec3::new(0.0, -5.0, 0.0);
```

### 2. Global/World Transform
Position/rotation/scale in **world coordinates** (absolute).

```rust
// Where is the hand in the world?
let world_pos = hand.global_transform.translation;
// This includes: character position + torso + arm + hand offset
```

**Conversion**: `Global = ParentGlobal × Local`

---

## In Our Drow Ranger Project

### What We Were Showing (Wrong)
```rust
// This queries ALL transforms in the scene
transforms: Query<&GlobalTransform>

// Result:
// - Camera at (0, 5, 10)
// - Light at (5, 10, 5)
// - Drow mesh at (0, 0, 0)
// - Individual mesh parts at various positions
// - UI elements
// - Debug grid markers
```

This is why the transforms looked random - they were for **every object**!

### What We Should Show (Fixed)
```rust
// Only transforms of animated bones
animated_entities: Query<&GlobalTransform, With<AnimationPlayer>>

// Result:
// - spine bone at (0, 30, 0)
// - shoulder bone at (-2, 35, 0)
// - elbow bone at (-5, 35, 0)
// - wrist bone at (-8, 35, 0)
```

These are the **actual skeleton bones** that move during animation.

---

## Visual Example: Bone Transforms

When the Drow Ranger raises her arm:

```
Frame 1 (Rest Pose):
  Shoulder: rotation = 0°, 0°, 0°
  Elbow:    rotation = 0°, 0°, 0°

Frame 2 (Arm Raised):
  Shoulder: rotation = 0°, 0°, 90°  <-- rotated up
  Elbow:    rotation = 0°, 0°, 45°  <-- bent
```

Each bone has its own **local transform** that rotates relative to its parent.

---

## Why Transforms Matter for Debugging

### The "Wobbly Animation" Problem
If transforms are wrong:
- Bones move independently (not following hierarchy)
- Scale mismatches cause stretching
- Wrong rotation axes cause weird flipping

### How We Use Them
With **F4 (transform gizmos)**, we draw XYZ axes at each bone to verify:
- Are bones positioned correctly?
- Do they rotate around the right axes?
- Is the hierarchy connected?

---

## Key Takeaways

1. **Transform = Position + Rotation + Scale**
2. **Local** = relative to parent
3. **Global** = absolute world position
4. **Hierarchy** = parents affect children
5. **Skeleton bones** are just transforms arranged in a hierarchy
6. **Animation** changes transform values over time

---

## Related Terms

| Term | Meaning |
|------|---------|
| **Matrix** | Math representation of transform (4×4 matrix) |
| **TRS** | Translate, Rotate, Scale (components) |
| **Joint** | A transform in a skeleton hierarchy |
| **Bind Pose** | Default/rest pose transforms |
| **Skinning** | Attaching mesh vertices to joints |
| **Scene Graph** | Tree of transforms |
