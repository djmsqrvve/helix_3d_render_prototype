use bevy::prelude::*;

use crate::components::*;

/// Toggle debug visualization modes via F-keys
pub fn toggle_debug_modes(keys: Res<ButtonInput<KeyCode>>, mut debug_state: ResMut<DebugState>) {
    if keys.just_pressed(KeyCode::F1) {
        debug_state.show_wireframe = !debug_state.show_wireframe;
        info!(
            "[DEBUG] Wireframe: {}",
            if debug_state.show_wireframe {
                "ON"
            } else {
                "OFF"
            }
        );
    }
    if keys.just_pressed(KeyCode::F2) {
        debug_state.show_skeleton = !debug_state.show_skeleton;
        info!(
            "[DEBUG] Skeleton: {}",
            if debug_state.show_skeleton {
                "ON"
            } else {
                "OFF"
            }
        );
    }
    if keys.just_pressed(KeyCode::F3) {
        debug_state.show_stats = !debug_state.show_stats;
    }
    if keys.just_pressed(KeyCode::F4) {
        debug_state.show_transforms = !debug_state.show_transforms;
    }
    if keys.just_pressed(KeyCode::F5) {
        debug_state.show_grid = !debug_state.show_grid;
    }
}

/// Update the debug stats overlay text
pub fn update_debug_ui(
    debug_state: Res<DebugState>,
    mut texts: Query<&mut Text, With<StatsText>>,
    transforms: Query<&Transform>,
    meshes: Res<Assets<Mesh>>,
    mesh_query: Query<&Mesh3d>,
    time: Res<Time>,
) {
    if !debug_state.show_stats {
        for mut text in texts.iter_mut() {
            text.0 = String::from("Debug Stats:\n  (Press F3)");
        }
        return;
    }

    let mut triangle_count = 0usize;
    let mut mesh_count = 0usize;

    for mesh_handle in mesh_query.iter() {
        if let Some(mesh) = meshes.get(&mesh_handle.0) {
            if let Some(indices) = mesh.indices() {
                triangle_count += indices.len() / 3;
            }
            mesh_count += 1;
        }
    }

    let entity_count = transforms.iter().count();
    let fps = 1.0 / time.delta_secs();

    let mut text_str = String::from("═══ Debug Stats ═══\n");
    text_str.push_str(&format!("FPS: {:.0}\n", fps));
    text_str.push_str(&format!("Triangles: {}\n", triangle_count));
    text_str.push_str(&format!("Meshes: {}\n", mesh_count));
    text_str.push_str(&format!("Entities: {}\n", entity_count));
    text_str.push_str(&format!(
        "Wireframe: {}",
        if debug_state.show_wireframe {
            "ON"
        } else {
            "OFF"
        }
    ));

    for mut text in texts.iter_mut() {
        text.0 = text_str.clone();
    }
}
