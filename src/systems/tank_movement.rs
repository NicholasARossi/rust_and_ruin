use bevy::prelude::*;
use crate::components::{MoveTarget, TankMovement, TankRotationState};

const ROTATION_TOLERANCE: f32 = 1.0; // degrees
const ARRIVAL_THRESHOLD: f32 = 0.5; // units

pub fn tank_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut TankMovement,
        Option<&MoveTarget>,
    )>,
) {
    for (entity, mut transform, mut tank_movement, move_target) in query.iter_mut() {
        match tank_movement.rotation_state {
            TankRotationState::Idle => {
                // Check if we have a move target
                if let Some(target) = move_target {
                    let current_pos = Vec2::new(transform.translation.x, transform.translation.z);
                    let distance = current_pos.distance(target.position);
                    
                    if distance > ARRIVAL_THRESHOLD {
                        // Calculate target rotation
                        let direction = target.position - current_pos;
                        tank_movement.target_rotation = direction.x.atan2(direction.y).to_degrees();
                        tank_movement.rotation_state = TankRotationState::Rotating;
                    } else {
                        // Too close to target, remove it
                        commands.entity(entity).remove::<MoveTarget>();
                    }
                }
            }
            
            TankRotationState::Rotating => {
                // Check if target still exists and recalculate if needed
                if let Some(target) = move_target {
                    let current_pos = Vec2::new(transform.translation.x, transform.translation.z);
                    let direction = target.position - current_pos;
                    let new_target_rotation = direction.x.atan2(direction.y).to_degrees();
                    
                    // Update target rotation if it changed significantly
                    let target_diff = shortest_angle_difference(tank_movement.target_rotation, new_target_rotation);
                    if target_diff.abs() > 5.0 {
                        tank_movement.target_rotation = new_target_rotation;
                    }
                }
                
                // Get current rotation in degrees
                let current_rotation = transform.rotation.to_euler(EulerRot::YXZ).0.to_degrees();
                
                // Calculate shortest angle difference
                let angle_diff = shortest_angle_difference(current_rotation, tank_movement.target_rotation);
                
                if angle_diff.abs() < ROTATION_TOLERANCE {
                    // Rotation complete, start moving
                    tank_movement.rotation_state = TankRotationState::Moving;
                } else {
                    // Continue rotating
                    let rotation_step = tank_movement.rotation_speed * time.delta_seconds();
                    let rotation_delta = angle_diff.signum() * rotation_step.min(angle_diff.abs());
                    
                    let new_rotation = current_rotation + rotation_delta;
                    transform.rotation = Quat::from_rotation_y(new_rotation.to_radians());
                }
            }
            
            TankRotationState::Moving => {
                if let Some(target) = move_target {
                    let current_pos = Vec2::new(transform.translation.x, transform.translation.z);
                    let distance = current_pos.distance(target.position);
                    
                    if distance > ARRIVAL_THRESHOLD {
                        // Accelerate up to max speed
                        tank_movement.current_speed = (tank_movement.current_speed + 
                            tank_movement.acceleration * time.delta_seconds())
                            .min(tank_movement.max_speed);
                        
                        // Move forward in the direction we're facing
                        let forward = transform.rotation * Vec3::Z;
                        let movement = forward * tank_movement.current_speed * time.delta_seconds();
                        transform.translation.x += movement.x;
                        transform.translation.z += movement.z;
                        
                        // Check if we need to adjust rotation while moving
                        let direction = target.position - current_pos;
                        let new_target_rotation = direction.x.atan2(direction.y).to_degrees();
                        let current_rotation = transform.rotation.to_euler(EulerRot::YXZ).0.to_degrees();
                        let angle_diff = shortest_angle_difference(current_rotation, new_target_rotation);
                        
                        // If target has moved significantly, transition back to rotating
                        if angle_diff.abs() > 30.0 {
                            tank_movement.target_rotation = new_target_rotation;
                            tank_movement.rotation_state = TankRotationState::Rotating;
                            // Start deceleration
                            tank_movement.current_speed *= 0.5;
                        }
                    } else {
                        // Arrived at target, decelerate and stop
                        tank_movement.current_speed = (tank_movement.current_speed - 
                            tank_movement.acceleration * 2.0 * time.delta_seconds())
                            .max(0.0);
                        
                        if tank_movement.current_speed == 0.0 {
                            tank_movement.rotation_state = TankRotationState::Idle;
                            commands.entity(entity).remove::<MoveTarget>();
                        }
                    }
                } else {
                    // No target, decelerate and stop
                    tank_movement.current_speed = (tank_movement.current_speed - 
                        tank_movement.acceleration * 2.0 * time.delta_seconds())
                        .max(0.0);
                    
                    if tank_movement.current_speed == 0.0 {
                        tank_movement.rotation_state = TankRotationState::Idle;
                    }
                }
            }
        }
    }
}

fn shortest_angle_difference(from: f32, to: f32) -> f32 {
    let diff = to - from;
    if diff > 180.0 {
        diff - 360.0
    } else if diff < -180.0 {
        diff + 360.0
    } else {
        diff
    }
}