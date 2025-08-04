use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Projectile, Enemy, Health, TankShell, HitFlash, FragmentShell, ShellFragment};
use crate::systems::visual_effects::{calculate_fragment_velocities, calculate_fragment_lifetime, calculate_fragment_max_distance};

pub fn collision_detection_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<(Entity, &Projectile, Option<&TankShell>, Option<&FragmentShell>, &Velocity, &Transform)>,
    mut enemy_query: Query<(Entity, &mut Health, &mut ExternalImpulse, &Transform), With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for collision_event in collision_events.read() {
        info!("Collision event detected: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let collision_data = 
                    if let Ok((proj_entity, projectile, tank_shell, fragment_shell, velocity, proj_transform)) = projectile_query.get(*entity1) {
                        if let Ok((enemy_entity, _, _, enemy_transform)) = enemy_query.get(*entity2) {
                            Some((proj_entity, projectile, tank_shell, fragment_shell, velocity, proj_transform, enemy_entity, enemy_transform))
                        } else {
                            None
                        }
                    } else if let Ok((proj_entity, projectile, tank_shell, fragment_shell, velocity, proj_transform)) = projectile_query.get(*entity2) {
                        if let Ok((enemy_entity, _, _, enemy_transform)) = enemy_query.get(*entity1) {
                            Some((proj_entity, projectile, tank_shell, fragment_shell, velocity, proj_transform, enemy_entity, enemy_transform))
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                
                if let Some((projectile_entity, projectile, tank_shell, fragment_shell, velocity, proj_transform, enemy_entity, enemy_transform)) = collision_data {
                    let projectile_damage = projectile.damage;
                    let projectile_velocity = velocity.linvel;
                    let is_tank_shell = tank_shell.is_some();
                    let is_fragment_shell = fragment_shell.is_some();
                
                    // Spawn fragments if this is a fragment shell
                    if is_fragment_shell && is_tank_shell {
                        spawn_fragments(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            proj_transform.translation,
                            Vec2::new(projectile_velocity.x, projectile_velocity.z),
                            enemy_transform.translation,
                            tank_shell.as_ref().map(|ts| ts.max_range).unwrap_or(15.0),
                            projectile_damage,
                        );
                        
                        // Always despawn fragment shells on impact
                        commands.entity(projectile_entity).despawn();
                    } else {
                        // Only despawn projectile for non-tank shells or if it's moving slowly
                        let should_despawn = !is_tank_shell || projectile_velocity.length() < 2.0;
                        if should_despawn {
                            commands.entity(projectile_entity).despawn();
                        }
                    }
                    
                    if let Ok((enemy_entity, mut health, mut impulse, _)) = enemy_query.get_mut(enemy_entity) {
                    health.current -= projectile_damage;
                    info!("Enemy hit! Damage: {}, Health: {}/{}", projectile_damage, health.current, health.max);
                    
                    // Apply knockback force for tank shells
                    if is_tank_shell {
                        let impact_force = projectile_velocity.normalize() * 50.0;  // Strong knockback
                        impulse.impulse = impact_force;
                        info!("Applied knockback force: {:?}", impact_force);
                        
                        // Visual feedback - flash the enemy by spawning a temporary bright entity
                        commands.entity(enemy_entity).insert(HitFlash {
                            timer: Timer::from_seconds(0.2, TimerMode::Once),
                        });
                    }
                    
                        if health.current <= 0.0 {
                            commands.entity(enemy_entity).despawn();
                            info!("Enemy destroyed!");
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn spawn_fragments(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    impact_position: Vec3,
    parent_velocity: Vec2,
    enemy_position: Vec3,
    parent_range: f32,
    parent_damage: f32,
) {
    // Calculate surface normal from impact to enemy center
    let impact_to_enemy = Vec2::new(
        enemy_position.x - impact_position.x,
        enemy_position.z - impact_position.z
    );
    let surface_normal = -impact_to_enemy.normalize(); // Points away from enemy
    
    // Calculate fragment velocities
    let fragment_velocities = calculate_fragment_velocities(parent_velocity, surface_normal);
    let fragment_lifetime_duration = calculate_fragment_lifetime(parent_range);
    let fragment_max_distance = calculate_fragment_max_distance(parent_range);
    let fragment_damage = parent_damage / 3.0; // Each fragment gets 1/3 damage
    
    // Create impact flash effect
    let flash_mesh = meshes.add(Mesh::from(shape::Box::new(0.8, 0.8, 0.8)));
    let flash_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 1.0, 0.0),
        emissive: Color::rgb(2.0, 1.5, 0.0),
        ..default()
    });
    
    commands.spawn((
        PbrBundle {
            mesh: flash_mesh,
            material: flash_material,
            transform: Transform::from_translation(impact_position),
            ..default()
        },
        HitFlash {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
    ));
    
    // Spawn three fragments
    for (i, velocity) in fragment_velocities.iter().enumerate() {
        let fragment_mesh = meshes.add(Mesh::from(shape::Box::new(0.2, 0.1, 0.2))); // Smaller than shell
        let fragment_material = materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 0.8, 0.0),
            emissive: Color::rgb(1.0, 0.5, 0.0),
            ..default()
        });
        
        commands.spawn((
            Projectile {
                damage: fragment_damage,
                speed: velocity.length(),
            },
            ShellFragment {
                parent_velocity: parent_velocity,
                lifetime: Timer::from_seconds(fragment_lifetime_duration, TimerMode::Once),
                max_distance: fragment_max_distance,
                spawn_position: Vec2::new(impact_position.x, impact_position.z),
                fragment_index: i as u8,
            },
            PbrBundle {
                mesh: fragment_mesh,
                material: fragment_material,
                transform: Transform::from_translation(impact_position),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(0.05), // Small collider
            ColliderMassProperties::Density(5.0),
            Restitution::coefficient(0.2),
            Friction::coefficient(0.5),
            Ccd::enabled(),
            Velocity {
                linvel: Vec3::new(velocity.x, 0.0, velocity.y),
                angvel: Vec3::ZERO,
            },
            ExternalImpulse::default(),
            GravityScale(0.2), // Minimal gravity
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
    
    info!("Spawned 3 fragments at impact position {:?}", impact_position);
}