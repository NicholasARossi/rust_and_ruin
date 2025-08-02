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
) {
    for (attack_target, children) in parent_query.iter() {
        // Propagate to immediate children
        for &child in children.iter() {
            if child_query.get(child).is_ok() {
                // Child doesn't have AttackTarget, add it
                commands.entity(child).insert(AttackTarget {
                    entity: attack_target.entity,
                });
            }
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
) {
    for children in parent_query.iter() {
        for &child in children.iter() {
            if child_query.get(child).is_ok() {
                // Parent doesn't have AttackTarget but child does, remove it
                commands.entity(child).remove::<AttackTarget>();
            }
        }
    }
}