use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub score: u32,
    pub game_time: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            game_time: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct MouseWorldPosition {
    pub position: Vec2,
}

impl Default for MouseWorldPosition {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
        }
    }
}