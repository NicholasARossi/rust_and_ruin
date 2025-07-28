use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod camera;
mod components;
mod mech;
mod rendering;
mod resources;
mod systems;

use components::*;
use mech::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust and Ruin".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<GameState>()
        .init_resource::<MouseWorldPosition>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mouse_position_system,
            click_to_move_system,
            enemy_selection_system,
            update_target_indicator_system,
            attack_move_system,
            turret_control_system,
            // spawn_projectile_system, // Disabled - using auto_fire_system instead
            auto_fire_system,
            movement_system,
            rocket_acceleration_system,
            tank_shell_movement_system,  // Update tank shell positions
            projectile_lifetime_system,
            tank_shell_lifetime_system,
            collision_detection_system,
        ))
        .run();
}

use crate::mech::{MechLowerPart, MechUpperPart, TurretRotation, TurretCannon, CannonBarrel};

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    rapier_config.gravity = Vec2::ZERO;
    
    camera::setup_orthographic_camera(&mut commands);
    
    // Add a ground plane for reference
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
        transform: Transform::from_xyz(0.0, -0.1, 0.0),
        ..default()
    });
    
    // Spawn hero mech with 3D shapes
    let hero_entity = commands.spawn((
        Hero,
        SpatialBundle {
            transform: Transform::from_xyz(-4.0, 0.0, 0.0),
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
    commands.entity(hero_entity).push_children(&[tank_base, turret_base]);
    commands.entity(turret_base).push_children(&[cannon]);
    
    // Spawn enemy
    commands.spawn((
        Enemy,
        Health::new(100.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.5 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(4.0, 0.75, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.75, 0.75),
    ));
    
    info!("Spawned 3D mech and enemy");
}
