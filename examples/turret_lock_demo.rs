use bevy::prelude::*;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy_rapier3d::prelude::*;
use rust_and_ruin::mech::*;
use rust_and_ruin::components::*;
use rust_and_ruin::systems::*;
use rust_and_ruin::resources::*;
use rust_and_ruin::systems::attack_target_propagation::propagate_attack_target_system;
use rand::Rng;

// Resource to signal enemy destruction and request respawn
#[derive(Resource)]
struct EnemyRespawnRequest {
    should_respawn: bool,
    respawn_timer: Timer,
}

impl Default for EnemyRespawnRequest {
    fn default() -> Self {
        Self {
            should_respawn: false,
            respawn_timer: Timer::from_seconds(2.0, TimerMode::Once),
        }
    }
}

// Enum for enemy shapes
#[derive(Clone, Copy)]
enum EnemyShape {
    Sphere,
    Cube,
    Cone,
}

// Custom enemy selection system that adds AttackTarget to tank_base instead of hero
fn demo_enemy_selection_system(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_world_pos: Res<MouseWorldPosition>,
    hero_query: Query<(Entity, &Transform, &Children), With<Hero>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    existing_indicators: Query<Entity, With<TargetIndicator>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Support both right-click and Q key for targeting
    if mouse_button.just_pressed(MouseButton::Right) || keyboard_input.just_pressed(KeyCode::Q) {
        let click_pos = mouse_world_pos.position;
        
        // Find the closest enemy within a reasonable distance
        let mut closest_enemy = None;
        let mut closest_distance = f32::MAX;
        const SELECTION_RADIUS: f32 = 2.0;
        
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let enemy_pos_2d = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
            let distance = enemy_pos_2d.distance(click_pos);
            
            if distance < SELECTION_RADIUS && distance < closest_distance {
                closest_distance = distance;
                closest_enemy = Some(enemy_entity);
            }
        }
        
        // If we found an enemy, set it as the attack target for all tank_bases
        if let Some(target_entity) = closest_enemy {
            info!("Selected enemy at distance: {}", closest_distance);
            
            // Remove any existing target indicators
            for indicator in existing_indicators.iter() {
                commands.entity(indicator).despawn();
            }
            
            // Spawn a visual indicator for the selected enemy
            if let Ok((_, enemy_transform)) = enemy_query.get(target_entity) {
                commands.spawn((
                    TargetIndicator {
                        target: target_entity,
                    },
                    PbrBundle {
                        mesh: meshes.add(shape::Torus {
                            radius: 1.0,
                            ring_radius: 0.1,
                            subdivisions_segments: 24,
                            subdivisions_sides: 12,
                        }.into()),
                        material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
                        transform: Transform::from_xyz(
                            enemy_transform.translation.x,
                            0.1,
                            enemy_transform.translation.z
                        ),
                        ..default()
                    },
                ));
            }
            
            // Add AttackTarget to hero entities (as expected by the game systems)
            for (hero_entity, _, _) in hero_query.iter() {
                // Remove any existing attack target
                commands.entity(hero_entity).remove::<AttackTarget>();
                // Add the new attack target to hero
                commands.entity(hero_entity).insert(AttackTarget {
                    entity: target_entity,
                });
            }
        }
    }
}

// Simple mech spawn function for demo using old components like main.rs
fn spawn_mech(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    rotation: f32,
) -> Entity {
    // Create main mech entity (just transform, no visual)
    let mech_entity = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_y(rotation)),
            ..default()
        },
    )).id();
    
    // Tank base (box shape)
    let tank_base = commands.spawn((
        MechLowerPart,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.5, 0.5, 2.0))),
            material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
            transform: Transform::from_xyz(0.0, 0.25, 0.0),
            ..default()
        },
    )).id();
    
    // Turret base (cylinder)
    let turret_base = commands.spawn((
        MechUpperPart,
        TurretRotation {
            target_angle: 0.0,
            current_angle: 0.0,
        },
        TurretCannon::default(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.5,
                height: 0.4,
                resolution: 16,
                segments: 1,
            })),
            material: materials.add(Color::rgb(0.2, 0.6, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    )).id();
    
    // Cannon barrel (box)
    let cannon = commands.spawn((
        CannonBarrel,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.2, 0.2, 1.0))),
            material: materials.add(Color::rgb(0.6, 0.6, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.6),
            ..default()
        },
    )).id();
    
    // Set up hierarchy
    commands.entity(mech_entity).push_children(&[tank_base]);
    commands.entity(tank_base).push_children(&[turret_base]);
    commands.entity(turret_base).push_children(&[cannon]);
    
    mech_entity
}

