use bevy::prelude::*;
use rust_and_ruin::components::{TankMovement, TankRotationState, MoveTarget, Hero};
use rust_and_ruin::systems::tank_movement::tank_movement_system;

#[cfg(test)]
mod tank_movement_unit_tests {
    use super::*;
    
    fn calculate_target_rotation(from: Vec2, to: Vec2) -> f32 {
        let direction = to - from;
        direction.x.atan2(direction.y).to_degrees()
    }
    
    fn normalize_angle(angle: f32) -> f32 {
        let mut normalized = angle % 360.0;
        if normalized < 0.0 {
            normalized += 360.0;
        }
        normalized
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
    
    #[test]
    fn test_rotation_calculation_cardinal_directions() {
        // Test North (0°)
        let rotation = calculate_target_rotation(Vec2::ZERO, Vec2::new(0.0, 1.0));
        assert!((normalize_angle(rotation) - 0.0).abs() < 0.1);
        
        // Test East (90°)
        let rotation = calculate_target_rotation(Vec2::ZERO, Vec2::new(1.0, 0.0));
        assert!((normalize_angle(rotation) - 90.0).abs() < 0.1);
        
        // Test South (180°)
        let rotation = calculate_target_rotation(Vec2::ZERO, Vec2::new(0.0, -1.0));
        assert!((normalize_angle(rotation) - 180.0).abs() < 0.1);
        
        // Test West (270°)
        let rotation = calculate_target_rotation(Vec2::ZERO, Vec2::new(-1.0, 0.0));
        assert!((normalize_angle(rotation) - 270.0).abs() < 0.1);
    }
    
    #[test]
    fn test_rotation_completion_detection() {
        let tolerance = 1.0; // 1 degree tolerance
        
        // Test exact match
        assert!(shortest_angle_difference(90.0, 90.0).abs() < tolerance);
        
        // Test within tolerance
        assert!(shortest_angle_difference(90.0, 90.5).abs() < tolerance);
        assert!(shortest_angle_difference(90.0, 89.5).abs() < tolerance);
        
        // Test outside tolerance
        assert!(shortest_angle_difference(90.0, 92.0).abs() > tolerance);
        assert!(shortest_angle_difference(90.0, 88.0).abs() > tolerance);
        
        // Test wrapping around 0/360
        assert!(shortest_angle_difference(359.0, 1.0).abs() < 3.0);
        assert!(shortest_angle_difference(1.0, 359.0).abs() < 3.0);
    }
    
    #[test]
    fn test_acceleration_to_max_speed() {
        let mut tank = TankMovement::default();
        let delta_time = 0.1; // 100ms
        
        // Start from 0
        assert_eq!(tank.current_speed, 0.0);
        
        // Accelerate for 2 seconds (20 frames) to ensure we reach max speed
        // With acceleration of 3.0 and max_speed of 5.0, we need ~1.67 seconds
        for _ in 0..20 {
            tank.current_speed = (tank.current_speed + tank.acceleration * delta_time)
                .min(tank.max_speed);
        }
        
        // Should be at max speed
        assert_eq!(tank.current_speed, tank.max_speed);
        
        // Further acceleration should not exceed max speed
        tank.current_speed = (tank.current_speed + tank.acceleration * delta_time)
            .min(tank.max_speed);
        assert_eq!(tank.current_speed, tank.max_speed);
    }
    
    #[test]
    fn test_deceleration_to_stop() {
        let mut tank = TankMovement::default();
        tank.current_speed = tank.max_speed;
        let delta_time = 0.1;
        let deceleration_rate = tank.acceleration * 2.0; // Faster deceleration
        
        // Decelerate over time
        while tank.current_speed > 0.0 {
            tank.current_speed = (tank.current_speed - deceleration_rate * delta_time).max(0.0);
        }
        
        assert_eq!(tank.current_speed, 0.0);
    }
    
    #[test]
    fn test_state_transitions() {
        let mut tank = TankMovement::default();
        
        // Initial state should be Idle
        assert_eq!(tank.rotation_state, TankRotationState::Idle);
        
        // Transition to Rotating when target is set
        tank.target_rotation = 90.0;
        tank.rotation_state = TankRotationState::Rotating;
        assert_eq!(tank.rotation_state, TankRotationState::Rotating);
        
        // Transition to Moving when rotation is complete
        tank.rotation_state = TankRotationState::Moving;
        assert_eq!(tank.rotation_state, TankRotationState::Moving);
        
        // Transition back to Idle when movement is complete
        tank.rotation_state = TankRotationState::Idle;
        tank.current_speed = 0.0;
        assert_eq!(tank.rotation_state, TankRotationState::Idle);
    }
    
    #[test]
    fn test_rotation_speed_timing() {
        let tank = TankMovement::default();
        let angle_to_rotate = 180.0; // Half rotation
        
        // Calculate time needed
        let time_needed = angle_to_rotate / tank.rotation_speed;
        assert!((time_needed - 2.0).abs() < 0.01); // Should take 2 seconds at 90°/s
        
        // Test smaller rotation
        let small_angle = 45.0;
        let time_needed = small_angle / tank.rotation_speed;
        assert!((time_needed - 0.5).abs() < 0.01); // Should take 0.5 seconds
    }
}

#[cfg(test)]
mod tank_movement_integration_tests {
    use super::*;
    use bevy::app::{App, Update};
    use bevy::time::{Time, TimePlugin};
    
