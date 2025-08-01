use bevy::prelude::*;
use rust_and_ruin::systems::turret_control::*;
use rust_and_ruin::mech::*;
use rust_and_ruin::components::*;
use rust_and_ruin::resources::*;

#[test]
fn test_calculate_turret_angle_basic() {
    // Test basic angle calculations for cardinal directions
    // In Bevy's coordinate system:
    // - Forward (+Z) should be 0°
    // - Right (+X) should be 90°
    // - Backward (-Z) should be 180°
    // - Left (-X) should be 270°
    
    struct TestCase {
        from: Vec2,
        to: Vec2,
        expected_angle: f32,
        description: &'static str,
    }
    
    let test_cases = vec![
        TestCase {
            from: Vec2::ZERO,
            to: Vec2::new(0.0, 10.0),  // +Z direction (forward in 3D)
            expected_angle: 0.0,
            description: "Target forward (+Z)",
        },
        TestCase {
            from: Vec2::ZERO,
            to: Vec2::new(10.0, 0.0),  // +X direction (right)
            expected_angle: 90.0,
            description: "Target right (+X)",
        },
        TestCase {
            from: Vec2::ZERO,
            to: Vec2::new(0.0, -10.0), // -Z direction (backward)
            expected_angle: 180.0,
            description: "Target backward (-Z)",
        },
        TestCase {
            from: Vec2::ZERO,
            to: Vec2::new(-10.0, 0.0), // -X direction (left)
            expected_angle: 270.0,
            description: "Target left (-X)",
        },
    ];
    
    println!("Testing calculate_turret_angle for cardinal directions:");
    println!("Expected: Forward=0°, Right=90°, Back=180°, Left=270°");
    println!();
    
    for test in test_cases {
        let calculated = calculate_turret_angle(test.from, test.to);
        let normalized = normalize_angle(calculated);
        
        println!("{}: from {:?} to {:?}", test.description, test.from, test.to);
        println!("  Calculated: {:.1}°", calculated);
        println!("  Normalized: {:.1}°", normalized);
        println!("  Expected:   {:.1}°", test.expected_angle);
        
        let diff = (normalized - test.expected_angle).abs();
        let pass = diff < 0.1;
        println!("  Result: {} (diff: {:.1}°)", if pass { "PASS" } else { "FAIL" }, diff);
        println!();
        
        // This will likely fail initially, showing us what the actual calculation returns
        assert!(
            diff < 0.1,
            "{}: Expected {:.1}°, got {:.1}° (diff: {:.1}°)",
            test.description,
            test.expected_angle,
            normalized,
            diff
        );
    }
}

#[test]
fn test_turret_forward_matches_rotation() {
    // Test that get_turret_forward_direction returns correct forward vector
    // based on turret rotation
    
    struct TestCase {
        rotation_degrees: f32,
        expected_forward: Vec2,
        description: &'static str,
    }
    
    let test_cases = vec![
        TestCase {
            rotation_degrees: 0.0,
            expected_forward: Vec2::new(0.0, 1.0), // +Z in 2D
            description: "0° rotation (facing +Z)",
        },
        TestCase {
            rotation_degrees: 90.0,
            expected_forward: Vec2::new(1.0, 0.0), // +X
            description: "90° rotation (facing +X)",
        },
        TestCase {
            rotation_degrees: 180.0,
            expected_forward: Vec2::new(0.0, -1.0), // -Z
            description: "180° rotation (facing -Z)",
        },
        TestCase {
            rotation_degrees: 270.0,
            expected_forward: Vec2::new(-1.0, 0.0), // -X
            description: "270° rotation (facing -X)",
        },
    ];
    
    println!("Testing get_turret_forward_direction:");
    println!();
    
    for test in test_cases {
        let transform = Transform::from_rotation(
            Quat::from_rotation_y(test.rotation_degrees.to_radians())
        );
        let forward = get_turret_forward_direction(&transform);
        
        println!("{} ({}°):", test.description, test.rotation_degrees);
        println!("  Forward vector: ({:.3}, {:.3})", forward.x, forward.y);
        println!("  Expected:       ({:.3}, {:.3})", test.expected_forward.x, test.expected_forward.y);
        
        let diff = (forward - test.expected_forward).length();
        let pass = diff < 0.01;
        println!("  Result: {} (diff: {:.3})", if pass { "PASS" } else { "FAIL" }, diff);
        println!();
        
        assert!(
            diff < 0.01,
            "{}: Expected forward {:?}, got {:?}",
            test.description,
            test.expected_forward,
            forward
        );
    }
}

