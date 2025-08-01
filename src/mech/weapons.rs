use bevy::prelude::*;
use super::traits::*;
use super::components::*;

#[derive(Component, Debug, Clone)]
pub struct CannonWeapon {
    pub weapon_stats: WeaponStats,
    pub barrel_length: f32,
}

impl Default for CannonWeapon {
    fn default() -> Self {
        Self {
            weapon_stats: WeaponStats {
                fire_rate: 1.5,
                damage: 25.0,
                range: 15.0,
                projectile_speed: 15.0,
            },
            barrel_length: 0.5,
        }
    }
}

impl CannonWeapon {
    pub fn new(fire_rate: f32, damage: f32, range: f32, projectile_speed: f32, barrel_length: f32) -> Self {
        Self {
            weapon_stats: WeaponStats {
                fire_rate,
                damage,
                range,
                projectile_speed,
            },
            barrel_length,
        }
    }

    pub fn heavy() -> Self {
        Self {
            weapon_stats: WeaponStats {
                fire_rate: 2.0,
                damage: 40.0,
                range: 20.0,
                projectile_speed: 12.0,
            },
            barrel_length: 0.7,
        }
    }

    pub fn light() -> Self {
        Self {
            weapon_stats: WeaponStats {
                fire_rate: 0.8,
                damage: 15.0,
                range: 12.0,
                projectile_speed: 18.0,
            },
            barrel_length: 0.4,
        }
    }
}

pub fn create_cannon_weapon(hardpoint_id: String) -> MechWeapon {
    let cannon = CannonWeapon::default();
    MechWeapon {
        weapon_stats: cannon.weapon_stats,
        hardpoint_id,
        last_fire_time: 0.0,
    }
}