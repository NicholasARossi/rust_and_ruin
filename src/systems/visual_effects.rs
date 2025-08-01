use bevy::prelude::*;
use crate::components::HitFlash;

pub fn hit_flash_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitFlash, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut hit_flash, material_handle) in query.iter_mut() {
        hit_flash.timer.tick(time.delta());
        
        if let Some(material) = materials.get_mut(material_handle) {
            // Flash between bright white and original color
            let t = hit_flash.timer.percent();
            let flash_intensity = 1.0 - t;  // Fade from white to original
            
            // Interpolate to white
            material.base_color = Color::rgb(
                1.0 - (1.0 - material.base_color.r()) * (1.0 - flash_intensity),
                1.0 - (1.0 - material.base_color.g()) * (1.0 - flash_intensity),
                1.0 - (1.0 - material.base_color.b()) * (1.0 - flash_intensity),
            );
            material.emissive = Color::rgb(flash_intensity, flash_intensity, flash_intensity);
        }
        
        if hit_flash.timer.finished() {
            commands.entity(entity).remove::<HitFlash>();
            // Reset material
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = Color::rgb(1.0, 0.0, 0.0);  // Reset to red for enemy
                material.emissive = Color::BLACK;
            }
        }
    }
}