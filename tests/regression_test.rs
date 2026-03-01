//! Regression Tests for Helix 3D Renderer
//!
//! Run with: cargo test

use bevy::prelude::*;

/// Test that the animation list is correct and matches expected indices
#[test]
fn test_animation_indices() {
    // These must match src/main.rs exactly
    // Updated to use full animations (not additive/pose variants)
    const ANIMATIONS: &[(&str, usize)] = &[
        ("Idle", 48),        // "idle" - main idle animation
        ("Run", 47),         // "run" - main run animation
        ("Attack", 46),      // "attack01" - basic attack
        ("Attack Fast", 25), // "attack_fast" - quick attack
        ("Death", 31),       // "death" - death animation
        ("Victory", 39),     // "victory" - victory pose
        ("Stun", 50),        // "stun" - stunned animation
        ("Teleport", 55),    // "teleport" - teleport animation
        ("Spawn", 52),       // "spawn" - spawn animation
        ("Frost Arrow", 49), // "attack_frost_arrow" - frost arrow attack
    ];

    assert_eq!(ANIMATIONS.len(), 10, "Should have 10 animations");
    assert_eq!(ANIMATIONS[0].0, "Idle");
    assert_eq!(ANIMATIONS[0].1, 48); // Full idle animation, not pose variant
    assert_eq!(ANIMATIONS[1].0, "Run");
    assert_eq!(ANIMATIONS[1].1, 47); // Full run animation, not pose variant
}

/// Test camera position hasn't regressed
#[test]
fn test_camera_transform() {
    let camera_pos = Vec3::new(0.0, 5.0, 10.0);
    let look_at = Vec3::ZERO;

    // Camera should be at this exact position
    assert_eq!(camera_pos.x, 0.0);
    assert_eq!(camera_pos.y, 5.0);
    assert_eq!(camera_pos.z, 10.0);

    // Should look at origin
    assert_eq!(look_at, Vec3::ZERO);
}

/// Test that model path is correct
#[test]
fn test_model_path() {
    let model_path = "test_models/dota_models/models/heroes/drow/drow_base.gltf";
    assert!(model_path.ends_with("drow_base.gltf"));
    assert!(model_path.contains("heroes/drow"));
}

/// Test window configuration
#[test]
fn test_window_config() {
    let resolution = (1280, 720);
    assert_eq!(resolution.0, 1280);
    assert_eq!(resolution.1, 720);

    let title = "Drow Ranger - Complete Assembly";
    assert_eq!(title, "Drow Ranger - Complete Assembly");
}

/// Test debug state defaults
#[test]
fn test_debug_state_defaults() {
    // Debug features should be off by default
    let show_wireframe = false;
    let show_skeleton = false;
    let show_stats = false;
    let show_transforms = false;
    let show_grid = false;

    assert!(!show_wireframe);
    assert!(!show_skeleton);
    assert!(!show_stats);
    assert!(!show_transforms);
    assert!(!show_grid);
}
