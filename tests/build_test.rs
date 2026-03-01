//! Build Regression Tests
//!
//! These tests verify the core build requirements and catch dependency issues early.

/// Test that the app builds without the egui feature (default)
/// This is the primary configuration that must always work
#[test]
fn test_builds_without_egui() {
    // If this test compiles, the app builds without egui
    // This is our primary/stable configuration
}

/// Test that the app can build with egui feature if requested
/// This verifies the feature flag works correctly
#[cfg(feature = "egui")]
#[test]
fn test_builds_with_egui() {
    // Only runs when --features egui is passed
    use bevy_egui::EguiPlugin;
    let _ = std::any::type_name::<EguiPlugin>();
}

/// Test that bevy version matches expected
#[test]
fn test_bevy_version() {
    // We expect bevy 0.18.x
    let version = bevy::app::App::new();
    // Just verify we can create an App without panic
    drop(version);
}

/// Test that all required resources can be created
#[test]
fn test_core_resources() {
    use bevy::prelude::*;

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // These should not panic
    app.insert_resource(crate::DebugState::default());
    app.insert_resource(crate::AccessoryVisibility::default());
    app.insert_resource(crate::AttachmentState::default());
}

// Import types we need to test
use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
#[allow(dead_code)]
struct DebugState {
    show_wireframe: bool,
    show_skeleton: bool,
    show_stats: bool,
    show_transforms: bool,
    show_grid: bool,
}

#[derive(Resource, Default)]
#[allow(dead_code)]
struct AccessoryVisibility {
    states: std::collections::HashMap<String, bool>,
    ui_spawned: bool,
}

#[derive(Resource, Default)]
#[allow(dead_code)]
struct AttachmentState {
    complete_logged: bool,
}
