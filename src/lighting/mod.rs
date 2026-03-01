//! Lighting Module
//!
//! Light management, IBL (Image-Based Lighting), environment maps, presets.
//! Comparable to Unreal's lighting system.
//!
//! Future systems:
//! - Lighting presets (studio, outdoor, dramatic)
//! - HDRI environment maps
//! - Light probe baking
//! - Shadow cascade configuration
//! - Volumetric lighting
//! - Light color temperature (Kelvin)

mod presets;

#[allow(unused_imports)]
pub use presets::{LightPreset, LightingConfig};
