use bevy::prelude::*;
use super::traits::*;
use super::components::*;

#[derive(Component, Debug, Clone)]
pub struct TankTreadsLower {
    pub movement_stats: MovementStats,
}

impl Default for TankTreadsLower {
    fn default() -> Self {
        Self {
            movement_stats: MovementStats {
                max_speed: 5.0,
                turn_rate: 90.0,
                acceleration: 3.0,
            },
        }
    }
}

impl TankTreadsLower {
    pub fn new(max_speed: f32, turn_rate: f32, acceleration: f32) -> Self {
        Self {
            movement_stats: MovementStats {
                max_speed,
                turn_rate,
                acceleration,
            },
        }
    }
}

pub fn create_tank_treads_lower() -> MechLowerBody {
    let treads = TankTreadsLower::default();
    MechLowerBody {
        movement_stats: treads.movement_stats,
    }
}