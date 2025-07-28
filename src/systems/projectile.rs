use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::{Hero, Enemy, Projectile, Rocket, TankShell, AttackTarget, Health};
use crate::resources::MouseWorldPosition;
use crate::rendering;
use crate::mech::{MechUpperPart, TurretCannon, CannonBarrel, TurretRotation};
use crate::systems::mech_assembly::get_barrel_tip_position;
use crate::systems::turret_control::shortest_angle_difference;

const ROCKET_INITIAL_SPEED: f32 = 0.5;
const ROCKET_MAX_SPEED: f32 = 8.0;
const ROCKET_ACCELERATION_RATE: f32 = 2.5;
const TANK_SHELL_SPEED: f32 = 8.0;  // Reduced from 20.0 for better visibility
const TANK_SHELL_RANGE: f32 = 15.0;
const ATTACK_RANGE: f32 = 10.0;  // Increased from 5.0 for easier testing
const ANGLE_TOLERANCE: f32 = 5.0; // degrees

pub fn spawn_projectile_system(
    mut commands: Commands,
    _mouse_button: Res<Input<MouseButton>>,
    _mouse_world_pos: Res<MouseWorldPosition>,
    hero_query: Query<(&Transform, &Children), With<Hero>>,
    upper_query: Query<(&Transform, &TurretCannon, &Children), With<MechUpperPart>>,
    _barrel_query: Query<&Transform, With<CannonBarrel>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Disabled - we now use auto_fire_system
    if false {
        info!("Right mouse button pressed!");
        
        if let Ok((hero_transform, children)) = hero_query.get_single() {
            if let Ok(enemy_transform) = enemy_query.get_single() {
                let mut projectile_spawn_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.z);
                let mut projectile_damage = 10.0;
                
                for child in children {
                    if let Ok((upper_transform, turret_cannon, _upper_children)) = upper_query.get(*child) {
                        projectile_damage = turret_cannon.projectile_damage;
                        
                        let global_upper_transform = hero_transform.mul_transform(*upper_transform);
                        let spawn_pos_3d = get_barrel_tip_position(&global_upper_transform, turret_cannon.barrel_length);
                        projectile_spawn_pos = Vec2::new(spawn_pos_3d.x, spawn_pos_3d.z);
                        break;
                    }
                }
                
                let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
                
                let direction = (enemy_pos - projectile_spawn_pos).normalize();
                let initial_velocity = direction * ROCKET_INITIAL_SPEED;
                
                info!("Spawning rocket at {:?} with initial velocity {:?}", projectile_spawn_pos, initial_velocity);
                
                let rocket_mesh = meshes.add(rendering::create_sprite_mesh(Vec2::new(0.2, 0.2)));
                let rocket_material = rendering::create_sprite_material(Color::rgb(1.0, 0.5, 0.0), &mut materials);
                
                commands.spawn((
                    Projectile {
                        damage: projectile_damage,
                        speed: ROCKET_INITIAL_SPEED,
                    },
                    Rocket {
                        initial_speed: ROCKET_INITIAL_SPEED,
                        max_speed: ROCKET_MAX_SPEED,
                        acceleration_rate: ROCKET_ACCELERATION_RATE,
                        current_speed: ROCKET_INITIAL_SPEED,
                        direction,
                    },
                    PbrBundle {
                        mesh: rocket_mesh,
                        material: rocket_material,
                        transform: Transform::from_xyz(projectile_spawn_pos.x, 0.05, projectile_spawn_pos.y),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Collider::ball(0.1),
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

pub fn auto_fire_system(
    mut commands: Commands,
    time: Res<Time>,
    hero_query: Query<(&Transform, &Children, &AttackTarget), With<Hero>>,
    upper_query: Query<(&Transform, &TurretCannon, &TurretRotation, &Children), With<MechUpperPart>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut last_fire_time: Local<f32>,
) {
    *last_fire_time += time.delta_seconds();
    
    for (hero_transform, children, attack_target) in hero_query.iter() {
        if let Ok(enemy_transform) = enemy_query.get(attack_target.entity) {
            let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.z);
            let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
            let distance = hero_pos.distance(enemy_pos);
            
            info!("Auto-fire check: distance={:.2}, range={:.2}, time={:.2}, fire_rate=1.0", distance, ATTACK_RANGE, *last_fire_time);
            
            // Check if enemy is in range
            if distance <= ATTACK_RANGE {
                for child in children {
                    if let Ok((upper_transform, turret_cannon, turret_rotation, _upper_children)) = upper_query.get(*child) {
                        // Check if turret is aimed at target (within tolerance)
                        let angle_diff = shortest_angle_difference(turret_rotation.current_angle, turret_rotation.target_angle).abs();
                        info!("Turret angle check: current={:.2}, target={:.2}, diff={:.2}, tolerance={:.2}", 
                              turret_rotation.current_angle, turret_rotation.target_angle, angle_diff, ANGLE_TOLERANCE);
                        if angle_diff <= ANGLE_TOLERANCE && *last_fire_time >= turret_cannon.fire_rate {
                            // Fire projectile
                            let global_upper_transform = hero_transform.mul_transform(*upper_transform);
                            let spawn_pos_3d = get_barrel_tip_position(&global_upper_transform, turret_cannon.barrel_length);
                            let projectile_spawn_pos = Vec2::new(spawn_pos_3d.x, spawn_pos_3d.z);
                            
                            let direction = (enemy_pos - projectile_spawn_pos).normalize();
                            let shell_velocity = direction * TANK_SHELL_SPEED;
                            
                            let shell_mesh = meshes.add(Mesh::from(shape::Box::new(0.4, 0.2, 0.4)));  // 3D box shape
                            let shell_material = materials.add(Color::rgb(1.0, 1.0, 0.0).into());  // Bright yellow
                            
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
                                    transform: Transform::from_xyz(projectile_spawn_pos.x, 0.75, projectile_spawn_pos.y),  // Same Y as enemy for 2D physics
                                    ..default()
                                },
                                RigidBody::KinematicPositionBased,  // Use kinematic to control position manually
                                Collider::ball(0.2),  // Increased from 0.05 for better collision detection
                            ));
                            
                            info!("Tank shell spawned at 3D pos ({}, {}, {}) with velocity {:?}", 
                                  projectile_spawn_pos.x, 0.75, projectile_spawn_pos.y, shell_velocity);
                            *last_fire_time = 0.0;
                        }
                        break;
                    }
                }
            }
        }
    }
}

pub fn tank_shell_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &TankShell, &Projectile)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Projectile>)>,
) {
    for (projectile_entity, mut projectile_transform, tank_shell, projectile) in projectile_query.iter_mut() {
        // Update position based on velocity (in X,Z plane)
        let delta = tank_shell.velocity * time.delta_seconds();
        projectile_transform.translation.x += delta.x;
        projectile_transform.translation.z += delta.y;  // velocity.y represents Z direction
        
        // Manual collision detection
        let projectile_pos = Vec2::new(projectile_transform.translation.x, projectile_transform.translation.z);
        
        for (enemy_entity, enemy_transform, mut enemy_health) in enemy_query.iter_mut() {
            let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
            let distance = projectile_pos.distance(enemy_pos);
            
            // Check if projectile hits enemy (0.2 projectile radius + 0.75 enemy half-size)
            if distance < 0.95 {
                // Hit!
                enemy_health.current -= projectile.damage;
                info!("Enemy hit! Damage: {}, Health: {}/{}", projectile.damage, enemy_health.current, enemy_health.max);
                
                // Despawn projectile
                commands.entity(projectile_entity).despawn();
                
                // Check if enemy is dead
                if enemy_health.current <= 0.0 {
                    commands.entity(enemy_entity).despawn();
                    info!("Enemy destroyed!");
                }
                
                break; // Projectile can only hit one enemy
            }
        }
    }
}

pub fn tank_shell_lifetime_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &TankShell), With<Projectile>>,
) {
    for (entity, transform, tank_shell) in query.iter() {
        let current_pos = Vec2::new(transform.translation.x, transform.translation.z);
        let distance_traveled = current_pos.distance(tank_shell.spawn_position);
        
        if distance_traveled >= tank_shell.max_range {
            commands.entity(entity).despawn();
        }
    }
}