use bevy::prelude::*;
use crate::components::{Hero, MoveTarget, AttackTarget, TankMovement};
use crate::mech::{MechLower, TankTreads, MechMovement};

const ARRIVAL_THRESHOLD: f32 = 0.05;
const ATTACK_RANGE: f32 = 10.0;  // Match projectile.rs

pub fn movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &MoveTarget, &Children), (With<Hero>, Without<TankMovement>, Without<MechMovement>)>,
    lower_query: Query<(&MechLower, &TankTreads)>,
) {
    for (entity, mut transform, target, children) in query.iter_mut() {
        let current_pos = Vec2::new(transform.translation.x, transform.translation.z);
        let direction = target.position - current_pos;
        let distance = direction.length();

        if distance > ARRIVAL_THRESHOLD {
            let mut move_speed = 2.0;
            
            for child in children {
                if let Ok((_, tank_treads)) = lower_query.get(*child) {
                    move_speed = tank_treads.speed;
                    break;
                }
            }
            
            let velocity = direction.normalize() * move_speed;
            transform.translation.x += velocity.x * time.delta_seconds();
            transform.translation.z += velocity.y * time.delta_seconds();
            
            // Face the direction of movement
            if direction.length() > 0.01 {
                let target_rotation = direction.x.atan2(direction.y);
                transform.rotation = Quat::from_rotation_y(target_rotation);
            }
        } else {
            commands.entity(entity).remove::<MoveTarget>();
        }
    }
}

pub fn attack_move_system(
    mut commands: Commands,
    hero_query: Query<(Entity, &Transform, Option<&AttackTarget>), With<Hero>>,
    enemy_transforms: Query<&Transform, With<crate::components::Enemy>>,
) {
    for (hero_entity, hero_transform, attack_target) in hero_query.iter() {
        if let Some(target) = attack_target {
            // Check if the target still exists
            if let Ok(enemy_transform) = enemy_transforms.get(target.entity) {
                let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.z);
                let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
                let distance = hero_pos.distance(enemy_pos);
                
                // If we're not in attack range, set a move target
                if distance > ATTACK_RANGE {
                    let direction = (enemy_pos - hero_pos).normalize();
                    let move_to_pos = enemy_pos - direction * (ATTACK_RANGE - 0.5);
                    
                    commands.entity(hero_entity).insert(MoveTarget {
                        position: move_to_pos,
                    });
                }
            } else {
                // Target no longer exists, remove attack target
                commands.entity(hero_entity).remove::<AttackTarget>();
            }
        }
    }
}