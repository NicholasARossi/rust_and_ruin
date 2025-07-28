use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Hero, Enemy, Projectile, Rocket, TankShell, AttackTarget};
use crate::resources::MouseWorldPosition;
use crate::rendering;
use crate::mech::{MechUpperPart, TurretCannon, CannonBarrel, TurretRotation};
use crate::systems::mech_assembly::get_barrel_tip_position;
use crate::systems::turret_control::shortest_angle_difference;

const ROCKET_INITIAL_SPEED: f32 = 0.5;
const ROCKET_MAX_SPEED: f32 = 8.0;
const ROCKET_ACCELERATION_RATE: f32 = 2.5;
const TANK_SHELL_SPEED: f32 = 15.0;  // Fast, impactful shells
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
                        linvel: Vec3::new(initial_velocity.x, 0.0, initial_velocity.y),
                        angvel: Vec3::ZERO,
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
            
            velocity.linvel = Vec3::new(rocket.direction.x * rocket.current_speed, 0.0, rocket.direction.y * rocket.current_speed);
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
                                    transform: Transform::from_xyz(projectile_spawn_pos.x, 0.75, projectile_spawn_pos.y),  // Y=0.75 for 3D physics at enemy height
                                    ..default()
                                },
                                RigidBody::Dynamic,
                                Collider::ball(0.2),  // Increased from 0.05 for better collision detection
                                ColliderMassProperties::Density(10.0),  // Heavy shells
                                Restitution::coefficient(0.4),  // Some bounce
                                Friction::coefficient(0.3),
                                Ccd::enabled(),  // Continuous collision detection for fast projectiles
                                Velocity {
                                    linvel: Vec3::new(shell_velocity.x, 0.0, shell_velocity.y),  // Convert 2D velocity to 3D
                                    angvel: Vec3::ZERO,
                                },
                                ExternalImpulse::default(),
                                GravityScale(0.3),  // Slight gravity for realistic arc
                                ActiveEvents::COLLISION_EVENTS
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
    projectile_query: Query<(&Velocity, &TankShell), With<Projectile>>,
) {
    // Physics are now handled by Rapier2D
    // This system just monitors shell velocity for effects
    for (velocity, _tank_shell) in projectile_query.iter() {
        let speed = velocity.linvel.length();
        if speed < 1.0 {
            // Shell is nearly stopped, could add effects here
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