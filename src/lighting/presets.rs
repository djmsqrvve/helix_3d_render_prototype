#![allow(dead_code)]
use bevy::prelude::*;

/// Global lighting configuration
#[derive(Resource, Debug)]
pub struct LightingConfig {
    /// Active lighting preset
    pub preset: LightPreset,
    /// Ambient light intensity
    pub ambient_intensity: f32,
    /// Ambient light color
    pub ambient_color: Color,
    /// Environment map path (HDRI)
    pub environment_map: Option<String>,
    /// Shadow enabled globally
    pub shadows_enabled: bool,
}

impl Default for LightingConfig {
    fn default() -> Self {
        Self {
            preset: LightPreset::Studio,
            ambient_intensity: 0.3,
            ambient_color: Color::srgb(0.8, 0.85, 1.0),
            environment_map: None,
            shadows_enabled: true,
        }
    }
}

/// Predefined lighting presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightPreset {
    /// 3-point studio lighting
    Studio,
    /// Outdoor daylight
    Outdoor,
    /// Golden hour
    GoldenHour,
    /// Dramatic single light
    Dramatic,
    /// Flat uniform lighting (for texture checking)
    Flat,
    /// Custom (user-defined)
    Custom,
}

// Future: implement apply_preset_system, hdri_loader, etc.
