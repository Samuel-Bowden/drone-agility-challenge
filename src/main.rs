#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

mod cleanup;
mod game;
mod level;
mod menu;

#[derive(Resource)]
pub struct KeyMap {
    up: KeyCode,
    down: KeyCode,
    anti_cw: KeyCode,
    cw: KeyCode,
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

#[derive(Resource)]
pub struct CurrentLevel(u32);

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

#[derive(Component)]
pub struct Camera;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(CurrentLevel(1))
        .insert_resource(KeyMap::default())
        .add_state::<AppState>()
        .add_system(setup.on_startup())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(menu::Config)
        .add_plugin(game::Config)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(Camera);
}