    fn setup_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, tank_movement_system);
        app
    }
    
    fn advance_time(app: &mut App, seconds: f32) {
        // Manually set delta time for the test
        let duration = std::time::Duration::from_secs_f32(seconds);
        app.world.resource_mut::<Time>().advance_by(duration);
        app.update();
    }
    
    #[test]
    fn test_tank_rotates_before_moving() {
        let mut app = setup_test_app();
        
        // Spawn tank facing north (0°) at origin
        let tank_entity = app.world.spawn((
            Hero,
            TankMovement::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
            MoveTarget { position: Vec2::new(5.0, 0.0) }, // Target to the east
        )).id();
        
        // Initial state should be idle
        let tank_movement = app.world.get::<TankMovement>(tank_entity).unwrap();
        assert_eq!(tank_movement.rotation_state, TankRotationState::Idle);
        
        // After first update, should be rotating
        app.update();
        let tank_movement = app.world.get::<TankMovement>(tank_entity).unwrap();
        assert_eq!(tank_movement.rotation_state, TankRotationState::Rotating);
        assert_eq!(tank_movement.current_speed, 0.0); // Not moving yet
        
        // Advance time in smaller steps to simulate realistic updates
        for _ in 0..10 {
            advance_time(&mut app, 0.1); // 100ms steps
        }
        
        // After ~1 second, rotation should be complete and tank should be moving
        let tank_movement = app.world.get::<TankMovement>(tank_entity).unwrap();
        assert_eq!(tank_movement.rotation_state, TankRotationState::Moving);
        assert!(tank_movement.current_speed > 0.0);
        
        // Check that tank has rotated to face east
        let transform = app.world.get::<Transform>(tank_entity).unwrap();
        let rotation_y = transform.rotation.to_euler(EulerRot::YXZ).0.to_degrees();
        assert!((rotation_y - 90.0).abs() < 2.0); // Should be facing east (90°)
    }
    
    #[test]
    fn test_tank_acceleration_and_deceleration() {
        let mut app = setup_test_app();
        
        // Spawn tank already facing target direction
        let tank_entity = app.world.spawn((
            Hero,
            TankMovement {
                rotation_state: TankRotationState::Moving,
                target_rotation: 0.0,
                current_speed: 0.0,
                acceleration: 3.0,
                max_speed: 5.0,
                rotation_speed: 90.0,
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            MoveTarget { position: Vec2::new(0.0, 10.0) },
        )).id();
        
        // Track speed over time
        let mut speeds = Vec::new();
        for _ in 0..30 { // 0.5 seconds
            app.update();
            let tank_movement = app.world.get::<TankMovement>(tank_entity).unwrap();
            speeds.push(tank_movement.current_speed);
            advance_time(&mut app, 0.016);
        }
        
        // Verify acceleration
        assert!(speeds[0] < speeds[10]); // Speed increases
        assert!(speeds[20] <= 5.0); // Doesn't exceed max speed
        
        // Remove target to test deceleration
        app.world.entity_mut(tank_entity).remove::<MoveTarget>();
        
        let mut decel_speeds = Vec::new();
        for _ in 0..30 {
            app.update();
            let tank_movement = app.world.get::<TankMovement>(tank_entity).unwrap();
            decel_speeds.push(tank_movement.current_speed);
            advance_time(&mut app, 0.016);
        }
        
        // Verify deceleration
        assert!(decel_speeds[0] > decel_speeds[10]); // Speed decreases
        assert_eq!(decel_speeds[29], 0.0); // Comes to a stop
    }
    
    #[test]
    fn test_tank_handles_close_targets() {
        let mut app = setup_test_app();
        
        // Spawn tank very close to target
        let tank_entity = app.world.spawn((
            Hero,
            TankMovement::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
            MoveTarget { position: Vec2::new(0.1, 0.0) }, // Very close target
        )).id();
        
        app.update();
        
        // Should remove target and stay idle
        assert!(app.world.get::<MoveTarget>(tank_entity).is_none());
        let tank_movement = app.world.get::<TankMovement>(tank_entity).unwrap();
        assert_eq!(tank_movement.rotation_state, TankRotationState::Idle);
    }
    
    #[test]
    fn test_tank_handles_target_change_during_rotation() {
        let mut app = setup_test_app();
        
        // Spawn tank facing north
        let tank_entity = app.world.spawn((
            Hero,
            TankMovement::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
            MoveTarget { position: Vec2::new(5.0, 0.0) }, // East
        )).id();
        
        // Start rotation
        app.update();
        advance_time(&mut app, 0.5); // Halfway through rotation
        
        // Change target to opposite direction
        app.world.entity_mut(tank_entity).insert(
            MoveTarget { position: Vec2::new(-5.0, 0.0) } // West
        );
        
        // Tank should re-enter rotation state with new target
        app.update();
        let tank_movement = app.world.get::<TankMovement>(tank_entity).unwrap();
        assert_eq!(tank_movement.rotation_state, TankRotationState::Rotating);
        assert!((tank_movement.target_rotation - 270.0).abs() < 1.0 || 
                (tank_movement.target_rotation + 90.0).abs() < 1.0); // West is 270° or -90°
    }
}