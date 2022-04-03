use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::game::{Podium, PodiumType};

pub fn description() -> &'static str {
    "The slide."
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
        extents: Vec2::new(400., 10.),
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
            half_extends: Vec3::new(200., 5., 0.),
            border_radius: None,
        });

    let start_box_top = shapes::Rectangle {
        extents: Vec2::new(400., 10.),
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
                    translation: Vec3::new(0., 400., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(200., 5., 0.),
            border_radius: None,
        });

    let start_box_left = shapes::Rectangle {
        extents: Vec2::new(10., 400.),
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
                    translation: Vec3::new(-195., 195., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 200., 0.),
            border_radius: None,
        });

    let start_box_right = shapes::Rectangle {
        extents: Vec2::new(10., 320.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &start_box_right,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(205., 245., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 160., 0.),
            border_radius: None,
        });

    //Dip Part A
    let dip_part_a = shapes::Polygon {
        points: vec![
            Vec2::new(-1500., 310.),
            Vec2::new(-2500., 1000.),
            Vec2::new(-2500., 990.),
            Vec2::new(-1500., 300.),
        ],
        closed: true,
    };

    //Top
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &dip_part_a,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::WHITE, 0.),
                },
                Transform {
                    translation: Vec3::new(2700., -905., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(-1500., 310., 0.),
                Vec3::new(-2500., 1000., 0.),
                Vec3::new(-2500., 990., 0.),
                Vec3::new(-1500., 300., 0.),
            ],
            border_radius: None,
        })
        .insert(RigidBody::Static);

    //Bottom
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &dip_part_a,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::WHITE, 0.),
                },
                Transform {
                    translation: Vec3::new(2700., -1005., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(-1500., 310., 0.),
                Vec3::new(-2500., 1000., 0.),
                Vec3::new(-2500., 990., 0.),
                Vec3::new(-1500., 300., 0.),
            ],
            border_radius: None,
        })
        .insert(RigidBody::Static);


    //Dip Part B
    let dip_part_b = shapes::Polygon {
        points: vec![
            Vec2::new(-500., 10.),
            Vec2::new(-1500., 310.),
            Vec2::new(-1500., 300.),
            Vec2::new(-500., 0.),
        ],
        closed: true,
    };

    //Top
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &dip_part_b,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::WHITE, 0.),
                },
                Transform {
                    translation: Vec3::new(2700., -905., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(-500., 10., 0.),
                Vec3::new(-1500., 310., 0.),
                Vec3::new(-1500., 300., 0.),
                Vec3::new(-500., 0., 0.),
            ],
            border_radius: None,
        })
        .insert(RigidBody::Static);

    //Bottom
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &dip_part_b,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::WHITE, 0.),
                },
                Transform {
                    translation: Vec3::new(2700., -1005., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(-500., 10., 0.),
                Vec3::new(-1500., 310., 0.),
                Vec3::new(-1500., 300., 0.),
                Vec3::new(-500., 0., 0.),
            ],
            border_radius: None,
        })
        .insert(RigidBody::Static);


    //Dip Part C
    let dip_part_c = shapes::Polygon {
        points: vec![
            Vec2::new(500., 0.),
            Vec2::new(500., 10.),
            Vec2::new(-500., 10.),
            Vec2::new(-500., 0.),
        ],
        closed: true,
    };

    //Top
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &dip_part_c,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::WHITE, 0.),
                },
                Transform {
                    translation: Vec3::new(2700., -905., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(500., 0., 0.),
                Vec3::new(500., 10., 0.),
                Vec3::new(-500., 10., 0.),
                Vec3::new(-500., 0., 0.),
            ],
            border_radius: None,
        })
        .insert(RigidBody::Static);

    //Bottom
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &dip_part_c,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::WHITE, 0.),
                },
                Transform {
                    translation: Vec3::new(2700., -1005., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(500., 0., 0.),
                Vec3::new(500., 10., 0.),
                Vec3::new(-500., 10., 0.),
                Vec3::new(-500., 0., 0.),
            ],
            border_radius: None,
        })
        .insert(RigidBody::Static);


    //Dip Part D
    let dip_part_d = shapes::Polygon {
        points: vec![
            Vec2::new(500., 0.),
            Vec2::new(1500., 300.),
            Vec2::new(1500., 310.),
            Vec2::new(500., 10.),
        ],
        closed: true,
    };

    //Top
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &dip_part_d,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::WHITE, 0.),
                },
                Transform {
                    translation: Vec3::new(2700., -905., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(500., 0., 0.),
                Vec3::new(1500., 300., 0.),
                Vec3::new(1500., 310., 0.),
                Vec3::new(500., 10., 0.),
            ],
            border_radius: None,
        })
        .insert(RigidBody::Static);

    //Bottom
    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &dip_part_d,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::WHITE, 0.),
                },
                Transform {
                    translation: Vec3::new(2700., -1005., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(500., 0., 0.),
                Vec3::new(1500., 300., 0.),
                Vec3::new(1500., 310., 0.),
                Vec3::new(500., 10., 0.),
            ],
            border_radius: None,
        })
        .insert(RigidBody::Static);

    let finish_box_bottom = shapes::Rectangle {
        extents: Vec2::new(400., 10.),
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
                    translation: Vec3::new(4400., -700., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(200., 5., 0.),
            border_radius: None,
        });

    let finish_box_top = shapes::Rectangle {
        extents: Vec2::new(400., 10.),
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
                    translation: Vec3::new(4400., -300., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(200., 5., 0.),
            border_radius: None,
        });

    let finish_box_right = shapes::Rectangle {
        extents: Vec2::new(10., 410.),
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
                    translation: Vec3::new(4600., -500., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 200., 0.),
            border_radius: None,
        });

    let finish_box_left = shapes::Rectangle {
        extents: Vec2::new(10., 310.),
        origin: shapes::RectangleOrigin::Center
    };

    commands
        .spawn()
        .insert_bundle(
            GeometryBuilder::build_as(
                &finish_box_left,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::WHITE),
                    outline_mode: StrokeMode::new(Color::BLACK, 0.),
                },
                Transform {
                    translation: Vec3::new(4195., -450., 0.),
                    ..Default::default()
                }
            )
        )
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 160., 0.),
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
                    translation: Vec3::new(4400., -690., 0.),
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