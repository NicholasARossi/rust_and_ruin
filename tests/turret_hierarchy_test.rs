use bevy::prelude::*;
use rust_and_ruin::*;
use rust_and_ruin::mech::*;
use rust_and_ruin::components::*;
use rust_and_ruin::systems::turret_control::*;
use rust_and_ruin::resources::*;
use rust_and_ruin::systems::attack_target_propagation::*;

/// Test that turret control works with nested hierarchy (Hero -> tank_base -> turret)
#[test]
fn test_turret_control_with_nested_hierarchy() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Create hero entity with AttackTarget
    let hero_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Hero,
    )).id();
    
    // Create tank_base as child of hero
    let tank_base = app.world.spawn((
        Transform::from_xyz(0.0, 0.25, 0.0),
        GlobalTransform::default(),
        MechLowerPart,
    )).id();
    
    // Create turret as child of tank_base, initially facing backward
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
            rotation_speed: 360.0, // Fast rotation for testing
            barrel_length: 2.0,
        },
        MechUpperPart,
    )).id();
    
    // Set up hierarchy: hero -> tank_base -> turret
    app.world.entity_mut(hero_entity).push_children(&[tank_base]);
    app.world.entity_mut(tank_base).push_children(&[turret_entity]);
    
    // Create enemy in front
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Add AttackTarget to hero (not tank_base)
    app.world.entity_mut(hero_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Run update - turret should NOT rotate because AttackTarget is on grandparent
    app.update();
    
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    
    // Without propagation, turret won't find the AttackTarget
    // It will maintain its current angle by calculating a maintain position
    assert_eq!(turret_rotation.current_angle, 180.0, 
        "Turret current angle should not change in one frame");
    
    // The target angle might change due to maintain position calculation, 
    // but it shouldn't be targeting the enemy at 0°
    assert_ne!(turret_rotation.target_angle, 0.0, 
        "Turret should not target the enemy without AttackTarget propagation");
}

/// Test that turret control works when AttackTarget is on immediate parent
#[test]
fn test_turret_control_with_immediate_parent_attack_target() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Create hero entity
    let hero_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Hero,
    )).id();
    
    // Create tank_base as child of hero
    let tank_base = app.world.spawn((
        Transform::from_xyz(0.0, 0.25, 0.0),
        GlobalTransform::default(),
        MechLowerPart,
    )).id();
    
    // Create turret as child of tank_base, initially facing backward
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
            rotation_speed: 360.0, // Fast rotation for testing
            barrel_length: 2.0,
        },
        MechUpperPart,
    )).id();
    
    // Set up hierarchy: hero -> tank_base -> turret
    app.world.entity_mut(hero_entity).push_children(&[tank_base]);
    app.world.entity_mut(tank_base).push_children(&[turret_entity]);
    
    // Create enemy in front
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Add AttackTarget to tank_base (immediate parent of turret)
    app.world.entity_mut(tank_base).insert(AttackTarget { entity: enemy_entity });
    
    // Run update - turret SHOULD rotate because AttackTarget is on immediate parent
    app.update();
    
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    
    // With AttackTarget on immediate parent, turret should start rotating
    assert_eq!(turret_rotation.target_angle, 0.0, 
        "Turret should target forward (0 degrees) when enemy is in front");
    // Current angle won't change in one frame with realistic rotation speed
    // but target angle should be set correctly
}

/// Test that propagation system allows turret to work with nested hierarchy
#[test]
fn test_turret_control_with_attack_target_propagation() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        propagate_attack_target_system,
        turret_control_system,
    ).chain());
    
    // Create hero entity with AttackTarget
    let hero_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Hero,
    )).id();
    
    // Create tank_base as child of hero
    let tank_base = app.world.spawn((
        Transform::from_xyz(0.0, 0.25, 0.0),
        GlobalTransform::default(),
        MechLowerPart,
    )).id();
    
    // Create turret as child of tank_base, initially facing backward
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
            rotation_speed: 360.0, // Fast rotation for testing
            barrel_length: 2.0,
        },
        MechUpperPart,
    )).id();
    
    // Set up hierarchy: hero -> tank_base -> turret
    app.world.entity_mut(hero_entity).push_children(&[tank_base]);
    app.world.entity_mut(tank_base).push_children(&[turret_entity]);
    
    // Create enemy to the right (+X direction = 90° in this coordinate system)
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Add AttackTarget to hero
    app.world.entity_mut(hero_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Run update - propagation should copy AttackTarget to tank_base, then turret should work
    app.update();
    
    // Check that AttackTarget was propagated to tank_base
    let tank_has_target = app.world.get::<AttackTarget>(tank_base).is_some();
    assert!(tank_has_target, "AttackTarget should be propagated to tank_base");
    
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    
    // With propagation, turret should now target the enemy
    // In this coordinate system with atan2(x, y), +X direction is 90°
    assert_eq!(turret_rotation.target_angle, 90.0, 
        "Turret should target right (+X = 90 degrees) with AttackTarget propagation");
}

