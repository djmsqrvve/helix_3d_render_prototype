//! Scene Management Module
//!
//! Scene graph, hierarchy management, serialization/deserialization.
//! Comparable to Unreal's Level/World system.
//!
//! Future systems:
//! - Scene save/load (serialize to .helix format)
//! - Scene hierarchy panel (outliner)
//! - Multi-scene management
//! - Scene transitions & streaming
//! - Undo/redo transaction system
//! - Entity templates / prefabs

mod graph;

#[allow(unused_imports)]
pub use graph::{SceneConfig, SceneState};