// Clear attack targets when the targeted enemy no longer exists
fn clear_invalid_attack_targets_system(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    attack_target_query: Query<(Entity, &AttackTarget)>,
    indicator_query: Query<(Entity, &TargetIndicator)>,
) {
    for (entity, attack_target) in attack_target_query.iter() {
        // Check if the target entity still exists as an enemy
        if enemy_query.get(attack_target.entity).is_err() {
            // Enemy no longer exists, remove the attack target
            commands.entity(entity).remove::<AttackTarget>();
            
            // Also remove any target indicators for this enemy
            for (indicator_entity, indicator) in indicator_query.iter() {
                if indicator.target == attack_target.entity {
                    commands.entity(indicator_entity).despawn();
                }
            }
        }
    }
}

// Monitor enemy health and request respawn when destroyed
fn enemy_health_monitor_system(
    time: Res<Time>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut respawn_request: ResMut<EnemyRespawnRequest>,
) {
    // If no enemies exist at all, start the respawn timer
    if enemy_query.iter().count() == 0 && !respawn_request.should_respawn {
        respawn_request.respawn_timer.reset();
        respawn_request.should_respawn = true;
    }
    
    // Tick the timer
    if respawn_request.should_respawn {
        respawn_request.respawn_timer.tick(time.delta());
    }
}

// Respawn enemies with random shapes and positions
fn enemy_respawn_system(
    mut commands: Commands,
    mut respawn_request: ResMut<EnemyRespawnRequest>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !respawn_request.should_respawn || !respawn_request.respawn_timer.finished() {
        return;
    }
    
    respawn_request.should_respawn = false;
    
    let mut rng = rand::thread_rng();
    
    // Random position within bounds
    let x = rng.gen_range(-15.0..15.0);
    let z = rng.gen_range(-15.0..15.0);
    let position = Vec3::new(x, 0.75, z);
    
    // Random shape
    let shape = match rng.gen_range(0..3) {
        0 => EnemyShape::Sphere,
        1 => EnemyShape::Cube,
        _ => EnemyShape::Cone,
    };
    
    // Create mesh based on shape
    let mesh = match shape {
        EnemyShape::Sphere => meshes.add(shape::UVSphere {
            radius: 0.75,
            sectors: 32,
            stacks: 16,
        }.into()),
        EnemyShape::Cube => meshes.add(shape::Box::new(1.5, 1.5, 1.5).into()),
        EnemyShape::Cone => meshes.add(shape::Torus {
            radius: 0.75,
            ring_radius: 0.35,
            subdivisions_segments: 32,
            subdivisions_sides: 16,
        }.into()),
    };
    
    // Spawn new enemy
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.0, 0.0),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(position),
            ..default()
        },
        Enemy,
        Health::new(100.0),  // Lower health for quicker gameplay
        RigidBody::Dynamic,
        Collider::ball(0.75),  // Keep same collision size for all shapes
        ColliderMassProperties::Density(10.0),
        Restitution::coefficient(0.4),
        Friction::coefficient(0.3),
        ExternalImpulse::default(),
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS,
    ));
    
    info!("Spawned new enemy with shape {:?} at position {:?}", 
          match shape {
              EnemyShape::Sphere => "Sphere",
              EnemyShape::Cube => "Cube", 
              EnemyShape::Cone => "Cone",
          }, 
          position);
}

// Zoom control resource
#[derive(Resource)]
struct ZoomLevel {
    current: f32,
    min: f32,
    max: f32,
    speed: f32,
}

impl Default for ZoomLevel {
    fn default() -> Self {
        Self {
            current: 0.1,   // Default zoom level
            min: 0.02,      // Zoomed in (much closer view)
            max: 0.3,       // Zoomed out (further view)
            speed: 0.01,    // Zoom speed
        }
    }
}

