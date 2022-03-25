use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::{menu::*, game::*, debug::*};

mod menu;
mod game;
mod debug;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(Gravity::from(Vec3::new(0.0, -180., 0.0)))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(SpawnTimer{t: Timer::from_seconds(0.1, false)})
        .add_state(AppState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu))
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
            .with_system(setup_game)
            .with_system(spawn_drone)
            .with_system(spawn_level_1)
            .with_system(position_text),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(update_position_text)
                .with_system(drone_movement)
        )
        .run();
}