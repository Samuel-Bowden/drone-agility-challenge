use crate::{cleanup::CleanUp, CurrentLevel};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

mod l1;
mod l2;
mod l3;

pub fn spawn_level(commands: Commands, current_level: Res<CurrentLevel>) {
    match current_level.0 {
        1 => l1::spawn(commands),
        2 => l2::spawn(commands),
        3 => l3::spawn(commands),
        _ => (),
    }
}

pub fn get_description(current_level: u32) -> &'static str {
    match current_level {
        1 => l1::DESC,
        2 => l2::DESC,
        3 => l3::DESC,
        _ => "Invalid Level",
    }
}

pub fn line(
    commands: &mut Commands,
    dimensions: (f32, f32),
    transform: (f32, f32),
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
                    translation: Vec3::new(transform.0, transform.1, 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Fill::color(fill_color),
            Stroke::new(Color::BLACK, 0.),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(dimensions.0 / 2., dimensions.1 / 2.))
        .insert(CleanUp)
        .id()
}

pub fn shape(
    commands: &mut Commands,
    points: &Vec<Vec2>,
    transform: (f32, f32),
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
                    translation: Vec3::new(transform.0, transform.1, 0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            Fill::color(fill_color),
            Stroke::new(Color::BLACK, 0.),
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::convex_hull(&points).unwrap())
        .insert(CleanUp)
        .id()
}
