use bevy::prelude::*;
use heron::prelude::*;
use crate::{AppState, CurrentLevel};

#[derive(Component)]
pub struct Drone;

#[derive(Component)]
pub struct Camera;

pub enum PodiumType {
    Start,
    Finish,
}

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

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn().insert_bundle(OrthographicCameraBundle::new_2d()).insert(Camera);
    commands.spawn().insert_bundle(UiCameraBundle::default());
    
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
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 20.0, 0.0),
                ..Default::default()
            },
            texture: mat.drone.clone(),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(30., 14., 0.),
            border_radius: None,
        })
        .insert(Acceleration::from_linear(Vec3::ZERO))
        .insert(Velocity::from_linear(Vec3::ZERO))
        .insert(Drone);

    commands.insert_resource(mat);
}

pub fn drone_movement(
    input: Res<Input<KeyCode>>,
    mut q: QuerySet<(
        QueryState<(&mut Acceleration, &Transform), With<Drone>>,
        QueryState<&mut Transform, With<Camera>>
    )>,
) {
    let mut rotation = 0;
    let mut thrust = 0;


    if input.pressed(KeyCode::D) {
        thrust += 1;
    }

    if input.pressed(KeyCode::S) {
        thrust -= 1;
    }

    if input.pressed(KeyCode::A) {
        rotation += 1;
    }

    if input.pressed(KeyCode::H) {
        rotation -= 1;
    }

    let mut x = 0.;
    let mut y = 0.;

    for (mut acceleration, transform) in q.q0().iter_mut() {
        acceleration.angular = AxisAngle::new(Vec3::Z, rotation as f32 * 4.);
        acceleration.linear = transform.rotation * (Vec3::Y * thrust as f32 * 400.);
        x = transform.translation.x;
        y = transform.translation.y;
    }

    for mut transform in q.q1().iter_mut() {
        transform.translation.x = x;
        transform.translation.y = y;
    } 
}

pub fn drone_rockets(
    input: Res<Input<KeyCode>>,
    mut drone: Query<&mut Handle<Image>, With<Drone>>,
    sprites: Res<Materials>,
) {
    if input.pressed(KeyCode::D) && input.pressed(KeyCode::H) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_blsr.clone();
    } else if input.pressed(KeyCode::D) && input.pressed(KeyCode::A) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_blrs.clone();
    } else if input.pressed(KeyCode::S) && input.pressed(KeyCode::H) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tlrs.clone();
    } else if input.pressed(KeyCode::S) && input.pressed(KeyCode::A) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tlsr.clone();
    } else if input.pressed(KeyCode::D) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_blr.clone();
    } else if input.pressed(KeyCode::S) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tlr.clone();
    } else if input.pressed(KeyCode::H) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tr_bl.clone();
    } else if input.pressed(KeyCode::A) {
        let mut handle = drone.single_mut();
        *handle = sprites.drone_tl_br.clone();
    } else {
        let mut handle = drone.single_mut();
        *handle = sprites.drone.clone();
    }
}

pub fn detect_collisions(
    mut events: EventReader<CollisionEvent>,
    mut state: ResMut<State<AppState>>,
    mut current_level: ResMut<CurrentLevel>,
    drones: Query<&Drone>,
    podiums: Query<&Podium>,
) {
    for event in events.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();

        //Fail condition
        if (drones.get_component::<Drone>(e1).is_ok() && !podiums.get_component::<Podium>(e2).is_ok())
            || (drones.get_component::<Drone>(e2).is_ok() && !podiums.get_component::<Podium>(e1).is_ok()) {
            state.set(AppState::FailedMenu).unwrap();
            break;
        }

        //Win Condition
        let podium = 
            if drones.get_component::<Drone>(e1).is_ok() {
                if let Ok(podium) = podiums.get_component::<Podium>(e2) { Some(podium) } else { None }
            } else if drones.get_component::<Drone>(e2).is_ok() {
                if let Ok(podium) = podiums.get_component::<Podium>(e1) { Some(podium) } else { None }
            } else {
                None
            };

        if let Some(p) = podium {
            if let PodiumType::Finish = p.0 {
                if current_level.0 < 3 {
                    current_level.0 += 1;
                    state.set(AppState::SuccessMenu).unwrap();
                    break;
                } else {
                    current_level.0 = 1;
                    state.set(AppState::EndMenu).unwrap();
                    break;
                }
            }
        }
    }
}