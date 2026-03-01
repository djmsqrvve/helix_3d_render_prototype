/// Animation indices that point to FULL animations (not additive/pose variants)
pub const ANIMATIONS: &[(&str, usize)] = &[
    ("Idle", 48),
    ("Run", 47),
    ("Attack", 46),
    ("Attack Fast", 25),
    ("Death", 31),
    ("Victory", 39),
    ("Stun", 50),
    ("Teleport", 55),
    ("Spawn", 52),
    ("Frost Arrow", 49),
];

/// Accessories and their expected root bone for attachment
pub const ACCESSORIES: &[(&str, &str)] = &[
    ("drow_armor", "root"),
    ("drow_bracer", "wrist_L"),
    ("drow_cape", "spine3"),
    ("drow_haircowl", "head"),
    ("drow_legs", "thigh_L"),
    ("drow_quiver", "spine2"),
    ("drow_weapon", "Bow1_0"),
    ("drow_marksmanship_arrow", "Bow1_0"),
];

/// --- APPRENTICE SECTION ---

/// LESSON 3: Change this number to make her spin faster or slower!
pub const ROTATION_SPEED: f32 = 0.5;
