use super::{line, shape};
use crate::game::{Podium, PodiumType};
use bevy::prelude::*;

pub const DESC: &'static str = "The slide.";

pub fn spawn(mut commands: Commands) {
    let start = line(&mut commands, (80., 10.), (0., 0.), Color::RED); // Start
    commands.entity(start).insert(Podium(PodiumType::Start));

    // Start Box
    line(&mut commands, (400., 10.), (0., -10.), Color::WHITE); // Bottom
    line(&mut commands, (10., 400.), (-195., 195.), Color::WHITE); // Left
    line(&mut commands, (10., 320.), (205., 245.), Color::WHITE); // Right
    line(&mut commands, (400., 10.), (0., 400.), Color::WHITE); // Top

    // Dip Part A
    let points = vec![
        Vec2::new(-1500., 310.),
        Vec2::new(-2500., 1000.),
        Vec2::new(-2500., 990.),
        Vec2::new(-1500., 300.),
    ];
    shape(&mut commands, &points, (2700., -905.), Color::WHITE); // Top
    shape(&mut commands, &points, (2700., -1005.), Color::WHITE); // Bottom

    // Dip Part B
    let points = vec![
        Vec2::new(-500., 10.),
        Vec2::new(-1500., 310.),
        Vec2::new(-1500., 300.),
        Vec2::new(-500., 0.),
    ];
    shape(&mut commands, &points, (2700., -905.), Color::WHITE); // Top
    shape(&mut commands, &points, (2700., -1005.), Color::WHITE); // Bottom

    // Dip Part C
    let points = vec![
        Vec2::new(500., 0.),
        Vec2::new(500., 10.),
        Vec2::new(-500., 10.),
        Vec2::new(-500., 0.),
    ];
    shape(&mut commands, &points, (2700., -905.), Color::WHITE); // Top
    shape(&mut commands, &points, (2700., -1005.), Color::WHITE); // Bottom

    // Dip Part D
    let points = vec![
        Vec2::new(500., 0.),
        Vec2::new(1500., 300.),
        Vec2::new(1500., 310.),
        Vec2::new(500., 10.),
    ];
    shape(&mut commands, &points, (2700., -905.), Color::WHITE); // Top
    shape(&mut commands, &points, (2700., -1005.), Color::WHITE); // Bottom

    // Finish Box
    line(&mut commands, (400., 10.), (4400., -700.), Color::WHITE); // Bottom
    line(&mut commands, (10., 310.), (4195., -450.), Color::WHITE); // Left
    line(&mut commands, (10., 410.), (4600., -500.), Color::WHITE); // Right
    line(&mut commands, (400., 10.), (4400., -300.), Color::WHITE); // Top

    let finish = line(&mut commands, (80., 10.), (4400., -690.), Color::GREEN); // Finish
    commands.entity(finish).insert(Podium(PodiumType::Finish));
}
