use super::line;
use crate::game::{Podium, PodiumType};
use bevy::prelude::*;

pub const DESC: &'static str = "A little bit harder now, navigate down the drop.";

pub fn spawn(mut commands: Commands) {
    let start = line(&mut commands, (80., 10.), (0., 0.), Color::RED); // Start
    commands.entity(start).insert(Podium(PodiumType::Start));

    // Start Box
    line(&mut commands, (140., 10.), (0., -10.), Color::WHITE); // Bottom
    line(&mut commands, (10., 140.), (-65., 65.), Color::WHITE); // Left
    line(&mut commands, (340., 10.), (100., 140.), Color::WHITE); // Top

    // Drop
    line(&mut commands, (10., 2000.), (70., -1005.), Color::WHITE); // Left
    line(&mut commands, (10., 2000.), (265., -860.), Color::WHITE); // Right

    // Obstacle 1
    line(&mut commands, (40., 10.), (95., -200.), Color::GRAY); // Left
    line(&mut commands, (40., 10.), (240., -200.), Color::GRAY); // Right

    // Obstacle 2
    line(&mut commands, (80., 10.), (115., -350.), Color::GRAY);

    // Obstacle 3
    line(&mut commands, (80., 10.), (220., -550.), Color::GRAY);

    // Obstacle 4
    line(&mut commands, (60., 80.), (105., -900.), Color::GRAY); // Left
    line(&mut commands, (60., 80.), (230., -900.), Color::GRAY); // Right

    // Finish Box
    line(&mut commands, (340., 10.), (235., -2005.), Color::WHITE); // Bottom
    line(&mut commands, (140., 10.), (330., -1865.), Color::WHITE); // Top
    line(&mut commands, (10., 140.), (400., -1930.), Color::WHITE); // Right

    let finish = line(&mut commands, (80., 10.), (330., -1995.), Color::GREEN); // Finish
    commands.entity(finish).insert(Podium(PodiumType::Finish));
}
