use bevy::prelude::*;

pub const CAMERA_ANGLE_DEGREES: f32 = 45.0;
pub const CAMERA_HEIGHT: f32 = 20.0;
pub const CAMERA_DISTANCE: f32 = 30.0;

pub fn create_orthographic_camera_transform(angle_degrees: f32) -> Transform {
    let angle_radians = angle_degrees.to_radians();
    let height = CAMERA_DISTANCE * angle_radians.sin();
    let horizontal_distance = CAMERA_DISTANCE * angle_radians.cos();
    
    Transform::from_xyz(0.0, height, horizontal_distance)
        .looking_at(Vec3::ZERO, Vec3::Y)
}

pub fn screen_to_world_position(
    screen_pos: Vec2,
    camera_transform: &Transform,
    window_width: f32,
    window_height: f32,
    orthographic_scale: f32,
) -> Option<Vec3> {
    let ndc_x = (screen_pos.x / window_width) * 2.0 - 1.0;
    let ndc_y = -((screen_pos.y / window_height) * 2.0 - 1.0);
    
    let half_width = window_width * 0.5 * orthographic_scale;
    let half_height = window_height * 0.5 * orthographic_scale;
    
    let camera_space = Vec3::new(
        ndc_x * half_width,
        ndc_y * half_height,
        0.0,
    );
    
    let world_pos = camera_transform.compute_matrix() * camera_space.extend(1.0);
    let world_pos = Vec3::new(world_pos.x, world_pos.y, world_pos.z);
    
    let ray_origin = camera_transform.translation;
    let ray_direction = (world_pos - ray_origin).normalize();
    
    if ray_direction.y.abs() > 0.001 {
        let t = -ray_origin.y / ray_direction.y;
        if t > 0.0 {
            let intersection = ray_origin + ray_direction * t;
            return Some(intersection);
        }
    }
    
    None
}

pub fn world_to_screen_position(
    world_pos: Vec3,
    camera_transform: &Transform,
    window_width: f32,
    window_height: f32,
    orthographic_scale: f32,
) -> Vec2 {
    let view_matrix = camera_transform.compute_matrix().inverse();
    let camera_space = view_matrix.transform_point3(world_pos);
    
    let half_width = window_width * 0.5 * orthographic_scale;
    let half_height = window_height * 0.5 * orthographic_scale;
    
    let ndc_x = camera_space.x / half_width;
    let ndc_y = camera_space.y / half_height;
    
    let screen_x = (ndc_x + 1.0) * 0.5 * window_width;
    let screen_y = (1.0 - ndc_y) * 0.5 * window_height;
    
    Vec2::new(screen_x, screen_y)
}

pub fn setup_orthographic_camera(commands: &mut Commands) {
    // Simple top-down camera for debugging
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 0.1).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection {
            scale: 0.02,
            ..default()
        }.into(),
        tonemapping: bevy::core_pipeline::tonemapping::Tonemapping::None,
        ..default()
    });
    
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}