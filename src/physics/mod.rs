//! Physics Module
//!
//! Raycasting, collision detection, physics debugging.
//! Comparable to Unreal's physics system / PhysX integration.
//!
//! Future systems:
//! - Raycasting from mouse cursor (for selection)
//! - Collision mesh visualization
//! - Physics simulation preview
//! - Ragdoll preview
//! - Cloth simulation preview
//! - Hit testing for editor selection

mod raycasting;

#[allow(unused_imports)]
pub use raycasting::RaycastConfig;
