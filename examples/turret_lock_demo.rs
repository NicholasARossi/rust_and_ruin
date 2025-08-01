use bevy::prelude::*;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy_rapier3d::prelude::*;
use rust_and_ruin::mech::*;
use rust_and_ruin::components::*;
use rust_and_ruin::systems::*;
use rust_and_ruin::resources::*;

// Simple mech spawn function for demo using new components
fn spawn_mech(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    rotation: f32,
) -> Entity {
    // Create main mech entity with hierarchy tracking, lower body stats, and visual
    let mech_entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(1.5, 0.375, 2.25).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.3, 0.3),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_y(rotation)),
            ..default()
        },
        Mech::new("DemoMech"),
        MechHierarchy::new(),
        create_tank_treads_lower(),  // MechLowerBody goes on main entity
    )).id();
    
    // Create upper body (turret) with hardpoints
    let upper_entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(1.125, 0.375, 1.125).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.6, 0.0),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.1, 0.0)),
            ..default()
        },
        create_turret_upper(),
        MechRotation {
            current_angle: 0.0,
            target_angle: 0.0,
        },
        // Add old components for compatibility
        TurretRotation {
            current_angle: 0.0,
            target_angle: 0.0,
        },
        TurretCannon::default(),
    )).id();
    
    // Create weapon (cannon) mounted on main hardpoint
    let weapon_entity = commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(0.225, 0.225, 1.5).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.2, 0.2, 0.2),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.025, 0.75)),
            ..default()
        },
        create_cannon_weapon("main".to_string()),
        CannonWeapon::default(),
    )).id();
    
    // Set up hierarchy - upper body is direct child of mech
    commands.entity(upper_entity).push_children(&[weapon_entity]);
    commands.entity(mech_entity).push_children(&[upper_entity]);
    
    // Update MechHierarchy
    commands.entity(mech_entity).insert(MechHierarchy {
        lower: None,  // No separate lower entity, it's part of the main mech
        upper: Some(upper_entity),
        weapons: vec![weapon_entity],
    });
    
    mech_entity
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
        .add_systems(Startup, setup)
        .add_systems(Update, (
            bevy::transform::systems::propagate_transforms,
            input::mouse_position_system,
            input::click_to_move_system,
            input::enemy_selection_system,
            input::update_target_indicator_system,
            // Use new systems directly
            mech_movement_system,
            movement::movement_system,
            upper_body_control_system,
            weapon_control_system,
            projectile::tank_shell_movement_system,
            projectile::tank_shell_lifetime_system,
            camera_zoom_system,
            debug_info_system,
        ).chain())
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
    
    // Add Hero and MechMovement components to mech
    commands.entity(mech_entity).insert((
        Hero,
        MechMovement::default(),
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
    
    // Spawn enemy at (5, 0, 5) - round target that doesn't move or get destroyed
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
        RigidBody::Fixed,  // Won't move from impacts
        Collider::ball(0.75),  // Collision shape
        ColliderMassProperties::Density(1000.0),  // Very heavy
        Restitution::coefficient(0.8),  // Bouncy for shell impacts
        Friction::coefficient(0.1),
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
    hero_query: Query<(&Transform, Option<&AttackTarget>, Option<&MechMovement>, Option<&MechLowerBody>, &Children), With<Hero>>,
    upper_query: Query<(&Transform, &MechRotation, &Parent), With<MechUpperBody>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut text_query: Query<&mut Text>,
    zoom_level: Res<ZoomLevel>,
) {
    if let Ok((hero_transform, attack_target, mech_movement, mech_lower, children)) = hero_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            let mut status = String::from("Press Q near enemy to lock turret\nLeft click to move tank\nMouse wheel or -/= or [/] to zoom\n\n");
            
            status.push_str(&format!("Mech Position: ({:.1}, {:.1})\n", 
                hero_transform.translation.x, 
                hero_transform.translation.z));
            
            // Add mech movement state info
            if let Some(movement) = mech_movement {
                let max_speed = mech_lower.map(|l| l.movement_stats.max_speed).unwrap_or(5.0);
                
                status.push_str(&format!("Mech State: {:?}, Speed: {:.1}/{:.1}\n", 
                    movement.movement_state,
                    movement.current_speed,
                    max_speed));
                status.push_str(&format!("Mech Rotation: current={:.1}°, target={:.1}°\n",
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
                
                // Find upper body info from children
                for child in children {
                    if let Ok((upper_transform, rotation, _)) = upper_query.get(*child) {
                        status.push_str(&format!("Upper Body Angle: {:.1}° (Target: {:.1}°)\n",
                            rotation.current_angle,
                            rotation.target_angle));
                        
                        // Calculate if upper body is facing target
                        let global_upper = hero_transform.mul_transform(*upper_transform);
                        let upper_pos = Vec2::new(global_upper.translation.x, global_upper.translation.z);
                        
                        if let Ok(enemy_transform) = enemy_query.get(attack_target.entity) {
                            let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
                            let is_facing = is_upper_facing_target(
                                upper_transform,
                                upper_pos,
                                enemy_pos,
                                5.0,
                            );
                            status.push_str(&format!("Facing Target: {}\n", if is_facing { "YES" } else { "NO" }));
                        }
                        break;
                    }
                }
            } else {
                status.push_str("Turret Status: No Target\n");
            }
            
            text.sections[0].value = status;
        }
    }
}