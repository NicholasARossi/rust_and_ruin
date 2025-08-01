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
        Query<(&mut Transform, &GlobalTransform, &mut TurretRotation, &TurretCannon, &Parent)>,
        Query<(&Transform, Option<&AttackTarget>), Without<TurretRotation>>,
        Query<&Transform, With<Enemy>>,
    )>,
    mouse_position: Res<MouseWorldPosition>,
    time: Res<Time>,
) {
    let _mouse_pos = mouse_position.position;
    
    // Collect data we need from queries
    let mut turret_data = Vec::new();
    for (entity_index, (_transform, global_transform, turret_rotation, turret_cannon, parent)) in set.p0().iter().enumerate() {
        turret_data.push((
            entity_index,
            parent.get(),
            turret_rotation.current_angle,
            turret_rotation.target_angle,
            turret_cannon.rotation_speed,
            Vec2::new(global_transform.translation().x, global_transform.translation().z),
        ));
    }
    
    if turret_data.is_empty() {
        // No turrets found - might be a timing issue
        return;
    }
    
    // Process each turret and collect parent data
    let mut parent_data = Vec::new();
    for (entity_index, parent_entity, current_angle, _, rotation_speed, turret_position) in &turret_data {
        // Get parent transform and attack target
        if let Ok((transform, attack_target)) = set.p1().get(*parent_entity) {
            let attack_entity = attack_target.map(|at| at.entity);
            // Extract the parent's Y rotation (first component in YXZ order)
            let (parent_y_rotation, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
            parent_data.push((*entity_index, *turret_position, attack_entity, *current_angle, *rotation_speed, parent_y_rotation));
        }
    }
    
    // Get enemy positions
    let mut enemy_positions = Vec::new();
    for (entity_index, _turret_position, attack_entity, _current_angle, _rotation_speed, _parent_rotation) in &parent_data {
        if let Some(enemy_entity) = attack_entity {
            if let Ok(enemy_transform) = set.p2().get(*enemy_entity) {
                enemy_positions.push((*entity_index, Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z)));
            }
        }
    }
    
    // Calculate updates
    let mut updates = Vec::new();
    for (entity_index, turret_position, attack_entity, current_angle, rotation_speed, parent_rotation) in parent_data {
        let (target_position, has_valid_target) = if let Some(_) = attack_entity {
            // Look for enemy position
            if let Some((_, enemy_pos)) = enemy_positions.iter()
                .find(|(idx, _)| *idx == entity_index) {
                (*enemy_pos, true)
            } else {
                // Enemy not found but we have an attack target
                // Maintain current angle by using a position that results in the current angle
                let current_rad = current_angle.to_radians();
                let maintain_pos = turret_position + Vec2::new(current_rad.cos(), current_rad.sin()) * 10.0;
                (maintain_pos, false)
            }
        } else {
            // No attack target - maintain current angle
            let current_rad = current_angle.to_radians();
            let maintain_pos = turret_position + Vec2::new(current_rad.cos(), current_rad.sin()) * 10.0;
            (maintain_pos, false)
        };
        
        // Calculate world-space angle to target
        let world_target_angle = calculate_turret_angle(turret_position, target_position);
        let world_target_angle_normalized = normalize_angle(world_target_angle);
        
        
        // Convert parent rotation from radians to degrees
        let parent_rotation_degrees = normalize_angle(parent_rotation.to_degrees());
        
        // Calculate the local turret angle by subtracting parent's rotation
        // This converts world-space angle to local-space angle
        let local_target_angle = normalize_angle(world_target_angle_normalized - parent_rotation_degrees);
        
        // Debug logging (commented out to reduce noise)
        // info!("Turret control: world_angle={:.1}, parent_rot={:.1}, local_angle={:.1}", 
        //       world_target_angle_normalized, parent_rotation_degrees, local_target_angle);
        
        // Current angle is already in local space
        let current_angle_normalized = normalize_angle(current_angle);
        
        // Only rotate if we have a valid target or are following mouse
        let new_angle = if has_valid_target {
            rotate_towards_angle(
                current_angle_normalized,
                local_target_angle,
                rotation_speed,
                time.delta_seconds(),
            )
        } else {
            // Maintain current angle when enemy is lost but attack target exists
            current_angle_normalized
        };
        
        // Store the new local angle and world target angle
        // The world target angle is what we're aiming at in world space
        updates.push((entity_index, new_angle, world_target_angle_normalized));
    }
    
    // Apply updates
    for (entity_index, new_angle, target_angle) in updates {
        if let Some((mut transform, _, mut turret_rotation, _, _)) = set.p0().iter_mut().nth(entity_index) {
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

pub fn is_turret_facing_target(
    turret_transform: &Transform,
    turret_position: Vec2,
    target_position: Vec2,
    angle_tolerance_degrees: f32,
) -> bool {
    // Get the forward direction of the turret
    let turret_forward = get_turret_forward_direction(turret_transform);
    
    // Calculate the direction from turret to target
    let to_target = (target_position - turret_position).normalize();
    
    // Calculate the dot product
    let dot_product = turret_forward.dot(to_target);
    
    // Convert angle tolerance from degrees to dot product threshold
    // cos(angle) gives us the dot product threshold
    let angle_tolerance_radians = angle_tolerance_degrees.to_radians();
    let dot_threshold = angle_tolerance_radians.cos();
    
    // Debug logging (commented out to reduce noise)
    // let angle_between = dot_product.acos().to_degrees();
    // info!("is_turret_facing_target: forward={:?}, to_target={:?}, dot={:.3}, angle_between={:.1}°, tolerance={:.1}°, threshold={:.3}", 
    //       turret_forward, to_target, dot_product, angle_between, angle_tolerance_degrees, dot_threshold);
    
    // Check if turret is facing within tolerance
    // Use a small epsilon to handle floating point precision issues
    dot_product >= dot_threshold - 0.0001
}