#[test]
fn test_turret_angle_debug() {
    // Simple debug test to understand what's happening
    println!("=== Turret Angle Debug Test ===");
    println!();
    
    // Scenario: Turret at origin, enemy at various positions
    let turret_pos = Vec2::ZERO;
    let test_positions = vec![
        (Vec2::new(10.0, 0.0), "Enemy at +X (right)"),
        (Vec2::new(0.0, 10.0), "Enemy at +Z (forward)"),
        (Vec2::new(-10.0, 0.0), "Enemy at -X (left)"),
        (Vec2::new(0.0, -10.0), "Enemy at -Z (backward)"),
        (Vec2::new(10.0, 10.0), "Enemy at +X+Z (forward-right)"),
    ];
    
    for (enemy_pos, description) in test_positions {
        println!("{}:", description);
        println!("  Enemy position: {:?}", enemy_pos);
        
        // Calculate using current function
        let angle = calculate_turret_angle(turret_pos, enemy_pos);
        let normalized = normalize_angle(angle);
        
        // Manual calculation for comparison
        let direction = enemy_pos - turret_pos;
        let atan2_result = direction.x.atan2(direction.y);
        let atan2_degrees = atan2_result.to_degrees();
        
        // Alternative calculation (y, x)
        let alt_atan2 = direction.y.atan2(direction.x);
        let alt_degrees = alt_atan2.to_degrees();
        
        println!("  Direction vector: {:?}", direction);
        println!("  atan2(x, y) radians: {:.3}", atan2_result);
        println!("  atan2(x, y) degrees: {:.1}°", atan2_degrees);
        println!("  atan2(y, x) degrees: {:.1}° (alternative)", alt_degrees);
        println!("  Normalized result: {:.1}°", normalized);
        
        // Create a transform with this rotation and check forward
        let transform = Transform::from_rotation(Quat::from_rotation_y(normalized.to_radians()));
        let forward = get_turret_forward_direction(&transform);
        let direction_normalized = direction.normalize();
        
        println!("  Turret forward: ({:.3}, {:.3})", forward.x, forward.y);
        println!("  Should point:   ({:.3}, {:.3})", direction_normalized.x, direction_normalized.y);
        
        let dot_product = forward.dot(direction_normalized);
        println!("  Dot product: {:.3} (should be close to 1.0)", dot_product);
        println!();
    }
}

#[test]
fn test_world_to_local_angle_conversion() {
    // Test the conversion from world angle to local angle when parent is rotated
    println!("=== Testing World to Local Angle Conversion ===");
    println!();
    
    struct TestCase {
        parent_rotation: f32,
        world_target_angle: f32,
        expected_local_angle: f32,
        description: &'static str,
    }
    
    let test_cases = vec![
        TestCase {
            parent_rotation: 0.0,
            world_target_angle: 90.0,
            expected_local_angle: 90.0,
            description: "Parent facing forward, target right",
        },
        TestCase {
            parent_rotation: 90.0,
            world_target_angle: 90.0,
            expected_local_angle: 0.0,
            description: "Parent facing right, target right (turret should face forward relative to parent)",
        },
        TestCase {
            parent_rotation: 180.0,
            world_target_angle: 90.0,
            expected_local_angle: 270.0,
            description: "Parent facing backward, target right (turret should face left relative to parent)",
        },
        TestCase {
            parent_rotation: 45.0,
            world_target_angle: 135.0,
            expected_local_angle: 90.0,
            description: "Parent at 45°, target at 135° (turret should face right relative to parent)",
        },
    ];
    
    for test in test_cases {
        let local_angle = normalize_angle(test.world_target_angle - test.parent_rotation);
        
        println!("{}:", test.description);
        println!("  Parent rotation: {}°", test.parent_rotation);
        println!("  World target angle: {}°", test.world_target_angle);
        println!("  Calculated local angle: {}°", local_angle);
        println!("  Expected local angle: {}°", test.expected_local_angle);
        
        let diff = shortest_angle_difference(local_angle, test.expected_local_angle).abs();
        let pass = diff < 0.1;
        println!("  Result: {} (diff: {:.1}°)", if pass { "PASS" } else { "FAIL" }, diff);
        println!();
        
        assert!(
            diff < 0.1,
            "{}: Expected local angle {:.1}°, got {:.1}°",
            test.description,
            test.expected_local_angle,
            local_angle
        );
    }
}

