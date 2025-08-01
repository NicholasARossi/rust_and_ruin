use bevy::prelude::*;
use rust_and_ruin::mech::*;
use rust_and_ruin::components::*;
use rust_and_ruin::systems::turret_control::*;
use rust_and_ruin::systems::movement::*;
use rust_and_ruin::resources::*;

/// Test that turret rotates towards target when initially facing away
#[test]
fn test_turret_rotates_towards_target_when_facing_away() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Create mech at origin facing forward (+Z)
    let mech_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
    )).id();
    
    // Create turret as child, initially facing backward (-Z, 180 degrees)
    let turret_entity = app.world.spawn((
        Transform::from_rotation(Quat::from_rotation_y(180.0_f32.to_radians())),
        GlobalTransform::default(),
        TurretRotation {
            current_angle: 180.0,
            target_angle: 180.0,
        },
        TurretCannon {
            fire_rate: 1.0,
            projectile_damage: 10.0,
            rotation_speed: 90.0, // 90 degrees per second
            barrel_length: 2.0,
        },
        MechUpperPart,
    )).id();
    
    app.world.entity_mut(turret_entity).set_parent(mech_entity);
    
    // Create enemy in front (+Z direction)
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Give mech attack target
    app.world.entity_mut(mech_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Run multiple updates to allow turret to rotate
    for _ in 0..5 {
        app.update();
    }
    
    // Check that turret is rotating towards target
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    
    // Target should be at 0 degrees (forward)
    assert_eq!(turret_rotation.target_angle, 0.0, "Turret should target forward (0 degrees)");
    
    // Turret should have rotated from 180 towards 0
    assert!(turret_rotation.current_angle < 180.0, "Turret should have started rotating towards target");
    assert!(turret_rotation.current_angle > 0.0, "Turret should not have instantly snapped to target");
}

/// Test that turret maintains lock on target when chassis moves
#[test]
fn test_turret_maintains_lock_during_chassis_movement() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        movement_system,
        turret_control_system,
    ).chain());
    
    // Create mech at origin
    let mech_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Hero,
    )).id();
    
    // Create turret facing right (90 degrees)
    let turret_entity = app.world.spawn((
        Transform::from_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
        GlobalTransform::default(),
        TurretRotation {
            current_angle: 90.0,
            target_angle: 90.0,
        },
        TurretCannon {
            fire_rate: 1.0,
            projectile_damage: 10.0,
            rotation_speed: 360.0, // Fast rotation for testing
            barrel_length: 2.0,
        },
        MechUpperPart,
    )).id();
    
    app.world.entity_mut(turret_entity).set_parent(mech_entity);
    
    // Create enemy to the right
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Give mech attack target
    app.world.entity_mut(mech_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Let turret lock onto target
    app.update();
    
    // Now move the mech backward
    app.world.entity_mut(mech_entity).insert(MoveTarget { 
        position: Vec2::new(0.0, -5.0)
    });
    
    // Run movement updates
    for _ in 0..10 {
        app.update();
    }
    
    // Get current positions
    let mech_transform = app.world.get::<Transform>(mech_entity).unwrap();
    let enemy_transform = app.world.get::<Transform>(enemy_entity).unwrap();
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    
    // Calculate expected angle from new mech position to enemy
    let mech_pos = Vec2::new(mech_transform.translation.x, mech_transform.translation.z);
    let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
    let expected_angle = calculate_turret_angle(mech_pos, enemy_pos);
    
    // Turret should still be aiming at enemy from new position
    assert!(
        (turret_rotation.target_angle - expected_angle).abs() < 1.0,
        "Turret should maintain lock on enemy. Expected: {}, Got: {}",
        expected_angle,
        turret_rotation.target_angle
    );
}

