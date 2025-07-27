use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::{Hero, Enemy, Projectile, Rocket};
use crate::resources::MouseWorldPosition;

const ROCKET_INITIAL_SPEED: f32 = 50.0;
const ROCKET_MAX_SPEED: f32 = 800.0;
const ROCKET_ACCELERATION_RATE: f32 = 2.5;

pub fn spawn_projectile_system(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    _mouse_world_pos: Res<MouseWorldPosition>,
    hero_query: Query<&Transform, With<Hero>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    if mouse_button.just_pressed(MouseButton::Right) {
        info!("Right mouse button pressed!");
        
        if let Ok(hero_transform) = hero_query.get_single() {
            if let Ok(enemy_transform) = enemy_query.get_single() {
                let hero_pos = hero_transform.translation.truncate();
                let enemy_pos = enemy_transform.translation.truncate();
                
                let direction = (enemy_pos - hero_pos).normalize();
                let initial_velocity = direction * ROCKET_INITIAL_SPEED;
                
                info!("Spawning rocket at {:?} with initial velocity {:?}", hero_pos, initial_velocity);
                
                commands.spawn((
                    Projectile {
                        damage: 10.0,
                        speed: ROCKET_INITIAL_SPEED,
                    },
                    Rocket {
                        initial_speed: ROCKET_INITIAL_SPEED,
                        max_speed: ROCKET_MAX_SPEED,
                        acceleration_rate: ROCKET_ACCELERATION_RATE,
                        current_speed: ROCKET_INITIAL_SPEED,
                        direction,
                    },
                    SpriteBundle {
                        transform: Transform::from_translation(hero_pos.extend(1.0)), // Higher z for visibility
                        sprite: Sprite {
                            color: Color::rgb(1.0, 0.5, 0.0), // Orange for rocket
                            custom_size: Some(Vec2::new(20.0, 20.0)), // Larger size
                            ..default()
                        },
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Collider::ball(10.0),
                    Velocity {
                        linvel: initial_velocity,
                        angvel: 0.0,
                    },
                ));
            } else {
                warn!("No enemy found to target!");
            }
        } else {
            warn!("No hero found to spawn projectile from!");
        }
    }
}

pub fn projectile_lifetime_system(
    mut commands: Commands,
    _time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Projectile>>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.y < -1000.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn rocket_acceleration_system(
    time: Res<Time>,
    mut query: Query<(&mut Rocket, &mut Velocity), With<Projectile>>,
) {
    for (mut rocket, mut velocity) in query.iter_mut() {
        if rocket.current_speed < rocket.max_speed {
            rocket.current_speed *= 1.0 + (rocket.acceleration_rate * time.delta_seconds());
            rocket.current_speed = rocket.current_speed.min(rocket.max_speed);
            
            velocity.linvel = rocket.direction * rocket.current_speed;
        }
    }
}