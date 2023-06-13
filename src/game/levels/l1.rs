use super::line;
use crate::podium::*;
use bevy::prelude::*;

pub const DESC: &'static str = "An easy level to start. Move from the first tower to the other.";

pub fn spawn(commands: &mut Commands, offset: (f32, f32)) {
    line(commands, (1000., 10.), (350., -15.), offset, Color::WHITE); // Floor
    line(commands, (10., 1000.), (-145., 485.), offset, Color::WHITE); // Left Wall
    line(commands, (10., 1000.), (845., 485.), offset, Color::WHITE); // Right Wall
    line(commands, (1000., 10.), (350., 985.), offset, Color::WHITE); // Ceiling

    line(commands, (10., 400.), (-50., 190.), offset, Color::GRAY); // Tower 1 Left Wall
    line(commands, (10., 400.), (50., 190.), offset, Color::GRAY); // Tower 1 Right Wall

    line(commands, (10., 400.), (650., 190.), offset, Color::GRAY); // Tower 2 Left Wall
    line(commands, (10., 400.), (750., 190.), offset, Color::GRAY); // Tower 2 Right Wall

    let start = line(commands, (80., 10.), (0., -5.), offset, Color::RED); // Start
    commands.entity(start).insert(Podium::Start);

    let finish = line(commands, (80., 10.), (700., -5.), offset, Color::GREEN); // Finish
    commands.entity(finish).insert(Podium::Finish);
}
