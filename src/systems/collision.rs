use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::components::{Projectile, Enemy, Health};

pub fn collision_detection_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    projectile_query: Query<Entity, With<Projectile>>,
    mut enemy_query: Query<(Entity, &mut Health), With<Enemy>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let (projectile_entity, enemy_entity) = 
                    if projectile_query.contains(*entity1) && enemy_query.contains(*entity2) {
                        (*entity1, *entity2)
                    } else if projectile_query.contains(*entity2) && enemy_query.contains(*entity1) {
                        (*entity2, *entity1)
                    } else {
                        continue;
                    };
                
                commands.entity(projectile_entity).despawn();
                
                if let Ok((enemy_entity, mut health)) = enemy_query.get_mut(enemy_entity) {
                    health.current -= 10.0;
                    info!("Enemy hit! Health: {}/{}", health.current, health.max);
                    
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