use bevy::prelude::*;
use super::traits::*;
use super::components::*;

#[derive(Component, Debug, Clone)]
pub struct TurretUpper {
    pub rotation_capability: RotationCapability,
    pub hardpoints: Vec<Hardpoint>,
}

impl Default for TurretUpper {
    fn default() -> Self {
        Self {
            rotation_capability: RotationCapability {
                can_rotate: true,
                rotation_speed: 120.0,
            },
            hardpoints: vec![
                Hardpoint::new("main".to_string(), Vec3::new(0.0, 0.0, 0.5)),
            ],
        }
    }
}

impl TurretUpper {
    pub fn new(rotation_speed: f32) -> Self {
        Self {
            rotation_capability: RotationCapability {
                can_rotate: true,
                rotation_speed,
            },
            hardpoints: vec![
                Hardpoint::new("main".to_string(), Vec3::new(0.0, 0.0, 0.5)),
            ],
        }
    }

    pub fn with_dual_mount(rotation_speed: f32) -> Self {
        Self {
            rotation_capability: RotationCapability {
                can_rotate: true,
                rotation_speed,
            },
            hardpoints: vec![
                Hardpoint::new("left".to_string(), Vec3::new(-0.3, 0.0, 0.5)),
                Hardpoint::new("right".to_string(), Vec3::new(0.3, 0.0, 0.5)),
            ],
        }
    }
}

pub fn create_turret_upper() -> MechUpperBody {
    let turret = TurretUpper::default();
    MechUpperBody {
        rotation_capability: turret.rotation_capability,
        hardpoints: turret.hardpoints,
    }
}

pub fn create_dual_turret_upper() -> MechUpperBody {
    let turret = TurretUpper::with_dual_mount(120.0);
    MechUpperBody {
        rotation_capability: turret.rotation_capability,
        hardpoints: turret.hardpoints,
    }
}