#[test]
fn test_turret_control_simple() {
    // Simple test without parent rotation
    use bevy::app::App;
    use bevy::hierarchy::{BuildChildren, HierarchyPlugin};
    use bevy::transform::TransformPlugin;
    
    println!("=== Simple Turret Control Test ===");
    
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Create parent at origin with NO rotation
    let parent_entity = app.world.spawn((
        Transform::default(),
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
            rotation_speed: 3600.0,
            barrel_length: 2.0,
        },
        MechUpperPart,
    )).id();
    
    app.world.entity_mut(turret_entity).set_parent(parent_entity);
    
    // Create enemy to the right
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Set attack target
    app.world.entity_mut(parent_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Run updates until turret reaches target
    let mut reached_target = false;
    for i in 0..2000 {
        app.update();
        let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
        
        if (turret_rotation.current_angle - 90.0).abs() < 1.0 {
            println!("Turret reached target at update {}: current_angle = {}°", i, turret_rotation.current_angle);
            reached_target = true;
            break;
        }
        
        if i % 200 == 0 {
            println!("Update {}: current_angle = {}°", i, turret_rotation.current_angle);
        }
    }
    
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    println!("Final: current_angle = {}°, target_angle = {}°", 
             turret_rotation.current_angle, turret_rotation.target_angle);
    
    // Turret should be at 90 degrees (pointing right)
    assert!(
        reached_target,
        "Turret did not reach target angle. Got: {}",
        turret_rotation.current_angle
    );
}

