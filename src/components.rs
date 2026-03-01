use bevy::prelude::*;
use std::collections::HashMap;

/// Debug visualization state
#[derive(Resource, Default, Debug)]
pub struct DebugState {
    pub show_wireframe: bool,
    pub show_skeleton: bool,
    pub show_stats: bool,
    pub show_transforms: bool,
    pub show_grid: bool,
}

/// State for egui debug panel (only compiled with egui feature)
#[cfg(feature = "egui")]
#[derive(Resource, Default)]
pub struct EguiDebugState {
    pub show_window: bool,
    pub selected_animation: usize,
    pub frames_waited: u32,
}

/// Animation graph data
#[derive(Resource)]
pub struct AnimationData {
    pub graph_handle: Handle<AnimationGraph>,
    pub node_indices: Vec<AnimationNodeIndex>,
}

/// Accessory visibility toggle state
#[derive(Resource, Default)]
pub struct AccessoryVisibility {
    pub states: HashMap<String, bool>,
    pub ui_spawned: bool,
}

impl AccessoryVisibility {
    pub fn is_visible(&self, name: &str) -> bool {
        *self.states.get(name).unwrap_or(&true)
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self, name: &str) {
        let current = self.states.get(name).copied().unwrap_or(true);
        self.states.insert(name.to_string(), !current);
    }
}

/// Tracks whether the attachment complete log has been emitted
#[derive(Resource, Default)]
pub struct AttachmentState {
    pub complete_logged: bool,
}

/// Screenshot capture state
#[derive(Resource, Default)]
pub struct ScreenshotState {
    pub frame_count: u32,
    pub capture_requested: bool,
    pub capture_path: Option<std::path::PathBuf>,
    pub capture_frame: u32,
    pub captured: bool,
}

/// Test results for automated testing
#[derive(Resource, Default, Debug)]
pub struct TestResults {
    pub start_time: Option<std::time::Instant>,
}

// --- Component markers ---

#[derive(Component)]
pub struct AnimationPlayerController {
    pub animation_index: usize,
    pub is_playing: bool,
}

#[derive(Component)]
pub struct DebugUI;

#[derive(Component)]
pub struct StatsText;

#[derive(Component)]
pub struct DebugMarker;

/// Marker for the base model (has the full skeleton)
#[derive(Component)]
pub struct BaseModel;

/// Marker for accessory models that need attachment
#[derive(Component)]
pub struct AccessoryModel {
    pub name: String,
    pub root_bone_name: String,
}

/// Marker for checkbox buttons (archived - using egui instead)
#[derive(Component)]
#[allow(dead_code)]
pub struct AccessoryToggleButton {
    pub accessory_name: String,
}

/// Marker for accessories that have been attached to the skeleton
#[derive(Component)]
pub struct AccessoryAttached {
    /// Whether visibility has been propagated after attachment
    pub visibility_propagated: bool,
}

/// Marker for the base model's AnimationPlayer (the only one we control)
#[derive(Component)]
pub struct BaseAnimationPlayer;

/// Marker for the GPU info text overlay
#[derive(Component)]
pub struct GpuInfoText;

/// Tracks GPU info display state
#[derive(Resource, Default)]
pub struct GpuInfoState {
    pub info_displayed: bool,
}
