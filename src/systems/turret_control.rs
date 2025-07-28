use bevy::prelude::*;
use crate::mech::*;
use crate::resources::*;
use crate::components::{AttackTarget, Enemy};

pub fn calculate_turret_angle(mech_position: Vec2, target_position: Vec2) -> f32 {
    let direction = target_position - mech_position;
    direction.x.atan2(direction.y).to_degrees()
}

pub fn normalize_angle(angle: f32) -> f32 {
    let mut normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized += 360.0;
    }
    normalized
}

pub fn shortest_angle_difference(from: f32, to: f32) -> f32 {
    let diff = to - from;
    if diff > 180.0 {
        diff - 360.0
    } else if diff < -180.0 {
        diff + 360.0
    } else {
        diff
    }
}

pub fn rotate_towards_angle(
    current_angle: f32,
    target_angle: f32,
    rotation_speed: f32,
    delta_time: f32,
) -> f32 {
    let max_rotation = rotation_speed * delta_time;
    let angle_diff = shortest_angle_difference(current_angle, target_angle);
    
    if angle_diff.abs() <= max_rotation {
        target_angle
    } else {
        current_angle + max_rotation * angle_diff.signum()
    }
}

pub fn turret_control_system(
    mut set: ParamSet<(
        Query<(&mut Transform, &mut TurretRotation, &TurretCannon, &Parent)>,
        Query<(&Transform, Option<&AttackTarget>), Without<TurretRotation>>,
        Query<&Transform, With<Enemy>>,
    )>,
    mouse_position: Res<MouseWorldPosition>,
    time: Res<Time>,
) {
    let mouse_pos = mouse_position.position;
    
    // Collect data we need from queries
    let mut turret_data = Vec::new();
    for (entity_index, (_transform, turret_rotation, turret_cannon, parent)) in set.p0().iter().enumerate() {
        turret_data.push((
            entity_index,
            parent.get(),
            turret_rotation.current_angle,
            turret_rotation.target_angle,
            turret_cannon.rotation_speed,
        ));
    }
    
    // Process each turret and collect parent data
    let mut parent_data = Vec::new();
    for (entity_index, parent_entity, current_angle, _, rotation_speed) in &turret_data {
        // Get parent transform and attack target
        if let Ok((transform, attack_target)) = set.p1().get(*parent_entity) {
            let mech_position = Vec2::new(transform.translation.x, transform.translation.z);
            let attack_entity = attack_target.map(|at| at.entity);
            parent_data.push((*entity_index, mech_position, attack_entity, *current_angle, *rotation_speed));
        }
    }
    
    // Get enemy positions
    let mut enemy_positions = Vec::new();
    for (entity_index, _mech_position, attack_entity, _current_angle, _rotation_speed) in &parent_data {
        if let Some(enemy_entity) = attack_entity {
            if let Ok(enemy_transform) = set.p2().get(*enemy_entity) {
                enemy_positions.push((*entity_index, Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z)));
            }
        }
    }
    
    // Calculate updates
    let mut updates = Vec::new();
    for (entity_index, mech_position, attack_entity, current_angle, rotation_speed) in parent_data {
        let (target_position, has_valid_target) = if let Some(_) = attack_entity {
            // Look for enemy position
            if let Some((_, enemy_pos)) = enemy_positions.iter()
                .find(|(idx, _)| *idx == entity_index) {
                (*enemy_pos, true)
            } else {
                // Enemy not found but we have an attack target
                // Maintain current angle by using a position that results in the current angle
                let current_rad = current_angle.to_radians();
                let maintain_pos = mech_position + Vec2::new(current_rad.cos(), current_rad.sin()) * 10.0;
                (maintain_pos, false)
            }
        } else {
            (mouse_pos, true)
        };
        
        let target_angle = calculate_turret_angle(mech_position, target_position);
        let target_angle_normalized = normalize_angle(target_angle);
        
        let current_angle_normalized = normalize_angle(current_angle);
        
        // Only rotate if we have a valid target or are following mouse
        let new_angle = if has_valid_target {
            rotate_towards_angle(
                current_angle_normalized,
                target_angle_normalized,
                rotation_speed,
                time.delta_seconds(),
            )
        } else {
            // Maintain current angle when enemy is lost but attack target exists
            current_angle_normalized
        };
        
        updates.push((entity_index, new_angle, target_angle_normalized));
    }
    
    // Apply updates
    for (entity_index, new_angle, target_angle) in updates {
        if let Some((mut transform, mut turret_rotation, _, _)) = set.p0().iter_mut().nth(entity_index) {
            turret_rotation.current_angle = new_angle;
            turret_rotation.target_angle = target_angle;
            transform.rotation = Quat::from_rotation_y(new_angle.to_radians());
        }
    }
}

pub fn get_turret_forward_direction(turret_transform: &Transform) -> Vec2 {
    let forward_3d = turret_transform.rotation * Vec3::Z;
    Vec2::new(forward_3d.x, forward_3d.z)
}