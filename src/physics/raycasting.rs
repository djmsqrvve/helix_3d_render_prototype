#![allow(dead_code)]
use bevy::prelude::*;

/// Raycasting configuration
#[derive(Resource, Debug)]
pub struct RaycastConfig {
    /// Maximum ray distance
    pub max_distance: f32,
    /// Layer mask for selective raycasting
    pub layer_mask: u32,
    /// Whether to raycast against mesh colliders (slower) or bounds only (faster)
    pub use_mesh_colliders: bool,
    /// Debug visualization of rays
    pub show_debug_rays: bool,
}

impl Default for RaycastConfig {
    fn default() -> Self {
        Self {
            max_distance: 1000.0,
            layer_mask: u32::MAX,
            use_mesh_colliders: false,
            show_debug_rays: false,
        }
    }
}

// Future: implement raycast_from_cursor, hit_test_system, etc.
