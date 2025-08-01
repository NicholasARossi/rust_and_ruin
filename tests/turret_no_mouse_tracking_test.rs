use bevy::prelude::*;
use rust_and_ruin::mech::*;
use rust_and_ruin::components::*;
use rust_and_ruin::systems::turret_control::*;
use rust_and_ruin::resources::*;

/// Test that turret does not track mouse when there's no AttackTarget
#[test]
fn test_turret_ignores_mouse_without_attack_target() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Create mech at origin
    let mech_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Hero,
    )).id();
    
    // Create turret facing forward (0 degrees)
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
            rotation_speed: 180.0,
            barrel_length: 2.0,
        },
        MechUpperPart,
    )).id();
    
    app.world.entity_mut(turret_entity).set_parent(mech_entity);
    
    // Set mouse position to the right (should be ignored)
    app.insert_resource(MouseWorldPosition { position: Vec2::new(10.0, 0.0) });
    
    // Run multiple updates
    for _ in 0..5 {
        app.update();
    }
    
    // Check that turret hasn't moved
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    assert_eq!(
        turret_rotation.current_angle, 0.0,
        "Turret should maintain its angle when there's no AttackTarget"
    );
    
    // Now move mouse to a different position
    app.insert_resource(MouseWorldPosition { position: Vec2::new(0.0, 10.0) });
    
    // Run more updates
    for _ in 0..5 {
        app.update();
    }
    
    // Turret should still not have moved
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    assert_eq!(
        turret_rotation.current_angle, 0.0,
        "Turret should continue to maintain its angle regardless of mouse movement"
    );
}

/// Test that turret maintains last angle when AttackTarget is removed
#[test]
fn test_turret_maintains_angle_when_target_removed() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Create mech
    let mech_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
    )).id();
    
    // Create turret facing forward
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
            rotation_speed: 360.0, // Fast rotation for test
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
    
    // Run update to let turret start tracking
    app.update();
    
    // Verify turret is targeting enemy (90 degrees)
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    assert_eq!(turret_rotation.target_angle, 90.0, "Turret should target enemy at 90 degrees");
    
    // Now remove the attack target
    app.world.entity_mut(mech_entity).remove::<AttackTarget>();
    
    // Move mouse to a different position
    app.insert_resource(MouseWorldPosition { position: Vec2::new(-10.0, 0.0) });
    
    // Run updates
    for _ in 0..5 {
        app.update();
    }
    
    // Turret should maintain its angle, not track mouse
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    let turret_transform = app.world.get::<Transform>(turret_entity).unwrap();
    
    // The turret was moving towards 90 degrees, so it should be at or near that angle
    println!("Turret angle after removing target: {}", turret_rotation.current_angle);
    
    // Turret should not be tracking mouse (which would be 270 degrees)
    assert!(
        turret_rotation.current_angle < 180.0,
        "Turret should not track mouse after AttackTarget is removed"
    );
}