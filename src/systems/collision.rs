use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Projectile, Enemy, Health, TankShell, HitFlash};

pub fn collision_detection_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<(Entity, &Projectile, Option<&TankShell>, &Velocity)>,
    mut enemy_query: Query<(Entity, &mut Health, &mut ExternalImpulse), With<Enemy>>,
) {
    for collision_event in collision_events.read() {
        info!("Collision event detected: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let (projectile_entity, projectile_damage, enemy_entity, projectile_velocity, is_tank_shell) = 
                    if let Ok((proj_entity, projectile, tank_shell, velocity)) = projectile_query.get(*entity1) {
                        if enemy_query.contains(*entity2) {
                            (proj_entity, projectile.damage, *entity2, velocity.linvel, tank_shell.is_some())
                        } else {
                            continue;
                        }
                    } else if let Ok((proj_entity, projectile, tank_shell, velocity)) = projectile_query.get(*entity2) {
                        if enemy_query.contains(*entity1) {
                            (proj_entity, projectile.damage, *entity1, velocity.linvel, tank_shell.is_some())
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    };
                
                // Only despawn projectile for non-tank shells or if it's moving slowly
                let should_despawn = !is_tank_shell || projectile_velocity.length() < 2.0;
                if should_despawn {
                    commands.entity(projectile_entity).despawn();
                }
                
                if let Ok((enemy_entity, mut health, mut impulse)) = enemy_query.get_mut(enemy_entity) {
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
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}