/// Test that turret maintains tracking when tank rotates
#[test]
fn test_turret_tracking_during_tank_rotation() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        propagate_attack_target_system,
        turret_control_system,
    ).chain());
    
    // Create hero entity at origin
    let hero_entity = app.world.spawn((
        Transform::from_translation(Vec3::ZERO),
        GlobalTransform::default(),
        Hero,
    )).id();
    
    // Create tank_base as child of hero
    let tank_base = app.world.spawn((
        Transform::from_xyz(0.0, 0.25, 0.0),
        GlobalTransform::default(),
        MechLowerPart,
    )).id();
    
    // Create turret as child of tank_base, initially facing forward (0°)
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
            rotation_speed: 360.0, // Fast rotation for testing
            barrel_length: 2.0,
        },
        MechUpperPart,
    )).id();
    
    // Set up hierarchy: hero -> tank_base -> turret
    app.world.entity_mut(hero_entity).push_children(&[tank_base]);
    app.world.entity_mut(tank_base).push_children(&[turret_entity]);
    
    // Create enemy to the right (+X direction)
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Add AttackTarget to hero
    app.world.entity_mut(hero_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Update once to establish tracking
    app.update();
    
    // Turret should be targeting right initially
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    println!("Initial turret target angle: {}", turret_rotation.target_angle);
    // In this coordinate system, enemy at +X seems to be 90°, not 270°
    assert_eq!(turret_rotation.target_angle, 90.0, 
        "Turret should initially target right (90°)");
    
    // Now rotate the tank 90° counter-clockwise
    let mut hero_transform = app.world.get_mut::<Transform>(hero_entity).unwrap();
    hero_transform.rotation = Quat::from_rotation_y(90.0_f32.to_radians());
    
    // Update to propagate transforms and recalculate turret angle
    app.update();
    
    // After tank rotates 90° CCW, the enemy (still at +X) is now to the tank's right in local space
    // World angle to enemy is still 90°, but tank is facing 90°, so local angle should be 0°
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    println!("After 90° tank rotation: current_angle={}, target_angle={}", 
        turret_rotation.current_angle, turret_rotation.target_angle);
    // current_angle stores the local angle, target_angle stores the world angle
    assert_eq!(turret_rotation.current_angle, 0.0, 
        "After tank rotates 90° CCW, turret should face forward (0°) in local space");
    
    // Rotate tank another 90° CCW (total 180°)
    let mut hero_transform = app.world.get_mut::<Transform>(hero_entity).unwrap();
    hero_transform.rotation = Quat::from_rotation_y(180.0_f32.to_radians());
    
    app.update();
    
    // After tank rotates 180°, enemy (at world +X) is now to the tank's left in local space  
    // World angle to enemy is 90°, tank is facing 180°, so local angle = 90° - 180° = -90° = 270°
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    println!("After 180° tank rotation: current_angle={}, target_angle={}", 
        turret_rotation.current_angle, turret_rotation.target_angle);
    // Account for floating point precision and angle normalization
    let expected_angle = 270.0;
    let actual_angle = if turret_rotation.current_angle < 0.0 {
        turret_rotation.current_angle + 360.0
    } else {
        turret_rotation.current_angle
    };
    assert!((actual_angle - expected_angle).abs() < 1.0, 
        "After tank rotates 180°, turret should face left (270°) in local space, got {}", actual_angle);
}