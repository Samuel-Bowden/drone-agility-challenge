use crate::{keymap::KeyMap, podium::*, AppState, CurrentLevel};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
struct DroneMaterials {
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
pub struct Drone;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app.add_system(spawn.on_startup())
            .add_system(teleport_to_level_start.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    movement,
                    detect_collisions,
                    camera_tracking.after(movement),
                    rockets,
                )
                    .in_set(OnUpdate(AppState::Game)),
            )
            .add_system(deactivate_rockets.in_schedule(OnExit(AppState::Game)));
    }
}

fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mat = DroneMaterials {
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
}

fn teleport_to_level_start(
    current_level: Res<CurrentLevel>,
    mut drone: Query<(&mut Transform, &mut Velocity, &mut ExternalForce), With<Drone>>,
) {
    let (mut transform, mut velocity, mut external_force) = drone.single_mut();

    match current_level.0 {
        1 => transform.translation = Vec3::new(-10000., -9980., 0.),
        2 => transform.translation = Vec3::new(10000., 10020., 0.),
        3 => transform.translation = Vec3::new(-10000., 10020., 0.),
        _ => panic!("Invalid Level"),
    }

    transform.rotation = Quat::IDENTITY;

    *velocity = Velocity {
        linvel: Vec2::ZERO,
        angvel: 0.,
    };

    *external_force = ExternalForce {
        force: Vec2::ZERO,
        torque: 0.,
    };
}

fn deactivate_rockets(
    mut drone: Query<(&mut ExternalForce, &mut Handle<Image>), With<Drone>>,
    materials: Res<DroneMaterials>,
) {
    let (mut external_force, mut handle) = drone.single_mut();

    *external_force = ExternalForce {
        force: Vec2::ZERO,
        torque: 0.,
    };

    *handle = materials.drone.clone();
}

fn movement(
    input: Res<Input<KeyCode>>,
    keymap: Res<KeyMap>,
    mut drone: Query<(&mut ExternalForce, &Transform), With<Drone>>,
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

fn detect_collisions(
    mut events: EventReader<CollisionEvent>,
    mut state: ResMut<NextState<AppState>>,
    drones: Query<Entity, With<Drone>>,
    podiums: Query<(Entity, &Podium)>,
) {
    for event in events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let Ok(drone) = drones.get_single() else { break; };

            if drone == *e1 {
                match podiums.iter().find_map(|x| (x.0 == *e2).then_some(x.1)) {
                    Some(Podium::Start) => (),
                    Some(Podium::Finish) => state.set(AppState::SuccessMenu),
                    None => state.set(AppState::FailedMenu),
                }
            }
        }
    }
}

fn rockets(
    input: Res<Input<KeyCode>>,
    keymap: Res<KeyMap>,
    mut drone: Query<&mut Handle<Image>, With<Drone>>,
    materials: Res<DroneMaterials>,
) {
    if input.pressed(keymap.up) && input.pressed(keymap.cw) {
        let mut handle = drone.single_mut();
        *handle = materials.drone_blsr.clone();
    } else if input.pressed(keymap.up) && input.pressed(keymap.anti_cw) {
        let mut handle = drone.single_mut();
        *handle = materials.drone_blrs.clone();
    } else if input.pressed(keymap.down) && input.pressed(keymap.cw) {
        let mut handle = drone.single_mut();
        *handle = materials.drone_tlrs.clone();
    } else if input.pressed(keymap.down) && input.pressed(keymap.anti_cw) {
        let mut handle = drone.single_mut();
        *handle = materials.drone_tlsr.clone();
    } else if input.pressed(keymap.up) {
        let mut handle = drone.single_mut();
        *handle = materials.drone_blr.clone();
    } else if input.pressed(keymap.down) {
        let mut handle = drone.single_mut();
        *handle = materials.drone_tlr.clone();
    } else if input.pressed(keymap.cw) {
        let mut handle = drone.single_mut();
        *handle = materials.drone_tr_bl.clone();
    } else if input.pressed(keymap.anti_cw) {
        let mut handle = drone.single_mut();
        *handle = materials.drone_tl_br.clone();
    } else {
        let mut handle = drone.single_mut();
        *handle = materials.drone.clone();
    }
}

fn camera_tracking(
    drone: Query<&Transform, (With<Drone>, Without<Camera>)>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Drone>)>,
) {
    let mut camera = cameras.single_mut();
    let drone = drone.single();

    camera.translation.x = drone.translation.x;
    camera.translation.y = drone.translation.y;
}
