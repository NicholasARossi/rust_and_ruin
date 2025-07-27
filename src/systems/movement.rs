use bevy::prelude::*;
use crate::components::{Hero, MoveTarget};

const MOVE_SPEED: f32 = 200.0;
const ARRIVAL_THRESHOLD: f32 = 5.0;

pub fn movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &MoveTarget), With<Hero>>,
) {
    for (entity, mut transform, target) in query.iter_mut() {
        let direction = target.position - transform.translation.truncate();
        let distance = direction.length();

        if distance > ARRIVAL_THRESHOLD {
            let velocity = direction.normalize() * MOVE_SPEED;
            transform.translation += velocity.extend(0.0) * time.delta_seconds();
        } else {
            commands.entity(entity).remove::<MoveTarget>();
        }
    }
}