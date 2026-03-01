//! Egui debug panel — only compiled when `--features egui` is passed.
//! Provides animation controls, debug toggles, file picker, and performance stats.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::*;
use crate::constants::{ACCESSORIES, ANIMATIONS};
use crate::ui::format_accessory_name;

/// Resource for managing file dialog state
#[derive(Resource, Default)]
pub struct FileDialogState {
    pub file_path: Option<String>,
    pub show_file_input: bool,
}

/// Egui debug panel system
#[allow(clippy::too_many_arguments)]
pub fn egui_debug_panel(
    mut contexts: EguiContexts,
    mut debug_state: ResMut<DebugState>,
    mut egui_state: ResMut<EguiDebugState>,
    mut accessory_visibility: ResMut<AccessoryVisibility>,
    mut animation_player: Query<&mut AnimationPlayer, With<BaseAnimationPlayer>>,
    mut controllers: Query<&mut AnimationPlayerController>,
    anim_data: Res<AnimationData>,
    time: Res<Time>,
    mut file_dialog_state: ResMut<FileDialogState>,
) -> Result {
    // Skip first 2 frames to let egui context initialize fonts
    if egui_state.frames_waited < 2 {
        egui_state.frames_waited += 1;
        return Ok(());
    }

    if !egui_state.show_window {
        egui_state.show_window = true;
    }

    let ctx = contexts.ctx_mut()?;

    // Sync egui animation selection with actual controller state
    for controller in controllers.iter() {
        egui_state.selected_animation = controller.animation_index;
    }

    egui::Window::new("Helix Debug Panel")
        .default_pos([10.0, 10.0])
        .default_size([300.0, 500.0])
        .collapsible(true)
        .resizable(true)
        .movable(true)
        .show(ctx, |ui| {
            // File Picker Section
            ui.heading("Model");
            ui.separator();

            if ui.button("📁 Open GLTF File...").clicked() {
                file_dialog_state.show_file_input = true;
            }

            // Simple text input for file path (file dialog coming soon)
            if file_dialog_state.show_file_input {
                ui.label("Enter GLTF path:");
                let mut path = file_dialog_state.file_path.clone().unwrap_or_default();
                if ui.text_edit_singleline(&mut path).changed() {
                    file_dialog_state.file_path = Some(path);
                }
                if ui.button("Load").clicked() {
                    if let Some(path) = &file_dialog_state.file_path {
                        info!("[EGUI][FILE] Load requested: {}", path);
                        // TODO: Implement actual file loading
                    }
                    file_dialog_state.show_file_input = false;
                }
                if ui.button("Cancel").clicked() {
                    file_dialog_state.show_file_input = false;
                }
            }

            if let Some(path) = &file_dialog_state.file_path {
                ui.label(
                    egui::RichText::new(format!("Selected: {}", path))
                        .small()
                        .color(egui::Color32::YELLOW),
                );
            } else {
                ui.label(
                    egui::RichText::new("Using default: Drow Ranger")
                        .small()
                        .weak(),
                );
            }

            ui.separator();
            ui.heading("Animation Controls");
            ui.separator();

            // Animation selector - using buttons for better reliability
            ui.label("Select Animation:");
            for (i, (name, _)) in ANIMATIONS.iter().enumerate() {
                let is_selected = egui_state.selected_animation == i;

                // Use a button with play indicator for selected
                let button_text = if is_selected {
                    format!("▶ {}", name)
                } else {
                    format!("  {}", name)
                };

                let button_color = if is_selected {
                    egui::Color32::GREEN
                } else {
                    ui.style().visuals.text_color()
                };

                let response = ui.button(egui::RichText::new(button_text).color(button_color));

                if response.clicked() && !is_selected {
                    egui_state.selected_animation = i;

                    // Play the animation
                    if i < anim_data.node_indices.len() {
                        let node_index = anim_data.node_indices[i];

                        // Get the animation player and play
                        if let Ok(mut player) = animation_player.single_mut() {
                            AnimationPlayer::stop_all(&mut player);
                            AnimationPlayer::play(&mut player, node_index).repeat();
                            info!("[EGUI][ANIMATION] Changed to: {}", name);
                        }

                        // Update controller
                        for mut controller in controllers.iter_mut() {
                            controller.animation_index = i;
                        }
                    }
                }
            }

            // Pause/Resume button
            if let Ok(controller) = controllers.single() {
                ui.separator();
                let button_text = if controller.is_playing {
                    "⏸ Pause"
                } else {
                    "▶ Resume"
                };
                if ui.button(button_text).clicked() {
                    if let Some(mut player) = animation_player.iter_mut().next() {
                        for mut controller in controllers.iter_mut() {
                            controller.is_playing = !controller.is_playing;
                            if controller.is_playing {
                                AnimationPlayer::resume_all(&mut player);
                                info!("[EGUI][ANIMATION] Resumed");
                            } else {
                                AnimationPlayer::pause_all(&mut player);
                                info!("[EGUI][ANIMATION] Paused");
                            }
                        }
                    }
                }
            }

            ui.separator();
            ui.heading("Debug Visualizations");

            // Debug toggles
            ui.checkbox(&mut debug_state.show_wireframe, "Wireframe (F1)");
            ui.checkbox(&mut debug_state.show_skeleton, "Skeleton (F2)");
            ui.checkbox(&mut debug_state.show_stats, "Stats (F3)");
            ui.checkbox(&mut debug_state.show_transforms, "Transforms (F4)");
            ui.checkbox(&mut debug_state.show_grid, "Grid (F5)");

            ui.separator();
            ui.heading("Accessories");

            // Accessory visibility toggles
            for (name, _) in ACCESSORIES.iter() {
                let display_name = format_accessory_name(name);
                let is_visible = accessory_visibility.is_visible(name);

                let button_text = if is_visible { "☑" } else { "☐" };
                if ui
                    .button(format!("{} {}", button_text, display_name))
                    .clicked()
                {
                    accessory_visibility.toggle(name);
                    let new_state = accessory_visibility.is_visible(name);
                    info!(
                        "[EGUI][TOGGLE] {}: {}",
                        name,
                        if new_state { "ON" } else { "OFF" }
                    );
                }
            }

            ui.separator();
            ui.heading("Performance");
            ui.label(format!("FPS: {:.1}", 1.0 / time.delta_secs()));
            ui.label(format!("Frame Time: {:.2}ms", time.delta_secs() * 1000.0));

            ui.separator();
            ui.small("Egui debug panel enabled via --features egui");
        });

    Ok(())
}
