use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::AppState;

#[derive(Component)]
pub struct Drone;

#[derive(Component)]
pub struct Camera;

enum PodiumType {
    Start,
    Finish,
}

#[derive(Component)]
pub struct Podium(PodiumType);

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
                    translation: Vec3::new(-350., 25., 0.),
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

pub fn spawn_level_1(
    mut commands: Commands,
) {
    let floor = shapes::Rectangle {
        extents: Vec2::new(1000., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &floor,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform::default(),
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(500., 5., 0.),
            border_radius: None,
        });

    let left_wall = shapes::Rectangle {
        extents: Vec2::new(10., 1000.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &left_wall,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(-495., 500., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 500., 0.),
            border_radius: None,
        });

    let tower1_left_wall = shapes::Rectangle {
        extents: Vec2::new(10., 400.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &tower1_left_wall,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(-400., 205., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 200., 0.),
            border_radius: None,
        });

    let tower1_right_wall = shapes::Rectangle {
        extents: Vec2::new(10., 400.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &tower1_right_wall,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(-300., 205., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 200., 0.),
            border_radius: None,
        });

    let tower2_left_wall = shapes::Rectangle {
        extents: Vec2::new(10., 400.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &tower2_left_wall,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(400., 205., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 200., 0.),
            border_radius: None,
        });

    let tower2_right_wall = shapes::Rectangle {
        extents: Vec2::new(10., 400.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &tower2_right_wall,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(300., 205., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 200., 0.),
            border_radius: None,
        });

    let right_wall = shapes::Rectangle {
        extents: Vec2::new(10., 1000.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &right_wall,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(495., 500., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 500., 0.),
            border_radius: None,
        });

    let ceiling = shapes::Rectangle {
        extents: Vec2::new(1000., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &ceiling,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(0., 1000., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(500., 5., 0.),
            border_radius: None,
        });

    let start = shapes::Rectangle {
        extents: Vec2::new(80., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &start,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::RED),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(-350., 10., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(Podium(PodiumType::Start))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(40., 5., 0.),
            border_radius: None,
        });

    let finish = shapes::Rectangle {
        extents: Vec2::new(80., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &finish,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GREEN),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(350., 10., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(Podium(PodiumType::Finish))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(40., 5., 0.),
            border_radius: None,
        });
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
            state.set(AppState::Failed).unwrap();
        }

        //Win condition
        if drones.get_component::<Drone>(e1).is_ok() {
            if let Ok(podium) = podiums.get_component::<Podium>(e2) {
                if let PodiumType::Finish = podium.0 {
                    state.set(AppState::Success).unwrap();
                }
            }
        }

        //Win condition
        if drones.get_component::<Drone>(e2).is_ok() {
            if let Ok(podium) = podiums.get_component::<Podium>(e1) {
                if let PodiumType::Finish = podium.0 {
                    state.set(AppState::Success).unwrap();
                }
            }
        }
    }
}