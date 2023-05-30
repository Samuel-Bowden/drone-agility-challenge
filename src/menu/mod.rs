use bevy::prelude::*;

pub mod end;
pub mod failed;
pub mod level;
pub mod main;
pub mod success;

pub struct Config;

impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_plugin(end::Config)
            .add_plugin(failed::Config)
            .add_plugin(level::Config)
            .add_plugin(main::Config)
            .add_plugin(success::Config);
    }
}
