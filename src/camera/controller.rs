#![allow(dead_code)]
use bevy::prelude::*;

/// Camera controller component — attach to a camera entity to enable viewport navigation
#[derive(Component, Debug)]
pub struct CameraController {
    pub mode: CameraMode,
    /// Orbit distance from target
    pub orbit_distance: f32,
    /// Orbit target point
    pub orbit_target: Vec3,
    /// Rotation speed (degrees/pixel of mouse movement)
    pub rotation_speed: f32,
    /// Zoom speed (distance/scroll tick)
    pub zoom_speed: f32,
    /// Pan speed (world units/pixel)
    pub pan_speed: f32,
    /// Fly-through movement speed
    pub fly_speed: f32,
    /// Current yaw angle (radians)
    pub yaw: f32,
    /// Current pitch angle (radians)
    pub pitch: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            mode: CameraMode::Orbit,
            orbit_distance: 10.0,
            orbit_target: Vec3::ZERO,
            rotation_speed: 0.3,
            zoom_speed: 1.0,
            pan_speed: 0.01,
            fly_speed: 5.0,
            yaw: 0.0,
            pitch: -0.4,
        }
    }
}

/// Camera navigation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    /// Orbit around a target point (like Maya/Blender)
    Orbit,
    /// Free fly-through (like Unreal viewport)
    Fly,
    /// Cinematic rail follow
    Cinematic,
    /// Locked / user has no control
    Locked,
}

// Future: implement orbit_camera_system, fly_camera_system, etc.
