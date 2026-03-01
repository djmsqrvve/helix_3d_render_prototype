#![allow(dead_code)]
use bevy::prelude::*;

/// Current editor state
#[derive(Resource, Debug)]
pub struct EditorState {
    /// Currently active tool
    pub active_tool: EditorTool,
    /// Transform space (local vs world)
    pub transform_space: TransformSpace,
    /// Grid snap enabled
    pub snap_enabled: bool,
    /// Grid snap size
    pub snap_size: f32,
    /// Rotation snap (degrees)
    pub rotation_snap: f32,
    /// Currently selected entity
    pub selected_entity: Option<Entity>,
    /// Selection history for undo
    pub selection_history: Vec<Option<Entity>>,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            active_tool: EditorTool::Select,
            transform_space: TransformSpace::World,
            snap_enabled: true,
            snap_size: 1.0,
            rotation_snap: 15.0,
            selected_entity: None,
            selection_history: Vec::new(),
        }
    }
}

/// Available editor tools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorTool {
    /// Click to select entity
    Select,
    /// Move (translate) entity
    Move,
    /// Rotate entity
    Rotate,
    /// Scale entity
    Scale,
    /// Place new entity
    Place,
}

/// Coordinate space for transforms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformSpace {
    /// World-aligned axes
    World,
    /// Object-local axes
    Local,
}

// Future: implement selection_system, gizmo_render_system, etc.
