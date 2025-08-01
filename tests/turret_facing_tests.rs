use bevy::prelude::*;
use rust_and_ruin::*;
use rust_and_ruin::mech::*;
use rust_and_ruin::components::*;
use rust_and_ruin::systems::turret_control::*;
use rust_and_ruin::resources::*;

#[test]
fn test_turret_faces_enemy_correctly() {
    // Create a test app
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(TransformPlugin);
    app.add_plugins(HierarchyPlugin);
    
    // Add resources
    app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
    app.insert_resource(Time::<()>::default());
    
    // Add the turret control system
    app.add_systems(Update, turret_control_system);
    
    // Create a mech with turret at origin
    let mech_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        GlobalTransform::default(),
    )).id();
    
    // Create a turret as child of mech
    let turret_entity = app.world.spawn((
        Transform::from_rotation(Quat::from_rotation_y(0.0)),
        GlobalTransform::default(),
        TurretRotation {
            current_angle: 0.0,
            target_angle: 0.0,
        },
        TurretCannon {
            fire_rate: 1.0,
            projectile_damage: 10.0,
            rotation_speed: 360.0, // Very fast for testing
            barrel_length: 2.0,
        },
    )).id();
    
    // Set up parent-child relationship
    app.world.entity_mut(turret_entity).set_parent(mech_entity);
    
    // Create an enemy to the right (+X direction)
    let enemy_entity = app.world.spawn((
        Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
        GlobalTransform::default(),
        Enemy,
    )).id();
    
    // Give the mech an attack target
    app.world.entity_mut(mech_entity).insert(AttackTarget { entity: enemy_entity });
    
    // Run one frame to calculate target angle
    app.update();
    
    // Get the target angle and set current angle to match (simulating instant rotation)
    {
        let mut turret_rotation = app.world.get_mut::<TurretRotation>(turret_entity).unwrap();
        turret_rotation.current_angle = turret_rotation.target_angle;
    }
    
    // Apply the rotation to the transform
    {
        let current_angle_radians = app.world.get::<TurretRotation>(turret_entity).unwrap().current_angle.to_radians();
        let mut turret_transform = app.world.get_mut::<Transform>(turret_entity).unwrap();
        turret_transform.rotation = Quat::from_rotation_y(current_angle_radians);
    }
    
    // Check turret rotation
    let turret_transform = app.world.get::<Transform>(turret_entity).unwrap();
    let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
    
    // Get the forward direction of the turret
    let forward = turret_transform.rotation * Vec3::Z;
    let turret_direction = Vec2::new(forward.x, forward.z).normalize();
    
    // Expected direction from mech to enemy (normalized)
    let expected_direction = Vec2::new(1.0, 0.0); // Enemy is to the right
    
    // Check if turret is facing the correct direction
    let dot_product = turret_direction.dot(expected_direction);
    
    println!("Turret current angle: {}", turret_rotation.current_angle);
    println!("Turret target angle: {}", turret_rotation.target_angle);
    println!("Turret forward direction: {:?}", turret_direction);
    println!("Expected direction: {:?}", expected_direction);
    println!("Dot product: {}", dot_product);
    
    // Dot product should be close to 1.0 if facing correctly
    assert!(dot_product > 0.9, "Turret should face towards enemy. Dot product: {}", dot_product);
}

