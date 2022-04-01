use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::game::{Podium, PodiumType};

pub fn description() -> &'static str {
    "An easy level to start. Move from the first tower to the other."
}

pub fn spawn(mut commands: Commands) {
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
                Transform {
                    translation: Vec3::new(350., -15., 0.),
                    ..Default::default()
                }
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
                    translation: Vec3::new(-145., 485., 0.),
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
                    translation: Vec3::new(-50., 190., 0.),
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
                    translation: Vec3::new(50., 190., 0.),
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
                    translation: Vec3::new(750., 190., 0.),
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
                    translation: Vec3::new(650., 190., 0.),
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
                    translation: Vec3::new(845., 485., 0.),
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
                    translation: Vec3::new(350., 985., 0.),
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
                    translation: Vec3::new(0., -5., 0.),
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
                    translation: Vec3::new(700., -5., 0.),
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