/// Test that turret compensates for chassis rotation
#[test]
fn test_turret_compensates_for_chassis_rotation() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Create enemy at fixed position (10, 0, 0)
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Test various chassis rotations
    let test_cases = vec![
        (0.0, "Chassis facing forward"),
        (90.0, "Chassis facing right"),
        (180.0, "Chassis facing backward"),
        (270.0, "Chassis facing left"),
        (45.0, "Chassis at 45 degrees"),
    ];
    
    for (chassis_angle, description) in test_cases {
        let chassis_angle = chassis_angle as f32;
        // Create mech with specific rotation
        let mech_entity = app.world.spawn((
            Transform::from_rotation(Quat::from_rotation_y(chassis_angle.to_radians())),
            GlobalTransform::default(),
        )).id();
        
        // Calculate expected local turret angle
        // Enemy is at (10, 0, 0), which is 90 degrees in world space
        let world_angle_to_enemy = 90.0;
        let expected_local_angle = normalize_angle(world_angle_to_enemy - chassis_angle);
        
        // Create turret already at the correct local angle
        let turret_entity = app.world.spawn((
            Transform::from_rotation(Quat::from_rotation_y(expected_local_angle.to_radians())),
            GlobalTransform::default(),
            TurretRotation {
                current_angle: expected_local_angle,
                target_angle: world_angle_to_enemy,
            },
            TurretCannon {
                fire_rate: 1.0,
                projectile_damage: 10.0,
                rotation_speed: 360.0,
                barrel_length: 2.0,
            },
            MechUpperPart,
        )).id();
        
        app.world.entity_mut(turret_entity).set_parent(mech_entity);
        app.world.entity_mut(mech_entity).insert(AttackTarget { entity: enemy_entity });
        
        // Just one update to propagate transforms
        app.update();
        
        // Debug: Check turret rotation values
        let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
        println!("{}: TurretRotation - current: {}, target: {}", 
                 description, turret_rotation.current_angle, turret_rotation.target_angle);
        
        // Get global transform of turret
        let turret_transform = app.world.get::<Transform>(turret_entity).unwrap();
        let parent_transform = app.world.get::<Transform>(mech_entity).unwrap();
        
        let global_turret_transform = parent_transform.mul_transform(*turret_transform);
        let turret_forward = global_turret_transform.rotation * Vec3::Z;
        let turret_forward_2d = Vec2::new(turret_forward.x, turret_forward.z).normalize();
        
        // Expected direction is always towards enemy at (10, 0, 0)
        let expected_direction = Vec2::new(1.0, 0.0); // Normalized direction to enemy
        
        let dot_product = turret_forward_2d.dot(expected_direction);
        
        println!("{}: chassis_angle={}, turret_forward={:?}, dot_product={}",
                 description, chassis_angle, turret_forward_2d, dot_product);
        
        assert!(
            dot_product > 0.98, // Allow small tolerance
            "{}: Turret should face enemy regardless of chassis rotation. Dot product: {}",
            description,
            dot_product
        );
        
        // Clean up entities for next test
        app.world.entity_mut(mech_entity).despawn_recursive();
    }
}

/// Test that turret tracks moving target
#[test]
fn test_turret_tracks_moving_target() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Create stationary mech
    let mech_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
    )).id();
    
    // Create turret
    let turret_entity = app.world.spawn((
        Transform::default(),
        GlobalTransform::default(),
        TurretRotation {
            current_angle: 0.0,
            target_angle: 0.0,
        },
        TurretCannon {
            fire_rate: 1.0,
            projectile_damage: 10.0,
            rotation_speed: 180.0, // 180 degrees per second
            barrel_length: 2.0,
        },
        MechUpperPart,
    )).id();
    
    app.world.entity_mut(turret_entity).set_parent(mech_entity);
    
    // Create enemy that will move
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)), // Start at right
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    app.world.entity_mut(mech_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Test enemy at different positions
    let positions = vec![
        Vec3::new(10.0, 0.0, 0.0),  // Right (90 degrees)
        Vec3::new(0.0, 0.0, 10.0),  // Forward (0 degrees)
        Vec3::new(-10.0, 0.0, 0.0), // Left (270 degrees)
        Vec3::new(0.0, 0.0, -10.0), // Backward (180 degrees)
    ];
    
    for (i, &enemy_pos) in positions.iter().enumerate() {
        // Move enemy
        app.world.entity_mut(enemy_entity).insert(Transform::from_translation(enemy_pos));
        
        // Run updates to let turret track
        for _ in 0..10 {
            app.update();
        }
        
        let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
        let expected_angle = calculate_turret_angle(Vec2::ZERO, Vec2::new(enemy_pos.x, enemy_pos.z));
        let expected_angle_normalized = normalize_angle(expected_angle);
        
        println!("Position {}: enemy at {:?}, turret target angle: {}, expected: {} (normalized: {})",
                 i, enemy_pos, turret_rotation.target_angle, expected_angle, expected_angle_normalized);
        
        // Check angle difference accounting for wrapping
        let angle_diff = shortest_angle_difference(turret_rotation.target_angle, expected_angle_normalized).abs();
        assert!(
            angle_diff < 1.0,
            "Turret should track to new enemy position. Angle diff: {}",
            angle_diff
        );
    }
}