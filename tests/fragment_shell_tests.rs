use bevy::prelude::*;
use rust_and_ruin::components::{FragmentShell, ShellFragment, TankShell};
use std::f32::consts::PI;

#[cfg(test)]
mod fragment_shell_tests {
    use super::*;
    
    #[test]
    fn test_fragment_shell_component_creation() {
        let fragment_shell = FragmentShell;
        // FragmentShell is a marker component
        assert!(std::mem::size_of::<FragmentShell>() == 0);
    }
    
    #[test]
    fn test_shell_fragment_component_creation() {
        let fragment = ShellFragment {
            parent_velocity: Vec2::new(10.0, 0.0),
            lifetime: Timer::from_seconds(0.5, TimerMode::Once),
            max_distance: 2.0,
            spawn_position: Vec2::ZERO,
            fragment_index: 0,
        };
        
        assert_eq!(fragment.parent_velocity, Vec2::new(10.0, 0.0));
        assert_eq!(fragment.max_distance, 2.0);
        assert_eq!(fragment.spawn_position, Vec2::ZERO);
        assert_eq!(fragment.fragment_index, 0);
    }
    
    #[test]
    fn test_perpendicular_impact_fragment_directions() {
        // Shell moving right (+X) hits perpendicular surface (normal facing left)
        let impact_velocity = Vec2::new(10.0, 0.0);
        let surface_normal = Vec2::new(-1.0, 0.0);
        
        let directions = calculate_fragment_directions(impact_velocity, surface_normal);
        
        // Should create cone pattern opposite to impact (spreading left)
        assert_eq!(directions.len(), 3);
        
        // Center fragment goes straight back
        assert!((directions[0] - Vec2::new(-1.0, 0.0)).length() < 0.01);
        
        // Side fragments at 120° intervals around the reflection direction
        let angle1 = directions[1].y.atan2(directions[1].x);
        let angle2 = directions[2].y.atan2(directions[2].x);
        let center_angle = PI; // Pointing left (-X)
        
        // Check angles are approximately 120° apart
        let diff1 = angle_diff(angle1, center_angle).abs();
        let diff2 = angle_diff(angle2, center_angle).abs();
        assert!((diff1 - 2.0 * PI / 3.0).abs() < 0.01);
        assert!((diff2 - 2.0 * PI / 3.0).abs() < 0.01);
    }
    
    #[test]
    fn test_angled_impact_fragment_directions() {
        // Shell moving at 45° angle hits surface
        let impact_velocity = Vec2::new(1.0, 1.0).normalize() * 10.0;
        let surface_normal = Vec2::new(0.0, 1.0); // Surface facing up
        
        let directions = calculate_fragment_directions(impact_velocity, surface_normal);
        
        // Reflection should be at same angle (45° from normal)
        // Center fragment follows reflection
        let expected_reflection = Vec2::new(1.0, -1.0).normalize();
        assert!((directions[0] - expected_reflection).length() < 0.01);
        
        // Side fragments spread ±30° in plane perpendicular to surface normal
        // This means they spread horizontally when hitting a horizontal surface
        let spread_angle = 30.0_f32.to_radians();
        
        // Calculate angle between side fragments and center
        let angle1 = directions[0].angle_between(directions[1]);
        let angle2 = directions[0].angle_between(directions[2]);
        
        assert!((angle1.abs() - spread_angle).abs() < 0.01);
        assert!((angle2.abs() - spread_angle).abs() < 0.01);
    }
    
    #[test]
    fn test_fragment_velocities() {
        let parent_velocity = Vec2::new(15.0, 0.0);
        let surface_normal = Vec2::new(-1.0, 0.0);
        
        let velocities = calculate_fragment_velocities(parent_velocity, surface_normal);
        
        // Each fragment should have 70% of original speed
        let expected_speed = parent_velocity.length() * 0.7;
        
        for velocity in &velocities {
            assert!((velocity.length() - expected_speed).abs() < 0.01);
        }
    }
    
    #[test]
    fn test_fragment_lifetime_calculation() {
        let parent_range = 15.0;
        let fragment_lifetime = calculate_fragment_lifetime(parent_range);
        
        // Fragment should travel 10-20% of parent range
        // At 70% velocity, this translates to a specific time
        let min_distance = parent_range * 0.1;
        let max_distance = parent_range * 0.2;
        
        // Assuming fragment speed is 70% of shell speed
        let fragment_distance = fragment_lifetime * 0.7;
        
        assert!(fragment_distance >= min_distance);
        assert!(fragment_distance <= max_distance);
    }
}

// Helper functions that would be implemented in the actual system
fn calculate_fragment_directions(impact_velocity: Vec2, surface_normal: Vec2) -> Vec<Vec2> {
    let impact_dir = impact_velocity.normalize();
    
    // Check if impact is perpendicular (within tolerance)
    let dot = impact_dir.dot(surface_normal).abs();
    if dot > 0.95 { // Nearly perpendicular
        // Cone pattern: reflection direction with 120° spread
        let reflection = -impact_dir;
        let angle_step = 2.0 * PI / 3.0; // 120 degrees
        
        vec![
            reflection,
            rotate_vec2(reflection, angle_step),
            rotate_vec2(reflection, -angle_step),
        ]
    } else {
        // Angled impact: ricochet with ±30° spread
        let reflection = reflect_vector(impact_dir, surface_normal);
        let spread_angle = 30.0_f32.to_radians();
        
        // Find perpendicular to reflection plane
        let perpendicular = Vec2::new(-surface_normal.y, surface_normal.x);
        
        vec![
            reflection,
            rotate_around_axis(reflection, perpendicular, spread_angle),
            rotate_around_axis(reflection, perpendicular, -spread_angle),
        ]
    }
}

fn calculate_fragment_velocities(parent_velocity: Vec2, surface_normal: Vec2) -> Vec<Vec2> {
    let directions = calculate_fragment_directions(parent_velocity, surface_normal);
    let fragment_speed = parent_velocity.length() * 0.7;
    
    directions.into_iter()
        .map(|dir| dir * fragment_speed)
        .collect()
}

fn calculate_fragment_lifetime(parent_range: f32) -> f32 {
    // Fragment travels 15% of parent range (middle of 10-20% range)
    parent_range * 0.15
}

fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
    let cos = angle.cos();
    let sin = angle.sin();
    Vec2::new(
        v.x * cos - v.y * sin,
        v.x * sin + v.y * cos
    )
}

fn reflect_vector(incident: Vec2, normal: Vec2) -> Vec2 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn rotate_around_axis(v: Vec2, _axis: Vec2, angle: f32) -> Vec2 {
    // For 2D top-down game, simply rotate the vector by the angle
    rotate_vec2(v, angle)
}

fn angle_diff(a: f32, b: f32) -> f32 {
    let diff = a - b;
    if diff > PI {
        diff - 2.0 * PI
    } else if diff < -PI {
        diff + 2.0 * PI
    } else {
        diff
    }
}