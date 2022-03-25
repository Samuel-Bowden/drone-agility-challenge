use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct Drone;

#[derive(Component)]
pub struct Camera;

pub struct SpawnTimer{
	pub t: Timer,
}

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
                    translation: Vec3::new(-300., 200., 0.),
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
                    fill_mode: FillMode::color(Color::GREEN),
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
                    fill_mode: FillMode::color(Color::GREEN),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(-500., 500., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 500., 0.),
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
                    fill_mode: FillMode::color(Color::GREEN),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(500., 500., 0.),
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
                    fill_mode: FillMode::color(Color::GREEN),
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
}