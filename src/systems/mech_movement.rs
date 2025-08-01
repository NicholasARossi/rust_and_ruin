use bevy::prelude::*;
use crate::components::{MoveTarget, TankMovement, TankRotationState};
use crate::mech::{MechMovement, MechMovementState, MechLowerBody, MechHierarchy};

const ROTATION_TOLERANCE: f32 = 1.0; // degrees
const ARRIVAL_THRESHOLD: f32 = 0.5; // units

pub fn mech_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut MechMovement,
        &MechLowerBody,
        Option<&MoveTarget>,
    )>,
) {
    for (entity, mut transform, mut movement, lower_body, move_target) in query.iter_mut() {
        let stats = &lower_body.movement_stats;
        
        match movement.movement_state {
            MechMovementState::Idle => {
                if let Some(target) = move_target {
                    let current_pos = Vec2::new(transform.translation.x, transform.translation.z);
                    let distance = current_pos.distance(target.position);
                    
                    if distance > ARRIVAL_THRESHOLD {
                        let direction = target.position - current_pos;
                        movement.target_rotation = direction.x.atan2(direction.y).to_degrees();
                        movement.movement_state = MechMovementState::Rotating;
                        // Don't reset speed - let it decelerate naturally
                    } else {
                        commands.entity(entity).remove::<MoveTarget>();
                    }
                }
            }
            
            MechMovementState::Rotating => {
                if let Some(target) = move_target {
                    let current_pos = Vec2::new(transform.translation.x, transform.translation.z);
                    let direction = target.position - current_pos;
                    let new_target_rotation = direction.x.atan2(direction.y).to_degrees();
                    
                    let target_diff = shortest_angle_difference(movement.target_rotation, new_target_rotation);
                    if target_diff.abs() > 5.0 {
                        movement.target_rotation = new_target_rotation;
                    }
                }
                
                let current_rotation = transform.rotation.to_euler(EulerRot::YXZ).0.to_degrees();
                let angle_diff = shortest_angle_difference(current_rotation, movement.target_rotation);
                
                if angle_diff.abs() < ROTATION_TOLERANCE {
                    movement.movement_state = MechMovementState::Moving;
                } else {
                    // Decelerate while rotating
                    if movement.current_speed > 0.0 {
                        movement.current_speed = (movement.current_speed - stats.acceleration * time.delta_seconds())
                            .max(0.0);
                            
                        // Continue moving forward while rotating
                        let forward = transform.rotation * Vec3::Z;
                        let move_delta = forward * movement.current_speed * time.delta_seconds();
                        transform.translation.x += move_delta.x;
                        transform.translation.z += move_delta.z;
                    }
                    
                    // Rotate towards target
                    let rotation_step = stats.turn_rate * time.delta_seconds();
                    let rotation_delta = angle_diff.signum() * rotation_step.min(angle_diff.abs());
                    
                    let new_rotation = current_rotation + rotation_delta;
                    transform.rotation = Quat::from_rotation_y(new_rotation.to_radians());
                }
            }
            
            MechMovementState::Moving => {
                if let Some(target) = move_target {
                    let current_pos = Vec2::new(transform.translation.x, transform.translation.z);
                    let distance = current_pos.distance(target.position);
                    
                    if distance > ARRIVAL_THRESHOLD {
                        movement.current_speed = (movement.current_speed + 
                            stats.acceleration * time.delta_seconds())
                            .min(stats.max_speed);
                        
                        let forward = transform.rotation * Vec3::Z;
                        let move_delta = forward * movement.current_speed * time.delta_seconds();
                        transform.translation.x += move_delta.x;
                        transform.translation.z += move_delta.z;
                        
                        let direction = target.position - current_pos;
                        let new_target_rotation = direction.x.atan2(direction.y).to_degrees();
                        let current_rotation = transform.rotation.to_euler(EulerRot::YXZ).0.to_degrees();
                        let angle_diff = shortest_angle_difference(current_rotation, new_target_rotation);
                        
                        if angle_diff.abs() > 30.0 {
                            movement.target_rotation = new_target_rotation;
                            movement.movement_state = MechMovementState::Rotating;
                            movement.current_speed *= 0.5;
                        }
                    } else {
                        movement.current_speed = (movement.current_speed - 
                            stats.acceleration * 2.0 * time.delta_seconds())
                            .max(0.0);
                        
                        if movement.current_speed == 0.0 {
                            movement.movement_state = MechMovementState::Idle;
                            commands.entity(entity).remove::<MoveTarget>();
                        }
                    }
                } else {
                    movement.current_speed = (movement.current_speed - 
                        stats.acceleration * 2.0 * time.delta_seconds())
                        .max(0.0);
                    
                    if movement.current_speed == 0.0 {
                        movement.movement_state = MechMovementState::Idle;
                    }
                }
            }
        }
    }
}

// Adapter system to convert old TankMovement to new MechMovement
pub fn tank_movement_adapter_system(
    mut commands: Commands,
    query: Query<(Entity, &TankMovement), Without<MechMovement>>,
) {
    for (entity, tank_movement) in query.iter() {
        let mech_movement = MechMovement {
            movement_state: match tank_movement.rotation_state {
                TankRotationState::Idle => MechMovementState::Idle,
                TankRotationState::Rotating => MechMovementState::Rotating,
                TankRotationState::Moving => MechMovementState::Moving,
            },
            target_rotation: tank_movement.target_rotation,
            current_speed: tank_movement.current_speed,
        };
        
        let mech_lower = MechLowerBody {
            movement_stats: crate::mech::MovementStats {
                max_speed: tank_movement.max_speed,
                turn_rate: tank_movement.rotation_speed,
                acceleration: tank_movement.acceleration,
            },
        };
        
        commands.entity(entity)
            .insert(mech_movement)
            .insert(mech_lower);
    }
}

fn shortest_angle_difference(from: f32, to: f32) -> f32 {
    // Normalize angles to 0-360 range
    let from_normalized = ((from % 360.0) + 360.0) % 360.0;
    let to_normalized = ((to % 360.0) + 360.0) % 360.0;
    
    let diff = to_normalized - from_normalized;
    if diff > 180.0 {
        diff - 360.0
    } else if diff < -180.0 {
        diff + 360.0
    } else {
        diff
    }
}

// System to sync lower body visual rotation with the main mech entity
pub fn sync_lower_body_rotation_system(
    mech_query: Query<(&Transform, &MechHierarchy), With<MechMovement>>,
    mut lower_query: Query<&mut Transform, Without<MechMovement>>,
) {
    for (mech_transform, hierarchy) in mech_query.iter() {
        if let Some(lower_entity) = hierarchy.lower {
            if let Ok(mut lower_transform) = lower_query.get_mut(lower_entity) {
                // Copy only the rotation from the main mech to the lower body visual
                lower_transform.rotation = mech_transform.rotation;
            }
        }
    }
}