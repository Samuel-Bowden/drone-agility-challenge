use super::line;
use crate::podium::*;
use bevy::prelude::*;

pub const DESC: &'static str = "A little bit harder now, navigate down the drop.";

pub fn spawn(commands: &mut Commands, offset: (f32, f32)) {
    let start = line(commands, (80., 10.), (0., 0.), offset, Color::RED); // Start
    commands.entity(start).insert(Podium::Start);

    // Start Box
    line(commands, (140., 10.), (0., -10.), offset, Color::WHITE); // Bottom
    line(commands, (10., 140.), (-65., 65.), offset, Color::WHITE); // Left
    line(commands, (340., 10.), (100., 140.), offset, Color::WHITE); // Top

    // Drop
    line(commands, (10., 2000.), (70., -1005.), offset, Color::WHITE); // Left
    line(commands, (10., 2000.), (265., -860.), offset, Color::WHITE); // Right

    // Obstacle 1
    line(commands, (40., 10.), (95., -200.), offset, Color::GRAY); // Left
    line(commands, (40., 10.), (240., -200.), offset, Color::GRAY); // Right

    // Obstacle 2
    line(commands, (80., 10.), (115., -350.), offset, Color::GRAY);

    // Obstacle 3
    line(commands, (80., 10.), (220., -550.), offset, Color::GRAY);

    // Obstacle 4
    line(commands, (60., 80.), (105., -900.), offset, Color::GRAY); // Left
    line(commands, (60., 80.), (230., -900.), offset, Color::GRAY); // Right

    // Finish Box
    line(commands, (340., 10.), (235., -2005.), offset, Color::WHITE); // Bottom
    line(commands, (140., 10.), (330., -1865.), offset, Color::WHITE); // Top
    line(commands, (10., 140.), (400., -1930.), offset, Color::WHITE); // Right

    let finish = line(commands, (80., 10.), (330., -1995.), offset, Color::GREEN); // Finish
    commands.entity(finish).insert(Podium::Finish);
}
