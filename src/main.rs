#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use keymap::KeyMap;

mod cleanup;
mod game;
mod keymap;
mod menu;
mod podium;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    LevelMenu,
    FailedMenu,
    SuccessMenu,
    EndMenu,
    Game,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(KeyMap::default())
        .add_system(spawn_camera.on_startup())
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(menu::Config)
        .add_plugin(game::Config)
        .run();
}

#[derive(Component)]
pub struct Camera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(Camera);
}
