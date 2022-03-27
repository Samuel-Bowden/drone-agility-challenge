use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::{title::*, game::*, failed::*};

mod title;
mod game;
mod failed;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    InGame,
    Failed,
}

fn main() {
    App::new()
        .insert_resource(Gravity::from(Vec3::new(0.0, -180., 0.0)))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_state(AppState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_title))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(title))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_title))
        .add_system_set(SystemSet::on_enter(AppState::Failed).with_system(setup_failed))
        .add_system_set(SystemSet::on_update(AppState::Failed).with_system(failed))
        .add_system_set(SystemSet::on_exit(AppState::Failed).with_system(cleanup_failed))
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
            .with_system(setup_game)
            .with_system(spawn_drone)
            .with_system(spawn_level_1)
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(drone_movement)
                .with_system(detect_collisions)
        )
        .add_system_set(SystemSet::on_exit(AppState::InGame).with_system(cleanup_game))
        .run();
}