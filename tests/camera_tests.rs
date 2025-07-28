use bevy::prelude::*;
use rust_and_ruin::camera::*;

#[cfg(test)]
mod camera_transformation_tests {
    use super::*;

    #[test]
    fn test_screen_to_world_conversion() {
        let screen_pos = Vec2::new(640.0, 360.0);
        let camera_transform = create_orthographic_camera_transform(CAMERA_ANGLE_DEGREES);
        
        let world_pos = screen_to_world_position(
            screen_pos,
            &camera_transform,
            1280.0,
            720.0,
            0.01
        );
        
        assert!(world_pos.is_some());
        if let Some(pos) = world_pos {
            assert_eq!(pos.y, 0.0);
        }
    }

    #[test]
    fn test_world_to_screen_conversion() {
        let world_pos = Vec3::new(0.0, 0.0, 0.0);
        let camera_transform = create_orthographic_camera_transform(CAMERA_ANGLE_DEGREES);
        
        let screen_pos = world_to_screen_position(
            world_pos,
            &camera_transform,
            1280.0,
            720.0,
            0.01
        );
        
        assert!((screen_pos.x - 640.0).abs() < 100.0);
        assert!((screen_pos.y - 360.0).abs() < 100.0);
    }

    #[test]
    fn test_orthographic_camera_angle() {
        let angle_degrees = 63.435;
        let transform = create_orthographic_camera_transform(angle_degrees);
        
        let look_direction = (Vec3::ZERO - transform.translation).normalize();
        let vertical_component = look_direction.y.abs();
        let horizontal_component = (look_direction.x.powi(2) + look_direction.z.powi(2)).sqrt();
        let actual_angle = vertical_component.atan2(horizontal_component).to_degrees();
        
        assert!((actual_angle - angle_degrees).abs() < 0.1);
    }
}