use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::game::{Podium, PodiumType};

pub fn description() -> &'static str {
    "A little bit harder now, navigate down the drop."
}

pub fn spawn(
    mut commands: Commands,
) {
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
                Transform::default(),
            )
        )
        .insert(RigidBody::Static)
        .insert(Podium(PodiumType::Start))
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(40., 5., 0.),
            border_radius: None,
        });


    let start_box_bottom = shapes::Rectangle {
        extents: Vec2::new(140., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &start_box_bottom,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(0., -10., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(70., 5., 0.),
            border_radius: None,
        });


    let start_box_left = shapes::Rectangle {
        extents: Vec2::new(10., 140.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &start_box_left,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(-65., 65., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 70., 0.),
            border_radius: None,
        });


    let start_box_top = shapes::Rectangle {
        extents: Vec2::new(340., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &start_box_top,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(100., 140., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(170., 5., 0.),
            border_radius: None,
        });

    let drop_right = shapes::Rectangle {
        extents: Vec2::new(10., 2000.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &drop_right,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(265., -860., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 1000., 0.),
            border_radius: None,
        });

    let drop_left = shapes::Rectangle {
        extents: Vec2::new(10., 2000.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &drop_left,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(70., -1005., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 1000., 0.),
            border_radius: None,
        });

    let obstacle_1_left = shapes::Rectangle {
        extents: Vec2::new(40., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &obstacle_1_left,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(95., -200., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20., 5., 0.),
            border_radius: None,
        });

    let obstacle_1_right = shapes::Rectangle {
        extents: Vec2::new(40., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &obstacle_1_right,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(240., -200., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(20., 5., 0.),
            border_radius: None,
        });

    let obstacle_2 = shapes::Rectangle {
        extents: Vec2::new(80., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &obstacle_2,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(115., -350., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(40., 5., 0.),
            border_radius: None,
        });

    let obstacle_3 = shapes::Rectangle {
        extents: Vec2::new(80., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &obstacle_3,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(220., -550., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(40., 5., 0.),
            border_radius: None,
        });

    let obstacle_4_left = shapes::Rectangle {
        extents: Vec2::new(60., 80.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &obstacle_4_left,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(105., -900., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(30., 40., 0.),
            border_radius: None,
        });

    let obstacle_4_right = shapes::Rectangle {
        extents: Vec2::new(60., 80.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &obstacle_4_right,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(230., -900., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(30., 40., 0.),
            border_radius: None,
        });

    let finish_box_bottom = shapes::Rectangle {
        extents: Vec2::new(340., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &finish_box_bottom,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(235., -2005., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(170., 5., 0.),
            border_radius: None,
        });

    let finish_box_top = shapes::Rectangle {
        extents: Vec2::new(140., 10.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &finish_box_top,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(330., -1865., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(70., 5., 0.),
            border_radius: None,
        });

    let finish_box_right = shapes::Rectangle {
        extents: Vec2::new(10., 140.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &finish_box_right,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(400., -1930., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 70., 0.),
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
                    translation: Vec3::new(330., -1995., 0.),
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