use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::{Hero, MoveTarget};
use crate::resources::MouseWorldPosition;

pub fn mouse_position_system(
    mut mouse_world_pos: ResMut<MouseWorldPosition>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_events: EventReader<CursorMoved>,
) {
    for event in cursor_events.iter() {
        if let Ok(_window) = windows.get_single() {
            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                if let Some(world_position) = camera.viewport_to_world_2d(
                    camera_transform,
                    event.position,
                ) {
                    mouse_world_pos.position = world_position;
                }
            }
        }
    }
}

pub fn click_to_move_system(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    mouse_world_pos: Res<MouseWorldPosition>,
    hero_query: Query<Entity, With<Hero>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        for hero_entity in hero_query.iter() {
            commands.entity(hero_entity).insert(MoveTarget {
                position: mouse_world_pos.position,
            });
        }
    }
}