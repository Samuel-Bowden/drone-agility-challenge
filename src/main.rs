use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;
use serde::{Deserialize};
use std::fs::File;
use std::io::BufReader;

mod game;
mod cleanup;
mod menus;
mod levels;

#[derive(Deserialize)]
pub struct KeyMap {
    up: KeyCode,
    down: KeyCode,
    anti_cw: KeyCode,
    cw: KeyCode,
}

pub struct CurrentLevel(u32);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    LevelMenu,
    FailedMenu,
    SuccessMenu,
    EndMenu,
    Game,
}

fn main() {
    let keymap_file = File::open("keymap.ron").expect("Couldn't open the required keymap file, 'keymap.ron'");
    let keymap_reader = BufReader::new(keymap_file);
    let keymap: KeyMap = ron::de::from_reader(keymap_reader).expect("Error when parsing 'keymap.ron', make sure the structure of the file is correct and valid keymaps are used");

    App::new()
        .insert_resource(Gravity::from(Vec3::new(0.0, -180., 0.0)))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(CurrentLevel(1))
        .insert_resource(keymap)
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin::default())
        //Main Menu
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(menus::main::setup))
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(menus::main::click))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup::cleanup))
        //Success Menu
        .add_system_set(SystemSet::on_enter(AppState::SuccessMenu).with_system(menus::success::setup))
        .add_system_set(SystemSet::on_update(AppState::SuccessMenu).with_system(menus::success::click))
        .add_system_set(SystemSet::on_exit(AppState::SuccessMenu).with_system(cleanup::cleanup))
        //Failed Menu
        .add_system_set(SystemSet::on_enter(AppState::FailedMenu).with_system(menus::failed::setup))
        .add_system_set(SystemSet::on_update(AppState::FailedMenu).with_system(menus::failed::click))
        .add_system_set(SystemSet::on_exit(AppState::FailedMenu).with_system(cleanup::cleanup))
        //Level Menu
        .add_system_set(SystemSet::on_enter(AppState::LevelMenu).with_system(menus::level::setup))
        .add_system_set(SystemSet::on_update(AppState::LevelMenu).with_system(menus::level::click))
        .add_system_set(SystemSet::on_exit(AppState::LevelMenu).with_system(cleanup::cleanup))
        //End Menu
        .add_system_set(SystemSet::on_enter(AppState::EndMenu).with_system(menus::end::setup))
        .add_system_set(SystemSet::on_update(AppState::EndMenu).with_system(menus::end::click))
        .add_system_set(SystemSet::on_exit(AppState::EndMenu).with_system(cleanup::cleanup))
        //Game
        .add_system_set(
            SystemSet::on_enter(AppState::Game)
            .with_system(game::setup_game)
            .with_system(levels::spawn_level)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(game::drone_movement)
                .with_system(game::drone_rockets)
                .with_system(game::detect_collisions)
        )
        .add_system_set(SystemSet::on_exit(AppState::Game).with_system(cleanup::cleanup))
        .run();
}