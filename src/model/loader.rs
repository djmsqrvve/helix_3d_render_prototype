use bevy::prelude::*;

use crate::cli::CliArgs;
use crate::components::*;
use crate::constants::*;

/// Setup system: spawns base model, accessories, camera, light, grid, UI
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut visibility: ResMut<AccessoryVisibility>,
    args: Res<CliArgs>,
) {
    super::log_system_header("Helix 3D Renderer - Starting Up");
    info!("Configuration: {:?}", args);

    // Parse disabled accessories from CLI
    let disabled: Vec<String> = args
        .disable_accessories
        .as_ref()
        .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    // Initialize all accessories as visible (unless disabled via CLI)
    for (name, _) in ACCESSORIES.iter() {
        let is_visible = !disabled.contains(&name.to_string());
        visibility.states.insert(name.to_string(), is_visible);
        info!(
            "[ACCESSORY] {}: {}",
            name,
            if is_visible { "ENABLED" } else { "DISABLED" }
        );
    }
    visibility.ui_spawned = false;

    let model_path = "test_models/dota_models/models/heroes/drow/";

    // Load base model
    let base_scene = asset_server.load(format!("{}drow_base.gltf#Scene0", model_path));
    commands.spawn((SceneRoot(base_scene), BaseModel));
    info!("[LOAD] Base model loaded");

    // Load accessories
    for (acc_name, root_bone) in ACCESSORIES.iter() {
        let scene = asset_server.load(format!("{}{}.gltf#Scene0", model_path, acc_name));
        commands.spawn((
            SceneRoot(scene),
            AccessoryModel {
                name: acc_name.to_string(),
                root_bone_name: root_bone.to_string(),
            },
            Visibility::default(),
        ));
        info!("[LOAD] Accessory: {} -> {}", acc_name, root_bone);
    }

    // Create animation graph
    let mut animation_graph = AnimationGraph::new();
    let mut node_indices = Vec::new();

    for (_, anim_index) in ANIMATIONS.iter() {
        let clip_handle: Handle<AnimationClip> = asset_server.load(
            GltfAssetLabel::Animation(*anim_index)
                .from_asset(format!("{}drow_base.gltf", model_path)),
        );
        let node_index = animation_graph.add_clip(clip_handle, 1.0, animation_graph.root);
        node_indices.push(node_index);
    }

    let graph_handle = animation_graphs.add(animation_graph);
    commands.insert_resource(AnimationData {
        graph_handle,
        node_indices,
    });

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    spawn_debug_grid(&mut commands, &mut meshes, &mut materials);
    crate::ui::spawn_debug_ui(&mut commands);

    super::log_system_header("Setup Complete");
}

fn spawn_debug_grid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let grid_size = 5;
    let grid_spacing = 5.0;
    let grid_extent = grid_size as f32 * grid_spacing;

    for i in -grid_size..=grid_size {
        let pos = i as f32 * grid_spacing;

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, grid_extent * 2.0))),
            MeshMaterial3d(materials.add(Color::srgba(0.5, 0.0, 0.0, 0.5))),
            Transform::from_xyz(pos, 0.0, 0.0),
            DebugMarker,
        ));

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(grid_extent * 2.0, 0.1, 0.1))),
            MeshMaterial3d(materials.add(Color::srgba(0.0, 0.0, 0.5, 0.5))),
            Transform::from_xyz(0.0, 0.0, pos),
            DebugMarker,
        ));
    }

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.1, 20.0, 0.1))),
        MeshMaterial3d(materials.add(Color::srgba(0.0, 0.5, 0.0, 0.5))),
        Transform::from_xyz(0.0, 10.0, 0.0),
        DebugMarker,
    ));
}

pub fn log_system_header(title: &str) {
    let line: String = "=".repeat(60);
    info!("{}", line);
    info!("  {}", title);
    info!("{}", line);
}

/// Simple system to rotate the hero based on the ROTATION_SPEED constant
pub fn rotate_hero(mut query: Query<&mut Transform, With<BaseModel>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(ROTATION_SPEED * time.delta_secs());
    }
}