#[test]
fn test_turret_system_integration() {
    // Integration test to understand what's happening in the actual system
    use bevy::app::App;
    use bevy::hierarchy::{BuildChildren, HierarchyPlugin, Parent};
    use bevy::transform::TransformPlugin;
    
    println!("=== Turret System Integration Test ===");
    println!();
    
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin));
    app.add_plugins(bevy::log::LogPlugin::default());
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    // Add the turret control system
    app.add_systems(Update, (
        bevy::transform::systems::propagate_transforms,
        turret_control_system,
    ).chain());
    
    // Create parent (tank chassis) at origin, rotated 45 degrees
    let parent_rotation = 45.0_f32;
    let parent_transform = Transform::from_rotation(Quat::from_rotation_y(parent_rotation.to_radians()));
    let (y, x, z) = parent_transform.rotation.to_euler(EulerRot::YXZ);
    println!("Creating parent with Y rotation: {:.2}°", y.to_degrees());
    let parent_entity = app.world.spawn((
        parent_transform,
        GlobalTransform::default(),
        Hero,
    )).id();
    
    // Create turret as child
    let turret_entity = app.world.spawn((
        Transform::default(), // Local transform starts at identity
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
        MechUpperPart,
    )).id();
    
    // Set up parent-child relationship
    app.world.entity_mut(turret_entity).set_parent(parent_entity);
    
    // Create enemy at (10, 0, 0) - to the right
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Give parent an attack target
    app.world.entity_mut(parent_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Run one update to let transforms propagate
    app.update();
    
    // Debug: Check query results
    println!("Debug - checking entities:");
    println!("  Parent has Hero: {}", app.world.get::<Hero>(parent_entity).is_some());
    println!("  Parent has AttackTarget: {}", app.world.get::<AttackTarget>(parent_entity).is_some());
    println!("  Turret has MechUpperPart: {}", app.world.get::<MechUpperPart>(turret_entity).is_some());
    println!("  Turret has TurretRotation: {}", app.world.get::<TurretRotation>(turret_entity).is_some());
    println!("  Turret parent: {:?}", app.world.get::<Parent>(turret_entity).map(|p| p.get()));
    
    // Check positions
    let parent_global = app.world.get::<GlobalTransform>(parent_entity).unwrap();
    let turret_global = app.world.get::<GlobalTransform>(turret_entity).unwrap();
    println!("  Parent global pos: {:?}", parent_global.translation());
    println!("  Turret global pos: {:?}", turret_global.translation());
    
    // Check state before turret control
    let (parent_pos, parent_rot_deg, turret_before_info) = {
        let parent_transform = app.world.get::<Transform>(parent_entity).unwrap();
        let turret_transform_before = app.world.get::<Transform>(turret_entity).unwrap();
        let turret_rotation_before = app.world.get::<TurretRotation>(turret_entity).unwrap();
        
        let (y, x, z) = parent_transform.rotation.to_euler(EulerRot::YXZ);
        println!("  Parent euler angles (YXZ): y={:.2}°, x={:.2}°, z={:.2}°", 
                 y.to_degrees(), x.to_degrees(), z.to_degrees());
        println!("  Parent quaternion: {:?}", parent_transform.rotation);
        
        let parent_y_rot = y.to_degrees();
        let parent_pos = Vec2::new(parent_transform.translation.x, parent_transform.translation.z);
        let info = (
            turret_transform_before.rotation.to_euler(EulerRot::YXZ).1.to_degrees(),
            turret_rotation_before.current_angle,
            turret_rotation_before.target_angle,
        );
        (parent_pos, parent_y_rot, info)
    };
    
    println!("Before turret control:");
    println!("  Parent rotation (expected): {}°", parent_rotation);
    println!("  Parent rotation (actual): {}°", parent_rot_deg);
    println!("  Turret local rotation: {}°", turret_before_info.0);
    println!("  Turret current_angle: {}°", turret_before_info.1);
    println!("  Turret target_angle: {}°", turret_before_info.2);
    
    // Run many updates to let turret fully rotate (test time is very small)
    for i in 0..2000 {
        app.update();
        if i % 400 == 0 {
            let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
            println!("  Update {}: current_angle = {}°, target_angle = {}°", 
                     i, turret_rotation.current_angle, turret_rotation.target_angle);
        }
    }
    
    // Check state after turret control
    let turret_transform_after = app.world.get::<Transform>(turret_entity).unwrap();
    let turret_rotation_after = app.world.get::<TurretRotation>(turret_entity).unwrap();
    let turret_global_after = app.world.get::<GlobalTransform>(turret_entity).unwrap();
    
    println!("\nAfter turret control:");
    println!("  Turret local Y rotation from transform: {}°", turret_transform_after.rotation.to_euler(EulerRot::YXZ).1.to_degrees());
    println!("  Turret current_angle: {}°", turret_rotation_after.current_angle);
    println!("  Turret target_angle: {}°", turret_rotation_after.target_angle);
    
    // Debug: Check what angle the transform actually has
    let actual_local_y_rad = turret_transform_after.rotation.to_euler(EulerRot::YXZ).1;
    let actual_local_y_deg = actual_local_y_rad.to_degrees();
    println!("  Transform Y rotation (radians): {}", actual_local_y_rad);
    println!("  Transform Y rotation (degrees): {}", actual_local_y_deg);
    println!("  Transform Y rotation normalized: {}", normalize_angle(actual_local_y_deg));
    
    // Debug: What happens if we convert current_angle as if it were radians?
    let current_as_radians = turret_rotation_after.current_angle; // This is 90
    let quat_from_current = Quat::from_rotation_y(current_as_radians); // Treating 90 as radians!
    let back_to_degrees = quat_from_current.to_euler(EulerRot::YXZ).1.to_degrees();
    println!("  If current_angle ({}) were radians, rotation would be: {}°", current_as_radians, back_to_degrees);
    
    // Calculate expected values
    let enemy_pos = Vec2::new(10.0, 0.0);
    let world_angle_to_enemy = calculate_turret_angle(parent_pos, enemy_pos);
    let expected_local_angle = normalize_angle(world_angle_to_enemy - parent_rotation);
    
    println!("\nExpected values:");
    println!("  World angle to enemy: {}°", world_angle_to_enemy);
    println!("  Expected local angle: {}°", expected_local_angle);
    
    // Check global turret forward direction
    let turret_forward = get_turret_forward_direction(&turret_global_after.compute_transform());
    let direction_to_enemy = (enemy_pos - parent_pos).normalize();
    let dot_product = turret_forward.dot(direction_to_enemy);
    
    println!("\nGlobal alignment check:");
    println!("  Turret forward (global): ({:.3}, {:.3})", turret_forward.x, turret_forward.y);
    println!("  Direction to enemy: ({:.3}, {:.3})", direction_to_enemy.x, direction_to_enemy.y);
    println!("  Dot product: {:.3} (should be close to 1.0)", dot_product);
    
    // The turret should be pointing at the enemy (or close to it)
    println!("\nAssertion check:");
    println!("  Dot product threshold: 0.99 (approximately 8 degrees tolerance)");
    assert!(
        dot_product > 0.99,
        "Turret should be pointing at enemy. Dot product: {}",
        dot_product
    );
}