use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn line(
    commands: &mut Commands,
    dimensions: (f32, f32),
    transform: (f32, f32),
    offset: (f32, f32),
    fill_color: Color,
) -> Entity {
    let shape = shapes::Rectangle {
        extents: Vec2::new(dimensions.0, dimensions.1),
        origin: shapes::RectangleOrigin::Center,
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform {
                    translation: Vec3::new(transform.0 + offset.0, transform.1 + offset.1, 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Fill::color(fill_color),
            Stroke::new(Color::BLACK, 0.),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(dimensions.0 / 2., dimensions.1 / 2.))
        .id()
}

pub fn shape(
    commands: &mut Commands,
    points: &Vec<Vec2>,
    transform: (f32, f32),
    offset: (f32, f32),
    fill_color: Color,
) -> Entity {
    let shape = shapes::Polygon {
        points: points.clone(),
        closed: true,
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform: Transform {
                    translation: Vec3::new(transform.0 + offset.0, transform.1 + offset.1, 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Fill::color(fill_color),
            Stroke::new(Color::BLACK, 0.),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::convex_hull(&points).unwrap())
        .id()
}
