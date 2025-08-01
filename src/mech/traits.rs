use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct MovementStats {
    pub max_speed: f32,
    pub turn_rate: f32,
    pub acceleration: f32,
}

#[derive(Debug, Clone)]
pub struct WeaponStats {
    pub fire_rate: f32,
    pub damage: f32,
    pub range: f32,
    pub projectile_speed: f32,
}

#[derive(Debug, Clone)]
pub struct RotationCapability {
    pub can_rotate: bool,
    pub rotation_speed: f32,
}

#[derive(Debug, Clone)]
pub struct Hardpoint {
    pub id: String,
    pub offset: Vec3,
    pub occupied_by: Option<Entity>,
}

impl Hardpoint {
    pub fn new(id: String, offset: Vec3) -> Self {
        Self {
            id,
            offset,
            occupied_by: None,
        }
    }
}