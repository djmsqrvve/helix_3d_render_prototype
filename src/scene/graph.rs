#![allow(dead_code)]
use bevy::prelude::*;

/// Scene-level configuration
#[derive(Resource, Debug)]
pub struct SceneConfig {
    /// Scene name
    pub name: String,
    /// Scene file path (for save/load)
    pub file_path: Option<std::path::PathBuf>,
    /// Whether the scene has unsaved changes
    pub is_dirty: bool,
}

impl Default for SceneConfig {
    fn default() -> Self {
        Self {
            name: "Untitled Scene".to_string(),
            file_path: None,
            is_dirty: false,
        }
    }
}

/// Current scene state
#[derive(Resource, Debug, Default)]
pub struct SceneState {
    /// Number of loaded entities
    pub entity_count: usize,
    /// Number of loaded meshes
    pub mesh_count: usize,
    /// Whether the scene is fully loaded
    pub is_loaded: bool,
    /// Total loading progress (0.0 - 1.0)
    pub load_progress: f32,
}

// Future: implement save_scene_system, load_scene_system, etc.
