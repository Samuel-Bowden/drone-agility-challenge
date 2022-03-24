use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
struct Drone;

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct PositionText {}

struct SpawnTimer(Timer);

struct CanonTimer(Timer);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    InGame,
}

struct MenuData {
    button_entity: Entity,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn main() {
    App::new()
        .insert_resource(Gravity::from(Vec3::new(0.0, -180., 0.0)))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(SpawnTimer(Timer::from_seconds(0.1, false)))
        .insert_resource(CanonTimer(Timer::from_seconds(0.5, false)))
        .add_state(AppState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu))
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
            .with_system(setup_game)
            .with_system(spawn_drone)
            .with_system(spawn_floor)
            .with_system(position_text),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(update_position_text)
                .with_system(drone_movement)
                .with_system(shoot_missiles),
        )
        .run();
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Start Level 1",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu(
    mut state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                state.set(AppState::InGame).unwrap();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn_recursive();
}

fn setup_game(
    mut commands: Commands,
) {
    commands.spawn().insert_bundle(OrthographicCameraBundle::new_2d()).insert(Camera);
    commands.spawn().insert_bundle(UiCameraBundle::default());
}

fn position_text(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::BLACK), 
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "x: _, y: _",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.,
                            color: Color::WHITE,
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(PositionText {} );
        });
}

fn update_position_text(
    mut q: Query<&mut Text, With<PositionText>>,
    x: Query<&Transform, With<Drone>>,
) {
    for mut text in q.iter_mut() {
        for transform in x.iter() {
            text.sections[0].value = format!(
                "x: {}, y {}",
                transform.translation.x.round() as i32,
                transform.translation.y.round() as i32,
            );
        }
    }
}

fn shoot_missiles(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    drone: Query<&Transform, With<Drone>>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        if input.pressed(KeyCode::Q) || input.pressed(KeyCode::R) {
            let mut y = 0.;
            let mut x = 30.;
            let mut x_vel = 1000.;

            if input.pressed(KeyCode::Q) {
                x = -x;
                x_vel = -x_vel;
            }

            for transform in drone.iter() {
                x += transform.translation.x;
                y += transform.translation.y;
            }

            let missle = shapes::Rectangle {
                extents: Vec2::new(10., 10.),
                origin: shapes::RectangleOrigin::Center
            };

            commands
                .spawn()
                .insert_bundle(
                    GeometryBuilder::build_as(
                        &missle,
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(Color::PINK),
                            outline_mode: StrokeMode::new(Color::BLACK, 0.),
                        },
                        Transform {
                            translation: Vec3::new(x, y, 0.),
                            ..Default::default()
                        }
                    )
                )
                .insert(RigidBody::Dynamic)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(5., 5., 0.),
                    border_radius: None,
                })
                .insert(Velocity::from_linear(Vec3::new(x_vel, 0., 0.)));
            timer.0.reset();
        }
    }
}

fn drone_movement(
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

fn spawn_drone(
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
                Transform::default(),
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

fn spawn_floor(
    mut commands: Commands,
) {
    let floor = shapes::Rectangle {
        extents: Vec2::new(1000., 3.),
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
            half_extends: Vec3::new(500., 1.5, 0.),
            border_radius: None,
        });
}