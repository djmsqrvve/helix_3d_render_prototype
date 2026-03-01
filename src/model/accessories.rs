use bevy::prelude::*;
use std::collections::HashMap;

use super::skeleton::collect_all_entities_recursive;
use crate::components::*;
use crate::constants::*;

pub type BaseQueryFilter = (
    With<BaseModel>,
    Without<AccessoryModel>,
    Without<AccessoryAttached>,
);

pub type AccessoryQueryFilter = (Without<BaseModel>, Without<AccessoryAttached>);

/// System that attaches accessories to the base skeleton
pub fn attach_accessories_to_skeleton(
    mut commands: Commands,
    base_query: Query<Entity, BaseQueryFilter>,
    accessory_query: Query<(Entity, &AccessoryModel), AccessoryQueryFilter>,
    children_query: Query<&Children>,
    name_query: Query<(Entity, &Name)>,
    mesh_query: Query<&Mesh3d>,
    mut attachment_state: ResMut<AttachmentState>,
) {
    // Early exit: no accessories waiting to attach - don't waste CPU building bone map
    if accessory_query.is_empty() {
        return;
    }

    let base_entity = match base_query.iter().next() {
        Some(e) => e,
        None => return,
    };

    let mut base_bones: HashMap<String, Entity> = HashMap::new();
    collect_bones_recursive(&children_query, &name_query, base_entity, &mut base_bones);

    if base_bones.is_empty() {
        return;
    }

    let has_key_bones = base_bones.contains_key("root") || base_bones.contains_key("head");
    if !has_key_bones {
        return;
    }

    for (acc_root_entity, accessory) in accessory_query.iter() {
        let acc_children = match children_query.get(acc_root_entity) {
            Ok(children) if !children.is_empty() => children,
            _ => continue,
        };

        let mut all_entities: Vec<Entity> = vec![acc_root_entity];
        collect_all_entities_recursive(&children_query, acc_children, &mut all_entities);

        let mesh_count = all_entities
            .iter()
            .filter(|&&e| mesh_query.get(e).is_ok())
            .count();

        let target_bone = match base_bones.get(&accessory.root_bone_name) {
            Some(entity) => *entity,
            None => {
                warn!(
                    "[ATTACH][FAIL] {}: bone '{}' not found in base skeleton",
                    accessory.name, accessory.root_bone_name
                );
                continue;
            }
        };

        info!(
            "[ATTACH][OK] {}: {} entities, {} meshes -> '{}'",
            accessory.name,
            all_entities.len(),
            mesh_count,
            accessory.root_bone_name
        );

        commands.entity(target_bone).add_child(acc_root_entity);
        commands.entity(acc_root_entity).insert(AccessoryAttached {
            visibility_propagated: false,
        });
    }

    let total = ACCESSORIES.len();
    let attached = accessory_query.iter().count();
    let actually_attached = total - attached;

    if actually_attached == total && total > 0 && !attachment_state.complete_logged {
        info!("[ATTACH][COMPLETE] All {} accessories attached", total);
        attachment_state.complete_logged = true;
    }
}

/// Update accessory visibility based on toggle state
/// Also runs when accessories are newly attached to propagate visibility to newly loaded meshes
pub fn update_accessory_visibility(
    visibility: Res<AccessoryVisibility>,
    accessory_query: Query<(Entity, &AccessoryModel, Option<&AccessoryAttached>)>,
    children_query: Query<&Children>,
    mut visibility_query: Query<&mut Visibility>,
    mut commands: Commands,
) {
    // Run when:
    // 1. Visibility resource changed (user toggled)
    // 2. First frame (initial state)
    // 3. Accessory was just attached and needs post-attachment visibility propagation
    let visibility_changed = visibility.is_changed();
    let is_first_frame = visibility.is_added();
    let needs_post_attach = accessory_query
        .iter()
        .any(|(_, _, attached)| attached.map(|a| !a.visibility_propagated).unwrap_or(false));

    if !visibility_changed && !is_first_frame && !needs_post_attach {
        return;
    }

    for (root_entity, accessory, attached) in accessory_query.iter() {
        let should_be_visible = visibility.is_visible(&accessory.name);
        let new_vis = if should_be_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        if let Ok(mut vis) = visibility_query.get_mut(root_entity) {
            *vis = new_vis;
        }

        let mut entities_to_toggle = vec![root_entity];
        if let Ok(children) = children_query.get(root_entity) {
            collect_all_entities_recursive(&children_query, children, &mut entities_to_toggle);
        }

        for &entity in &entities_to_toggle {
            if let Ok(mut vis) = visibility_query.get_mut(entity) {
                *vis = new_vis;
            }
        }

        // Mark as propagated if this was a newly attached accessory
        let is_newly_attached = attached.map(|a| !a.visibility_propagated).unwrap_or(false);
        if is_newly_attached {
            commands.entity(root_entity).insert(AccessoryAttached {
                visibility_propagated: true,
            });
        }

        // Log on first frame, visibility change, or when newly attached (to see full entity count)
        if is_first_frame || visibility_changed || is_newly_attached {
            info!(
                "[VISIBILITY] {}: {:?} ({} entities){}",
                accessory.name,
                new_vis,
                entities_to_toggle.len(),
                if is_newly_attached {
                    " [POST-ATTACH]"
                } else {
                    ""
                }
            );
        }
    }
}

fn collect_bones_recursive(
    children_query: &Query<&Children>,
    name_query: &Query<(Entity, &Name)>,
    entity: Entity,
    bones: &mut HashMap<String, Entity>,
) {
    if let Ok((_, name)) = name_query.get(entity) {
        let name_str = name.as_str();
        if !name_str.contains("/")
            && !name_str.contains(".vmdl")
            && !name_str.contains(".gltf")
            && !name_str.starts_with("models/")
            && name_str.len() < 50
            && !name_str.is_empty()
        {
            bones.insert(name_str.to_string(), entity);
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            collect_bones_recursive(children_query, name_query, child, bones);
        }
    }
}
