use crate::{
    cleanup::{cleanup, CleanUp},
    level::spawn_level,
    AppState, CurrentLevel, KeyMap,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Drone;

pub enum PodiumType {
    Start,
    Finish,
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
pub struct Podium(pub PodiumType);

pub struct Config;

impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_systems((setup_game, spawn_level).in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    drone_movement,
                    drone_rockets,
                    camera_tracking.after(drone_movement),
                    detect_collisions,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_system(cleanup.in_schedule(OnExit(AppState::Game)));
    }
}

pub fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                translation: Vec3::new(0.0, 20.0, 0.0),
                ..Default::default()
            },
            texture: mat.drone.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce {
            force: Vec2::new(0., 0.),
            torque: 0.,
        })
        .insert(Collider::cuboid(30., 14.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CleanUp)
        .insert(Drone);

    commands.insert_resource(mat);
}

pub fn drone_movement(
    input: Res<Input<KeyCode>>,
    keymap: Res<KeyMap>,
    mut drone: Query<(&mut ExternalForce, &Transform), With<Drone>>,
    //mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut rotation = 0.;
    let mut thrust = 0.;

    if input.pressed(keymap.up) {
        thrust += 1.;
    }

    if input.pressed(keymap.down) {
        thrust -= 1.;
    }

    if input.pressed(keymap.anti_cw) {
        rotation += 1.;
    }

    if input.pressed(keymap.cw) {
        rotation -= 1.;
    }

    let (mut external_force, transform) = drone.single_mut();
    let linear_acceleration = transform.rotation * (Vec3::Y * thrust * 400000.);
    external_force.force = Vec2::new(linear_acceleration.x, linear_acceleration.y);
    external_force.torque = rotation * 4000000.;
}

pub fn camera_tracking(
    drone: Query<&Transform, (With<Drone>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Drone>)>,
) {
    let mut camera = camera.single_mut();
    let drone = drone.single();

    camera.translation.x = drone.translation.x;
    camera.translation.y = drone.translation.y;
}

pub fn drone_rockets(
    input: Res<Input<KeyCode>>,
    keymap: Res<KeyMap>,
    mut drone: Query<&mut Handle<Image>, With<Drone>>,
    sprites: Res<Materials>,
) {
    if input.pressed(keymap.up) && input.pressed(keymap.cw) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_blsr.clone();
    } else if input.pressed(keymap.up) && input.pressed(keymap.anti_cw) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_blrs.clone();
    } else if input.pressed(keymap.down) && input.pressed(keymap.cw) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tlrs.clone();
    } else if input.pressed(keymap.down) && input.pressed(keymap.anti_cw) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tlsr.clone();
    } else if input.pressed(keymap.up) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_blr.clone();
    } else if input.pressed(keymap.down) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tlr.clone();
    } else if input.pressed(keymap.cw) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tr_bl.clone();
    } else if input.pressed(keymap.anti_cw) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tl_br.clone();
    } else {
        let mut handle = drone.single_mut();
        *handle = sprites.drone.clone();
    }
}

pub fn detect_collisions(
    mut events: EventReader<CollisionEvent>,
    mut state: ResMut<NextState<AppState>>,
    mut current_level: ResMut<CurrentLevel>,
    drones: Query<&Drone>,
    podiums: Query<&Podium>,
) {
    for event in events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            //Fail condition
            if (drones.get_component::<Drone>(*e1).is_ok()
                && !podiums.get_component::<Podium>(*e2).is_ok())
                || (drones.get_component::<Drone>(*e2).is_ok()
                    && !podiums.get_component::<Podium>(*e1).is_ok())
            {
                state.set(AppState::FailedMenu);
                break;
            }

            //Win Condition
            let podium = if drones.get_component::<Drone>(*e1).is_ok() {
                if let Ok(podium) = podiums.get_component::<Podium>(*e2) {
                    Some(podium)
                } else {
                    None
                }
            } else if drones.get_component::<Drone>(*e2).is_ok() {
                if let Ok(podium) = podiums.get_component::<Podium>(*e1) {
                    Some(podium)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(p) = podium {
                if let PodiumType::Finish = p.0 {
                    if current_level.0 < 3 {
                        current_level.0 += 1;
                        state.set(AppState::SuccessMenu);
                        break;
                    } else {
                        current_level.0 = 1;
                        state.set(AppState::EndMenu);
                        break;
                    }
                }
            }
        }
    }
}
