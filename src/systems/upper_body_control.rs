use bevy::prelude::*;
use crate::mech::{MechUpperBody, MechRotation, TurretRotation};
use crate::components::{AttackTarget, Enemy};
use crate::resources::MouseWorldPosition;

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

pub fn upper_body_control_system(
    mut set: ParamSet<(
        Query<(&mut Transform, &GlobalTransform, &mut MechRotation, &MechUpperBody, &Parent)>,
        Query<(&Transform, Option<&AttackTarget>), Without<MechRotation>>,
        Query<&Transform, With<Enemy>>,
    )>,
    mouse_position: Res<MouseWorldPosition>,
    time: Res<Time>,
) {
    let _mouse_pos = mouse_position.position;
    
    let mut upper_data = Vec::new();
    for (entity_index, (_transform, global_transform, rotation, upper_body, parent)) in set.p0().iter().enumerate() {
        upper_data.push((
            entity_index,
            parent.get(),
            rotation.current_angle,
            rotation.target_angle,
            upper_body.rotation_capability.rotation_speed,
            Vec2::new(global_transform.translation().x, global_transform.translation().z),
        ));
    }
    
    if upper_data.is_empty() {
        return;
    }
    
    let mut parent_data = Vec::new();
    for (entity_index, parent_entity, current_angle, _, rotation_speed, upper_position) in &upper_data {
        if let Ok((transform, attack_target)) = set.p1().get(*parent_entity) {
            let attack_entity = attack_target.map(|at| at.entity);
            let (parent_y_rotation, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
            parent_data.push((*entity_index, *upper_position, attack_entity, *current_angle, *rotation_speed, parent_y_rotation));
        }
    }
    
    let mut enemy_positions = Vec::new();
    for (entity_index, _upper_position, attack_entity, _current_angle, _rotation_speed, _parent_rotation) in &parent_data {
        if let Some(enemy_entity) = attack_entity {
            if let Ok(enemy_transform) = set.p2().get(*enemy_entity) {
                enemy_positions.push((*entity_index, Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z)));
            }
        }
    }
    
    let mut updates = Vec::new();
    for (entity_index, upper_position, attack_entity, current_angle, rotation_speed, parent_rotation) in parent_data {
        let (target_position, has_valid_target) = if let Some(_) = attack_entity {
            if let Some((_, enemy_pos)) = enemy_positions.iter()
                .find(|(idx, _)| *idx == entity_index) {
                (*enemy_pos, true)
            } else {
                let current_rad = current_angle.to_radians();
                let maintain_pos = upper_position + Vec2::new(current_rad.cos(), current_rad.sin()) * 10.0;
                (maintain_pos, false)
            }
        } else {
            let current_rad = current_angle.to_radians();
            let maintain_pos = upper_position + Vec2::new(current_rad.cos(), current_rad.sin()) * 10.0;
            (maintain_pos, false)
        };
        
        let world_target_angle = calculate_turret_angle(upper_position, target_position);
        let world_target_angle_normalized = normalize_angle(world_target_angle);
        
        let parent_rotation_degrees = normalize_angle(parent_rotation.to_degrees());
        let local_target_angle = normalize_angle(world_target_angle_normalized - parent_rotation_degrees);
        
        let current_angle_normalized = normalize_angle(current_angle);
        
        let new_angle = if has_valid_target {
            rotate_towards_angle(
                current_angle_normalized,
                local_target_angle,
                rotation_speed,
                time.delta_seconds(),
            )
        } else {
            current_angle_normalized
        };
        
        updates.push((entity_index, new_angle, world_target_angle_normalized));
    }
    
    for (entity_index, new_angle, target_angle) in updates {
        if let Some((mut transform, _, mut rotation, _, _)) = set.p0().iter_mut().nth(entity_index) {
            rotation.current_angle = new_angle;
            rotation.target_angle = target_angle;
            transform.rotation = Quat::from_rotation_y(new_angle.to_radians());
        }
    }
}

// Adapter system to convert old TurretRotation to new MechRotation
pub fn turret_rotation_adapter_system(
    mut commands: Commands,
    query: Query<(Entity, &TurretRotation), Without<MechRotation>>,
) {
    for (entity, turret_rotation) in query.iter() {
        commands.entity(entity).insert(MechRotation {
            target_angle: turret_rotation.target_angle,
            current_angle: turret_rotation.current_angle,
        });
    }
}

pub fn get_upper_forward_direction(upper_transform: &Transform) -> Vec2 {
    let forward_3d = upper_transform.rotation * Vec3::Z;
    Vec2::new(forward_3d.x, forward_3d.z)
}

pub fn is_upper_facing_target(
    upper_transform: &Transform,
    upper_position: Vec2,
    target_position: Vec2,
    angle_tolerance_degrees: f32,
) -> bool {
    let upper_forward = get_upper_forward_direction(upper_transform);
    let to_target = (target_position - upper_position).normalize();
    
    let dot_product = upper_forward.dot(to_target);
    let angle_tolerance_radians = angle_tolerance_degrees.to_radians();
    let dot_threshold = angle_tolerance_radians.cos();
    
    dot_product >= dot_threshold - 0.0001
}