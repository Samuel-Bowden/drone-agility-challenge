use super::line;
use crate::game::{Podium, PodiumType};
use bevy::prelude::*;

pub const DESC: &'static str = "An easy level to start. Move from the first tower to the other.";

pub fn spawn(mut commands: Commands) {
    line(&mut commands, (1000., 10.), (350., -15.), Color::WHITE); // Floor
    line(&mut commands, (10., 1000.), (-145., 485.), Color::WHITE); // Left Wall
    line(&mut commands, (10., 1000.), (845., 485.), Color::WHITE); // Right Wall
    line(&mut commands, (1000., 10.), (350., 985.), Color::WHITE); // Ceiling

    line(&mut commands, (10., 400.), (-50., 190.), Color::GRAY); // Tower 1 Left Wall
    line(&mut commands, (10., 400.), (50., 190.), Color::GRAY); // Tower 1 Right Wall

    line(&mut commands, (10., 400.), (650., 190.), Color::GRAY); // Tower 2 Left Wall
    line(&mut commands, (10., 400.), (750., 190.), Color::GRAY); // Tower 2 Right Wall

    let start = line(&mut commands, (80., 10.), (0., -5.), Color::RED); // Start
    commands.entity(start).insert(Podium(PodiumType::Start));

    let finish = line(&mut commands, (80., 10.), (700., -5.), Color::GREEN); // Finish
    commands.entity(finish).insert(Podium(PodiumType::Finish));
}
