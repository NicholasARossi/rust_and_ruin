use bevy::prelude::*;
use rust_and_ruin::*;
use rust_and_ruin::mech::*;
use rust_and_ruin::components::*;
use rust_and_ruin::systems::turret_control::*;
use rust_and_ruin::systems::movement::*;
use rust_and_ruin::systems::input::*;
use rust_and_ruin::resources::*;

#[test]
fn test_q_key_sets_attack_target() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::new(5.0, 0.0) }); // Near enemy
    app.insert_resource(Input::<KeyCode>::default());
    app.insert_resource(Input::<MouseButton>::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    
    app.add_systems(Update, enemy_selection_system);
    
    // Create hero without attack target
    let hero_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Hero,
    )).id();
    
    // Create enemy nearby
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Simulate Q key press
    app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Q);
    
    // Run update
    app.update();
    
    // Check that attack target was set
    let hero_has_target = app.world.get::<AttackTarget>(hero_entity).is_some();
    assert!(hero_has_target, "Hero should have AttackTarget after pressing Q");
    
    if let Some(attack_target) = app.world.get::<AttackTarget>(hero_entity) {
        assert_eq!(attack_target.entity, enemy_entity, "AttackTarget should point to the enemy");
    }
}

#[test]
fn test_turret_rotates_to_face_target_after_q() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::new(10.0, 0.0) });
    app.insert_resource(Time::<()>::default());
    app.insert_resource(Input::<KeyCode>::default());
    app.insert_resource(Input::<MouseButton>::default());
    app.insert_resource(Assets::<Mesh>::default());
    app.insert_resource(Assets::<StandardMaterial>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        enemy_selection_system,
        turret_control_system,
    ).chain());
    
    // Create mech with turret facing backward
    let mech_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Hero,
    )).id();
    
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
            rotation_speed: 360.0, // Fast for testing
            barrel_length: 2.0,
        },
    )).id();
    
    app.world.entity_mut(turret_entity).set_parent(mech_entity);
    
    // Create enemy to the right
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Simulate Q key press
    app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Q);
    
    // Run updates
    for _ in 0..5 {
        app.update();
    }
    
    // Check turret rotation
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    
    // Turret should be aiming right (90 degrees) at the enemy
    assert!(
        (turret_rotation.target_angle - 90.0).abs() < 1.0,
        "Turret should target 90 degrees (right). Got: {}",
        turret_rotation.target_angle
    );
    
    // Turret should have started rotating
    assert!(
        turret_rotation.current_angle != 180.0,
        "Turret should have started rotating from 180 degrees"
    );
}

#[test]
fn test_turret_maintains_lock_while_tank_moves() {
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
            rotation_speed: 3600.0, // Very fast for testing
            barrel_length: 2.0,
        },
    )).id();
    
    app.world.entity_mut(turret_entity).set_parent(mech_entity);
    
    // Create enemy at fixed position
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 10.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Set attack target
    app.world.entity_mut(mech_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Let turret acquire target
    app.update();
    
    // Move tank to different positions and verify turret tracking
    let test_positions = vec![
        Vec2::new(-5.0, 0.0),   // Move left
        Vec2::new(0.0, -5.0),   // Move back
        Vec2::new(5.0, 0.0),    // Move right
        Vec2::new(0.0, 5.0),    // Move forward
        Vec2::new(-5.0, -5.0),  // Move diagonal
    ];
    
    for test_pos in test_positions {
        // Set move target
        app.world.entity_mut(mech_entity).insert(MoveTarget { position: test_pos });
        
        // Run movement for several frames
        for _ in 0..50 {
            app.update();
        }
        
        // Debug: Check if mech still has attack target
        let has_attack_target = app.world.get::<AttackTarget>(mech_entity).is_some();
        println!("Mech has AttackTarget: {}", has_attack_target);
        
        // Get current positions
        let mech_transform = app.world.get::<Transform>(mech_entity).unwrap();
        let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
        
        let mech_pos = Vec2::new(mech_transform.translation.x, mech_transform.translation.z);
        let enemy_pos = Vec2::new(10.0, 10.0); // Fixed enemy position
        
        // Calculate expected angle from new position
        let expected_angle = calculate_turret_angle(mech_pos, enemy_pos);
        let expected_angle_normalized = normalize_angle(expected_angle);
        
        // Account for parent rotation
        let parent_rotation_degrees = normalize_angle(mech_transform.rotation.to_euler(EulerRot::YXZ).1.to_degrees());
        let expected_local_angle = normalize_angle(expected_angle_normalized - parent_rotation_degrees);
        
        let angle_diff = shortest_angle_difference(turret_rotation.current_angle, expected_local_angle).abs();
        
        println!("Tank at {:?}, turret angle: {:.1}, expected: {:.1}, diff: {:.1}",
                 test_pos, turret_rotation.current_angle, expected_local_angle, angle_diff);
        
        assert!(
            angle_diff < 5.0,
            "Turret should maintain lock on enemy from position {:?}. Angle diff: {}",
            test_pos,
            angle_diff
        );
    }
}

