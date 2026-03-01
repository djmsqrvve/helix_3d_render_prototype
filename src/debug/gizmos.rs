use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::components::*;

/// Draw debug gizmos (transform axes, skeleton lines)
pub fn draw_debug_gizmos(
    debug_state: Res<DebugState>,
    mut gizmos: Gizmos,
    animated_entities: Query<&GlobalTransform, With<AnimationPlayer>>,
    // O(N) skeleton: query parent-child relationships directly
    bone_query: Query<(Entity, &GlobalTransform, Option<&ChildOf>), With<AnimationPlayer>>,
    parent_transform_query: Query<&GlobalTransform>,
) {
    if debug_state.show_transforms {
        for transform in animated_entities.iter() {
            let translation = transform.translation();
            let (_scale, rotation, _trans) = transform.to_scale_rotation_translation();

            let scale = 2.0;
            let right = rotation * Vec3::X * scale;
            let up = rotation * Vec3::Y * scale;
            let forward = rotation * Vec3::Z * scale;

            gizmos.line(translation, translation + right, Color::srgb(1.0, 0.0, 0.0));
            gizmos.line(translation, translation + up, Color::srgb(0.0, 1.0, 0.0));
            gizmos.line(
                translation,
                translation + forward,
                Color::srgb(0.0, 0.0, 1.0),
            );
        }
    }

    if debug_state.show_skeleton {
        // O(N) parent-child lookup instead of O(N²) distance check
        for (_entity, child_transform, parent) in bone_query.iter() {
            if let Some(parent) = parent {
                // Get parent's transform
                if let Ok(parent_transform) = parent_transform_query.get(parent.get()) {
                    let start: Vec3 = parent_transform.translation();
                    let end: Vec3 = child_transform.translation();
                    // Only draw if bones are reasonably close (filters out root-to-random-bone)
                    let dist = start.distance(end);
                    if dist < 50.0 {
                        gizmos.line(start, end, Color::srgba(1.0, 0.8, 0.0, 0.7));
                    }
                }
            }
        }
    }
}
