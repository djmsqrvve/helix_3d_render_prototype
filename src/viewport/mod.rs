//! Viewport Module
//!
//! Multi-viewport management, render settings per viewport, split views.
//! Comparable to Unreal's viewport system (4-up views, wireframe, etc.).
//!
//! Future systems:
//! - Multi-viewport (perspective, top, front, side)
//! - Viewport render modes (lit, unlit, wireframe, normals)
//! - Viewport overlays (grid, bounds, icons)
//! - Resolution scaling per viewport
//! - Viewport bookmarks
//! - PIP (picture-in-picture) preview

mod config;

#[allow(unused_imports)]
pub use config::{ViewportConfig, ViewportMode};
