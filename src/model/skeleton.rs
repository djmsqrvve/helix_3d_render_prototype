use bevy::prelude::*;

/// Recursively collect all child entities into a flat list
pub fn collect_all_entities_recursive(
    children_query: &Query<&Children>,
    children: &Children,
    entities: &mut Vec<Entity>,
) {
    for child in children.iter() {
        entities.push(child);
        if let Ok(grandchildren) = children_query.get(child) {
            collect_all_entities_recursive(children_query, grandchildren, entities);
        }
    }
}
