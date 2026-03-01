use bevy::prelude::*;

use crate::components::*;

/// Spawn the controls help text and status overlays
pub fn spawn_debug_ui(commands: &mut Commands) {
    commands.spawn((
        Text::new(get_controls_text()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));

    commands.spawn((
        Text::new("Animation: Idle (Playing)"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(12.0),
            ..default()
        },
        AnimationPlayerController {
            animation_index: 0,
            is_playing: true,
        },
    ));

    commands.spawn((
        Text::new(get_debug_text()),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
        DebugUI,
        StatsText,
    ));

    commands.spawn((
        Text::new(get_debug_controls_text()),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            right: Val::Px(12.0),
            ..default()
        },
    ));
}

fn get_controls_text() -> String {
    use crate::constants::ANIMATIONS;
    let mut text = String::from("Drow Ranger - Complete Assembly\n\n");
    text.push_str("Controls:\n");
    for (i, (name, _)) in ANIMATIONS.iter().enumerate() {
        text.push_str(&format!("  {} - {}\n", i + 1, name));
    }
    text.push_str("\nSpace - Pause/Resume\n");
    text.push_str("ESC - Exit\n");
    text.push_str("F6 - Screenshot");
    text
}

fn get_debug_controls_text() -> String {
    let mut text = String::from("Debug Controls:\n");
    text.push_str("  F1 - Toggle Wireframe\n");
    text.push_str("  F2 - Toggle Skeleton\n");
    text.push_str("  F3 - Toggle Stats\n");
    text.push_str("  F4 - Toggle Transforms\n");
    text.push_str("  F5 - Toggle Grid\n");
    text.push_str("  F6 - Screenshot");
    text
}

fn get_debug_text() -> String {
    String::from("Debug Stats:\n  Press F3")
}
