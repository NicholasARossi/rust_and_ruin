use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Mech {
    pub name: String,
}

impl Mech {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

#[derive(Component, Debug)]
pub struct MechLower {
    pub turn_rate: f32,
    pub max_speed: f32,
}

#[derive(Component, Debug)]
pub struct MechUpper {
    pub rotation_speed: f32,
    pub weapon_mount_offset: Vec3,
}

#[derive(Component, Debug, Clone)]
pub struct TankTreads {
    pub speed: f32,
    pub turn_rate: f32,
    pub acceleration: f32,
}

impl Default for TankTreads {
    fn default() -> Self {
        Self {
            speed: 4.0,
            turn_rate: 60.0,
            acceleration: 2.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct TurretCannon {
    pub fire_rate: f32,
    pub projectile_damage: f32,
    pub rotation_speed: f32,
    pub barrel_length: f32,
}

impl Default for TurretCannon {
    fn default() -> Self {
        Self {
            fire_rate: 1.5,  // Slower, more impactful shots
            projectile_damage: 25.0,
            rotation_speed: 120.0,
            barrel_length: 0.5,
        }
    }
}

#[derive(Component, Debug)]
pub struct MechParts {
    pub lower: Option<Entity>,
    pub upper: Option<Entity>,
}

impl MechParts {
    pub fn new() -> Self {
        Self {
            lower: None,
            upper: None,
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

#[derive(Debug, Clone)]
pub struct MechStats {
    pub max_speed: f32,
    pub turn_rate: f32,
    pub fire_rate: f32,
    pub damage: f32,
}

impl MechStats {
    pub fn from_parts(lower: &TankTreads, upper: &TurretCannon) -> Self {
        Self {
            max_speed: lower.speed,
            turn_rate: lower.turn_rate,
            fire_rate: upper.fire_rate,
            damage: upper.projectile_damage,
        }
    }
}

#[derive(Component)]
pub struct MechLowerPart;

#[derive(Component)]
pub struct MechUpperPart;

#[derive(Component)]
pub struct CannonBarrel;

#[derive(Component)]
pub struct TurretRotation {
    pub target_angle: f32,
    pub current_angle: f32,
}