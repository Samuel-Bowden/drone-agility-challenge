use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::{title::*, game::*, failed::*, success::*, level_title::*, cleanup::*};

mod title;
mod level_title;
mod game;
mod failed;
mod success;
mod cleanup;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Title,
    LevelTitle,
    Game,
    Failed,
    Success,
}

fn main() {
    App::new()
        .insert_resource(Gravity::from(Vec3::new(0.0, -180., 0.0)))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_state(AppState::Title)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_system_set(SystemSet::on_enter(AppState::Title).with_system(setup_title))
        .add_system_set(SystemSet::on_update(AppState::Title).with_system(title))
        .add_system_set(SystemSet::on_exit(AppState::Title).with_system(cleanup))
        .add_system_set(SystemSet::on_enter(AppState::Success).with_system(setup_success))
        .add_system_set(SystemSet::on_update(AppState::Success).with_system(success))
        .add_system_set(SystemSet::on_exit(AppState::Success).with_system(cleanup))
        .add_system_set(SystemSet::on_enter(AppState::Failed).with_system(setup_failed))
        .add_system_set(SystemSet::on_update(AppState::Failed).with_system(failed))
        .add_system_set(SystemSet::on_exit(AppState::Failed).with_system(cleanup))
        .add_system_set(SystemSet::on_enter(AppState::LevelTitle).with_system(setup_level_title))
        .add_system_set(SystemSet::on_update(AppState::LevelTitle).with_system(level_title))
        .add_system_set(SystemSet::on_exit(AppState::LevelTitle).with_system(cleanup))
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
            .with_system(setup_game)
            .with_system(spawn_drone)
            .with_system(spawn_level_1)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(drone_movement)
                .with_system(detect_collisions)
        )
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(cleanup))
        .run();
}