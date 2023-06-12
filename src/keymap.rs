use bevy::prelude::*;

#[derive(Resource)]
pub struct KeyMap {
    pub up: KeyCode,
    pub down: KeyCode,
    pub anti_cw: KeyCode,
    pub cw: KeyCode,
}

impl Default for KeyMap {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            down: KeyCode::S,
            anti_cw: KeyCode::A,
            cw: KeyCode::D,
        }
    }
}
