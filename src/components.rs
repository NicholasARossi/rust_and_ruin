use bevy::prelude::*;

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct AttackTarget {
    pub entity: Entity,
}

#[derive(Component)]
pub struct TargetIndicator {
    pub target: Entity,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

#[derive(Component)]
pub struct MoveTarget {
    pub position: Vec2,
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Component)]
pub struct Rocket {
    pub initial_speed: f32,
    pub max_speed: f32,
    pub acceleration_rate: f32,
    pub current_speed: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct TankShell {
    pub velocity: Vec2,
    pub spawn_position: Vec2,
    pub max_range: f32,
}

#[derive(Component)]
pub struct FragmentShell;

#[derive(Component)]
pub struct ShellFragment {
    pub parent_velocity: Vec2,
    pub lifetime: Timer,
    pub max_distance: f32,
    pub spawn_position: Vec2,
    pub fragment_index: u8,
}

#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TankRotationState {
    Idle,
    Rotating,
    Moving,
}

#[derive(Component)]
pub struct TankMovement {
    pub rotation_state: TankRotationState,
    pub target_rotation: f32,
    pub current_speed: f32,
    pub acceleration: f32,
    pub max_speed: f32,
    pub rotation_speed: f32, // degrees per second
}

impl Default for TankMovement {
    fn default() -> Self {
        Self {
            rotation_state: TankRotationState::Idle,
            target_rotation: 0.0,
            current_speed: 0.0,
            acceleration: 3.0,
            max_speed: 5.0,
            rotation_speed: 90.0,
        }
    }
}