// Camera zoom system
fn camera_zoom_system(
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    keyboard_input: Res<Input<KeyCode>>,
    mut zoom_level: ResMut<ZoomLevel>,
    mut camera_query: Query<&mut Projection, With<Camera3d>>,
) {
    let mut zoom_delta = 0.0;
    
    // Mouse wheel zoom
    for event in scroll_events.read() {
        zoom_delta -= event.y * zoom_level.speed;
    }
    
    // Keyboard zoom - zoom in with = or ] keys
    if keyboard_input.just_pressed(KeyCode::Equals) 
        || keyboard_input.just_pressed(KeyCode::NumpadAdd)
        || keyboard_input.just_pressed(KeyCode::BracketRight) {
        zoom_delta -= zoom_level.speed;
    }
    
    // Keyboard zoom - zoom out with - or [ keys
    if keyboard_input.just_pressed(KeyCode::Minus) 
        || keyboard_input.just_pressed(KeyCode::NumpadSubtract)
        || keyboard_input.just_pressed(KeyCode::BracketLeft) {
        zoom_delta += zoom_level.speed;
    }
    
    // Continuous zoom when holding keys
    if keyboard_input.pressed(KeyCode::Equals) 
        || keyboard_input.pressed(KeyCode::NumpadAdd)
        || keyboard_input.pressed(KeyCode::BracketRight) {
        zoom_delta -= zoom_level.speed * 0.5;
    }
    if keyboard_input.pressed(KeyCode::Minus) 
        || keyboard_input.pressed(KeyCode::NumpadSubtract)
        || keyboard_input.pressed(KeyCode::BracketLeft) {
        zoom_delta += zoom_level.speed * 0.5;
    }
    
    // Apply zoom changes
    if zoom_delta != 0.0 {
        zoom_level.current = (zoom_level.current + zoom_delta).clamp(zoom_level.min, zoom_level.max);
        
        for mut projection in camera_query.iter_mut() {
            if let Projection::Orthographic(ref mut ortho) = projection.as_mut() {
                ortho.scale = zoom_level.current;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .insert_resource(MouseWorldPosition { position: Vec2::ZERO })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .init_resource::<ZoomLevel>()
        .init_resource::<EnemyRespawnRequest>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            (
                bevy::transform::systems::propagate_transforms,
                input::mouse_position_system,
                input::click_to_move_system,
                demo_enemy_selection_system,  // Use custom version that adds AttackTarget to hero
                input::update_target_indicator_system,
                movement::attack_move_system,
                propagate_attack_target_system,  // Propagate AttackTarget down hierarchy
            ).chain(),
            (
                // Use proper turret and firing systems
                turret_control_system,
                projectile::auto_fire_system,
            ).chain(),
            (
                movement::movement_system,
                tank_movement_system,
                projectile::rocket_acceleration_system,
                projectile::tank_shell_movement_system,
                projectile::projectile_lifetime_system,
                projectile::tank_shell_lifetime_system,
            ).chain(),
            (
                collision_detection_system,
                visual_effects::hit_flash_system,
                visual_effects::fragment_lifetime_system,
                visual_effects::fragment_visual_fade_system,
                clear_invalid_attack_targets_system,
                enemy_health_monitor_system,
                enemy_respawn_system,
            ).chain(),
            camera_zoom_system,
            debug_info_system,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera with orthographic projection
    let camera_angle = 63.435_f32.to_radians();
    let camera_distance = 50.0;
    let camera_height = camera_distance * camera_angle.sin();
    let camera_horizontal = camera_distance * camera_angle.cos();
    
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 0.1,
            ..default()
        }.into(),
        transform: Transform::from_xyz(0.0, camera_height, camera_horizontal)
            .looking_at(Vec3::ZERO, Vec3::Y),
        tonemapping: Tonemapping::None,
        ..default()
    });
    
    // Add lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -45.0_f32.to_radians(),
            -45.0_f32.to_radians(),
            0.0,
        )),
        ..default()
    });
    
    // Spawn mech at center
    let mech_entity = spawn_mech(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::ZERO,
        0.0,
    );
    
    // Add Hero and movement components to mech
    commands.entity(mech_entity).insert((
        Hero,
        TankMovement::default(),
    ));
    
    // Add ground plane for shells to bounce on
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.2, 0.2),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.01, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(25.0, 0.005, 25.0),
        Friction::coefficient(0.8),
        Restitution::coefficient(0.2),
    ));
    
    // Spawn enemy at (5, 0, 5) - round target with health
    let _enemy_entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::UVSphere {
                radius: 0.75,
                sectors: 32,
                stacks: 16,
            }.into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.0, 0.0),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(5.0, 0.75, 5.0),  // Raised to match shell height
            ..default()
        },
        Enemy,
        Health::new(100.0),  // Lower health for quicker testing
        RigidBody::Dynamic,  // Will move from impacts
        Collider::ball(0.75),  // Collision shape
        ColliderMassProperties::Density(10.0),  // Heavy but not immovable
        Restitution::coefficient(0.4),  // Some bounce
        Friction::coefficient(0.3),
        ExternalImpulse::default(),
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,  // Don't spin
        ActiveEvents::COLLISION_EVENTS,
    )).id();
    
    // UI text for debug info
    commands.spawn(
        TextBundle::from_section(
            "Press Q near enemy to lock turret\nLeft click to move tank\nMouse wheel or -/= or [/] to zoom\nTurret Status: No Target",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
    
    info!("Turret Lock Demo Started");
    info!("- Press Q near the red enemy to lock turret");
    info!("- Left click to move the tank");
    info!("- Use mouse wheel or -/= keys (or [/] keys) to zoom in/out");
    info!("- Watch how the turret tracks the enemy while moving");
}

fn debug_info_system(
    hero_query: Query<(&Transform, Option<&AttackTarget>, Option<&TankMovement>, &Children), With<Hero>>,
    children_query: Query<&Children>,
    turret_query: Query<(&Transform, &TurretRotation, &TurretCannon), With<TurretCannon>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut text_query: Query<&mut Text>,
    zoom_level: Res<ZoomLevel>,
) {
    if let Ok((hero_transform, attack_target, tank_movement, children)) = hero_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            let mut status = String::from("Press Q near enemy to lock turret\nLeft click to move tank\nMouse wheel or -/= or [/] to zoom\n\n");
            
            status.push_str(&format!("Mech Position: ({:.1}, {:.1})\n", 
                hero_transform.translation.x, 
                hero_transform.translation.z));
            
            // Add tank movement state info
            if let Some(movement) = tank_movement {
                status.push_str(&format!("Tank State: {:?}, Speed: {:.1}/{:.1}\n", 
                    movement.rotation_state,
                    movement.current_speed,
                    movement.max_speed));
                status.push_str(&format!("Tank Rotation: current={:.1}°, target={:.1}°\n",
                    hero_transform.rotation.to_euler(EulerRot::YXZ).0.to_degrees(),
                    movement.target_rotation));
            }
            
            status.push_str(&format!("Zoom Level: {:.2} (Min: {:.2}, Max: {:.2})\n", 
                zoom_level.current, 
                zoom_level.min, 
                zoom_level.max));
            
            if let Some(attack_target) = attack_target {
                status.push_str("Turret Status: LOCKED ON TARGET\n");
                
                if let Ok(enemy_transform) = enemy_query.get(attack_target.entity) {
                    status.push_str(&format!("Enemy Position: ({:.1}, {:.1})\n",
                        enemy_transform.translation.x,
                        enemy_transform.translation.z));
                }
                
                // Find turret in the hierarchy (hero -> tank_base -> turret_base)
                let mut found_turret = false;
                // First, find tank_base
                if let Some(&tank_base_entity) = children.iter().next() {
                    // Then get children of tank_base (which should include turret_base)
                    if let Ok(tank_children) = children_query.get(tank_base_entity) {
                        for turret_entity in tank_children {
                            if let Ok((turret_transform, turret_rotation, turret_cannon)) = turret_query.get(*turret_entity) {
                                found_turret = true;
                                status.push_str(&format!("Turret Angle: {:.1}° (Target: {:.1}°)\n",
                                    turret_rotation.current_angle,
                                    turret_rotation.target_angle));
                                
                                // Check firing conditions
                                if let Ok(enemy_transform) = enemy_query.get(attack_target.entity) {
                                    let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.z);
                                    let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
                                    let distance = hero_pos.distance(enemy_pos);
                                    
                                    status.push_str(&format!("Enemy Distance: {:.1} (Range: 10.0)\n", distance));
                                    status.push_str(&format!("Fire Rate: {} shots/sec\n", 1.0 / turret_cannon.fire_rate));
                                    
                                    if distance > 10.0 {
                                        status.push_str("Firing Status: OUT OF RANGE\n");
                                    } else {
                                        // Check if turret is facing target
                                        let angle_diff = (turret_rotation.current_angle - turret_rotation.target_angle).abs();
                                        if angle_diff > 5.0 && angle_diff < 355.0 {
                                            status.push_str(&format!("Firing Status: ROTATING (diff: {:.1}°)\n", angle_diff.min(360.0 - angle_diff)));
                                        } else {
                                            status.push_str("Firing Status: READY TO FIRE\n");
                                        }
                                    }
                                }
                                
                                // Calculate if turret is facing target
                                // Need to calculate global transform through hierarchy
                                let tank_transform = Transform::from_xyz(0.0, 0.25, 0.0); // tank's local transform
                                let turret_local = Transform::from_xyz(0.0, 0.5, 0.0); // turret's local transform
                                let global_turret = hero_transform
                                    .mul_transform(tank_transform)
                                    .mul_transform(turret_local)
                                    .mul_transform(Transform::from_rotation(turret_transform.rotation));
                                let turret_pos = Vec2::new(global_turret.translation.x, global_turret.translation.z);
                                
                                if let Ok(enemy_transform) = enemy_query.get(attack_target.entity) {
                                    let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
                                    let is_facing = is_turret_facing_target(
                                        &global_turret,
                                        turret_pos,
                                        enemy_pos,
                                        5.0,
                                    );
                                    status.push_str(&format!("Facing Target: {}\n", if is_facing { "YES" } else { "NO" }));
                                }
                                break;
                            }
                        }
                    }
                }
                
                if !found_turret {
                    status.push_str("Turret: Not found in hierarchy\n");
                }
            } else {
                status.push_str("Turret Status: No Target\n");
            }
            
            text.sections[0].value = status;
        }
    }
}