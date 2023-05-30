#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use game::Drone;
use level::spawn_levels;

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

#[derive(Resource)]
pub struct Materials {
    drone: Handle<Image>,
    drone_blr: Handle<Image>,
    drone_tlr: Handle<Image>,
    drone_tr_bl: Handle<Image>,
    drone_tl_br: Handle<Image>,
    drone_blsr: Handle<Image>,
    drone_blrs: Handle<Image>,
    drone_tlsr: Handle<Image>,
    drone_tlrs: Handle<Image>,
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default()).insert(Camera);

    let mat = Materials {
        drone: asset_server.load("sprites/drone.png"),
        drone_blr: asset_server.load("sprites/drone_blr.png"),
        drone_tlr: asset_server.load("sprites/drone_tlr.png"),
        drone_tr_bl: asset_server.load("sprites/drone_tr_bl.png"),
        drone_tl_br: asset_server.load("sprites/drone_tl_br.png"),
        drone_blsr: asset_server.load("sprites/drone_blsr.png"),
        drone_blrs: asset_server.load("sprites/drone_blrs.png"),
        drone_tlsr: asset_server.load("sprites/drone_tlsr.png"),
        drone_tlrs: asset_server.load("sprites/drone_tlrs.png"),
    };

    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-10000., -9980., 0.0),
                ..Default::default()
            },
            texture: mat.drone.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.,
        })
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.,
        })
        .insert(Collider::cuboid(30., 14.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Drone);

    commands.insert_resource(mat);

    spawn_levels(commands);
}