#[test]
fn test_turret_compensates_for_chassis_rotation_with_lock() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Fixed enemy position
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Test different chassis rotations
    let chassis_rotations = vec![0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0];
    
    for chassis_angle in chassis_rotations {
        let chassis_angle = chassis_angle as f32;
        // Create mech with specific rotation
        let mech_entity = app.world.spawn((
            Transform::from_rotation(Quat::from_rotation_y(chassis_angle.to_radians())),
            GlobalTransform::default(),
            Hero,
            AttackTarget { entity: enemy_entity },
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
                rotation_speed: 360.0,
                barrel_length: 2.0,
            },
        )).id();
        
        app.world.entity_mut(turret_entity).set_parent(mech_entity);
        
        // Run updates to let turret track
        for _ in 0..5 {
            app.update();
        }
        
        // Check global turret orientation
        let mech_transform = app.world.get::<Transform>(mech_entity).unwrap();
        let turret_transform = app.world.get::<Transform>(turret_entity).unwrap();
        let global_turret = mech_transform.mul_transform(*turret_transform);
        
        let turret_forward = global_turret.rotation * Vec3::Z;
        let turret_forward_2d = Vec2::new(turret_forward.x, turret_forward.z).normalize();
        
        // Expected direction towards enemy at (10, 0, 0)
        let expected_direction = Vec2::new(1.0, 0.0);
        let dot_product = turret_forward_2d.dot(expected_direction);
        
        println!("Chassis angle: {}, turret forward: {:?}, dot: {}",
                 chassis_angle, turret_forward_2d, dot_product);
        
        assert!(
            dot_product > 0.95,
            "Turret should face enemy regardless of chassis rotation {}. Dot: {}",
            chassis_angle,
            dot_product
        );
        
        // Clean up for next iteration
        app.world.entity_mut(mech_entity).despawn_recursive();
    }
}

#[test]
fn test_turret_tracks_during_circular_movement() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        movement_system,
        turret_control_system,
    ).chain());
    
    // Create mech
    let mech_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Hero,
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
            rotation_speed: 3600.0, // Very fast for testing
            barrel_length: 2.0,
        },
    )).id();
    
    app.world.entity_mut(turret_entity).set_parent(mech_entity);
    
    // Create enemy at center
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Set attack target
    app.world.entity_mut(mech_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Move in a circle around the enemy
    let radius = 5.0;
    let steps = 8;
    
    for i in 0..steps {
        let angle = (i as f32 / steps as f32) * std::f32::consts::TAU;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        
        app.world.entity_mut(mech_entity).insert(MoveTarget { position: Vec2::new(x, z) });
        
        // Run movement
        for _ in 0..30 {
            app.update();
        }
        
        // Check turret is still facing center
        let mech_transform = app.world.get::<Transform>(mech_entity).unwrap();
        let turret_transform = app.world.get::<Transform>(turret_entity).unwrap();
        let global_turret = mech_transform.mul_transform(*turret_transform);
        
        let turret_pos = Vec2::new(global_turret.translation.x, global_turret.translation.z);
        let is_facing = is_turret_facing_target(
            &global_turret,
            turret_pos,
            Vec2::ZERO, // Enemy at center
            10.0, // Slightly larger tolerance for movement
        );
        
        println!("Circle position {}/{}: tank at ({:.1}, {:.1}), facing center: {}",
                 i + 1, steps, x, z, is_facing);
        
        assert!(is_facing, "Turret should face enemy at center during circular movement");
    }
}