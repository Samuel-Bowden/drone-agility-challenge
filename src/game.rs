use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::AppState;

#[derive(Component)]
pub struct Drone;

#[derive(Component)]
pub struct Camera;

pub enum PodiumType {
    Start,
    Finish,
}

#[derive(Component)]
pub struct Podium(pub PodiumType);

pub fn setup_game(
    mut commands: Commands,
) {
    commands.spawn().insert_bundle(OrthographicCameraBundle::new_2d()).insert(Camera);
    commands.spawn().insert_bundle(UiCameraBundle::default());
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

pub fn spawn_drone(
    mut commands: Commands,
) {
    let drone = shapes::Rectangle {
        extents: Vec2::new(60., 20.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &drone,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::CYAN),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(0., 10., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(30., 10., 0.),
            border_radius: None,
        })
        .insert(Acceleration::from_linear(Vec3::ZERO))
        .insert(Velocity::from_linear(Vec3::ZERO))
        .insert(Drone);
}

pub fn detect_collisions(
    mut events: EventReader<CollisionEvent>,
    mut state: ResMut<State<AppState>>,
    drones: Query<&Drone>,
    podiums: Query<&Podium>,
) {
    for event in events.iter().filter(|e| e.is_started()) {
        let (e1, e2) = event.rigid_body_entities();

        //Fail condition
        if (drones.get_component::<Drone>(e1).is_ok() && !podiums.get_component::<Podium>(e2).is_ok())
            || (drones.get_component::<Drone>(e2).is_ok() && !podiums.get_component::<Podium>(e1).is_ok()) {
            state.set(AppState::FailedMenu).unwrap();
        }

        //Win condition
        if drones.get_component::<Drone>(e1).is_ok() {
            if let Ok(podium) = podiums.get_component::<Podium>(e2) {
                if let PodiumType::Finish = podium.0 {
                    state.set(AppState::SuccessMenu).unwrap();
                }
            }
        }

        //Win condition
        if drones.get_component::<Drone>(e2).is_ok() {
            if let Ok(podium) = podiums.get_component::<Podium>(e1) {
                if let PodiumType::Finish = podium.0 {
                    state.set(AppState::SuccessMenu).unwrap();
                }
            }
        }
    }
}