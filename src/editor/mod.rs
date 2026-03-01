//! Editor Tools Module
//!
//! Selection, transform gizmos, property inspector, content browser.
//! Comparable to Unreal's Editor viewport tools.
//!
//! Future systems:
//! - Entity selection (click to select, box select)
//! - Transform gizmos (move, rotate, scale) with snapping
//! - Property inspector panel
//! - Content browser / asset browser
//! - Entity spawning / placement tools
//! - Copy/paste, duplicate
//! - Group/ungroup
//! - Snap to grid, snap to surface

mod tools;

#[allow(unused_imports)]
pub use tools::{EditorState, EditorTool, TransformSpace};
