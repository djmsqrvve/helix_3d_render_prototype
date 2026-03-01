use bevy::app::AppExit;
use bevy::prelude::*;
use chrono::Local;
use std::path::PathBuf;

use crate::cli::CliArgs;
use crate::components::*;
use crate::constants::ANIMATIONS;

/// Handle keyboard input for animation switching, pause/resume, screenshot, exit
pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit_writer: MessageWriter<AppExit>,
    mut animation_player: Query<&mut AnimationPlayer, With<BaseAnimationPlayer>>,
    mut controllers: Query<&mut AnimationPlayerController>,
    anim_data: Res<AnimationData>,
    mut screenshot_state: ResMut<ScreenshotState>,
    args: Res<CliArgs>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit_writer.write(AppExit::Success);
    }

    // F6 for screenshot
    if keys.just_pressed(KeyCode::F6) {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let path = PathBuf::from(format!("screenshot_{}.png", timestamp));
        screenshot_state.capture_requested = true;
        screenshot_state.capture_path = Some(path);
        screenshot_state.capture_frame = screenshot_state.frame_count + 1;
        info!(
            "[SCREENSHOT] Requested capture on frame {}",
            screenshot_state.capture_frame
        );
    }

    let mut new_animation: Option<usize> = None;

    if keys.just_pressed(KeyCode::Digit1) {
        new_animation = Some(0);
    } else if keys.just_pressed(KeyCode::Digit2) {
        new_animation = Some(1);
    } else if keys.just_pressed(KeyCode::Digit3) {
        new_animation = Some(2);
    } else if keys.just_pressed(KeyCode::Digit4) {
        new_animation = Some(3);
    } else if keys.just_pressed(KeyCode::Digit5) {
        new_animation = Some(4);
    } else if keys.just_pressed(KeyCode::Digit6) {
        new_animation = Some(5);
    } else if keys.just_pressed(KeyCode::Digit7) {
        new_animation = Some(6);
    } else if keys.just_pressed(KeyCode::Digit8) {
        new_animation = Some(7);
    } else if keys.just_pressed(KeyCode::Digit9) {
        new_animation = Some(8);
    }

    let anim_index = new_animation.unwrap_or(args.animation);

    if new_animation.is_some() || anim_index != 0 {
        for mut controller in controllers.iter_mut() {
            controller.animation_index = anim_index;
        }

        if anim_index < anim_data.node_indices.len() {
            let node_index = anim_data.node_indices[anim_index];

            if let Some(mut player) = animation_player.iter_mut().next() {
                AnimationPlayer::stop_all(&mut player);
                AnimationPlayer::play(&mut player, node_index).repeat();
                info!("[ANIMATION] Changed to: {}", ANIMATIONS[anim_index].0);
            }
        }
    }

    if keys.just_pressed(KeyCode::Space) {
        if let Some(mut player) = animation_player.iter_mut().next() {
            for mut controller in controllers.iter_mut() {
                controller.is_playing = !controller.is_playing;
                if controller.is_playing {
                    AnimationPlayer::resume_all(&mut player);
                } else {
                    AnimationPlayer::pause_all(&mut player);
                }
            }
        }
    }
}
