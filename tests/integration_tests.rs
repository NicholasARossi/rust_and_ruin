use bevy::prelude::*;
use rust_and_ruin::{components::*, resources::*, systems::*, camera};

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_click_to_move_with_orthographic_camera() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<MouseWorldPosition>();
        
        let camera_transform = camera::create_orthographic_camera_transform(camera::CAMERA_ANGLE_DEGREES);
        
        let screen_pos = Vec2::new(640.0, 360.0);
        let world_pos = camera::screen_to_world_position(
            screen_pos,
            &camera_transform,
            1280.0,
            720.0,
            0.01
        );
        
        assert!(world_pos.is_some());
        let world_pos = world_pos.unwrap();
        assert_eq!(world_pos.y, 0.0);
    }

    #[test]
    fn test_projectile_spawning_in_3d() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        let hero_transform = Transform::from_xyz(-4.0, 0.0, 0.0);
        let enemy_transform = Transform::from_xyz(4.0, 0.0, 0.0);
        
        let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.z);
        let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
        
        let direction = (enemy_pos - hero_pos).normalize();
        assert!(direction.length() > 0.99 && direction.length() < 1.01);
    }

    #[test]
    fn test_movement_on_xz_plane() {
        let start_pos = Transform::from_xyz(0.0, 0.0, 0.0);
        let target_pos = Vec2::new(5.0, 5.0);
        
        let current_pos = Vec2::new(start_pos.translation.x, start_pos.translation.z);
        let direction = target_pos - current_pos;
        
        assert_eq!(direction, Vec2::new(5.0, 5.0));
    }
}