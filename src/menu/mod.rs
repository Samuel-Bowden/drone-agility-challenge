use bevy::prelude::*;

mod choose_level;
mod end;
mod failed;
mod level;
mod main;
mod success;

pub struct Config;

impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_plugin(end::Config)
            .add_plugin(failed::Config)
            .add_plugin(level::Config)
            .add_plugin(choose_level::Config)
            .add_plugin(main::Config)
            .add_plugin(success::Config);
    }
}
