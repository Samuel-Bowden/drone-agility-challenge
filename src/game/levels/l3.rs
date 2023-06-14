use super::{builder_func::*, Level};
use crate::podium::*;
use bevy::prelude::*;

pub fn new() -> Level {
    Level {
        description: "The slide.",
        offset: (-10000., 10000.),
        spawn,
        unlocked: false,
    }
}

fn spawn(commands: &mut Commands, offset: (f32, f32)) {
    let start = line(commands, (80., 10.), (0., 0.), offset, Color::RED); // Start
    commands.entity(start).insert(Podium::Start);

    // Start Box
    line(commands, (400., 10.), (0., -10.), offset, Color::WHITE); // Bottom
    line(commands, (10., 400.), (-195., 195.), offset, Color::WHITE); // Left
    line(commands, (10., 320.), (205., 245.), offset, Color::WHITE); // Right
    line(commands, (400., 10.), (0., 400.), offset, Color::WHITE); // Top

    // Dip Part A
    let points = vec![
        Vec2::new(-1500., 310.),
        Vec2::new(-2500., 1000.),
        Vec2::new(-2500., 990.),
        Vec2::new(-1500., 300.),
    ];
    shape(commands, &points, (2700., -905.), offset, Color::WHITE); // Top
    shape(commands, &points, (2700., -1005.), offset, Color::WHITE); // Bottom

    // Dip Part B
    let points = vec![
        Vec2::new(-500., 10.),
        Vec2::new(-1500., 310.),
        Vec2::new(-1500., 300.),
        Vec2::new(-500., 0.),
    ];
    shape(commands, &points, (2700., -905.), offset, Color::WHITE); // Top
    shape(commands, &points, (2700., -1005.), offset, Color::WHITE); // Bottom

    // Dip Part C
    let points = vec![
        Vec2::new(500., 0.),
        Vec2::new(500., 10.),
        Vec2::new(-500., 10.),
        Vec2::new(-500., 0.),
    ];
    shape(commands, &points, (2700., -905.), offset, Color::WHITE); // Top
    shape(commands, &points, (2700., -1005.), offset, Color::WHITE); // Bottom

    // Dip Part D
    let points = vec![
        Vec2::new(500., 0.),
        Vec2::new(1500., 300.),
        Vec2::new(1500., 310.),
        Vec2::new(500., 10.),
    ];
    shape(commands, &points, (2700., -905.), offset, Color::WHITE); // Top
    shape(commands, &points, (2700., -1005.), offset, Color::WHITE); // Bottom

    // Finish Box
    line(commands, (400., 10.), (4400., -700.), offset, Color::WHITE); // Bottom
    line(commands, (10., 310.), (4195., -450.), offset, Color::WHITE); // Left
    line(commands, (10., 410.), (4600., -500.), offset, Color::WHITE); // Right
    line(commands, (400., 10.), (4400., -300.), offset, Color::WHITE); // Top

    let finish = line(commands, (80., 10.), (4400., -690.), offset, Color::GREEN); // Finish
    commands.entity(finish).insert(Podium::Finish);
}
