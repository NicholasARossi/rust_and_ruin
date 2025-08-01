use bevy::prelude::*;
use rust_and_ruin::mech::*;
use rust_and_ruin::components::*;
use rust_and_ruin::systems::*;

#[test]
fn test_mech_rotation_not_snapping_with_movement_system() {
    let mut app = App::new();
    
    // Add minimal plugins
    app.add_plugins(MinimalPlugins);
    
    // Add both movement systems in the same order as the demo
    app.add_systems(Update, (
        mech_movement_system,
        movement_system,
    ).chain());
    
    // Spawn a mech with Hero component (like in the demo)
    let mech_entity = app.world.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Hero,
        MechMovement::default(),
        create_tank_treads_lower(),
    )).id();
    
    // Add a move target requiring 90 degree rotation
    app.world.entity_mut(mech_entity).insert(
        MoveTarget { position: Vec2::new(10.0, 0.0) }
    );
    
    // Get initial rotation
    app.update();
    let initial_rotation = app.world.query::<&Transform>()
        .get(&app.world, mech_entity)
        .unwrap()
        .rotation.to_euler(EulerRot::YXZ).0.to_degrees();
    
    // Run a few more updates
    for _ in 0..5 {
        app.update();
    }
    
    // Get rotation after updates
    let current_rotation = app.world.query::<&Transform>()
        .get(&app.world, mech_entity)
        .unwrap()
        .rotation.to_euler(EulerRot::YXZ).0.to_degrees();
    
    // If movement_system is snapping, rotation would jump to 90 degrees instantly
    // If only mech_movement_system is working, rotation should be very small (due to tiny time deltas in tests)
    println!("Initial rotation: {:.2}°, Current rotation: {:.2}°", initial_rotation, current_rotation);
    
    // This test will fail if movement_system is snapping the rotation
    assert!(current_rotation.abs() < 85.0, 
        "Rotation jumped to {:.2}° - movement_system is snapping rotation!", current_rotation);
}

#[test]
fn test_movement_system_excludes_mech_movement() {
    let mut app = App::new();
    
    app.add_plugins(MinimalPlugins);
    app.add_systems(Update, movement_system);
    
    // Test 1: Entity with MechMovement should not be processed by movement_system
    let mech_entity = app.world.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Hero,
        MechMovement::default(),
        MoveTarget { position: Vec2::new(10.0, 0.0) },
    )).id();
    
    // Test 2: Entity with TankMovement should not be processed by movement_system
    let tank_entity = app.world.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Hero,
        TankMovement::default(),
        MoveTarget { position: Vec2::new(10.0, 0.0) },
    )).id();
    
    // Test 3: Entity with neither should be processed by movement_system
    let basic_entity = app.world.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Hero,
        MoveTarget { position: Vec2::new(10.0, 0.0) },
    )).id();
    
    // Add empty children component since movement_system requires it
    app.world.entity_mut(mech_entity).push_children(&[]);
    app.world.entity_mut(tank_entity).push_children(&[]);
    app.world.entity_mut(basic_entity).push_children(&[]);
    
    // Run update
    app.update();
    
    // Check rotations
    let mech_rotation = app.world.query::<&Transform>()
        .get(&app.world, mech_entity)
        .unwrap()
        .rotation.to_euler(EulerRot::YXZ).0.to_degrees();
        
    let tank_rotation = app.world.query::<&Transform>()
        .get(&app.world, tank_entity)
        .unwrap()
        .rotation.to_euler(EulerRot::YXZ).0.to_degrees();
        
    let basic_rotation = app.world.query::<&Transform>()
        .get(&app.world, basic_entity)
        .unwrap()
        .rotation.to_euler(EulerRot::YXZ).0.to_degrees();
    
    println!("Mech rotation: {:.2}°", mech_rotation);
    println!("Tank rotation: {:.2}°", tank_rotation);
    println!("Basic rotation: {:.2}°", basic_rotation);
    
    // After fix: MechMovement entity should not rotate (movement_system should skip it)
    assert!(mech_rotation.abs() < 1.0, "MechMovement entity should not be rotated by movement_system");
    
    // TankMovement entity should not rotate (already excluded)
    assert!(tank_rotation.abs() < 1.0, "TankMovement entity should not be rotated by movement_system");
    
    // Basic entity should rotate (movement_system should process it)
    assert!(basic_rotation.abs() > 80.0, "Basic entity should be rotated by movement_system to face target");
}