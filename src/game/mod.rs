use crate::{cleanup::cleanup, AppState};
use bevy::prelude::*;

mod clock;
mod drone;
pub mod levels;

#[derive(Resource)]
pub struct LevelTime(pub u64);

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_plugin(drone::Config)
            .insert_resource(LevelTime(0))
            .add_plugin(clock::Config)
            .add_plugin(levels::Config)
            .add_system(cleanup.in_schedule(OnExit(AppState::Game)));
    }
}
