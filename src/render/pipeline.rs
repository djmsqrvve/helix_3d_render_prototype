#![allow(dead_code)]
use bevy::prelude::*;

/// Global render configuration
#[derive(Resource, Debug)]
pub struct RenderConfig {
    /// Ambient occlusion enabled
    pub ao_enabled: bool,
    /// Bloom post-processing enabled
    pub bloom_enabled: bool,
    /// Tone mapping style
    pub tone_mapping: ToneMappingStyle,
    /// Shadow quality level (0=off, 1=low, 2=medium, 3=high, 4=ultra)
    pub shadow_quality: u8,
    /// Anti-aliasing mode
    pub aa_mode: AntiAliasingMode,
    /// Background color
    pub clear_color: Color,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            ao_enabled: false,
            bloom_enabled: false,
            tone_mapping: ToneMappingStyle::AcesFitted,
            shadow_quality: 2,
            aa_mode: AntiAliasingMode::Msaa4x,
            clear_color: Color::srgb(0.1, 0.1, 0.15),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToneMappingStyle {
    None,
    Reinhard,
    AcesFitted,
    AgX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntiAliasingMode {
    None,
    Fxaa,
    Msaa2x,
    Msaa4x,
    Msaa8x,
    // Future: TAA, DLSS
}
