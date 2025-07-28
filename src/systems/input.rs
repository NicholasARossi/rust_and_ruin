use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::{Hero, MoveTarget, Enemy, AttackTarget, TargetIndicator};
use crate::resources::MouseWorldPosition;

pub fn mouse_position_system(
    mut mouse_world_pos: ResMut<MouseWorldPosition>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform, &Projection)>,
    mut cursor_events: EventReader<CursorMoved>,
) {
    let event_count = cursor_events.len();
    if event_count > 0 {
        info!("Processing {} cursor events", event_count);
    }
    
    for event in cursor_events.read() {
        info!("Cursor moved to: {:?}", event.position);
        
        if let Ok(window) = windows.get_single() {
            if let Ok((_camera, _camera_transform, projection)) = camera_query.get_single() {
                if let Projection::Orthographic(ortho) = projection {
                    // For a top-down orthographic camera, we can use simple math
                    let window_size = Vec2::new(window.width(), window.height());
                    let cursor_ndc = (event.position / window_size) * 2.0 - Vec2::ONE;
                    
                    // The orthographic view width/height in world units
                    let view_width = ortho.scale * window_size.x;
                    let view_height = ortho.scale * window_size.y;
                    
                    // Convert NDC to world coordinates
                    // Since camera is looking down from above, we need to account for the view
                    let world_x = cursor_ndc.x * view_width * 0.5;
                    let world_z = cursor_ndc.y * view_height * 0.5;  // Don't negate for our camera setup
                    
                    mouse_world_pos.position = Vec2::new(world_x, world_z);
                    info!("Screen: {:?}, NDC: {:?}, World: {:?}", event.position, cursor_ndc, mouse_world_pos.position);
                }
            } else {
                warn!("No camera found!");
            }
        } else {
            warn!("No window found!");
        }
    }
}

pub fn click_to_move_system(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    mouse_world_pos: Res<MouseWorldPosition>,
    hero_query: Query<Entity, With<Hero>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        let target_pos = mouse_world_pos.position;
        info!("Click target: {:?}", target_pos);
        
        // Spawn a visual marker at the click position
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
            material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(target_pos.x, 0.5, target_pos.y),
            ..default()
        });
        
        for hero_entity in hero_query.iter() {
            commands.entity(hero_entity).insert(MoveTarget {
                position: target_pos,
            });
        }
    }
}

pub fn enemy_selection_system(
    mut commands: Commands,
    mouse_button: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_world_pos: Res<MouseWorldPosition>,
    hero_query: Query<Entity, With<Hero>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    existing_indicators: Query<Entity, With<TargetIndicator>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Support both right-click and Q key for targeting
    if mouse_button.just_pressed(MouseButton::Right) || keyboard_input.just_pressed(KeyCode::Q) {
        let click_pos = mouse_world_pos.position;
        
        // Find the closest enemy within a reasonable distance
        let mut closest_enemy = None;
        let mut closest_distance = f32::MAX;
        const SELECTION_RADIUS: f32 = 2.0;
        
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let enemy_pos_2d = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
            let distance = enemy_pos_2d.distance(click_pos);
            
            if distance < SELECTION_RADIUS && distance < closest_distance {
                closest_distance = distance;
                closest_enemy = Some(enemy_entity);
            }
        }
        
        // If we found an enemy, set it as the attack target for all heroes
        if let Some(target_entity) = closest_enemy {
            info!("Selected enemy at distance: {}", closest_distance);
            
            // Remove any existing target indicators
            for indicator in existing_indicators.iter() {
                commands.entity(indicator).despawn();
            }
            
            // Spawn a visual indicator for the selected enemy
            if let Ok((_, enemy_transform)) = enemy_query.get(target_entity) {
                commands.spawn((
                    TargetIndicator {
                        target: target_entity,
                    },
                    PbrBundle {
                        mesh: meshes.add(shape::Torus {
                            radius: 1.0,
                            ring_radius: 0.1,
                            subdivisions_segments: 24,
                            subdivisions_sides: 12,
                        }.into()),
                        material: materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
                        transform: Transform::from_xyz(
                            enemy_transform.translation.x,
                            0.1,
                            enemy_transform.translation.z
                        ),
                        ..default()
                    },
                ));
            }
            
            for hero_entity in hero_query.iter() {
                // Remove any existing attack target
                commands.entity(hero_entity).remove::<AttackTarget>();
                // Add the new attack target
                commands.entity(hero_entity).insert(AttackTarget {
                    entity: target_entity,
                });
            }
        }
    }
}

pub fn update_target_indicator_system(
    mut commands: Commands,
    mut indicator_query: Query<(Entity, &mut Transform, &TargetIndicator)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<TargetIndicator>)>,
) {
    for (indicator_entity, mut indicator_transform, target_indicator) in indicator_query.iter_mut() {
        if let Ok(enemy_transform) = enemy_query.get(target_indicator.target) {
            // Update indicator position to follow the enemy
            indicator_transform.translation.x = enemy_transform.translation.x;
            indicator_transform.translation.z = enemy_transform.translation.z;
        } else {
            // Enemy no longer exists, remove the indicator
            commands.entity(indicator_entity).despawn();
        }
    }
}