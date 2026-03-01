#![allow(dead_code)]
use bevy::prelude::*;

/// Per-viewport configuration
#[derive(Resource, Debug)]
pub struct ViewportConfig {
    /// Render mode for main viewport
    pub render_mode: ViewportMode,
    /// Show grid overlay
    pub show_grid: bool,
    /// Show entity bounds
    pub show_bounds: bool,
    /// Show light icons
    pub show_light_icons: bool,
    /// Show camera frustums
    pub show_camera_frustums: bool,
    /// Target FPS for the viewport (0 = uncapped)
    pub target_fps: u32,
}

impl Default for ViewportConfig {
    fn default() -> Self {
        Self {
            render_mode: ViewportMode::Lit,
            show_grid: true,
            show_bounds: false,
            show_light_icons: true,
            show_camera_frustums: false,
            target_fps: 0,
        }
    }
}

/// Viewport rendering mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewportMode {
    /// Full shading with lighting
    Lit,
    /// No lighting, flat color
    Unlit,
    /// Wireframe overlay
    Wireframe,
    /// Normals visualization
    Normals,
    /// UV coordinates visualization
    UVs,
    /// Vertex colors
    VertexColors,
    /// Overdraw heat map
    Overdraw,
    /// LOD visualization
    LodColoring,
}

// Future: implement viewport_mode_system, multi_viewport_layout, etc.
