# Skeleton Mismatch Analysis

## The Problem

Each GLTF file has its **own skeleton** with the same bone **names** but different **indices**.

## Example: "head" Bone

| File | Bone Name | Local Index | Base Model Index |
|------|-----------|-------------|------------------|
| drow_base | head | 31 | 31 |
| drow_armor | head | 14 | 31 |
| drow_cape | head | 20 | 31 |
| drow_haircowl | head | 6 | 31 |
| drow_quiver | head | 12 | 31 |

## What Happens When Animating

The animation data says: *"Rotate bone #31"*

- **Base model**: Bone #31 is `head` ✓
- **Armor**: Bone #14 is `head`, but it receives animation for #31 ✗
- **Hair**: Bone #6 is `head`, but it receives animation for #31 ✗

Result: Accessories animate the **wrong bones** or don't animate at all!

## Full Bone Mapping

### Base Model (84 nodes, 82 joints)
```
[0]  models/heroes/drow/drow_base.vmdl_c (NOT a joint)
[1]  root                              ← root
[2]  spine1                            ← spine
[3]  spine2
[4]  spine3
[5]  clavicle_L                        ← left shoulder
[6]  bicep_L                           ← left upper arm
[7]  elbow_L                           ← left elbow
[8]  wrist_L                           ← left wrist
[9]  Bow1_0                            ← bow handle
[10] bow8_0_D_L
[11] bow8_1_D_L
[12] Bow2_0_A_L
[13] bow9_0_D_R
[14] bow9_1_D_R
[15] Bow3_0_A_R
[16] attach_bone                       ← weapon attach point
[17] index_0_L                         ← left index finger
[18] index_1_L
[19] index_2_L
[20] thumb_0_L                         ← left thumb
[21] thumb_1_L
[22] thumb_2_L
[23] mid_0_L                           ← left middle finger
[24] mid_1_L
[25] deltoid_0_l                       ← left shoulder pad
[26] Arrows2_0                         ← quiver arrows
[27] Arrows5_0_B_R
[28] Arrows4_0_B_L
[29] Arrows6_0_C_L
[30] neck1                             ← neck
[31] head                              ← HEAD
[32] drow_hair_bangs_0                 ← hair
[33] drow_hair_bangs_1
[34] tresses_0_l                       ← left hair tress
[35] tresses_1_l
[36] tresses_2_l
[37] tresses_0_r                       ← right hair tress
[38] tresses_1_r
[39] tresses_2_r
[40] clavicle_R                        ← right shoulder
[41] bicep_R                           ← right upper arm
[42] elbow_R                           ← right elbow
[43] wrist_R                           ← right wrist
[44] mid_0_R                           ← right middle finger
[45] mid_1_R
[46] thumb_0_R                         ← right thumb
[47] thumb_1_R
[48] thumb_2_R
[49] index_0_R                         ← right index finger
[50] index_1_R
[51] index_2_R
[52] finger_a_0_r
[53] finger_a_1_r
[54] finger_a_2_r
[55] deltoid_0_r                       ← right shoulder pad
[56] cape_R0C0                         ← cape bones (5×3 grid)
[57] cape_R1C0
[58] cape_R2C0
[59] cape_R3C0
[60] cape_R4C0
[61] cape_R0C1
[62] cape_R1C1
[63] cape_R2C1
[64] cape_R3C1
[65] cape_R4C1
[66] cape_R0C2
[67] cape_R1C2
[68] cape_R2C2
[69] cape_R3C2
[70] cape_R4C2
[71] thigh_L                           ← left thigh
[72] knee_L                            ← left knee
[73] ankle_L                           ← left ankle
[74] toeBase_L                         ← left toes
[75] leg_upper_0_l
[76] thigh_R                           ← right thigh
[77] knee_R                            ← right knee
[78] ankle_R                           ← right ankle
[79] toeBase_R                         ← right toes
[80] leg_upper_0_r
[81] skirt_0                           ← skirt bones
[82] skirt_1
[83] models/heroes/drow/drow_base.vmdl_c.drow_base (NOT a joint)
```

### Accessory Joint Counts

| Accessory | Joints | Key Bones |
|-----------|--------|-----------|
| drow_armor | 22 | spine, clavicles, arms, thighs, head |
| drow_bracer | 15 | arms, wrists, fingers |
| drow_cape | 20 | spine3, cape grid, shoulders, head |
| drow_haircowl | 18 | head, tresses, bangs |
| drow_legs | 17 | thighs, knees, ankles, cape bones |
| drow_quiver | 13 | spine, clavicles, arms, head, thigh |
| drow_weapon | 9 | Bow1_0 and bow string bones |

## The Solution

### Option 1: Bone Name Matching (Recommended)
1. Load base model with full skeleton
2. Load accessory meshes
3. For each accessory bone, find matching base bone by **name**
4. Either:
   - **Reparent**: Attach accessory mesh to base bone entity
   - **Copy Transforms**: Each frame, copy base bone transform to accessory bone

### Option 2: Merge at Runtime
1. Load all GLTFs
2. Create a mapping: `accessory_index → base_index` by name
3. When animating, redirect indices through this mapping

### Option 3: Re-export (Artist)
Re-export all accessories with the **full 82-joint skeleton** (same indices as base).

## Implementation Plan

```rust
// 1. Build name→entity map from base model
let bone_map: HashMap<String, Entity> = base_bones
    .iter()
    .map(|(entity, name)| (name.to_string(), entity))
    .collect();

// 2. For each accessory bone, find base equivalent
for (acc_entity, acc_name) in accessory_bones {
    if let Some(base_entity) = bone_map.get(acc_name) {
        // Option A: Reparent mesh to base bone
        commands.entity(acc_entity).set_parent(base_entity);
        
        // Option B: Copy transform each frame
        // (requires system to sync transforms)
    }
}
```