#[test]
fn test_turret_rotation_for_all_directions() {
    struct TestCase {
        enemy_position: Vec3,
        expected_angle: f32,
        description: &'static str,
    }
    
    let test_cases = vec![
        TestCase {
            enemy_position: Vec3::new(10.0, 0.0, 0.0),  // Right (+X)
            expected_angle: 90.0,  // In Bevy's coordinate system with atan2(x,y)
            description: "Enemy to the right",
        },
        TestCase {
            enemy_position: Vec3::new(-10.0, 0.0, 0.0), // Left (-X)
            expected_angle: 270.0,
            description: "Enemy to the left",
        },
        TestCase {
            enemy_position: Vec3::new(0.0, 0.0, 10.0),  // Forward (+Z)
            expected_angle: 0.0,
            description: "Enemy forward",
        },
        TestCase {
            enemy_position: Vec3::new(0.0, 0.0, -10.0), // Backward (-Z)
            expected_angle: 180.0,
            description: "Enemy backward",
        },
    ];
    
    for test_case in test_cases {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(TransformPlugin);
        app.add_plugins(HierarchyPlugin);
        
        app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
        app.insert_resource(Time::<()>::default());
        app.add_systems(Update, turret_control_system);
        
        // Create mech at origin
        let mech_entity = app.world.spawn((
            Transform::from_translation(Vec3::ZERO),
            GlobalTransform::default(),
        )).id();
        
        // Create turret
        let turret_entity = app.world.spawn((
            Transform::from_rotation(Quat::from_rotation_y(0.0)),
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
        
        // Create enemy at test position
        let enemy_entity = app.world.spawn((
            Transform::from_translation(test_case.enemy_position),
            GlobalTransform::default(),
            Enemy,
        )).id();
        
        app.world.entity_mut(mech_entity).insert(AttackTarget { entity: enemy_entity });
        
        // Run frame
        app.update();
        
        let turret_rotation = app.world.get::<TurretRotation>(turret_entity).unwrap();
        
        println!("{}: target angle = {}", test_case.description, turret_rotation.target_angle);
        
        // Check if angle is approximately correct
        let angle_diff = (turret_rotation.target_angle - test_case.expected_angle).abs();
        let angle_diff = if angle_diff > 180.0 { 360.0 - angle_diff } else { angle_diff };
        
        assert!(
            angle_diff < 1.0,
            "{}: Expected angle {}, got {}",
            test_case.description,
            test_case.expected_angle,
            turret_rotation.target_angle
        );
    }
}

#[test]
fn test_turret_forward_direction_calculation() {
    // Test various rotations and their resulting forward directions
    struct TestCase {
        rotation_angle: f32,
        expected_forward: Vec2,
        description: &'static str,
    }
    
    let test_cases = vec![
        TestCase {
            rotation_angle: 0.0,
            expected_forward: Vec2::new(0.0, 1.0),  // +Z in 3D becomes (0,1) in 2D
            description: "0 degrees - facing forward (+Z)",
        },
        TestCase {
            rotation_angle: 90.0,
            expected_forward: Vec2::new(1.0, 0.0), // +X (right)
            description: "90 degrees - facing right (+X)",
        },
        TestCase {
            rotation_angle: 180.0,
            expected_forward: Vec2::new(0.0, -1.0), // -Z
            description: "180 degrees - facing backward (-Z)",
        },
        TestCase {
            rotation_angle: 270.0,
            expected_forward: Vec2::new(-1.0, 0.0),  // -X (left)
            description: "270 degrees - facing left (-X)",
        },
    ];
    
    for test_case in test_cases {
        let transform = Transform::from_rotation(Quat::from_rotation_y(test_case.rotation_angle.to_radians()));
        let forward_direction = get_turret_forward_direction(&transform);
        
        println!("{}: forward = {:?}", test_case.description, forward_direction);
        
        let diff = (forward_direction - test_case.expected_forward).length();
        assert!(
            diff < 0.01,
            "{}: Expected forward {:?}, got {:?}",
            test_case.description,
            test_case.expected_forward,
            forward_direction
        );
    }
}

// Test version of auto_fire_system that fires immediately for testing
fn test_auto_fire_system(
    mut commands: Commands,
    hero_query: Query<(&Transform, &Children, &AttackTarget), With<Hero>>,
    upper_query: Query<(&Transform, &TurretCannon, &TurretRotation, Option<&Children>), With<MechUpperPart>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use rust_and_ruin::systems::mech_assembly::get_barrel_tip_position;
    use rust_and_ruin::systems::turret_control::is_turret_facing_target;
    use bevy_rapier3d::prelude::*;
    
    const ATTACK_RANGE: f32 = 10.0;
    const ANGLE_TOLERANCE: f32 = 5.0;
    const TANK_SHELL_SPEED: f32 = 15.0;
    const TANK_SHELL_RANGE: f32 = 15.0;
    
    for (hero_transform, children, attack_target) in hero_query.iter() {
        if let Ok(enemy_transform) = enemy_query.get(attack_target.entity) {
            let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.z);
            let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
            let distance = hero_pos.distance(enemy_pos);
            
            if distance <= ATTACK_RANGE {
                for child in children {
                    if let Ok((upper_transform, turret_cannon, _turret_rotation, _upper_children)) = upper_query.get(*child) {
                        let global_upper_transform = hero_transform.mul_transform(*upper_transform);
                        let turret_position = Vec2::new(global_upper_transform.translation.x, global_upper_transform.translation.z);
                        let is_facing = is_turret_facing_target(&global_upper_transform, turret_position, enemy_pos, ANGLE_TOLERANCE);
                        
                        println!("Test auto_fire: turret_pos={:?}, enemy_pos={:?}, is_facing={}", 
                               turret_position, enemy_pos, is_facing);
                        
                        if is_facing {
                            let spawn_pos_3d = get_barrel_tip_position(&global_upper_transform, turret_cannon.barrel_length);
                            let projectile_spawn_pos = Vec2::new(spawn_pos_3d.x, spawn_pos_3d.z);
                            
                            let direction = (enemy_pos - projectile_spawn_pos).normalize();
                            let shell_velocity = direction * TANK_SHELL_SPEED;
                            
                            let shell_mesh = meshes.add(Mesh::from(shape::Box::new(0.4, 0.2, 0.4)));
                            let shell_material = materials.add(Color::rgb(1.0, 1.0, 0.0).into());
                            
                            commands.spawn((
                                Projectile {
                                    damage: turret_cannon.projectile_damage,
                                    speed: TANK_SHELL_SPEED,
                                },
                                TankShell {
                                    velocity: shell_velocity,
                                    spawn_position: projectile_spawn_pos,
                                    max_range: TANK_SHELL_RANGE,
                                },
                                PbrBundle {
                                    mesh: shell_mesh,
                                    material: shell_material,
                                    transform: Transform::from_xyz(projectile_spawn_pos.x, 0.75, projectile_spawn_pos.y),
                                    ..default()
                                },
                                RigidBody::Dynamic,
                                Collider::ball(0.2),
                                ColliderMassProperties::Density(10.0),
                                Restitution::coefficient(0.4),
                                Friction::coefficient(0.3),
                                Ccd::enabled(),
                                Velocity {
                                    linvel: Vec3::new(shell_velocity.x, 0.0, shell_velocity.y),
                                    angvel: Vec3::ZERO,
                                },
                                ExternalImpulse::default(),
                                GravityScale(0.3),
                                ActiveEvents::COLLISION_EVENTS
                            ));
                        }
                        break;
                    }
                }
            }
        }
    }
}

#[test]
fn test_turret_only_fires_when_facing_target() {
    use bevy::scene::SceneSpawner;
    
    // Test cases with turret at different angles relative to enemy
    struct TestCase {
        turret_angle: f32,
        enemy_position: Vec3,
        should_fire: bool,
        description: &'static str,
    }
    
    let test_cases = vec![
        TestCase {
            turret_angle: 90.0,  // Facing right
            enemy_position: Vec3::new(10.0, 0.0, 0.0), // Enemy to the right
            should_fire: true,
            description: "Turret facing enemy directly",
        },
        TestCase {
            turret_angle: 270.0,  // Facing left
            enemy_position: Vec3::new(10.0, 0.0, 0.0), // Enemy to the right
            should_fire: false,
            description: "Turret facing opposite direction",
        },
        TestCase {
            turret_angle: 0.0,  // Facing forward (+Z)
            enemy_position: Vec3::new(10.0, 0.0, 0.0), // Enemy to the right
            should_fire: false,
            description: "Turret facing perpendicular to enemy",
        },
        TestCase {
            turret_angle: 85.0,  // Almost facing right (5 degrees off)
            enemy_position: Vec3::new(10.0, 0.0, 0.0), // Enemy to the right
            should_fire: true,  // Within 5 degree tolerance
            description: "Turret within angle tolerance",
        },
        TestCase {
            turret_angle: 80.0,  // 10 degrees off from right
            enemy_position: Vec3::new(10.0, 0.0, 0.0), // Enemy to the right
            should_fire: false,  // Outside 5 degree tolerance
            description: "Turret outside angle tolerance",
        },
    ];
    
    for test_case in test_cases {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, TransformPlugin, HierarchyPlugin, bevy::log::LogPlugin::default()));
        app.insert_resource(MouseWorldPosition { position: Vec2::ZERO });
        app.insert_resource(Time::<()>::default());
        app.insert_resource(Assets::<Mesh>::default());
        app.insert_resource(Assets::<StandardMaterial>::default());
        app.insert_resource(Assets::<bevy::scene::Scene>::default());
        app.insert_resource(SceneSpawner::default());
        
        // Add the test auto fire system (without timer)
        app.add_systems(Update, (
            bevy::transform::systems::propagate_transforms,
            test_auto_fire_system,
        ).chain());
        
        // Create hero/mech at origin
        let mech_entity = app.world.spawn((
            Transform::from_translation(Vec3::ZERO),
            GlobalTransform::default(),
            Hero,
        )).id();
        
        // Create turret with specific rotation
        let turret_entity = app.world.spawn((
            Transform::from_rotation(Quat::from_rotation_y(test_case.turret_angle.to_radians())),
            GlobalTransform::default(),
            MechUpperPart,
            TurretRotation {
                current_angle: test_case.turret_angle,
                target_angle: test_case.turret_angle, // Simulating turret at rest
            },
            TurretCannon {
                fire_rate: 0.1, // Fast fire rate for testing
                projectile_damage: 10.0,
                rotation_speed: 360.0,
                barrel_length: 2.0,
            },
        )).id();
        
        // Set up parent-child relationship
        app.world.entity_mut(turret_entity).set_parent(mech_entity);
        
        // Create enemy
        let enemy_entity = app.world.spawn((
            Transform::from_translation(test_case.enemy_position),
            GlobalTransform::default(),
            Enemy,
        )).id();
        
        // Give mech an attack target
        app.world.entity_mut(mech_entity).insert(AttackTarget { entity: enemy_entity });
        
        
        // Count projectiles before update
        let projectiles_before = app.world.query::<&Projectile>().iter(&app.world).count();
        
        // For testing, we need to manually set the timer since Local<f32> starts at 0
        // and we can't easily control time advancement in tests
        
        // Run update to trigger auto fire system
        app.update();
        
        // Count projectiles after update
        let projectiles_after = app.world.query::<&Projectile>().iter(&app.world).count();
        
        let fired = projectiles_after > projectiles_before;
        
        // Debug logging
        println!("=== Test case: {} ===", test_case.description);
        println!("Projectiles before: {}, after: {}", projectiles_before, projectiles_after);
        
        // Check what components the mech has
        if app.world.get::<Hero>(mech_entity).is_some() {
            println!("Mech has Hero component");
        }
        if app.world.get::<AttackTarget>(mech_entity).is_some() {
            println!("Mech has AttackTarget component");
        }
        
        // Check turret components
        if let Some(turret_transform) = app.world.get::<Transform>(turret_entity) {
            let forward = get_turret_forward_direction(&turret_transform);
            println!("Turret forward direction: {:?}", forward);
        }
        if app.world.get::<MechUpperPart>(turret_entity).is_some() {
            println!("Turret has MechUpperPart component");
        }
        if app.world.get::<TurretCannon>(turret_entity).is_some() {
            println!("Turret has TurretCannon component");
        }
        if app.world.get::<TurretRotation>(turret_entity).is_some() {
            println!("Turret has TurretRotation component");
        }
        if app.world.get::<Children>(turret_entity).is_some() {
            println!("Turret has Children component");
        } else {
            println!("Turret MISSING Children component!");
        }
        
        println!("{}: turret_angle={}, enemy_pos={:?}, fired={}, expected={}",
                 test_case.description,
                 test_case.turret_angle,
                 test_case.enemy_position,
                 fired,
                 test_case.should_fire);
        
        assert_eq!(
            fired, test_case.should_fire,
            "{}: Expected firing={}, but got={}",
            test_case.description,
            test_case.should_fire,
            fired
        );
    }
}