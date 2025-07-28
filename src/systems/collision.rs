use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::{Projectile, Enemy, Health};

pub fn collision_detection_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<(Entity, &Projectile)>,
    mut enemy_query: Query<(Entity, &mut Health), With<Enemy>>,
) {
    for collision_event in collision_events.read() {
        info!("Collision event detected: {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let (projectile_entity, projectile_damage, enemy_entity) = 
                    if let Ok((proj_entity, projectile)) = projectile_query.get(*entity1) {
                        if enemy_query.contains(*entity2) {
                            (proj_entity, projectile.damage, *entity2)
                        } else {
                            continue;
                        }
                    } else if let Ok((proj_entity, projectile)) = projectile_query.get(*entity2) {
                        if enemy_query.contains(*entity1) {
                            (proj_entity, projectile.damage, *entity1)
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    };
                
                commands.entity(projectile_entity).despawn();
                
                if let Ok((enemy_entity, mut health)) = enemy_query.get_mut(enemy_entity) {
                    health.current -= projectile_damage;
                    info!("Enemy hit! Damage: {}, Health: {}/{}", projectile_damage, health.current, health.max);
                    
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