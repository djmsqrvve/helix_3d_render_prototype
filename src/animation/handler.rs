use bevy::prelude::*;

use crate::cli::CliArgs;
use crate::components::*;
use crate::constants::ANIMATIONS;

/// Start animation on load — assigns graph and plays initial animation
pub fn start_animation_on_load(
    mut commands: Commands,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    anim_data: Res<AnimationData>,
    base_model_query: Query<Entity, With<BaseModel>>,
    args: Res<CliArgs>,
) {
    let Some(_base_entity) = base_model_query.iter().next() else {
        return;
    };

    for (entity, mut player) in players.iter_mut() {
        commands
            .entity(entity)
            .insert(AnimationGraphHandle(anim_data.graph_handle.clone()));
        commands.entity(entity).insert(BaseAnimationPlayer);

        let anim_index = if args.animation < anim_data.node_indices.len() {
            args.animation
        } else {
            0
        };

        player.play(anim_data.node_indices[anim_index]).repeat();
        info!("[ANIMATION] Started with: {}", ANIMATIONS[anim_index].0);
    }
}

/// Update the animation status text overlay
pub fn update_animation(
    mut text_query: Query<&mut Text, (With<AnimationPlayerController>, Without<StatsText>)>,
    controllers: Query<&AnimationPlayerController>,
) {
    for mut text in text_query.iter_mut() {
        for controller in controllers.iter() {
            let anim_name = ANIMATIONS[controller.animation_index].0;
            let status = if controller.is_playing {
                "Playing"
            } else {
                "Paused"
            };
            text.0 = format!("Animation: {} ({})", anim_name, status);
        }
    }
}
