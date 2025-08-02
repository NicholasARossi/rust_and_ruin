use bevy::prelude::*;
use crate::components::AttackTarget;

/// Propagates AttackTarget component from parent entities to their children.
/// This allows turrets in nested hierarchies to access AttackTarget from ancestor entities.
pub fn propagate_attack_target_system(
    mut commands: Commands,
    // Query for entities with AttackTarget and their children
    parent_query: Query<(&AttackTarget, &Children)>,
    // Query to check if child already has AttackTarget
    child_query: Query<Entity, Without<AttackTarget>>,
    // Query to get children of any entity
    children_query: Query<&Children>,
) {
    for (attack_target, children) in parent_query.iter() {
        // Recursively propagate to all descendants
        propagate_to_children(&mut commands, &child_query, &children_query, attack_target, children);
    }
}

fn propagate_to_children(
    commands: &mut Commands,
    child_query: &Query<Entity, Without<AttackTarget>>,
    children_query: &Query<&Children>,
    attack_target: &AttackTarget,
    children: &Children,
) {
    for &child in children.iter() {
        if child_query.get(child).is_ok() {
            // Child doesn't have AttackTarget, add it
            commands.entity(child).insert(AttackTarget {
                entity: attack_target.entity,
            });
        }
        
        // Recursively propagate to grandchildren
        if let Ok(grandchildren) = children_query.get(child) {
            propagate_to_children(commands, child_query, children_query, attack_target, grandchildren);
        }
    }
}

/// Removes AttackTarget from children when parent no longer has it.
/// This keeps the hierarchy consistent when targets are cleared.
pub fn cleanup_attack_target_system(
    mut commands: Commands,
    // Query for entities without AttackTarget but with children
    parent_query: Query<&Children, Without<AttackTarget>>,
    // Query for children with AttackTarget
    child_query: Query<(Entity, &AttackTarget)>,
    // Query to get children of any entity
    children_query: Query<&Children>,
) {
    for children in parent_query.iter() {
        cleanup_children(&mut commands, &child_query, &children_query, children);
    }
}

fn cleanup_children(
    commands: &mut Commands,
    child_query: &Query<(Entity, &AttackTarget)>,
    children_query: &Query<&Children>,
    children: &Children,
) {
    for &child in children.iter() {
        if let Ok((entity, _)) = child_query.get(child) {
            // Parent doesn't have AttackTarget but child does, remove it
            commands.entity(entity).remove::<AttackTarget>();
        }
        
        // Recursively cleanup grandchildren
        if let Ok(grandchildren) = children_query.get(child) {
            cleanup_children(commands, child_query, children_query, grandchildren);
        }
    }
}