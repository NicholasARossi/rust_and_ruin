use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod resources;
mod systems;

use components::*;
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
        .add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<GameState>()
        .init_resource::<MouseWorldPosition>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mouse_position_system,
            click_to_move_system,
            spawn_projectile_system,
            movement_system,
            rocket_acceleration_system,
            projectile_lifetime_system,
            collision_detection_system,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vec2::ZERO;
    
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn((
        Hero,
        SpriteBundle {
            transform: Transform::from_xyz(-400.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::rgb(0.0, 0.5, 1.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
    ));
    
    commands.spawn((
        Enemy,
        Health::new(100.0),
        SpriteBundle {
            transform: Transform::from_xyz(400.0, 0.0, 0.0),
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(60.0, 60.0)),
                ..default()
            },
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(30.0, 30.0),
    ));
}
