//! Camera Controller Module
//!
//! Camera rigs, orbital controls, fly-through mode, cinematic sequences.
//! Comparable to Unreal's camera system / viewport navigation.
//!
//! Future systems:
//! - Orbit camera (current: static)
//! - Fly-through / FPS camera
//! - Cinematic camera rails
//! - Multi-camera switching
//! - Camera bookmarks / saved positions
//! - Focus-on-selection

mod controller;

#[allow(unused_imports)]
pub use controller::{CameraController, CameraMode};
