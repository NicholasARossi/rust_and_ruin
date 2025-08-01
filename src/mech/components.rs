use bevy::prelude::*;
use super::traits::*;

#[derive(Component, Debug)]
pub struct MechLowerBody {
    pub movement_stats: MovementStats,
}

#[derive(Component, Debug)]
pub struct MechUpperBody {
    pub rotation_capability: RotationCapability,
    pub hardpoints: Vec<Hardpoint>,
}

#[derive(Component, Debug)]
pub struct MechWeapon {
    pub weapon_stats: WeaponStats,
    pub hardpoint_id: String,
    pub last_fire_time: f32,
}

#[derive(Component, Debug)]
pub struct MechRotation {
    pub target_angle: f32,
    pub current_angle: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MechMovementState {
    Idle,
    Rotating,
    Moving,
}

#[derive(Component, Debug)]
pub struct MechMovement {
    pub movement_state: MechMovementState,
    pub target_rotation: f32,
    pub current_speed: f32,
}

impl Default for MechMovement {
    fn default() -> Self {
        Self {
            movement_state: MechMovementState::Idle,
            target_rotation: 0.0,
            current_speed: 0.0,
        }
    }
}

#[derive(Component, Debug)]
pub struct MechHierarchy {
    pub lower: Option<Entity>,
    pub upper: Option<Entity>,
    pub weapons: Vec<Entity>,
}

impl MechHierarchy {
    pub fn new() -> Self {
        Self {
            lower: None,
            upper: None,
            weapons: vec![],
        }
    }

    pub fn has_lower(&self) -> bool {
        self.lower.is_some()
    }

    pub fn has_upper(&self) -> bool {
        self.upper.is_some()
    }

    pub fn is_complete(&self) -> bool {
        self.has_lower() && self.has_upper()
    }
}