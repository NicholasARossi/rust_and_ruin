use bevy::prelude::*;
use std::f32::consts::PI;
use crate::components::{ShellFragment, Projectile, HitFlash};

pub fn calculate_fragment_directions(impact_velocity: Vec2, surface_normal: Vec2) -> Vec<Vec2> {
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
        
        vec![
            reflection,
            rotate_vec2(reflection, spread_angle),
            rotate_vec2(reflection, -spread_angle),
        ]
    }
}

pub fn calculate_fragment_velocities(parent_velocity: Vec2, surface_normal: Vec2) -> Vec<Vec2> {
    let directions = calculate_fragment_directions(parent_velocity, surface_normal);
    let fragment_speed = parent_velocity.length() * 0.7;
    
    directions.into_iter()
        .map(|dir| dir * fragment_speed)
        .collect()
}

pub fn calculate_fragment_lifetime(parent_range: f32) -> f32 {
    // Fragment travels 15% of parent range (middle of 10-20% range)
    parent_range * 0.15
}

pub fn calculate_fragment_max_distance(parent_range: f32) -> f32 {
    // Fragment travels 15% of parent range
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

pub fn fragment_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &Transform, &mut ShellFragment), With<Projectile>>,
) {
    for (entity, transform, mut fragment) in query.iter_mut() {
        fragment.lifetime.tick(time.delta());
        
        let current_pos = Vec2::new(transform.translation.x, transform.translation.z);
        let distance_traveled = current_pos.distance(fragment.spawn_position);
        
        // Despawn if lifetime expired or max distance reached
        if fragment.lifetime.finished() || distance_traveled >= fragment.max_distance {
            commands.entity(entity).despawn();
        }
    }
}

// TODO: Add visual fade system for fragments
pub fn fragment_visual_fade_system(
    mut query: Query<(&ShellFragment, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (fragment, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            // Calculate fade based on lifetime remaining
            let alpha = 1.0 - fragment.lifetime.percent();
            material.base_color.set_a(alpha);
            
            // Optional: Also reduce emissive intensity
            material.emissive = Color::rgb(1.0 * alpha, 0.8 * alpha, 0.0);
        }
    }
}

pub fn hit_flash_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitFlash)>,
) {
    for (entity, mut hit_flash) in query.iter_mut() {
        hit_flash.timer.tick(time.delta());
        
        if hit_flash.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}