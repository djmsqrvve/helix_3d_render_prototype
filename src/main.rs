mod animation;
mod cli;
mod components;
mod constants;
mod debug;
mod input;
mod logging;
mod model;
mod ui;

// Future engine systems — structured and ready to build out
mod assets;
mod camera;
mod editor;
mod lighting;
mod physics;
mod render;
mod scene;
mod viewport;

#[cfg(feature = "egui")]
mod egui_panel;

use bevy::prelude::*;
use clap::Parser;

#[cfg(feature = "egui")]
use bevy_egui::EguiPlugin;

use cli::CliArgs;
use components::*;

fn main() {
    let args = CliArgs::parse();

    // Initialize logging (local or remote based on feature/args)
    logging::init_logging(args.log_endpoint.as_deref());

    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Drow Ranger - Complete Assembly".to_string(),
            resolution: bevy::window::WindowResolution::new(1920, 1080),
            ..default()
        }),
        ..default()
    };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(window_plugin).disable::<bevy::log::LogPlugin>())
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1))) // LESSON 2: CHANGE THIS COLOR!
        .insert_resource(args)
        .insert_resource(AccessoryVisibility::default())
        .insert_resource(AttachmentState::default())
        .insert_resource(DebugState::default())
        .insert_resource(ScreenshotState::default())
        .insert_resource(GpuInfoState::default())
        .insert_resource(TestResults::default());

    // Egui plugin: only added when `--features egui` is passed
    #[cfg(feature = "egui")]
    {
        app.add_plugins(EguiPlugin::default())
            .insert_resource(EguiDebugState::default())
            .insert_resource(egui_panel::FileDialogState::default())
            .add_systems(Update, egui_panel::egui_debug_panel);
        info!("[EGUI] Debug panel enabled via feature flag");
    }

    app.add_systems(Startup, model::setup)
        .add_systems(
            Update,
            (
                animation::start_animation_on_load,
                animation::update_animation,
                model::attach_accessories_to_skeleton,
                model::update_accessory_visibility,
                model::rotate_hero,
                debug::toggle_debug_modes,
                debug::update_debug_ui,
                debug::draw_debug_gizmos,
                input::keyboard_input,
                input::screenshot_capture_system,
                input::log_system_diagnostics,
                render::display_gpu_info,
            ),
        )
        .run();
}
